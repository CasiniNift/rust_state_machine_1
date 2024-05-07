// The support module helps bring in various types and traits. 
// The traits will be used to enhance our simple state machine.

// The two components of a block are the header and the extrinsic.
pub struct Block<Header, Extrinsic> {
	pub header: Header,
	pub extrinsics: Vec<Extrinsic>,
}

// The header has the block number.
pub struct Header<BlockNumber> {
	pub block_number: BlockNumber,
}

// The extrinsic has the caller and the cals it makes. 
pub struct Extrinsic<Caller, Call> {
	pub caller: Caller,
	pub call: Call,
}

// Shows the reults of the calls to those specific functions. 
pub type DispatchResult = Result<(), &'static str>;


// This makes sure the calls are calling the right functions in the right pallets. 
pub trait Dispatch {
	type Caller;
	type Call;
	fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult;
}
