// The system pallet  stores all the metadata needed for your blockchain to function.
// For example, the current blocknumber or the nonce of users on your blockchain.
// A nonce is a value that exists only once, like a transaction ID. 

use core::ops::AddAssign;
use num::traits::{One, Zero};
use std::collections::BTreeMap; // Used to map user addresses to balances.


// Here you are making these types configurable in the future. 
pub trait Config {
	type AccountId: Ord + Clone;
	type BlockNumber: Zero + One + AddAssign + Copy;
	type Nonce: Zero + One + Copy;
}
// This is the System Pallet.
// It handles low level state needed for your blockchain.
#[derive(Debug)]
pub struct Pallet<T: Config> {               // "T: Config" is used to make the pallet configurable and scalable.  
    block_number: T::BlockNumber,            // The current block number.
    nonce: BTreeMap<T::AccountId, T::Nonce>, // A map from an account to their nonce
}

// Here you are implementing the Pallet and specifying you want it to be configurable
impl<T: Config> Pallet<T> {
	
	// Initiating a new instance. 
	pub fn new() -> Self {
		Self { block_number: T::BlockNumber::zero(), nonce: BTreeMap::new() }
	}

	// Fist block number. 
	pub fn block_number(&self) -> T::BlockNumber {
		self.block_number
	}
	
	// Increase in block numbers. 
	pub fn inc_block_number(&mut self) {
		self.block_number += T::BlockNumber::one();
	}

	// Increase nonce.
	pub fn inc_nonce(&mut self, who: &T::AccountId) {
		let nonce = *self.nonce.get(&who).unwrap_or(&T::Nonce::zero());
		let new_nonce = nonce + T::Nonce::one();
		self.nonce.insert(who.clone(), new_nonce);
	}
}

#[cfg(test)]
mod test {
	struct TestConfig;
	impl super::Config for TestConfig {
		type AccountId = String;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	#[test]
	fn init_system() {
		let mut system = super::Pallet::<TestConfig>::new(); // Instantiate the system pallet
		system.inc_block_number();  // Increment the block number.
		system.inc_nonce(&"alice".to_string()); // Increment nonce for 'alice'.

		assert_eq!(system.block_number(), 1); // Assert block number is incremented to 1
		assert_eq!(system.nonce.get(&"alice".to_string()), Some(&1)); // Assert nonce for 'alice' is correctly set and incremented to 1.
		assert_eq!(system.nonce.get(&"bob".to_string()), None); // Assert nonce for 'bob' is `None` since it has not been initialized.
	}
}


