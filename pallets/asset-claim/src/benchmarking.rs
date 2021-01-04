//! pallet asset-claim benchmarking.

#![cfg(feature = "runtime-benchmarks")]

use super::*;

use frame_system::RawOrigin;
use frame_benchmarking::{benchmarks, account, whitelisted_caller};
use sp_runtime::traits::Bounded;
use sp_std::vec;
use sp_std::boxed::Box;

use crate::Module as AssetClaim;

const SEED: u32 = 0;

benchmarks! {
    _ { }

    create_claim {
        let caller: T::AccountId = whitelisted_caller();
        let proof = b"hello world".to_vec();

    }: {
		AssetClaim::<T>::create_claim(RawOrigin::Signed(caller.clone()).into(), proof)?;
	}

	transfer_claim {
		let caller: T::AccountId = whitelisted_caller();
		let recipient: T::AccountId = account("recipient", 0, SEED);
		let proof = b"hello world".to_vec();
		
    }: {
		AssetClaim::<T>::create_claim(RawOrigin::Signed(caller.clone()).into(), proof.clone())?;
		AssetClaim::<T>::transfer_claim(RawOrigin::Signed(caller.clone()).into(), recipient, proof.clone())?;
	}

	revoke_claim {
        let caller: T::AccountId = whitelisted_caller();
        let proof = b"hello world".to_vec();

    }: {
		AssetClaim::<T>::create_claim(RawOrigin::Signed(caller.clone()).into(), proof.clone())?;
		AssetClaim::<T>::revoke_claim(RawOrigin::Signed(caller.clone()).into(), proof.clone())?;
	}

}

#[cfg(test)]
mod tests {
    use super::*;
	use crate::mock::{new_test_ext, Runtime};
    use frame_support::assert_ok;
    
	#[test]
	fn test_benchmarks() {
		new_test_ext().execute_with(|| {
			assert_ok!(test_benchmark_create_claim::<Runtime>());
			assert_ok!(test_benchmark_transfer_claim::<Runtime>());
			assert_ok!(test_benchmark_revoke_claim::<Runtime>());
		});
	}
}
