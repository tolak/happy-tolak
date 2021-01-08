// For better debugging (printout) support
use frame_support::{ debug, dispatch };
use frame_system::offchain;
use sp_runtime::{
	offchain::http,
	transaction_validity::{
	TransactionValidity, TransactionLongevity, ValidTransaction, InvalidTransaction
	},
};

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"tolak");

pub mod crypto {
  pub use super::KEY_TYPE;
  use sp_runtime::app_crypto::{app_crypto, sr25519};
  app_crypto!(sr25519, KEY_TYPE);
}

pub trait Config: pallet_timestamp::Config + frame_system::Config {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
	type Call: From<Call<Self>>;

	type SubmitSignedTransaction: offchain::SubmitSignedTransaction<Self, <Self as Config>::Call>;
	type SubmitUnsignedTransaction: offchain::SubmitUnsignedTransaction<Self, <Self as Config>::Call>;
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {

		pub fn onchain_callback(origin, _block: T::BlockNumber, input: Vec<u8>) -> dispatch::Result {
			let who = ensure_signed(origin)?;
			debug::info!("{:?}", core::str::from_utf8(&input).unwrap());
			Ok(())
		}

		// function to be called back in the next block import phase
		fn offchain_worker(block: T::BlockNumber) {
			let call = Call::onchain_callback(block, b"hello world!".to_vec());
			T::SubmitSignedTransaction::submit_signed(call);
			// submit unsigned transaction to chain
			// T::SubmitUnsignedTransaction::submit_unsigned(call);
		}
	}
}

// uncomment to allow unsigned transaction submit
// #[allow(deprecated)]
// impl<T: Config> frame_support::unsigned::ValidateUnsigned for Module<T> {
// 	type Call = Call<T>;

// 	fn validate_unsigned(call: &Self::Call) -> TransactionValidity {

// 	match call {
// 		Call::onchain_callback(block, input) => Ok(ValidTransaction {
// 			priority: 0,
// 			requires: vec![],
// 			provides: vec![(block, input).encode()],
// 			longevity: TransactionLongevity::max_value(),
// 			propagate: true,
// 		}),
// 		_ => InvalidTransaction::Call.into()
// 	}
// 	}
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
