#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::vec::Vec;
use sp_runtime::{DispatchResult, FixedPointOperand};

use frame_support::{
    decl_module, decl_storage, decl_event, decl_error, ensure, StorageMap,
    traits::{Currency, ExistenceRequirement, WithdrawReason},
    weights::{
		Weight, DispatchInfo, PostDispatchInfo, GetDispatchInfo, Pays, WeightToFeePolynomial,
		WeightToFeeCoefficient, DispatchClass,
	},
};
use frame_system::ensure_signed;

type BalanceOf<T> =
    <<T as Trait>::ReserveCurrency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
    type ReserveCurrency: Currency<Self::AccountId>;
	/// Convert a weight value into a deductible fee based on the currency type.
	type WeightToFee: WeightToFeePolynomial<Balance=BalanceOf<Self>>;
}

decl_storage! {
    trait Store for Module<T: Trait> as AssetClaimModule {
        /// It maps a asset proof to the user who made the claim and when they claim it.
        /// Here we use a Vec<u8> to storage a asset proof, this proof maybe just a hash
        /// or even a smart contract address of Ethereum.
        Proofs: map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, T::BlockNumber);
    }
}

decl_event! {
    pub enum Event<T> where 
        AccountId = <T as frame_system::Trait>::AccountId
    {
        /// Event emitted when a asset proof has been claimed. [who, claim]
        ClaimCreated(AccountId, Vec<u8>),
        /// Event emitted when a claim is revoked by the owner. [who, claim]
        ClaimRevoked(AccountId, Vec<u8>),
        /// Event emitted when a asset proof is transfered to another. [origin, who, claim]
        ClaimTransferd(AccountId, AccountId, Vec<u8>),
    }
}

decl_error! {
    pub enum Error for Module<T: Trait> {
        /// The asset has already been claimed.
        AssetAlreadyClaimed,
        /// The asset does not exist, so it cannot be revoked.
        NoAssetFound,
        /// The asset is claimed by another account, so caller can't revoke it.
        NotAssetOwner,
        /// Unhandled error.
        UnHandledError,
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        type Error = Error<T>;

        fn deposit_event() = default;

        /// Allow a user to claim ownership of an unclaimed proof.
        #[weight = 10_000]
        fn create_claim(origin, proof: Vec<u8>) {
            let sender = ensure_signed(origin)?;

            // Verify that the specified proof has not already been claimed.
            ensure!(!Proofs::<T>::contains_key(&proof), Error::<T>::AssetAlreadyClaimed);

            // Get the block number from the FRAME System module.
            let current_block = <frame_system::Module<T>>::block_number();

            // Store the proof with the sender and block number.
            Proofs::<T>::insert(&proof, (&sender, current_block));

            // Emit an event that the claim was created.
            Self::deposit_event(RawEvent::ClaimCreated(sender.clone(), proof));
        }

        /// Allow the owner to revoke their claim.
        #[weight = 10_000]
        fn revoke_claim(origin, proof: Vec<u8>) {
            let sender = ensure_signed(origin)?;

            // Verify that the specified proof has been claimed.
            ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoAssetFound);

            // Get owner of the claim.
            let (owner, _) = Proofs::<T>::get(&proof);

            // Verify that sender of the current call is the claim owner.
            ensure!(sender == owner, Error::<T>::NotAssetOwner);

            // Remove claim from storage.
            Proofs::<T>::remove(&proof);

            // Self::pay_fee(sender, Self::weight_to_fee(weight));

            // Emit an event that the claim was erased.
            Self::deposit_event(RawEvent::ClaimRevoked(sender.clone(), proof));
        }

        /// Allow the owner to transfer their claim to another.
        #[weight = 10_000]
        fn transfer_claim(origin, who: T::AccountId, proof: Vec<u8>) {
            let sender = ensure_signed(origin)?;

            // Verify that the specified proof has been claimed.
            ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoAssetFound);

            // Get owner of the claim.
            let (owner, _) = Proofs::<T>::get(&proof);

            // Verify that sender of the current call is the claim owner.
            ensure!(sender == owner, Error::<T>::NotAssetOwner);

            // Update owner of asset proof.
			Proofs::<T>::mutate(&proof, |(new_owner, _)| -> DispatchResult {
                *new_owner = who.clone();
				Ok(())
			})?;

            // Self::pay_fee(sender, Self::weight_to_fee(weight));

            // Emit an event that the claim was erased.
            Self::deposit_event(RawEvent::ClaimTransferd(sender.clone(), who, proof));
        }
    }
}

impl<T: Trait> Module<T> {

    fn weight_to_fee(weight: Weight) -> BalanceOf<T> {
		T::WeightToFee::calc(weight)
    }
    
    fn pay_fee(who: T::AccountId, amount: BalanceOf<T>) -> Result {
        let _ = T::ReserveCurrency::withdraw(
            &who,
            balance_amount,
            WithdrawReasons::Fee,
            ExistenceRequirement::KeepAlive,
        )
        .map_err(|_| Error::UnHandledError)?;
    
        Ok(())
    }
}

impl<T> Convert<Weight, BalanceOf<T>> for Module<T> where
	T: Config,
	BalanceOf<T>: FixedPointOperand,
{
	fn convert(weight: Weight) -> BalanceOf<T> {
		Self::weight_to_fee(weight)
	}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
