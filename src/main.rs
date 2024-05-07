// main.rs serves as the primary entry point for setting up the blockchain runtime.
// It links various modules, including pallets that form the components of the runtime.

mod balances; // Balance management for accounts and allows them to transfer.
mod proof_of_existence;
mod support; // Support types and traits used across the runtime.
mod system; // Core system functionality for the blockchain. // Pallet for managing proofs of data existence.
use crate::support::Dispatch; // Interface for dispatching calls.

// These are the concrete types we will use in our simple state machine.
// Modules are configured for these types directly, and they satisfy all of our
// trait requirements. We want everything to be generic and customizable.
mod types {
    pub type AccountId = String; // Identifies accounts uniquely within the system.
    pub type Balance = u128; // Supports high precision for account balances.
    pub type BlockNumber = u32; // Tracks the sequence of blocks.
    pub type Nonce = u32; // Nonce to ensure transaction uniqueness.
    pub type Extrinsic = crate::support::Extrinsic<AccountId, crate::RuntimeCall>; // A call a user makes. composed of a Call (the function we will execute) and a Caller (the account that wants to execute that function).
    pub type Header = crate::support::Header<BlockNumber>; // Contains metadata about the block which is used to verify that the block is valid ( block number, Parent Hash, State Root).
    pub type Block = crate::support::Block<Header, Extrinsic>; // Two parts: the header and a vector of extrinsics.
    pub type Content = &'static str; // Static reference to data content, used in proofs.
}


#[derive(Debug)]   // This macro enabels us to use the debug trait to better analyze runtime.
#[macros::runtime] // This is a macro used for runtime build up. 
pub struct Runtime {
    system: system::Pallet<Self>,     // This is the system pallet.
    balances: balances::Pallet<Self>, // This is the balances pallet.
    proof_of_existence: proof_of_existence::Pallet<Self>, // This is the PoE pallet.
}

// Implementing the system pallet in the runtime, makig it configurable and generic.
impl system::Config for Runtime {
    type AccountId = types::AccountId;
    type BlockNumber = types::BlockNumber;
    type Nonce = types::Nonce;
}

// Implementing the balances pallet in the runtime, makig it configurable and generic.
impl balances::Config for Runtime {
    type Balance = types::Balance;
}

// Implementing the PoE pallet in the runtime, makig it configurable and generic.
impl proof_of_existence::Config for Runtime {
    type Content = types::Content;
}


fn main() {
    // Main function to instantiate the runtime and execute blocks.
    let mut runtime = Runtime::new(); // Mutable runtime.
    let alice = "alice".to_string(); // Asigns wallet address to alice.
    let bob = "bob".to_string(); // Asigns wallet address to bob.
    let charlie = "charlie".to_string(); // Asigns wallet address to charlie.

    // Initializes the system.
    runtime.balances.set_balance(&alice, 100); // sets alice's balance to 100

    // Here are the extrinsics in our block.
    // You can add or remove these based on the modules and calls you have set up.
    // We also added the PoE pallet into the blocks 2 and 3.
    let block_1 = types::Block {
        header: support::Header { block_number: 1 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::balances(balances::Call::transfer {
                    to: bob.clone(),
                    amount: 20,
                }),
            },
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::balances(balances::Call::transfer {
                    to: charlie,
                    amount: 20,
                }),
            },
        ],
    };

    // Block 2 contains operations related to the Proof of Existence pallet.
    let block_2 = types::Block {
        header: support::Header { block_number: 2 },
        extrinsics: vec![
            // The first extrinsic involves Alice creating a claim on the string "Hello, world!".
            // This operation registers a proof of existence claim in the blockchain's state,
            // asserting that the content "Hello, world!" was claimed at block number 2 by Alice.
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim {
                    claim: &"Hello, world!",
                }),
            },
            // The second extrinsic similarly involves Bob attempting to create a claim on the
            // same string "Hello, world!". If Alice's claim was successfully registered,
            // Bob's claim should fail because the content is already claimed.
            support::Extrinsic {
                caller: bob.clone(),
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim {
                    claim: &"Hello, world!",
                }),
            },
        ],
    };

    // Block 3 handles subsequent operations in the Proof of Existence pallet.
    let block_3 = types::Block {
        header: support::Header { block_number: 3 },
        extrinsics: vec![
            // The first extrinsic in this block involves Alice revoking her claim on "Hello, world!".
            // This action, if successful, removes the claim from the state, indicating that the content
            // is no longer claimed by Alice. This could be useful for relinquishing rights or correcting
            // an erroneous claim.
            support::Extrinsic {
                caller: alice,
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::revoke_claim {
                    claim: &"Hello, world!",
                }),
            },
            // Following Alice's revocation, Bob attempts to create a claim again on "Hello, world!".
            // If Alice's revocation was successful, Bob should now be able to register the claim
            // under his name, effectively taking ownership of the proof of existence for this content.
            support::Extrinsic {
                caller: bob,
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim {
                    claim: &"Hello, world!",
                }),
            },
        ],
    };

    // These blocks demonstrate the functionality of the Proof of Existence pallet within a
    // blockchain context, highlighting how claims can be created and revoked dynamically,
    // and how the system handles conflicts and ownership changes.

    // Execute the extrinsics which make up our block.
    // If there are any errors, our system panics, since we should not execute invalid blocks.
    runtime.execute_block(block_1).expect("invalid block");
    runtime.execute_block(block_2).expect("invalid block");
    runtime.execute_block(block_3).expect("invalid block");

    // Simply print the debug format of our runtime state.
    println!("{:#?}", runtime);
}
