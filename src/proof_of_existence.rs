// The Proof of Existence Pallet uses the blockchain to provide a secure and immutable ledger that can be used
// to verify the existence of a particular document, file, or piece of data at a specific point in time.
use crate::support::DispatchResult;
use core::fmt::Debug;
use std::collections::BTreeMap;

pub trait Config: crate::system::Config {
	// The type which represents the content that can be claimed using this pallet.
	// Could be the content directly as bytes, or better yet the hash of that content.
	// We leave that decision to the runtime developer.
	type Content: Debug + Ord;
}

// This is the Proof of Existence Module.
// It is a simple module that allows accounts to claim existence of some data.
#[derive(Debug)]
pub struct Pallet<T: Config> {
	// A simple storage map from content to the owner of that content.
	// Accounts can make multiple different claims, but each claim can only have one owner.
	claims: BTreeMap<T::Content, T::AccountId>,
}

impl<T: Config> Pallet<T> {
	// Create a new instance of the Proof of Existence Module.
	pub fn new() -> Self {
		Self { claims: BTreeMap::new() }
	}

    // Get the owner (if any) of a claim.
	pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
		self.claims.get(&claim)
	}
}
#[macros::call] // This is the call macro. 
impl<T: Config> Pallet<T>{

	// Create a new claim on behalf of the `caller`.
	// This function will return an error if someone already has claimed that content.
	pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
		if self.claims.contains_key(&claim) {
			return Err(&"this content is already claimed");
		}
		self.claims.insert(claim, caller);
		Ok(())
	}

	// Revoke an existing claim on some content.
	// This function should only succeed if the caller is the owner of an existing claim.
	// It will return an error if the claim does not exist, or if the caller is not the owner.
	pub fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
		let owner = self.get_claim(&claim).ok_or("claim does not exist")?;
		if caller != *owner {
			return Err(&"this content is owned by someone else");
		}
		self.claims.remove(&claim);
		Ok(())
	}
}


// This module defines tests for the Proof of Existence pallet.
#[cfg(test)]
mod test {
    // Configuration for the tests using `TestConfig` which implements necessary traits.
    struct TestConfig;

    // Implement the `Config` trait for `TestConfig` to specify types for testing.
    impl super::Config for TestConfig {
        type Content = &'static str;  // Use static string slices for the content type.
    }

    // Implement the `system::Config` for `TestConfig` to specify additional system types.
    impl crate::system::Config for TestConfig {
        type AccountId = &'static str;  // Use static string slices for account IDs.
        type BlockNumber = u32;         // Define BlockNumber as an unsigned 32-bit integer.
        type Nonce = u32;               // Define Nonce as an unsigned 32-bit integer.
    }

    // Define a test case for basic proof of existence functionality.
    #[test]
    fn basic_proof_of_existence() {
        // Create a new instance of the Pallet with the test configuration.
        let mut poe = super::Pallet::<TestConfig>::new();

        // Verify that initially there is no claim for "Hello, world!".
        assert_eq!(poe.get_claim(&"Hello, world!"), None);

        // Create a claim for "Hello, world!" by "alice" and verify it succeeds.
        assert_eq!(poe.create_claim(&"alice", &"Hello, world!"), Ok(()));

        // Verify that "alice" is now the owner of the "Hello, world!" claim.
        assert_eq!(poe.get_claim(&"Hello, world!"), Some(&"alice"));

        // Attempt to create another claim for "Hello, world!" by "bob" and check for failure
        // because it is already claimed by "alice".
        assert_eq!(
            poe.create_claim(&"bob", &"Hello, world!"),
            Err("this content is already claimed")
        );

        // Revoke "alice"'s claim on "Hello, world!" and verify it succeeds.
        assert_eq!(poe.revoke_claim(&"alice", &"Hello, world!"), Ok(()));

        // Verify that "bob" can now claim "Hello, world!" successfully.
        assert_eq!(poe.create_claim(&"bob", &"Hello, world!"), Ok(()));
    }
}



