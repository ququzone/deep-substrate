use crate::{Event, Module, Trait};
use sp_core::H256;
use frame_support::{
	impl_outer_origin, impl_outer_event, parameter_types, weights::Weight,
};
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup}, testing::Header, Perbill,
};
use frame_system::{self as system, EventRecord, Phase};
use crate::*;

impl_outer_origin! {
	pub enum Origin for Test {}
}

pub(crate) type AccountId = u64;

// Configure a mock runtime to test the pallet.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Test;
parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: Weight = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
}

impl system::Trait for Test {
	type BaseCallFilter = ();
	type Origin = Origin;
	type Call = ();
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = TestEvent;
	type BlockHashCount = BlockHashCount;
	type MaximumBlockWeight = MaximumBlockWeight;
	type DbWeight = ();
	type BlockExecutionWeight = ();
	type ExtrinsicBaseWeight = ();
	type MaximumExtrinsicWeight = MaximumBlockWeight;
	type MaximumBlockLength = MaximumBlockLength;
	type AvailableBlockRatio = AvailableBlockRatio;
	type Version = ();
	type PalletInfo = ();
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
}

mod kitties_event {
	pub use crate::Event;
}

impl_outer_event! {
	pub enum TestEvent for Test {
		kitties_event<T>,
		system<T>,
	}
}

type Randomness = pallet_randomness_collective_flip::Module<Test>;

impl Trait for Test {
	type Event = TestEvent;
	type Randomness = Randomness;
	type KittyIndex = u32;
}

pub type System = frame_system::Module<Test>;
pub type KittiesModule = Module<Test>;

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}
