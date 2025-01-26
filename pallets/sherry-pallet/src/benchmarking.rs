//! Benchmarking for the Sherry Pallet

#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::{benchmarks, account, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
    // Benchmark for the `set_value` extrinsic
    set_value {
        let caller: T::AccountId = whitelisted_caller();
        let value: u32 = 100; // Example value to be set
    }: _(RawOrigin::Signed(caller.clone()), value)
    verify {
        assert_eq!(StoredValue::<T>::get(), Some(value));
    }

    // Benchmark for the `increment_value` extrinsic
    increment_value {
        let caller: T::AccountId = whitelisted_caller();
        let initial_value: u32 = 50;

        // Initialize storage with a value
        StoredValue::<T>::put(Some(initial_value));
    }: _(RawOrigin::Signed(caller.clone()))
    verify {
        let expected_value = 51; // Incremented value
        assert_eq!(StoredValue::<T>::get(), Some(expected_value));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use frame_benchmarking::impl_benchmark_test_suite;
    use crate::mock::{new_test_ext, Test};

    impl_benchmark_test_suite!(
        Pallet,
        new_test_ext(),
        Test
    );
}
