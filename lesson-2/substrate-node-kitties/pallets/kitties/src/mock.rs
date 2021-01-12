// use crate::{Event, Module, Trait};
// use sp_core::H256;
// use frame_support::{
// 	impl_outer_origin, impl_outer_event, parameter_types, weights::Weight,
// };
// use sp_runtime::{
// 	traits::{BlakeTwo256, IdentityLookup}, testing::Header, Perbill,
// };
// use frame_system::{self as system, EventRecord, Phase};
// use crate::*;
// use balances;

// use crate::*;
// use balances;
// use frame_support::{assert_ok, impl_outer_event, impl_outer_origin, parameter_types};
// use frame_system::{self as system};
// use sp_core::H256;
// use sp_io;
// use sp_runtime::{
// 	testing::Header,
// 	traits::{BlakeTwo256, IdentityLookup},
// 	Perbill,
// };


// impl_outer_origin! {
// 	pub enum Origin for TestRuntime {}
// }

// // Configure a mock runtime to test the pallet.
// #[derive(Clone, Eq, PartialEq, Debug)]
// pub struct TestRuntime;
// parameter_types! {
// 	pub const BlockHashCount: u64 = 250;
// 	pub const MaximumBlockWeight: u32 = 1024;
// 	pub const MaximumBlockLength: u32 = 2 * 1024;
// 	pub const AvailableBlockRatio: Perbill = Perbill::one();

// 	pub const ExistentialDeposit: u64 = 1;
// 	pub const TransferFee: u64 = 0;
// 	pub const CreationFee: u64 = 0;
// }

// impl system::Trait for TestRuntime {
// 	type BaseCallFilter = ();
// 	type Origin = Origin;
// 	type Index = u64;
// 	type Call = ();
// 	type BlockNumber = u64;
// 	type Hash = H256;
// 	type Hashing = BlakeTwo256;
// 	type AccountId = u64;
// 	type Lookup = IdentityLookup<Self::AccountId>;
// 	type Header = Header;
// 	type Event = TestEvent;
// 	type BlockHashCount = BlockHashCount;
// 	type MaximumBlockWeight = MaximumBlockWeight;
// 	type DbWeight = ();
// 	type BlockExecutionWeight = ();
// 	type ExtrinsicBaseWeight = ();
// 	type MaximumExtrinsicWeight = MaximumBlockWeight;
// 	type MaximumBlockLength = MaximumBlockLength;
// 	type AvailableBlockRatio = AvailableBlockRatio;
// 	type Version = ();
// 	type PalletInfo = ();
// 	type AccountData = balances::AccountData<u64>;
// 	type OnNewAccount = ();
// 	type OnKilledAccount = ();
// 	type SystemWeightInfo = ();
// }

// impl balances::Trait for TestRuntime {
// 	type Balance = u64;
// 	type MaxLocks = ();
// 	type Event = TestEvent;
// 	type DustRemoval = ();
// 	type ExistentialDeposit = ExistentialDeposit;
// 	type AccountStore = system::Module<TestRuntime>;
// 	type WeightInfo = ();
// }

// mod kitties_event {
// 	pub use crate::Event;
// }

// impl_outer_event! {
// 	pub enum TestEvent for TestRuntime {
// 		system<T>,
// 		kitties_event<T>,
// 		balances<T>,
// 	}
// }

// type Randomness = pallet_randomness_collective_flip::Module<TestRuntime>;

// impl Trait for TestRuntime {
// 	type Event = TestEvent;
// 	type Randomness = Randomness;
// 	type KittyIndex = u32;
// 	type Currency = balances::Module<Self>;
// }

// pub type System = frame_system::Module<TestRuntime>;
// pub type Balances = balances::Module<TestRuntime>;
// pub type KittiesModule = Module<TestRuntime>;

// // Build genesis storage according to the mock runtime.
// pub fn new_test_ext() -> sp_io::TestExternalities {
// 	//system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
// 	let mut t = system::GenesisConfig::default()
// 		.build_storage::<TestRuntime>()
// 		.unwrap();
// 	balances::GenesisConfig::<TestRuntime> {
// 		// Provide some initial balances
// 		balances: vec![(1, 10000), (2, 11000), (3, 12000), (4, 13000), (5, 14000)],
// 	}
// 	.assimilate_storage(&mut t)
// 	.unwrap();
// 	let mut ext: sp_io::TestExternalities = t.into();
// 	ext.execute_with(|| System::set_block_number(1));
// 	ext
// }

use crate::*;
use balances;
use frame_support::{assert_ok, impl_outer_event, impl_outer_origin, parameter_types};
use frame_system::{self as system};
use sp_core::H256;
use sp_io;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
	Perbill,
};

impl_outer_origin! {
	pub enum Origin for TestRuntime {}
}

// Workaround for https://github.com/rust-lang/rust/issues/26925 . Remove when sorted.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TestRuntime;
parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: u32 = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::one();

	pub const ExistentialDeposit: u64 = 1;
	pub const TransferFee: u64 = 0;
	pub const CreationFee: u64 = 0;
}

type Randomness = pallet_randomness_collective_flip::Module<TestRuntime>;

impl system::Trait for TestRuntime {
	type BaseCallFilter = ();
	type Origin = Origin;
	type Index = u64;
	type Call = ();
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
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
	type AccountData = balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
}

impl balances::Trait for TestRuntime {
	type Balance = u64;
	type MaxLocks = ();
	type Event = TestEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = system::Module<TestRuntime>;
	type WeightInfo = ();
}

mod kitties_event {
	pub use crate::Event;
}

impl_outer_event! {
	pub enum TestEvent for TestRuntime {
		system<T>,
		kitties_event<T>,
		balances<T>,
	}
}

impl Trait for TestRuntime {
	type Event = TestEvent;
	type Randomness = Randomness;
	type KittyIndex = u32;
	type Currency = balances::Module<Self>;
}

pub type System = system::Module<TestRuntime>;
pub type Balances = balances::Module<TestRuntime>;
pub type KittiesModule = Module<TestRuntime>;

// An alternative to `ExternalityBuilder` which includes custom configuration
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = system::GenesisConfig::default()
		.build_storage::<TestRuntime>()
		.unwrap();
	balances::GenesisConfig::<TestRuntime> {
		// Provide some initial balances
		balances: vec![(1, 10000), (2, 11000), (3, 12000), (4, 13000), (5, 14000)],
	}
	.assimilate_storage(&mut t)
	.unwrap();
	let mut ext: sp_io::TestExternalities = t.into();
	ext.execute_with(|| System::set_block_number(1));
	ext
}
