// The balance pallet manages the balances of users and allow them to transfer tokens to one another.

use num::traits::{CheckedAdd, CheckedSub, Zero}; // can import traits which define types which expose functions.
use std::collections::BTreeMap; // used to map user addresses to balances.

// Here you are making these types configurable in the future.
pub trait Config: crate::system::Config {
    type Balance: Zero + CheckedSub + CheckedAdd + Copy;
}

// The "pub struct" provides the entry point into the Pallet.
#[derive(Debug)]
pub struct Pallet<T: Config> {
    // "T: Config" is used to make the pallet configurable and scalable.
    balances: BTreeMap<T::AccountId, T::Balance>, // This is used to match account ID with their balances.
}

// Here you are implementing the Pallet and specifying you want it to be configurable.
impl<T: Config> Pallet<T> {
    // This function initializes the state.
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    // Set the balance of an account `who` to some `amount`.
    pub fn set_balance(&mut self, who: &T::AccountId, amount: T::Balance) {
        self.balances.insert(who.clone(), amount);
    }

    // Get the balance of an account `who`.
    // If the account has no stored balance, we return zero.
    pub fn balance(&self, who: &T::AccountId) -> T::Balance {
        *self.balances.get(who).unwrap_or(&T::Balance::zero())
    }
}
#[macros::call]
impl<T: Config> Pallet<T> {
    // Transfer `amount` from one account to another.
    // This function verifies that `from` has at least `amount` balance to transfer,
    // and that no mathematical overflows occur. We made everything generic and customizable.
    pub fn transfer(
        &mut self,
        caller: T::AccountId, // The account ID of the sender.
        to: T::AccountId,     // The account ID of the receiver.
        amount: T::Balance,   // The amount being sent to the receiver.
    ) -> crate::support::DispatchResult {
        let caller_balance = self.balance(&caller); // this is the balance of the caller
        let to_balance = self.balance(&to); // this is the balance of the receiver

        let new_caller_balance = caller_balance // this is the new caller balance
            .checked_sub(&amount) // this checks if the subtraction can actually happen
            .ok_or("Not enough funds.")?; // calls an error if there is one
        let new_to_balance = to_balance.checked_add(&amount).ok_or("Overflow")?;

        self.balances.insert(caller, new_caller_balance); // the new balane for the caller
        self.balances.insert(to, new_to_balance); // the new ba;ance for the receiver

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    struct TestConfig;

    impl crate::system::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    impl super::Config for TestConfig {
        type Balance = u128;
    }

    #[test]
    fn init_balances() {
        let mut balances = super::Pallet::<TestConfig>::new();

        assert_eq!(balances.balance(&"alice".to_string()), 0); // asserting that the balance of alice is 0 ( The assert_eq! macro in Rust is used for testing).
        balances.set_balance(&"alice".to_string(), 100); // setting the balance of alice to 100.
        assert_eq!(balances.balance(&"alice".to_string()), 100); // asserting that the balance of alice is 100.
        assert_eq!(balances.balance(&"bob".to_string()), 0); // aserting that the balance of bob is 0.
    }
    #[test]
    fn transfer_balance() {
        let mut balances = super::Pallet::<TestConfig>::new();

        assert_eq!(
            balances.transfer("alice".to_string(), "bob".to_string(), 51), // Transfering 51 from alice to bob.
            Err("Not enough funds.")
        );

        balances.set_balance(&"alice".to_string(), 100); // Sets the balance to 100.
        assert_eq!(
            balances.transfer("alice".to_string(), "bob".to_string(), 51), // This tests a successful transfer of 51 units from "alice" to "bob",
            Ok(()) // asserting that it returns Ok(()), indicating success.
        );
        assert_eq!(balances.balance(&"alice".to_string()), 49); // asserts that after the transfer, the balance of "alice" is reduced to 49.
        assert_eq!(balances.balance(&"bob".to_string()), 51); // asserts that after the transfer, the balance of "bob" is increased to 51.

        assert_eq!(
            balances.transfer("alice".to_string(), "bob".to_string(), 51), // Tests another transfer attempt from "alice" to "bob" with insufficient funds,
            Err("Not enough funds.") // expecting it to return an error.
        );
    }
}
