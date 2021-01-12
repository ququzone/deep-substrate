use crate::{Error, mock::*};
use frame_support::{
	assert_ok, assert_noop,
	traits::{OnFinalize, OnInitialize},
};
use super::*;

fn run_to_block(n: u64) {
	while System::block_number() < n {
		KittiesModule::on_finalize(System::block_number());
		System::on_finalize(System::block_number());
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
		KittiesModule::on_initialize(System::block_number());
	}
}

#[test]
fn create_works() {
    new_test_ext().execute_with(|| {
		run_to_block(2);
		assert_ok!(KittiesModule::create(Origin::signed(1)));
	});
}
