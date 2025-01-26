//! # Sherry Pallet
//! 
//! 
//! 1. Pallet Config
//! 2. Pallet Storage
//! 3. Pallet Event
//! 4. Pallet Error
//! 5. Pallet Calls

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    /// Storage item for the example pallet.
    #[pallet::storage]
    #[pallet::getter(fn stored_value)]
    pub type StoredValue<T> = StorageValue<_, Option<u32>, ValueQuery>;


    /// Events emitted by this pallet.
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A new value was set.
        ValueSet { value: u32, who: T::AccountId },
        /// The value was incremented.
        ValueIncremented { new_value: u32 },
    }

    /// Errors returned by this pallet.
    #[pallet::error]
    pub enum Error<T> {
        /// The storage value is uninitialized.
        NoValue,
        /// The value overflowed when incrementing.
        Overflow,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Set a new value in storage.
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn set_value(origin: OriginFor<T>, value: u32) -> DispatchResult {
            let who = ensure_signed(origin)?;
            // Wrap the value in `Some` since the storage type is `Option<u32>`
            StoredValue::<T>::put(Some(value));
            Self::deposit_event(Event::ValueSet { value, who });
            Ok(())
        }

        /// Increment the stored value.
#[pallet::call_index(1)]
#[pallet::weight(10_000)]
pub fn increment_value(origin: OriginFor<T>) -> DispatchResult {
    let _who = ensure_signed(origin)?;

    // Mutate the stored value
    StoredValue::<T>::try_mutate(|maybe_value| {
        match maybe_value {
            None => Err(Error::<T>::NoValue.into()), // Return an error if no value exists
            Some(v) => {
                // Increment the value safely, checking for overflow
                *v = v.checked_add(1).ok_or(Error::<T>::Overflow)?;
                Self::deposit_event(Event::ValueIncremented { new_value: *v });
                Ok(())
            }
        }
    })
}





    }
}
