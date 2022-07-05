#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_primes::Config { }
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		// type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/*
	#[pallet::storage]
	pub(super) type Factors<T: Config> = StorageValue<_, Vec<u64>, ValueQuery>;

	#[pallet::storage]
	pub(super) type Target<T: Config> = StorageValue<_, u64, OptionQuery>;

	#[pallet::storage]
	pub(super type WorkReady<T: Config>) = StorageValue<_, u64, ValueQuery>;

	#[pallet::storage]
	pub(super type Current<T: Config>) = StorageValue<_, u64, ValueQuery>;

	#[pallet::storage]
	pub(super) type Complete<T: Config> = StorageValue<_, bool, ValueQuery>;
	*/

	/*
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn offchain_worker(block_number: T::BlockNumber) {
			Self::start_working()
		}
	}*/

	impl<T: Config> Pallet<T> {
		/*
		fn start_working() {
			work_ready = Self::WorkReady<T>::get()
			if !work_ready {
				return
			}
			Self::<WorkReady<T>>::set(false)

			let target = Self::Target<T>::get());
			let p = find_prime_factor(Self::Target<T>::get());

			Self::Factors.append(p);

			if p == target {
				Self::Complete<T>>::set(true)
			}
		}

		fn find_prime_factor(working: u64) -> u64 {
			let mut p = 2;
			loop {
				if working % p == 0 {
					return p;
				}
				p = next_prime(p);
			}
		}

		fn next_prime(cur: u64) -> u64 {
			if cur < 2 {
				return 2;
			}
			if cur == 2 {
				return 3;
			}
			let mut p = 3;
			loop {
				if cur % p == 0 {
					return false;
				}
				p += 2;
				if p > cur {
					return p;
				}
			}
		}
		*/
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/*
		#[pallet:weight(0)]
		pub fn set_target(new_target: u64) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			match <Target<T>>::get() {
				None => _
				Some(_t) => {
					<Factors<T>>::kill()
					<Complete<T>>::set(false);
				}
				<Target<T>>::set(new_target);
				<WorkReady<T>>::set(true);
			}
		}

		#[pallet::weight(0)]
		pub fn submit_price_unsigned(
			origin: OriginFor<T>,
			_block_number: T::BlockNumber,
			price: u32,
		) -> DispatchResultWithPostInfo {
			// This ensures that the function can only be called via unsigned transaction.
			ensure_none(origin)?;
			// Add the price to the on-chain list, but mark it as coming from an empty address.
			Self::add_price(None, price);
			// now increment the block number at which we expect next unsigned transaction.
			let current_block = <system::Pallet<T>>::block_number();
			<NextUnsignedAt<T>>::put(current_block + T::UnsignedInterval::get());
			Ok(().into())
		}
		*/
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
			if let Call::submit_price_unsigned_with_signed_payload {
				price_payload: ref payload,
				ref signature,
			} = call
			{
				let signature_valid =
					SignedPayload::<T>::verify::<T::AuthorityId>(payload, signature.clone());
				if !signature_valid {
					return InvalidTransaction::BadProof.into()
				}
				Self::validate_transaction_parameters(&payload.block_number, &payload.price)
			} else if let Call::submit_price_unsigned { block_number, price: new_price } = call {
				Self::validate_transaction_parameters(block_number, new_price)
			} else {
				InvalidTransaction::Call.into()
			}
		}
	}
	*/
}
