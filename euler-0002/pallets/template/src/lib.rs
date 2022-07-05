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

	#[pallet::config]
	pub trait Config: frame_system::Config { }

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	pub(super) type A<T> = StorageValue<_, u64, ValueQuery>;

	#[pallet::type_value]
	pub(super) fn DefaultB<T: Config>() -> u64 { 1 }
	#[pallet::storage]
	pub(super) type B<T> = StorageValue<Value = u64, QueryKind = ValueQuery, OnEmpty = DefaultB<T>>;

	#[pallet::storage]
	pub type Limit<T> = StorageValue<_, u64>;

	#[pallet::storage]
	pub type Acc<T> = StorageValue<_, u64>;

	#[pallet::genesis_config]
	pub struct GenesisConfig {
		pub init_b: u64,
	}

	#[cfg(feature = "std")]
	impl Default for GenesisConfig {
		fn default() -> Self {
			Self {
				init_b: 1,
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig {
		fn build(&self) {
			B::<T>::put(self.init_b);
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(_n: T::BlockNumber) -> Weight {
			0
		}

		fn on_finalize(_n: T::BlockNumber) {
			let limit = Limit::<T>::get().unwrap_or(0);

			let b = B::<T>::get();
			let b_plus_a = b + A::<T>::get();
			if  b_plus_a >= limit {
				return
			}

			if b_plus_a % 2 == 0 {
				Acc::<T>::set(Some(Acc::<T>::get().unwrap_or(0) + b_plus_a))
			}

			A::<T>::set(b);
			B::<T>::set(b_plus_a)
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn set_limit(origin: OriginFor<T>, new_limit: u64) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			match <Limit<T>>::get() {
				Some(limit) if limit < new_limit => <Limit<T>>::set(Some(new_limit)),
				None => <Limit<T>>::set(Some(new_limit)),
			 	_ => ()
			}

			Ok(())
		}
	}

}
