#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>

use codec::{Decode, Encode};
use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
use frame_system::pallet_prelude::*;
use frame_support::traits::Get;
use frame_system::{
	self as system,
	offchain::{
		AppCrypto, CreateSignedTransaction, SendSignedTransaction, SendUnsignedTransaction,
		SignedPayload, Signer, SigningTypes, SubmitTransaction,
	},
};
use sp_core::crypto::KeyTypeId;
use sp_runtime::{
	offchain::{
		http,
		storage::{MutateStorageError, StorageRetrievalError, StorageValueRef},
		Duration,
	},
	traits::Zero,
	transaction_validity::{InvalidTransaction, TransactionValidity, ValidTransaction},
	RuntimeDebug,
};
use sp_std::vec::Vec;
pub use pallet::*;

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"demo");

pub mod crypto {
	use super::KEY_TYPE;
	use sp_core::sr25519::Signature as Sr25519Signature;
	use sp_runtime::{
		app_crypto::{app_crypto, sr25519},
		traits::Verify, MultiSignature, MultiSigner
	};
	app_crypto!(sr25519, KEY_TYPE);

	pub struct TestAuthId;

	// implemented for runtime
	impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for TestAuthId {
		type RuntimeAppPublic = Public;
		type GenericSignature = sp_core::sr25519::Signature;
		type GenericPublic = sp_core::sr25519::Public;
	}

	impl frame_system::offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature>
		for TestAuthId
	{
		type RuntimeAppPublic = Public;
		type GenericSignature = sp_core::sr25519::Signature;
		type GenericPublic = sp_core::sr25519::Public;
	}
}


//use sp_std::prelude::*;
//use sp_std::vec;

//#[cfg(test)]
//mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: CreateSignedTransaction<Call<Self>> + frame_system::Config {
		type AuthorityId: AppCrypto<Self::Public, Self::Signature>;

		type Call: From<Call<Self>>;

	}
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		// type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		// type AuthorityId: AppCrypto<Self::Public, Self::Signature>;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn primes)]
	pub(super) type Primes<T: Config> = StorageValue<_, Vec<u64>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn largest_prime)]
	pub(super) type LargestPrime<T: Config> = StorageValue<_, u64, ValueQuery>;

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn offchain_worker(block_number: T::BlockNumber) {
			let res = Self::find_prime_and_send_signed();
			if let  Err(e) = res {
				log::error!("Error: {}", e);
			}
		}
	}

	/*
	#[pallet::validate_unsigned]
	impl<T: Config> ValidateUnsigned for Pallet<T> {
		type Call = Call<T>;

		/// Validate unsigned call to this module.
		///
		/// By default unsigned transactions are disallowed, but implementing the validator
		/// here we make sure that some particular calls (the ones produced by offchain worker)
		/// are being whitelisted and marked as valid.
		fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
			// Firstly let's check that we call the right function.
			//Self::validate_transaction_parameters(block_number, new_price)
			ValidTransaction::with_tag_prefix("Primessss")
				.priority(T::UnsignedPriority::get().saturating_add(0))
				.longevity(5)
				.propagate(true)
				.build()
		}
	}
	*/

	impl<T: Config> Pallet<T> {
		fn find_next_prime() -> u64 {
			let largest_known: u64 = *<Primes<T>>::get().last().unwrap_or(&0);
			if largest_known < 2 {
				return 2;
			}

			if largest_known == 2 {
				return 3;
			}

			let mut candidate = largest_known + 2;

			loop {
				for prime in <Primes<T>>::get().into_iter() {
					if candidate % prime == 0 {
						candidate += 2;
						break;
					}
				}

				return candidate;
			}
		}

		/*
		fn find_prime_and_send_unsigned() -> Result<(), &'static str> {
			let prime = Self::find_next_prime();

			let call = Call::submit_prime_unsigned { prime };

			SubmitTransaction::<T, pallet::Call<T>>::submit_unsigned_transaction(call.into())
				.map_err(|()| "Unable to submit unsigned transaction :(")?;

			/*
			for (acc, res) in &results {
				match res {
					Ok(()) => log::info!("[{:?}] Submitted price of {} cents", acc.id, price),
					Err(e) => log::error!("[{:?}] Failed to submit transaction {:?}", acc.id, e),
				}
			}
			*/

			Ok(())
		}
		*/

		fn find_prime_and_send_signed() -> Result<(), &'static str> {
			let signer = Signer::<T, T::AuthorityId>::all_accounts();
			if !signer.can_sign() {
				return Err(
					"No local accounts :("
				)
			}
			log::info!("prime start");

			let prime = Self::find_next_prime();
			log::info!("prime {:?}", prime);

			let results = signer.send_signed_transaction(|_account| {
				Call::submit_prime_signed { prime }
			});


			/*
			for (acc, res) in &results {
				match res {
					Ok(()) => log::info!("[{:?}] Submitted price of {} cents", acc.id, price),
					Err(e) => log::error!("[{:?}] Failed to submit transaction {:?}", acc.id, e),
				}
			}
			*/

			Ok(())
		}

		fn add_prime(maybe_who: Option<T::AccountId>, prime: u64) {
			//log::info!("Adding prime: {}", prime);

			let largest_known = *<Primes<T>>::get().last().unwrap_or(&0);
			if prime <= largest_known {
				log::info!("Ignoring prime {:?}", prime);
				return;
			}

			log::info!("Added prime {:?}", prime);
			<Primes<T>>::mutate(|ps| {
				ps.push(prime)
			})
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn submit_prime_signed(
			origin: OriginFor<T>,
			prime: u64
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			Self::add_prime(Some(who), prime);
			Ok(().into())
		}

		/*
		#[pallet::weight(0)]
		pub fn submit_prime_unsigned(
			origin: OriginFor<T>,
			prime: u64
		) -> DispatchResultWithPostInfo {
			ensure_none(origin)?;
			Self::add_prime(None, prime);
			Ok(().into())
		}
		*/
	}

}
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, scale_info::TypeInfo)]
pub struct PrimePayload<Public, BlockNumber> {
	block_number: BlockNumber,
	prime: u64,
	public: Public,
}

impl<T: SigningTypes> SignedPayload<T> for PrimePayload<T::Public, T::BlockNumber> {
	fn public(&self) -> T::Public {
		self.public.clone()
	}
}
