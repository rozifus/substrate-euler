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
	use sp_std::vec::Vec;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_primes::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn target)]
	pub type Target<T> = StorageValue<_, u64, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn remaining)]
	pub type Remaining<T> = StorageValue<_, u64, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn factors)]
	pub(super) type Factors<T: Config> = StorageValue<_, Vec<u64>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn worker_running)]

	/*
	#[pallet::type_value]
	pub(super) fn DefaultNext<T: Config>() -> u32 { 1 }
	#[pallet::storage]
	pub(super) type Next<T: Config> = StorageValue<Value = u32, QueryKind = ValueQuery, OnEmpty = DefaultNext<T>>;

	#[pallet::type_value]
	pub(super) fn DefaultLimit<T: Config>() -> u32 { 0 }
	#[pallet::storage]
	pub(super) type Limit<T: Config> = StorageValue<Value = u32, QueryKind = ValueQuery, OnEmpty = DefaultLimit<T>>;

	#[pallet::genesis_config]
	pub struct GenesisConfig {
		pub init_next: u32,
		pub init_limit: u32,
	}*/

	/*
	#[cfg(feature = "std")]
	impl Default for GenesisConfig {
		fn default() -> Self {
			Self { init_next: Default::default(), init_limit: Default::default() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig {
		fn build(&self) {
			Next::<T>::put(self.init_next);
			Limit::<T>::put(self.init_limit);
		}
	}
	*/

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn offchain_worker(block_number: T::BlockNumber) {
			let res = Self::entry();
			if let  Err(e) = res {
				log::error!("Error: {}", e);
			}
		}
	}

	/*
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(_n: T::BlockNumber) -> Weight {
			0
		}

		fn on_finalize(_n: T::BlockNumber) {
			let nex = Next::<T>::get();
			let lim = Limit::<T>::get();

			if nex < lim {
				if nex % 3 == 0 || nex % 5 == 0 {
					FizzBuzz::<T>::set(FizzBuzz::<T>::get() + nex)
				}
				Next::<T>::set(nex + 1)
			}
		}
	}
	*/

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		WrongNext(u32, T::AccountId),
		NextOverflow(u32, T::AccountId),
	}

	/*
	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		AlreadyCrunched,
		NotFinished,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}
	*/

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.

	impl<T: Config> Pallet<T> {
		fn entry() -> Result<(), &'static str> {
			if target() < 2 {
				Ok(())
			}

			if remaining() == 1 {
				Ok(())
			}



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

	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn set_target(origin: OriginFor<T>, new_target: u64) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			let current_target = <Target<T>>::get();

			match new_target {
				nt if nt == current_target => (),
				nt if nt < 2 => (),
				nt =>
				{
					<Target<T>>::set(nt);
					<Remaining<T>>::set(nt);
					<Factors<T>>::mutate(|facs| {
						facs.clear()
					})
				},
			}

			Ok(())
		}

		/*
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn do_next(origin: OriginFor<T>, ne: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;

			match <Next<T>>::get() {
				i if i % 3 != 0 && i % 5 != 0 => (),
				i => <FizzBuzz<T>>::put(i + <FizzBuzz<T>>::get())
			}

/*
			match <FizzBuzz<T>>::get() {
				// Return an error if the value has not been set.
				None => Err(Error::<T>::NoneValue)?,
				Some(result) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}*/

			// Update storage.
			<FizzBuzz<T>>::put(1);

			// Emit an event.
			//Self::deposit_event(Event::SomethingStored(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
		*/

	}

}
