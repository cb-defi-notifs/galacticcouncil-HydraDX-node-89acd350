#![cfg(test)]

use crate::polkadot_test_net::*;
use frame_support::traits::LockIdentifier;
use frame_support::{
	assert_noop, assert_ok,
	dispatch::DispatchResult,
	traits::{Bounded, OnInitialize, StorePreimage},
};
use frame_system::RawOrigin;
use hydradx_runtime::{
	Balances, BlockNumber, Currencies, Democracy, Omnipool, Preimage, Scheduler, Staking, System, Tokens, Vesting,
};
use orml_traits::currency::MultiCurrency;
use orml_vesting::VestingSchedule;
use pallet_democracy::{AccountVote, Conviction, ReferendumIndex, ReferendumInfo, Vote};
use pretty_assertions::assert_eq;
use primitives::constants::time::DAYS;
use primitives::AccountId;
use sp_runtime::AccountId32;
use xcm_emulator::TestExt;

type CallOf<T> = <T as frame_system::Config>::RuntimeCall;
type BoundedCallOf<T> = Bounded<CallOf<T>>;
type Schedule = VestingSchedule<BlockNumber, Balance>;

fn vesting_schedule() -> Schedule {
	Schedule {
		start: 0,
		period: 1,
		period_count: 10,
		per_period: 10_000 * UNITS,
	}
}

fn set_balance_proposal(who: AccountId, value: u128) -> BoundedCallOf<hydradx_runtime::Runtime> {
	let inner = pallet_balances::Call::force_set_balance { who, new_free: value };
	let outer = hydradx_runtime::RuntimeCall::Balances(inner);
	Preimage::bound(outer).unwrap()
}

fn propose_set_balance(who: AccountId, dest: AccountId, value: u128) -> DispatchResult {
	Democracy::propose(
		hydradx_runtime::RuntimeOrigin::signed(who),
		set_balance_proposal(dest, value),
		100_000 * UNITS,
	)
}

fn begin_referendum() -> ReferendumIndex {
	assert_ok!(propose_set_balance(ALICE.into(), CHARLIE.into(), 2));
	fast_forward_to(3 * DAYS);
	0
}

fn end_referendum() {
	fast_forward_to(7 * DAYS);
}

fn fast_forward_by(n: u32) {
	for _ in 1..n {
		next_block();
	}
}

fn fast_forward_to(n: u32) {
	while System::block_number() < n {
		next_block();
	}
}
fn next_block() {
	System::set_block_number(System::block_number() + 1);
	Scheduler::on_initialize(System::block_number());
	Democracy::on_initialize(System::block_number());
}
const AYE: Vote = Vote {
	aye: true,
	conviction: Conviction::None,
};

fn aye(amount: u128) -> AccountVote<u128> {
	AccountVote::Standard {
		vote: AYE,
		balance: amount,
	}
}

fn aye6x(amount: u128) -> AccountVote<u128> {
	AccountVote::Standard {
		vote: Vote {
			aye: true,
			conviction: Conviction::Locked6x,
		},
		balance: amount,
	}
}

#[test]
fn staking_should_transfer_hdx_fees_to_pot_account_when_omnipool_trade_is_executed() {
	TestNet::reset();
	Hydra::execute_with(|| {
		init_omnipool();
		assert_ok!(Staking::initialize_staking(RawOrigin::Root.into()));

		assert_ok!(Tokens::set_balance(
			RawOrigin::Root.into(),
			CHARLIE.into(),
			DAI,
			20_000_000 * UNITS,
			0,
		));

		assert_ok!(Omnipool::sell(
			hydradx_runtime::RuntimeOrigin::signed(CHARLIE.into()),
			DAI,
			HDX,
			1_000_000_000_000_000_000,
			0u128,
		));

		let staking_account = pallet_staking::Pallet::<hydradx_runtime::Runtime>::pot_account_id();
		assert_eq!(Currencies::free_balance(HDX, &staking_account), 1_093_580_529_359);
	});
}

#[test]
fn democracy_vote_should_record_stake_vote() {
	TestNet::reset();
	Hydra::execute_with(|| {
		init_omnipool();
		assert_ok!(Staking::initialize_staking(RawOrigin::Root.into()));

		let staking_account = pallet_staking::Pallet::<hydradx_runtime::Runtime>::pot_account_id();
		assert_ok!(Currencies::update_balance(
			RawOrigin::Root.into(),
			staking_account,
			HDX,
			(10_000 * UNITS) as i128,
		));
		assert_ok!(Balances::force_set_balance(
			RawOrigin::Root.into(),
			ALICE.into(),
			1_000_000 * UNITS,
		));
		let r = begin_referendum();
		assert_ok!(Staking::stake(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			1_000 * UNITS
		));

		assert_ok!(Democracy::vote(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			r,
			aye(2 * UNITS)
		));

		let stake_position_id = pallet_staking::Pallet::<hydradx_runtime::Runtime>::get_user_position_id(
			&sp_runtime::AccountId32::from(ALICE),
		)
		.unwrap()
		.unwrap();
		let stake_voting = pallet_staking::Pallet::<hydradx_runtime::Runtime>::get_position_votes(stake_position_id);

		assert!(!stake_voting.votes.is_empty());
		let (ref_vote_idx, vote) = stake_voting.votes[0];
		assert_eq!(ref_vote_idx, 0);
		assert_eq!(
			vote,
			pallet_staking::types::Vote::new(2 * UNITS, pallet_staking::types::Conviction::None)
		);
		end_referendum();
	});
}

#[test]
fn staking_action_should_claim_points_for_finished_referendums_when_voted() {
	TestNet::reset();
	Hydra::execute_with(|| {
		init_omnipool();
		assert_ok!(Staking::initialize_staking(RawOrigin::Root.into()));

		let staking_account = pallet_staking::Pallet::<hydradx_runtime::Runtime>::pot_account_id();
		assert_ok!(Currencies::update_balance(
			RawOrigin::Root.into(),
			staking_account,
			HDX,
			(10_000 * UNITS) as i128,
		));
		assert_ok!(Balances::force_set_balance(
			RawOrigin::Root.into(),
			ALICE.into(),
			1_000_000 * UNITS,
		));
		assert_ok!(Staking::stake(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			1_000 * UNITS
		));

		let r = begin_referendum();

		assert_ok!(Democracy::vote(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			r,
			aye(1_000 * UNITS)
		));
		end_referendum();

		let alice_position_id = pallet_staking::Pallet::<hydradx_runtime::Runtime>::get_user_position_id(
			&sp_runtime::AccountId32::from(ALICE),
		)
		.unwrap()
		.unwrap();
		assert_ok!(Staking::increase_stake(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			alice_position_id,
			1_000 * UNITS
		));

		let stake_voting = pallet_staking::Pallet::<hydradx_runtime::Runtime>::get_position_votes(alice_position_id);
		let stake_position =
			pallet_staking::Pallet::<hydradx_runtime::Runtime>::get_position(alice_position_id).unwrap();

		assert_eq!(stake_position.get_action_points(), 1);
		assert!(stake_voting.votes.is_empty());
	});
}

#[test]
fn staking_should_transfer_rewards_when_claimed() {
	TestNet::reset();
	Hydra::execute_with(|| {
		init_omnipool();
		assert_ok!(Staking::initialize_staking(RawOrigin::Root.into()));

		let staking_account = pallet_staking::Pallet::<hydradx_runtime::Runtime>::pot_account_id();
		assert_ok!(Currencies::update_balance(
			RawOrigin::Root.into(),
			staking_account,
			HDX,
			(10_000 * UNITS) as i128,
		));
		assert_ok!(Balances::force_set_balance(
			RawOrigin::Root.into(),
			ALICE.into(),
			1_000_000 * UNITS,
		));
		assert_ok!(Staking::stake(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			1_000 * UNITS
		));

		let r = begin_referendum();

		assert_ok!(Democracy::vote(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			r,
			aye(1_000 * UNITS)
		));
		end_referendum();

		let alice_position_id = pallet_staking::Pallet::<hydradx_runtime::Runtime>::get_user_position_id(
			&sp_runtime::AccountId32::from(ALICE),
		)
		.unwrap()
		.unwrap();
		let alice_balance = Currencies::free_balance(HDX, &AccountId32::from(ALICE));

		assert_ok!(Staking::increase_stake(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			alice_position_id,
			1_000 * UNITS
		));

		assert_ok!(Democracy::remove_vote(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			r
		));

		assert_ok!(Staking::claim(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			alice_position_id
		));

		let alice_balance_after_claim = Currencies::free_balance(HDX, &AccountId32::from(ALICE));

		assert!(alice_balance_after_claim > alice_balance);

		let stake_voting = pallet_staking::Pallet::<hydradx_runtime::Runtime>::get_position_votes(alice_position_id);
		let stake_position =
			pallet_staking::Pallet::<hydradx_runtime::Runtime>::get_position(alice_position_id).unwrap();

		assert_eq!(stake_position.get_action_points(), 1);
		assert!(stake_voting.votes.is_empty());
	});
}

#[test]
fn staking_should_not_reward_when_double_claimed() {
	TestNet::reset();
	Hydra::execute_with(|| {
		init_omnipool();
		assert_ok!(Staking::initialize_staking(RawOrigin::Root.into()));

		let staking_account = pallet_staking::Pallet::<hydradx_runtime::Runtime>::pot_account_id();
		assert_ok!(Currencies::update_balance(
			RawOrigin::Root.into(),
			staking_account,
			HDX,
			(10_000 * UNITS) as i128,
		));
		assert_ok!(Balances::force_set_balance(
			RawOrigin::Root.into(),
			ALICE.into(),
			1_000_000 * UNITS,
		));
		assert_ok!(Staking::stake(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			1_000 * UNITS
		));

		let r = begin_referendum();

		assert_ok!(Democracy::vote(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			r,
			aye(2 * UNITS)
		));
		end_referendum();

		assert_ok!(Democracy::remove_vote(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			r
		));

		let alice_position_id = pallet_staking::Pallet::<hydradx_runtime::Runtime>::get_user_position_id(
			&sp_runtime::AccountId32::from(ALICE),
		)
		.unwrap()
		.unwrap();

		// first claim
		let alice_balance = Currencies::free_balance(HDX, &AccountId32::from(ALICE));
		assert_ok!(Staking::claim(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			alice_position_id
		));
		let alice_balance_after_claim = Currencies::free_balance(HDX, &AccountId32::from(ALICE));
		assert!(alice_balance_after_claim > alice_balance);
		// second claim
		let alice_balance = Currencies::free_balance(HDX, &AccountId32::from(ALICE));
		assert_ok!(Staking::claim(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			alice_position_id
		));
		let alice_balance_after_claim = Currencies::free_balance(HDX, &AccountId32::from(ALICE));
		assert_eq!(alice_balance, alice_balance_after_claim);
	});
}

#[test]
fn staking_should_not_reward_when_increase_stake_again_and_no_vote_activity() {
	TestNet::reset();
	Hydra::execute_with(|| {
		init_omnipool();
		assert_ok!(Staking::initialize_staking(RawOrigin::Root.into()));

		let staking_account = pallet_staking::Pallet::<hydradx_runtime::Runtime>::pot_account_id();
		assert_ok!(Currencies::update_balance(
			RawOrigin::Root.into(),
			staking_account,
			HDX,
			(10_000 * UNITS) as i128,
		));
		assert_ok!(Balances::force_set_balance(
			RawOrigin::Root.into(),
			ALICE.into(),
			1_000_000 * UNITS,
		));
		assert_ok!(Staking::stake(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			1_000 * UNITS
		));

		let r = begin_referendum();

		assert_ok!(Democracy::vote(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			r,
			aye(2 * UNITS)
		));
		end_referendum();

		let alice_position_id = pallet_staking::Pallet::<hydradx_runtime::Runtime>::get_user_position_id(
			&sp_runtime::AccountId32::from(ALICE),
		)
		.unwrap()
		.unwrap();
		assert_ok!(Staking::increase_stake(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			alice_position_id,
			1_000 * UNITS
		));

		assert_ok!(Democracy::remove_vote(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			r
		));

		// second increase
		let alice_balance = Currencies::free_balance(HDX, &AccountId32::from(ALICE));
		assert_ok!(Staking::increase_stake(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			alice_position_id,
			1_000 * UNITS
		));
		let alice_balance_after_claim = Currencies::free_balance(HDX, &AccountId32::from(ALICE));
		assert_eq!(alice_balance_after_claim, alice_balance);

		// claim
		let alice_balance = Currencies::free_balance(HDX, &AccountId32::from(ALICE));
		assert_ok!(Staking::claim(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			alice_position_id
		));
		let alice_balance_after_claim = Currencies::free_balance(HDX, &AccountId32::from(ALICE));
		assert_eq!(alice_balance, alice_balance_after_claim);
	});
}

#[test]
fn increase_should_slash_min_amount_when_increase_is_low() {
	TestNet::reset();
	Hydra::execute_with(|| {
		init_omnipool();
		assert_ok!(Staking::initialize_staking(RawOrigin::Root.into()));

		let staking_account = pallet_staking::Pallet::<hydradx_runtime::Runtime>::pot_account_id();
		assert_ok!(Currencies::update_balance(
			RawOrigin::Root.into(),
			staking_account,
			HDX,
			(10_000 * UNITS) as i128,
		));
		assert_ok!(Balances::force_set_balance(
			RawOrigin::Root.into(),
			ALICE.into(),
			1_000_000 * UNITS,
		));
		assert_ok!(Staking::stake(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			100_000 * UNITS
		));

		let r = begin_referendum();

		assert_ok!(Democracy::vote(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			r,
			aye6x(100_000 * UNITS)
		));
		end_referendum();

		assert_ok!(propose_set_balance(ALICE.into(), CHARLIE.into(), 2));
		fast_forward_to(10 * DAYS);

		assert_ok!(Democracy::vote(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			1,
			aye6x(100_000 * UNITS)
		));
		fast_forward_to(17 * DAYS);

		assert_ok!(Democracy::remove_vote(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			r
		));

		assert_ok!(Democracy::remove_vote(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			1
		));

		let alice_position_id = pallet_staking::Pallet::<hydradx_runtime::Runtime>::get_user_position_id(
			&sp_runtime::AccountId32::from(ALICE),
		)
		.unwrap()
		.unwrap();

		assert_ok!(Staking::increase_stake(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			alice_position_id,
			1_000 * UNITS
		));

		let stake_position =
			pallet_staking::Pallet::<hydradx_runtime::Runtime>::get_position(alice_position_id).unwrap();
		assert_eq!(stake_position.accumulated_slash_points, 50);
	});
}

#[test]
fn staking_should_claim_and_unreserve_rewards_when_unstaked() {
	TestNet::reset();
	Hydra::execute_with(|| {
		init_omnipool();
		assert_ok!(Staking::initialize_staking(RawOrigin::Root.into()));

		let staking_account = pallet_staking::Pallet::<hydradx_runtime::Runtime>::pot_account_id();
		assert_ok!(Currencies::update_balance(
			RawOrigin::Root.into(),
			staking_account,
			HDX,
			(10_000 * UNITS) as i128,
		));
		assert_ok!(Balances::force_set_balance(
			RawOrigin::Root.into(),
			ALICE.into(),
			1_000_000 * UNITS,
		));
		assert_ok!(Staking::stake(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			1_000 * UNITS
		));

		let r = begin_referendum();

		assert_ok!(Democracy::vote(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			r,
			aye(2 * UNITS)
		));
		end_referendum();

		let alice_position_id = pallet_staking::Pallet::<hydradx_runtime::Runtime>::get_user_position_id(
			&sp_runtime::AccountId32::from(ALICE),
		)
		.unwrap()
		.unwrap();
		let alice_balance = Currencies::free_balance(HDX, &AccountId32::from(ALICE));
		assert_ok!(Staking::increase_stake(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			alice_position_id,
			1_000 * UNITS
		));

		assert_ok!(Democracy::remove_vote(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			r
		));

		assert_ok!(Staking::unstake(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			alice_position_id
		));
		let alice_balance_after_claim = Currencies::free_balance(HDX, &AccountId32::from(ALICE));
		assert!(alice_balance_after_claim > alice_balance);
		assert_eq!(alice_balance_after_claim, 1000000001920799631);

		let stake_position_id = pallet_staking::Pallet::<hydradx_runtime::Runtime>::get_user_position_id(
			&sp_runtime::AccountId32::from(ALICE),
		)
		.unwrap();

		assert!(stake_position_id.is_none());
	});
}

#[test]
fn staking_should_remove_vote_when_democracy_removes_vote() {
	TestNet::reset();
	Hydra::execute_with(|| {
		init_omnipool();
		assert_ok!(Staking::initialize_staking(RawOrigin::Root.into()));

		let staking_account = pallet_staking::Pallet::<hydradx_runtime::Runtime>::pot_account_id();
		assert_ok!(Currencies::update_balance(
			RawOrigin::Root.into(),
			staking_account,
			HDX,
			(10_000 * UNITS) as i128,
		));
		assert_ok!(Balances::force_set_balance(
			RawOrigin::Root.into(),
			ALICE.into(),
			1_000_000 * UNITS,
		));
		let r = begin_referendum();
		assert_ok!(Staking::stake(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			1_000 * UNITS
		));

		assert_ok!(Democracy::vote(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			r,
			aye(2 * UNITS)
		));

		let stake_position_id = pallet_staking::Pallet::<hydradx_runtime::Runtime>::get_user_position_id(
			&sp_runtime::AccountId32::from(ALICE),
		)
		.unwrap()
		.unwrap();
		let stake_voting = pallet_staking::Pallet::<hydradx_runtime::Runtime>::get_position_votes(stake_position_id);
		assert!(!stake_voting.votes.is_empty());
		let (ref_vote_idx, vote) = stake_voting.votes[0];
		assert_eq!(ref_vote_idx, 0);
		assert_eq!(
			vote,
			pallet_staking::types::Vote::new(2 * UNITS, pallet_staking::types::Conviction::None)
		);

		assert_ok!(Democracy::remove_vote(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			r
		));
		let stake_voting = pallet_staking::Pallet::<hydradx_runtime::Runtime>::get_position_votes(stake_position_id);
		assert!(stake_voting.votes.is_empty());
		end_referendum();
	});
}

#[test]
fn staking_should_not_reward_when_refenrendum_is_ongoing() {
	TestNet::reset();
	Hydra::execute_with(|| {
		init_omnipool();
		assert_ok!(Staking::initialize_staking(RawOrigin::Root.into()));

		let staking_account = pallet_staking::Pallet::<hydradx_runtime::Runtime>::pot_account_id();
		assert_ok!(Currencies::update_balance(
			RawOrigin::Root.into(),
			staking_account,
			HDX,
			(10_000 * UNITS) as i128,
		));
		assert_ok!(Balances::force_set_balance(
			RawOrigin::Root.into(),
			ALICE.into(),
			1_000_000 * UNITS,
		));
		let r = begin_referendum();
		assert_ok!(Staking::stake(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			1_000 * UNITS
		));

		assert_ok!(Democracy::vote(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			r,
			aye(2 * UNITS)
		));

		let stake_position_id = pallet_staking::Pallet::<hydradx_runtime::Runtime>::get_user_position_id(
			&sp_runtime::AccountId32::from(ALICE),
		)
		.unwrap()
		.unwrap();
		let stake_voting = pallet_staking::Pallet::<hydradx_runtime::Runtime>::get_position_votes(stake_position_id);
		assert!(!stake_voting.votes.is_empty());
		let alice_balance = Currencies::free_balance(HDX, &AccountId32::from(ALICE));
		assert_ok!(Staking::claim(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			stake_position_id
		));
		let alice_balance_after_claim = Currencies::free_balance(HDX, &AccountId32::from(ALICE));
		assert_eq!(alice_balance, alice_balance_after_claim);
		end_referendum();
	});
}

#[test]
fn democracy_vote_should_work_correctly_when_account_has_no_stake() {
	TestNet::reset();
	Hydra::execute_with(|| {
		init_omnipool();
		assert_ok!(Staking::initialize_staking(RawOrigin::Root.into()));

		let staking_account = pallet_staking::Pallet::<hydradx_runtime::Runtime>::pot_account_id();
		assert_ok!(Currencies::update_balance(
			RawOrigin::Root.into(),
			staking_account,
			HDX,
			(10_000 * UNITS) as i128,
		));
		assert_ok!(Balances::force_set_balance(
			RawOrigin::Root.into(),
			ALICE.into(),
			1_000_000 * UNITS,
		));
		let r = begin_referendum();
		assert_ok!(Democracy::vote(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			r,
			aye(2 * UNITS)
		));
		end_referendum();
	});
}

#[test]
fn democracy_remote_vote_should_work_correctly_when_account_has_no_stake() {
	TestNet::reset();
	Hydra::execute_with(|| {
		System::set_block_number(0);
		init_omnipool();
		assert_ok!(Staking::initialize_staking(RawOrigin::Root.into()));

		let staking_account = pallet_staking::Pallet::<hydradx_runtime::Runtime>::pot_account_id();
		assert_ok!(Tokens::set_balance(
			RawOrigin::Root.into(),
			staking_account,
			HDX,
			10_000 * UNITS,
			0,
		));
		assert_ok!(Balances::force_set_balance(
			RawOrigin::Root.into(),
			ALICE.into(),
			1_000_000 * UNITS,
		));
		let r = begin_referendum();
		assert_ok!(Democracy::vote(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			r,
			aye(2 * UNITS)
		));
		assert_ok!(Democracy::remove_vote(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			r
		));
		end_referendum();
	});
}

#[test]
fn staking_position_transfer_should_fail_when_origin_is_owner() {
	TestNet::reset();
	Hydra::execute_with(|| {
		System::set_block_number(1);
		init_omnipool();
		assert_ok!(Staking::initialize_staking(RawOrigin::Root.into()));

		let staking_account = pallet_staking::Pallet::<hydradx_runtime::Runtime>::pot_account_id();
		assert_ok!(Currencies::update_balance(
			RawOrigin::Root.into(),
			staking_account,
			HDX,
			(10_000 * UNITS) as i128,
		));
		assert_ok!(Balances::force_set_balance(
			RawOrigin::Root.into(),
			ALICE.into(),
			1_000_000 * UNITS,
		));

		assert_ok!(Staking::stake(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			1_000 * UNITS
		));

		let stake_position_id = pallet_staking::Pallet::<hydradx_runtime::Runtime>::get_user_position_id(
			&sp_runtime::AccountId32::from(ALICE),
		)
		.unwrap()
		.unwrap();

		use sp_core::Get;
		let staking_collection: u128 = <hydradx_runtime::Runtime as pallet_staking::Config>::NFTCollectionId::get();
		assert_noop!(
			pallet_uniques::Pallet::<hydradx_runtime::Runtime>::transfer(
				hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
				staking_collection,
				stake_position_id,
				BOB.into()
			),
			pallet_uniques::Error::<hydradx_runtime::Runtime>::Frozen
		);
	});
}

#[test]
fn thaw_staking_position_should_fail_when_origin_is_position_owner() {
	TestNet::reset();
	Hydra::execute_with(|| {
		System::set_block_number(1);
		init_omnipool();
		assert_ok!(Staking::initialize_staking(RawOrigin::Root.into()));

		let staking_account = pallet_staking::Pallet::<hydradx_runtime::Runtime>::pot_account_id();
		assert_ok!(Currencies::update_balance(
			RawOrigin::Root.into(),
			staking_account,
			HDX,
			(10_000 * UNITS) as i128,
		));
		assert_ok!(Balances::force_set_balance(
			RawOrigin::Root.into(),
			ALICE.into(),
			1_000_000 * UNITS,
		));

		assert_ok!(Staking::stake(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			1_000 * UNITS
		));

		let stake_position_id = pallet_staking::Pallet::<hydradx_runtime::Runtime>::get_user_position_id(
			&sp_runtime::AccountId32::from(ALICE),
		)
		.unwrap()
		.unwrap();

		use sp_core::Get;
		let staking_collection: u128 = <hydradx_runtime::Runtime as pallet_staking::Config>::NFTCollectionId::get();
		assert_noop!(
			pallet_uniques::Pallet::<hydradx_runtime::Runtime>::thaw(
				hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
				staking_collection,
				stake_position_id,
			),
			pallet_uniques::Error::<hydradx_runtime::Runtime>::NoPermission
		);
	});
}

#[test]
fn thaw_staking_collection_should_fail_when_origin_is_not_pallet_account() {
	TestNet::reset();
	Hydra::execute_with(|| {
		System::set_block_number(1);
		init_omnipool();
		assert_ok!(Staking::initialize_staking(RawOrigin::Root.into()));

		let staking_account = pallet_staking::Pallet::<hydradx_runtime::Runtime>::pot_account_id();
		assert_ok!(Currencies::update_balance(
			RawOrigin::Root.into(),
			staking_account,
			HDX,
			(10_000 * UNITS) as i128,
		));
		assert_ok!(Balances::force_set_balance(
			RawOrigin::Root.into(),
			ALICE.into(),
			1_000_000 * UNITS,
		));

		assert_ok!(Staking::stake(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			1_000 * UNITS
		));

		use sp_core::Get;
		let staking_collection: u128 = <hydradx_runtime::Runtime as pallet_staking::Config>::NFTCollectionId::get();
		assert_noop!(
			pallet_uniques::Pallet::<hydradx_runtime::Runtime>::thaw_collection(
				hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
				staking_collection,
			),
			pallet_uniques::Error::<hydradx_runtime::Runtime>::NoPermission
		);
	});
}

#[test]
fn stake_should_fail_when_tokens_are_vested() {
	TestNet::reset();
	Hydra::execute_with(|| {
		System::set_block_number(1);
		init_omnipool();
		assert_ok!(Staking::initialize_staking(RawOrigin::Root.into()));

		let staking_account = pallet_staking::Pallet::<hydradx_runtime::Runtime>::pot_account_id();
		assert_ok!(Currencies::update_balance(
			RawOrigin::Root.into(),
			staking_account,
			HDX,
			(10_000 * UNITS) as i128,
		));

		assert_ok!(Currencies::update_balance(
			RawOrigin::Root.into(),
			vesting_account(),
			HDX,
			(1_000_000 * UNITS) as i128,
		));

		assert_ok!(Vesting::vested_transfer(
			RawOrigin::Root.into(),
			ALICE.into(),
			vesting_schedule()
		));

		//Act & assert
		assert_noop!(
			Staking::stake(hydradx_runtime::RuntimeOrigin::signed(ALICE.into()), 11_000 * UNITS),
			pallet_staking::Error::<hydradx_runtime::Runtime>::InsufficientBalance
		);
	});
}

#[test]
fn stake_should_fail_when_tokens_are_already_staked() {
	TestNet::reset();
	Hydra::execute_with(|| {
		System::set_block_number(1);
		init_omnipool();
		assert_ok!(Staking::initialize_staking(RawOrigin::Root.into()));

		let staking_account = pallet_staking::Pallet::<hydradx_runtime::Runtime>::pot_account_id();
		assert_ok!(Currencies::update_balance(
			RawOrigin::Root.into(),
			staking_account,
			HDX,
			(10_000 * UNITS) as i128,
		));

		assert_ok!(Currencies::update_balance(
			RawOrigin::Root.into(),
			ALICE.into(),
			HDX,
			(20_000 * UNITS) as i128,
		));

		assert_eq!(Currencies::free_balance(HDX, &ALICE.into()), 21_000 * UNITS);

		assert_ok!(Staking::stake(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			15_000 * UNITS
		));

		let alice_position_id = pallet_staking::Pallet::<hydradx_runtime::Runtime>::get_user_position_id(
			&sp_runtime::AccountId32::from(ALICE),
		)
		.unwrap()
		.unwrap();
		//Act & assert
		assert_noop!(
			Staking::increase_stake(
				hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
				alice_position_id,
				10_000 * UNITS
			),
			pallet_staking::Error::<hydradx_runtime::Runtime>::InsufficientBalance
		);
	});
}

#[test]
fn staking_should_assign_less_action_points_when_portion_of_staking_lock_is_vested() {
	TestNet::reset();
	Hydra::execute_with(|| {
		init_omnipool();
		assert_ok!(Staking::initialize_staking(RawOrigin::Root.into()));

		let staking_account = pallet_staking::Pallet::<hydradx_runtime::Runtime>::pot_account_id();
		assert_ok!(Currencies::update_balance(
			RawOrigin::Root.into(),
			staking_account,
			HDX,
			(10_000 * UNITS) as i128,
		));

		assert_ok!(Currencies::update_balance(
			RawOrigin::Root.into(),
			vesting_account(),
			HDX,
			(1_000_000 * UNITS) as i128,
		));

		assert_ok!(Currencies::update_balance(
			RawOrigin::Root.into(),
			ALICE.into(),
			HDX,
			(1_000_000 * UNITS) as i128,
		));

		assert_ok!(Currencies::update_balance(
			RawOrigin::Root.into(),
			BOB.into(),
			HDX,
			(99_000 * UNITS) as i128,
		));

		assert_ok!(Vesting::vested_transfer(
			RawOrigin::Root.into(),
			BOB.into(),
			vesting_schedule()
		));

		assert_eq!(Currencies::free_balance(HDX, &BOB.into()), 200_000 * UNITS);
		assert_ok!(Staking::stake(
			hydradx_runtime::RuntimeOrigin::signed(BOB.into()),
			100_000 * UNITS
		));

		//Transfer 50% so there is not enough tokens to satify both locks withou overlay.
		assert_ok!(Currencies::transfer(
			hydradx_runtime::RuntimeOrigin::signed(BOB.into()),
			ALICE.into(),
			HDX,
			50_000 * UNITS
		));

		let r = begin_referendum();

		assert_ok!(Democracy::vote(
			hydradx_runtime::RuntimeOrigin::signed(BOB.into()),
			r,
			AccountVote::Standard {
				vote: Vote {
					aye: true,
					conviction: Conviction::Locked6x,
				},
				balance: 150_000 * UNITS,
			}
		));
		end_referendum();

		let stake_position_id = pallet_staking::Pallet::<hydradx_runtime::Runtime>::get_user_position_id(
			&sp_runtime::AccountId32::from(BOB),
		)
		.unwrap()
		.unwrap();
		let position_votes =
			pallet_staking::Pallet::<hydradx_runtime::Runtime>::get_position_votes(stake_position_id).votes;

		assert_eq!(position_votes.len(), 1);
		assert_eq!(
			position_votes[0].1,
			pallet_staking::types::Vote::new(50_000 * UNITS, pallet_staking::types::Conviction::Locked6x)
		);

		assert_ok!(Staking::claim(
			hydradx_runtime::RuntimeOrigin::signed(BOB.into()),
			stake_position_id
		));

		let position = pallet_staking::Pallet::<hydradx_runtime::Runtime>::positions(stake_position_id).unwrap();

		assert_eq!(position.get_action_points(), 50_u128);
	});
}

#[test]
fn staking_should_allow_to_remove_vote_and_lock_when_referendum_is_finished_and_staking_position_exists_and_user_lost()
{
	TestNet::reset();
	Hydra::execute_with(|| {
		init_omnipool();
		assert_ok!(Staking::initialize_staking(RawOrigin::Root.into()));

		let staking_account = pallet_staking::Pallet::<hydradx_runtime::Runtime>::pot_account_id();
		assert_ok!(Currencies::update_balance(
			RawOrigin::Root.into(),
			staking_account,
			HDX,
			(10_000 * UNITS) as i128,
		));
		assert_ok!(Balances::force_set_balance(
			RawOrigin::Root.into(),
			ALICE.into(),
			1_000_000 * UNITS,
		));
		assert_ok!(Balances::force_set_balance(
			RawOrigin::Root.into(),
			BOB.into(),
			1_000_000 * UNITS,
		));
		let r = begin_referendum();
		assert_ok!(Staking::stake(
			hydradx_runtime::RuntimeOrigin::signed(BOB.into()),
			1_000_000 * UNITS
		));
		assert_ok!(Staking::stake(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			1_000_000 * UNITS
		));

		assert_ok!(Democracy::vote(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			r,
			AccountVote::Standard {
				vote: Vote {
					aye: true,
					conviction: Conviction::Locked6x,
				},
				balance: 1_000_000 * UNITS,
			}
		));

		assert_ok!(Democracy::vote(
			hydradx_runtime::RuntimeOrigin::signed(BOB.into()),
			r,
			AccountVote::Standard {
				vote: Vote {
					aye: false,
					conviction: Conviction::Locked6x,
				},
				balance: 1_000_000 * UNITS,
			}
		));
		end_referendum();

		let stake_position_id = pallet_staking::Pallet::<hydradx_runtime::Runtime>::get_user_position_id(
			&sp_runtime::AccountId32::from(BOB),
		)
		.unwrap()
		.unwrap();
		assert_ok!(Democracy::remove_vote(
			hydradx_runtime::RuntimeOrigin::signed(BOB.into()),
			r
		),);
		assert_ok!(Democracy::unlock(
			hydradx_runtime::RuntimeOrigin::signed(BOB.into()),
			BOB.into()
		),);
		let stake_voting = pallet_staking::Pallet::<hydradx_runtime::Runtime>::get_position_votes(stake_position_id);
		assert!(stake_voting.votes.is_empty());
		let position = pallet_staking::Pallet::<hydradx_runtime::Runtime>::get_position(stake_position_id).unwrap();
		assert_eq!(position.get_action_points(), 100);
		assert_lock(&BOB.into(), 1_000_000 * UNITS, DEMOCRACY_ID);
	});
}

#[test]
fn staking_should_allow_to_remove_vote_when_user_lost_and_conviction_expires() {
	TestNet::reset();
	Hydra::execute_with(|| {
		init_omnipool();
		assert_ok!(Staking::initialize_staking(RawOrigin::Root.into()));

		let staking_account = pallet_staking::Pallet::<hydradx_runtime::Runtime>::pot_account_id();
		assert_ok!(Currencies::update_balance(
			RawOrigin::Root.into(),
			staking_account,
			HDX,
			(10_000 * UNITS) as i128,
		));
		assert_ok!(Balances::force_set_balance(
			RawOrigin::Root.into(),
			ALICE.into(),
			1_000_000 * UNITS,
		));
		assert_ok!(Balances::force_set_balance(
			RawOrigin::Root.into(),
			BOB.into(),
			1_000_000 * UNITS,
		));
		let r = begin_referendum();
		assert_ok!(Staking::stake(
			hydradx_runtime::RuntimeOrigin::signed(BOB.into()),
			3_000 * UNITS
		));
		assert_ok!(Staking::stake(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			1_000_000 * UNITS
		));

		assert_ok!(Democracy::vote(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			r,
			AccountVote::Standard {
				vote: Vote {
					aye: true,
					conviction: Conviction::Locked6x,
				},
				balance: 1_000_000 * UNITS,
			}
		));

		assert_ok!(Democracy::vote(
			hydradx_runtime::RuntimeOrigin::signed(BOB.into()),
			r,
			AccountVote::Standard {
				vote: Vote {
					aye: false,
					conviction: Conviction::Locked1x,
				},
				balance: 222 * UNITS,
			}
		));
		end_referendum();

		fast_forward_to(18 * DAYS);

		let stake_position_id = pallet_staking::Pallet::<hydradx_runtime::Runtime>::get_user_position_id(
			&sp_runtime::AccountId32::from(BOB),
		)
		.unwrap()
		.unwrap();
		assert_ok!(Democracy::remove_vote(
			hydradx_runtime::RuntimeOrigin::signed(BOB.into()),
			r
		),);
		assert_ok!(Democracy::unlock(
			hydradx_runtime::RuntimeOrigin::signed(BOB.into()),
			BOB.into()
		),);
		let stake_voting = pallet_staking::Pallet::<hydradx_runtime::Runtime>::get_position_votes(stake_position_id);
		assert!(stake_voting.votes.is_empty());
		let position = pallet_staking::Pallet::<hydradx_runtime::Runtime>::get_position(stake_position_id).unwrap();
		assert_eq!(position.get_action_points(), 1);
		assert_no_lock(&BOB.into(), DEMOCRACY_ID);
	});
}

#[test]
fn staking_should_allow_to_remove_vote_when_user_won() {
	TestNet::reset();
	Hydra::execute_with(|| {
		init_omnipool();
		assert_ok!(Staking::initialize_staking(RawOrigin::Root.into()));

		let staking_account = pallet_staking::Pallet::<hydradx_runtime::Runtime>::pot_account_id();
		assert_ok!(Currencies::update_balance(
			RawOrigin::Root.into(),
			staking_account,
			HDX,
			(10_000 * UNITS) as i128,
		));
		assert_ok!(Balances::force_set_balance(
			RawOrigin::Root.into(),
			ALICE.into(),
			1_000_000 * UNITS,
		));
		assert_ok!(Balances::force_set_balance(
			RawOrigin::Root.into(),
			BOB.into(),
			1_000_000 * UNITS,
		));
		let r = begin_referendum();
		assert_ok!(Staking::stake(
			hydradx_runtime::RuntimeOrigin::signed(BOB.into()),
			3_000 * UNITS
		));
		assert_ok!(Staking::stake(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			1_000_000 * UNITS
		));

		assert_ok!(Democracy::vote(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			r,
			AccountVote::Standard {
				vote: Vote {
					aye: true,
					conviction: Conviction::Locked6x,
				},
				balance: 1_000_000 * UNITS,
			}
		));

		assert_ok!(Democracy::vote(
			hydradx_runtime::RuntimeOrigin::signed(BOB.into()),
			r,
			AccountVote::Standard {
				vote: Vote {
					aye: false,
					conviction: Conviction::Locked1x,
				},
				balance: 222 * UNITS,
			}
		));
		end_referendum();

		let stake_position_id = pallet_staking::Pallet::<hydradx_runtime::Runtime>::get_user_position_id(
			&sp_runtime::AccountId32::from(ALICE),
		)
		.unwrap()
		.unwrap();
		assert_ok!(Democracy::remove_vote(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			r
		),);
		assert_ok!(Democracy::unlock(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			ALICE.into()
		),);
		let stake_voting = pallet_staking::Pallet::<hydradx_runtime::Runtime>::get_position_votes(stake_position_id);
		assert!(stake_voting.votes.is_empty());
		let position = pallet_staking::Pallet::<hydradx_runtime::Runtime>::get_position(stake_position_id).unwrap();
		assert_eq!(position.get_action_points(), 100);
		assert_lock(&ALICE.into(), 1_000_000 * UNITS, DEMOCRACY_ID);
	});
}

#[test]
fn staking_should_allow_to_remove_vote_when_user_lost_with_no_conviction() {
	TestNet::reset();
	Hydra::execute_with(|| {
		init_omnipool();
		assert_ok!(Staking::initialize_staking(RawOrigin::Root.into()));

		let staking_account = pallet_staking::Pallet::<hydradx_runtime::Runtime>::pot_account_id();
		assert_ok!(Currencies::update_balance(
			RawOrigin::Root.into(),
			staking_account,
			HDX,
			(10_000 * UNITS) as i128,
		));
		assert_ok!(Balances::force_set_balance(
			RawOrigin::Root.into(),
			ALICE.into(),
			1_000_000 * UNITS,
		));
		assert_ok!(Balances::force_set_balance(
			RawOrigin::Root.into(),
			BOB.into(),
			1_000_000 * UNITS,
		));
		let r = begin_referendum();
		assert_ok!(Staking::stake(
			hydradx_runtime::RuntimeOrigin::signed(BOB.into()),
			3_000 * UNITS
		));
		assert_ok!(Staking::stake(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			1_000_000 * UNITS
		));

		assert_ok!(Democracy::vote(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			r,
			AccountVote::Standard {
				vote: Vote {
					aye: true,
					conviction: Conviction::Locked6x,
				},
				balance: 1_000_000 * UNITS,
			}
		));

		assert_ok!(Democracy::vote(
			hydradx_runtime::RuntimeOrigin::signed(BOB.into()),
			r,
			AccountVote::Standard {
				vote: Vote {
					aye: false,
					conviction: Conviction::None,
				},
				balance: 3_000 * UNITS,
			}
		));
		end_referendum();

		let stake_position_id = pallet_staking::Pallet::<hydradx_runtime::Runtime>::get_user_position_id(
			&sp_runtime::AccountId32::from(BOB),
		)
		.unwrap()
		.unwrap();
		assert_ok!(Democracy::remove_vote(
			hydradx_runtime::RuntimeOrigin::signed(BOB.into()),
			r
		),);
		assert_ok!(Democracy::unlock(
			hydradx_runtime::RuntimeOrigin::signed(BOB.into()),
			BOB.into()
		),);
		let stake_voting = pallet_staking::Pallet::<hydradx_runtime::Runtime>::get_position_votes(stake_position_id);
		assert!(stake_voting.votes.is_empty());
		let position = pallet_staking::Pallet::<hydradx_runtime::Runtime>::get_position(stake_position_id).unwrap();
		assert_eq!(position.get_action_points(), 1);
		assert_no_lock(&BOB.into(), DEMOCRACY_ID);
	});
}

#[test]
fn remove_vote_should_not_lock_when_no_stake_and_lost() {
	TestNet::reset();
	Hydra::execute_with(|| {
		init_omnipool();
		assert_ok!(Staking::initialize_staking(RawOrigin::Root.into()));

		let staking_account = pallet_staking::Pallet::<hydradx_runtime::Runtime>::pot_account_id();
		assert_ok!(Currencies::update_balance(
			RawOrigin::Root.into(),
			staking_account,
			HDX,
			(10_000 * UNITS) as i128,
		));
		assert_ok!(Balances::force_set_balance(
			RawOrigin::Root.into(),
			ALICE.into(),
			1_000_000 * UNITS,
		));
		assert_ok!(Balances::force_set_balance(
			RawOrigin::Root.into(),
			BOB.into(),
			1_000_000 * UNITS,
		));
		let r = begin_referendum();
		assert_ok!(Staking::stake(
			hydradx_runtime::RuntimeOrigin::signed(BOB.into()),
			1_000_000 * UNITS
		));

		assert_ok!(Democracy::vote(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			r,
			AccountVote::Standard {
				vote: Vote {
					aye: true,
					conviction: Conviction::Locked6x,
				},
				balance: 1_000_000 * UNITS,
			}
		));

		assert_ok!(Democracy::vote(
			hydradx_runtime::RuntimeOrigin::signed(BOB.into()),
			r,
			AccountVote::Standard {
				vote: Vote {
					aye: false,
					conviction: Conviction::Locked6x,
				},
				balance: 1_000_000 * UNITS,
			}
		));
		end_referendum();

		fast_forward_to(DAYS);

		assert_ok!(Democracy::remove_vote(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			r
		));
		assert_ok!(Democracy::unlock(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			ALICE.into()
		));
		assert_no_lock(&ALICE.into(), DEMOCRACY_ID);
	});
}

#[test]
fn remove_vote_should_extend_lock_when_vote_not_in_favor() {
	TestNet::reset();
	Hydra::execute_with(|| {
		init_omnipool();
		assert_ok!(Staking::initialize_staking(RawOrigin::Root.into()));

		let staking_account = pallet_staking::Pallet::<hydradx_runtime::Runtime>::pot_account_id();
		assert_ok!(Currencies::update_balance(
			RawOrigin::Root.into(),
			staking_account,
			HDX,
			(10_000 * UNITS) as i128,
		));
		assert_ok!(Balances::force_set_balance(
			RawOrigin::Root.into(),
			ALICE.into(),
			1_000_000 * UNITS,
		));
		assert_ok!(Balances::force_set_balance(
			RawOrigin::Root.into(),
			BOB.into(),
			1_000_000 * UNITS,
		));
		let r = begin_referendum();
		assert_ok!(Staking::stake(
			hydradx_runtime::RuntimeOrigin::signed(BOB.into()),
			1_000_000 * UNITS
		));
		assert_ok!(Staking::stake(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			1_000_000 * UNITS
		));

		assert_ok!(Democracy::vote(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			r,
			AccountVote::Standard {
				vote: Vote {
					aye: true,
					conviction: Conviction::Locked6x,
				},
				balance: 1_000_000 * UNITS,
			}
		));

		assert_ok!(Democracy::vote(
			hydradx_runtime::RuntimeOrigin::signed(BOB.into()),
			r,
			AccountVote::Standard {
				vote: Vote {
					aye: false,
					conviction: Conviction::Locked6x,
				},
				balance: 1_000_000 * UNITS,
			}
		));
		end_referendum();

		fast_forward_to(DAYS);

		assert_ok!(Democracy::remove_vote(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			r
		));
		assert_ok!(Democracy::unlock(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			ALICE.into()
		));
		assert_lock(&ALICE.into(), 1_000_000 * UNITS, DEMOCRACY_ID);
	});
}

#[test]
fn remove_vote_should_extend_lock_for_partial_amount_when_vote_not_in_favor() {
	TestNet::reset();
	Hydra::execute_with(|| {
		init_omnipool();
		assert_ok!(Staking::initialize_staking(RawOrigin::Root.into()));

		let staking_account = pallet_staking::Pallet::<hydradx_runtime::Runtime>::pot_account_id();
		assert_ok!(Currencies::update_balance(
			RawOrigin::Root.into(),
			staking_account,
			HDX,
			(10_000 * UNITS) as i128,
		));
		assert_ok!(Balances::force_set_balance(
			RawOrigin::Root.into(),
			ALICE.into(),
			1_000_000 * UNITS,
		));
		assert_ok!(Balances::force_set_balance(
			RawOrigin::Root.into(),
			BOB.into(),
			1_000_000 * UNITS,
		));
		let r = begin_referendum();
		assert_ok!(Staking::stake(
			hydradx_runtime::RuntimeOrigin::signed(BOB.into()),
			1_000_000 * UNITS
		));
		assert_ok!(Staking::stake(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			222_222 * UNITS
		));

		assert_ok!(Democracy::vote(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			r,
			AccountVote::Standard {
				vote: Vote {
					aye: true,
					conviction: Conviction::Locked6x,
				},
				balance: 1_000_000 * UNITS,
			}
		));

		assert_ok!(Democracy::vote(
			hydradx_runtime::RuntimeOrigin::signed(BOB.into()),
			r,
			AccountVote::Standard {
				vote: Vote {
					aye: false,
					conviction: Conviction::Locked6x,
				},
				balance: 1_000_000 * UNITS,
			}
		));
		end_referendum();

		fast_forward_to(DAYS);

		assert_ok!(Democracy::remove_vote(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			r
		));
		assert_ok!(Democracy::unlock(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			ALICE.into()
		));
		assert_lock(&ALICE.into(), 222_222 * UNITS, DEMOCRACY_ID);
	});
}

#[test]
fn unstake_should_fail_when_position_has_existing_votes() {
	TestNet::reset();
	Hydra::execute_with(|| {
		init_omnipool();
		assert_ok!(Staking::initialize_staking(RawOrigin::Root.into()));

		let staking_account = pallet_staking::Pallet::<hydradx_runtime::Runtime>::pot_account_id();
		assert_ok!(Currencies::update_balance(
			RawOrigin::Root.into(),
			staking_account,
			HDX,
			(10_000 * UNITS) as i128,
		));
		assert_ok!(Balances::force_set_balance(
			RawOrigin::Root.into(),
			ALICE.into(),
			1_000_000 * UNITS,
		));
		assert_ok!(Balances::force_set_balance(
			RawOrigin::Root.into(),
			BOB.into(),
			1_000_000 * UNITS,
		));
		let r = begin_referendum();
		assert_ok!(Staking::stake(
			hydradx_runtime::RuntimeOrigin::signed(BOB.into()),
			1_000_000 * UNITS
		));
		assert_ok!(Staking::stake(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			222_222 * UNITS
		));

		assert_ok!(Democracy::vote(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			r,
			AccountVote::Standard {
				vote: Vote {
					aye: true,
					conviction: Conviction::Locked6x,
				},
				balance: 1_000_000 * UNITS,
			}
		));

		assert_ok!(Democracy::vote(
			hydradx_runtime::RuntimeOrigin::signed(BOB.into()),
			r,
			AccountVote::Standard {
				vote: Vote {
					aye: false,
					conviction: Conviction::Locked6x,
				},
				balance: 1_000_000 * UNITS,
			}
		));
		end_referendum();
		fast_forward_to(DAYS);
		assert_noop!(
			Staking::unstake(hydradx_runtime::RuntimeOrigin::signed(ALICE.into()), 1),
			pallet_staking::Error::<hydradx_runtime::Runtime>::ExistingVotes
		);
	});
}

#[test]
fn unstake_should_fail_when_position_has_existing_processed_votes() {
	TestNet::reset();
	Hydra::execute_with(|| {
		init_omnipool();
		assert_ok!(Staking::initialize_staking(RawOrigin::Root.into()));

		let staking_account = pallet_staking::Pallet::<hydradx_runtime::Runtime>::pot_account_id();
		assert_ok!(Currencies::update_balance(
			RawOrigin::Root.into(),
			staking_account,
			HDX,
			(10_000 * UNITS) as i128,
		));
		assert_ok!(Balances::force_set_balance(
			RawOrigin::Root.into(),
			ALICE.into(),
			1_000_000 * UNITS,
		));
		assert_ok!(Balances::force_set_balance(
			RawOrigin::Root.into(),
			BOB.into(),
			1_000_000 * UNITS,
		));
		let r = begin_referendum();
		assert_ok!(Staking::stake(
			hydradx_runtime::RuntimeOrigin::signed(BOB.into()),
			1_000_000 * UNITS
		));
		assert_ok!(Staking::stake(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			222_222 * UNITS
		));

		assert_ok!(Democracy::vote(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			r,
			AccountVote::Standard {
				vote: Vote {
					aye: true,
					conviction: Conviction::Locked6x,
				},
				balance: 1_000_000 * UNITS,
			}
		));

		assert_ok!(Democracy::vote(
			hydradx_runtime::RuntimeOrigin::signed(BOB.into()),
			r,
			AccountVote::Standard {
				vote: Vote {
					aye: false,
					conviction: Conviction::Locked6x,
				},
				balance: 1_000_000 * UNITS,
			}
		));
		end_referendum();
		fast_forward_to(DAYS);
		assert_ok!(Staking::increase_stake(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			1,
			222_222 * UNITS
		));

		assert_noop!(
			Staking::unstake(hydradx_runtime::RuntimeOrigin::signed(ALICE.into()), 1),
			pallet_staking::Error::<hydradx_runtime::Runtime>::ExistingVotes
		);
	});
}

#[test]
fn unstake_should_work_when_processed_votes_are_removed() {
	TestNet::reset();
	Hydra::execute_with(|| {
		init_omnipool();
		assert_ok!(Staking::initialize_staking(RawOrigin::Root.into()));

		let staking_account = pallet_staking::Pallet::<hydradx_runtime::Runtime>::pot_account_id();
		assert_ok!(Currencies::update_balance(
			RawOrigin::Root.into(),
			staking_account,
			HDX,
			(10_000 * UNITS) as i128,
		));
		assert_ok!(Balances::force_set_balance(
			RawOrigin::Root.into(),
			ALICE.into(),
			1_000_000 * UNITS,
		));
		assert_ok!(Balances::force_set_balance(
			RawOrigin::Root.into(),
			BOB.into(),
			1_000_000 * UNITS,
		));
		let r = begin_referendum();
		assert_ok!(Staking::stake(
			hydradx_runtime::RuntimeOrigin::signed(BOB.into()),
			1_000_000 * UNITS
		));
		assert_ok!(Staking::stake(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			222_222 * UNITS
		));

		assert_ok!(Democracy::vote(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			r,
			AccountVote::Standard {
				vote: Vote {
					aye: true,
					conviction: Conviction::Locked6x,
				},
				balance: 1_000_000 * UNITS,
			}
		));

		assert_ok!(Democracy::vote(
			hydradx_runtime::RuntimeOrigin::signed(BOB.into()),
			r,
			AccountVote::Standard {
				vote: Vote {
					aye: false,
					conviction: Conviction::Locked6x,
				},
				balance: 1_000_000 * UNITS,
			}
		));
		end_referendum();
		fast_forward_to(DAYS);
		// Votes are processed
		assert_ok!(Staking::increase_stake(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			1,
			222_222 * UNITS
		));

		// remove vote to allow unstake
		assert_ok!(Democracy::remove_vote(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			r
		));

		assert_ok!(Staking::unstake(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			1
		));
		assert_ok!(Democracy::unlock(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			ALICE.into()
		));
		// the amount should be locked
		assert_lock(&ALICE.into(), 222_222 * UNITS, DEMOCRACY_ID);
	});
}

#[test]
fn remove_vote_should_not_lock_nor_assign_rewards_when_referendum_was_cancelled() {
	TestNet::reset();
	Hydra::execute_with(|| {
		init_omnipool();
		assert_ok!(Staking::initialize_staking(RawOrigin::Root.into()));

		let staking_account = pallet_staking::Pallet::<hydradx_runtime::Runtime>::pot_account_id();
		assert_ok!(Currencies::update_balance(
			RawOrigin::Root.into(),
			staking_account,
			HDX,
			(10_000 * UNITS) as i128,
		));
		assert_ok!(Balances::force_set_balance(
			RawOrigin::Root.into(),
			ALICE.into(),
			1_000_000 * UNITS,
		));
		assert_ok!(Balances::force_set_balance(
			RawOrigin::Root.into(),
			BOB.into(),
			1_000_000 * UNITS,
		));
		let r = begin_referendum();
		assert_ok!(Staking::stake(
			hydradx_runtime::RuntimeOrigin::signed(BOB.into()),
			1_000_000 * UNITS
		));
		assert_ok!(Staking::stake(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			222_222 * UNITS
		));

		assert_ok!(Democracy::vote(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			r,
			AccountVote::Standard {
				vote: Vote {
					aye: true,
					conviction: Conviction::Locked6x,
				},
				balance: 1_000_000 * UNITS,
			}
		));

		assert_ok!(Democracy::vote(
			hydradx_runtime::RuntimeOrigin::signed(BOB.into()),
			r,
			AccountVote::Standard {
				vote: Vote {
					aye: false,
					conviction: Conviction::Locked6x,
				},
				balance: 1_000_000 * UNITS,
			}
		));
		assert_ok!(Democracy::cancel_referendum(RawOrigin::Root.into(), r,));
		fast_forward_to(DAYS);
		assert_ok!(Staking::increase_stake(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			1,
			222_222 * UNITS
		));

		assert_ok!(Democracy::remove_vote(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			r
		));
		assert_ok!(Democracy::remove_vote(
			hydradx_runtime::RuntimeOrigin::signed(BOB.into()),
			r
		));

		assert_ok!(Democracy::unlock(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			ALICE.into()
		));
		assert_ok!(Democracy::unlock(
			hydradx_runtime::RuntimeOrigin::signed(BOB.into()),
			BOB.into()
		));
		assert_no_lock(&BOB.into(), DEMOCRACY_ID);
		assert_no_lock(&ALICE.into(), DEMOCRACY_ID);

		let stake_voting = pallet_staking::Pallet::<hydradx_runtime::Runtime>::get_position_votes(1);
		assert!(stake_voting.votes.is_empty());
		assert!(
			pallet_staking::Pallet::<hydradx_runtime::Runtime>::processed_votes::<AccountId, u32>(ALICE.into(), r)
				.is_none()
		);
		let stake_position = pallet_staking::Pallet::<hydradx_runtime::Runtime>::get_position(1).unwrap();
		assert_eq!(stake_position.get_action_points(), 0);

		let stake_voting = pallet_staking::Pallet::<hydradx_runtime::Runtime>::get_position_votes(0);
		assert!(stake_voting.votes.is_empty());
		assert!(
			pallet_staking::Pallet::<hydradx_runtime::Runtime>::processed_votes::<AccountId, u32>(BOB.into(), r)
				.is_none()
		);
		let stake_position = pallet_staking::Pallet::<hydradx_runtime::Runtime>::get_position(0).unwrap();
		assert_eq!(stake_position.get_action_points(), 0);
	});
}

#[test]
fn remove_vote_should_extend_lock_when_votes_are_already_processed() {
	TestNet::reset();
	Hydra::execute_with(|| {
		init_omnipool();
		assert_ok!(Staking::initialize_staking(RawOrigin::Root.into()));

		let staking_account = pallet_staking::Pallet::<hydradx_runtime::Runtime>::pot_account_id();
		assert_ok!(Currencies::update_balance(
			RawOrigin::Root.into(),
			staking_account,
			HDX,
			(10_000 * UNITS) as i128,
		));
		assert_ok!(Balances::force_set_balance(
			RawOrigin::Root.into(),
			ALICE.into(),
			1_000_000 * UNITS,
		));
		assert_ok!(Balances::force_set_balance(
			RawOrigin::Root.into(),
			BOB.into(),
			1_000_000 * UNITS,
		));
		let r = begin_referendum();
		assert_ok!(Staking::stake(
			hydradx_runtime::RuntimeOrigin::signed(BOB.into()),
			1_000_000 * UNITS
		));
		assert_ok!(Staking::stake(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			222_222 * UNITS
		));

		assert_ok!(Democracy::vote(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			r,
			AccountVote::Standard {
				vote: Vote {
					aye: true,
					conviction: Conviction::Locked6x,
				},
				balance: 1_000_000 * UNITS,
			}
		));

		assert_ok!(Democracy::vote(
			hydradx_runtime::RuntimeOrigin::signed(BOB.into()),
			r,
			AccountVote::Standard {
				vote: Vote {
					aye: false,
					conviction: Conviction::Locked6x,
				},
				balance: 1_000_000 * UNITS,
			}
		));
		end_referendum();
		fast_forward_to(DAYS);
		// Votes are processed
		assert_ok!(Staking::increase_stake(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			1,
			222_222 * UNITS
		));

		assert_ok!(Democracy::remove_vote(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			r
		));

		assert_ok!(Democracy::unlock(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			ALICE.into()
		));
		// the amount should be locked
		assert_lock(&ALICE.into(), 222_222 * UNITS, DEMOCRACY_ID);

		let stake_voting = pallet_staking::Pallet::<hydradx_runtime::Runtime>::get_position_votes(1);
		assert!(stake_voting.votes.is_empty());
		assert!(
			pallet_staking::Pallet::<hydradx_runtime::Runtime>::processed_votes::<AccountId, u32>(ALICE.into(), r)
				.is_none()
		);
		let stake_position = pallet_staking::Pallet::<hydradx_runtime::Runtime>::get_position(1).unwrap();
		assert_eq!(stake_position.get_action_points(), 100);
	});
}

#[test]
fn increase_stake_should_fail_when_position_has_existing_processed_votes() {
	TestNet::reset();
	Hydra::execute_with(|| {
		init_omnipool();
		assert_ok!(Staking::initialize_staking(RawOrigin::Root.into()));

		let staking_account = pallet_staking::Pallet::<hydradx_runtime::Runtime>::pot_account_id();
		assert_ok!(Currencies::update_balance(
			RawOrigin::Root.into(),
			staking_account,
			HDX,
			(10_000 * UNITS) as i128,
		));
		assert_ok!(Balances::force_set_balance(
			RawOrigin::Root.into(),
			ALICE.into(),
			1_000_000 * UNITS,
		));
		assert_ok!(Balances::force_set_balance(
			RawOrigin::Root.into(),
			BOB.into(),
			1_000_000 * UNITS,
		));
		let r = begin_referendum();
		assert_ok!(Staking::stake(
			hydradx_runtime::RuntimeOrigin::signed(BOB.into()),
			1_000_000 * UNITS
		));
		assert_ok!(Staking::stake(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			222_222 * UNITS
		));

		assert_ok!(Democracy::vote(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			r,
			AccountVote::Standard {
				vote: Vote {
					aye: true,
					conviction: Conviction::Locked6x,
				},
				balance: 1_000_000 * UNITS,
			}
		));

		assert_ok!(Democracy::vote(
			hydradx_runtime::RuntimeOrigin::signed(BOB.into()),
			r,
			AccountVote::Standard {
				vote: Vote {
					aye: false,
					conviction: Conviction::Locked6x,
				},
				balance: 1_000_000 * UNITS,
			}
		));
		end_referendum();
		fast_forward_to(DAYS);
		// Votes are processed
		assert_ok!(Staking::increase_stake(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			1,
			222_222 * UNITS
		));
		assert_noop!(
			Staking::increase_stake(hydradx_runtime::RuntimeOrigin::signed(ALICE.into()), 1, 222_222 * UNITS),
			pallet_staking::Error::<hydradx_runtime::Runtime>::ExistingProcessedVotes
		);
	});
}

#[test]
fn claim_should_fail_when_position_has_existing_processed_votes() {
	TestNet::reset();
	Hydra::execute_with(|| {
		init_omnipool();
		assert_ok!(Staking::initialize_staking(RawOrigin::Root.into()));

		let staking_account = pallet_staking::Pallet::<hydradx_runtime::Runtime>::pot_account_id();
		assert_ok!(Currencies::update_balance(
			RawOrigin::Root.into(),
			staking_account,
			HDX,
			(10_000 * UNITS) as i128,
		));
		assert_ok!(Balances::force_set_balance(
			RawOrigin::Root.into(),
			ALICE.into(),
			1_000_000 * UNITS,
		));
		assert_ok!(Balances::force_set_balance(
			RawOrigin::Root.into(),
			BOB.into(),
			1_000_000 * UNITS,
		));
		let r = begin_referendum();
		assert_ok!(Staking::stake(
			hydradx_runtime::RuntimeOrigin::signed(BOB.into()),
			1_000_000 * UNITS
		));
		assert_ok!(Staking::stake(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			222_222 * UNITS
		));

		assert_ok!(Democracy::vote(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			r,
			AccountVote::Standard {
				vote: Vote {
					aye: true,
					conviction: Conviction::Locked6x,
				},
				balance: 1_000_000 * UNITS,
			}
		));

		assert_ok!(Democracy::vote(
			hydradx_runtime::RuntimeOrigin::signed(BOB.into()),
			r,
			AccountVote::Standard {
				vote: Vote {
					aye: false,
					conviction: Conviction::Locked6x,
				},
				balance: 1_000_000 * UNITS,
			}
		));
		end_referendum();
		fast_forward_to(DAYS);
		// Votes are processed
		assert_ok!(Staking::increase_stake(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			1,
			222_222 * UNITS
		));
		assert_noop!(
			Staking::claim(hydradx_runtime::RuntimeOrigin::signed(ALICE.into()), 1,),
			pallet_staking::Error::<hydradx_runtime::Runtime>::ExistingProcessedVotes
		);
	});
}

#[test]
fn claim_should_work_when_processed_votes_are_removed() {
	TestNet::reset();
	Hydra::execute_with(|| {
		init_omnipool();
		assert_ok!(Staking::initialize_staking(RawOrigin::Root.into()));

		let staking_account = pallet_staking::Pallet::<hydradx_runtime::Runtime>::pot_account_id();
		assert_ok!(Currencies::update_balance(
			RawOrigin::Root.into(),
			staking_account,
			HDX,
			(10_000 * UNITS) as i128,
		));
		assert_ok!(Balances::force_set_balance(
			RawOrigin::Root.into(),
			ALICE.into(),
			1_000_000 * UNITS,
		));
		assert_ok!(Balances::force_set_balance(
			RawOrigin::Root.into(),
			BOB.into(),
			1_000_000 * UNITS,
		));
		let r = begin_referendum();
		assert_ok!(Staking::stake(
			hydradx_runtime::RuntimeOrigin::signed(BOB.into()),
			1_000_000 * UNITS
		));
		assert_ok!(Staking::stake(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			222_222 * UNITS
		));

		assert_ok!(Democracy::vote(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			r,
			AccountVote::Standard {
				vote: Vote {
					aye: true,
					conviction: Conviction::Locked6x,
				},
				balance: 1_000_000 * UNITS,
			}
		));

		assert_ok!(Democracy::vote(
			hydradx_runtime::RuntimeOrigin::signed(BOB.into()),
			r,
			AccountVote::Standard {
				vote: Vote {
					aye: false,
					conviction: Conviction::Locked6x,
				},
				balance: 1_000_000 * UNITS,
			}
		));
		end_referendum();

		let ref_info = pallet_democracy::Pallet::<hydradx_runtime::Runtime>::referendum_info(r).unwrap();
		assert_eq!(
			ref_info,
			ReferendumInfo::Finished {
				approved: false,
				end: 43200
			}
		);

		fast_forward_to(DAYS);
		// Votes are processed
		assert_ok!(Staking::increase_stake(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			1,
			222_222 * UNITS
		));
		assert_ok!(Democracy::remove_vote(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			r
		));
		assert_ok!(Staking::claim(hydradx_runtime::RuntimeOrigin::signed(ALICE.into()), 1,));
		assert_ok!(Democracy::unlock(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			ALICE.into()
		));
		let stake_voting = pallet_staking::Pallet::<hydradx_runtime::Runtime>::get_position_votes(1);
		assert!(stake_voting.votes.is_empty());
		assert!(
			pallet_staking::Pallet::<hydradx_runtime::Runtime>::processed_votes::<AccountId, u32>(ALICE.into(), r)
				.is_none()
		);
		let stake_position = pallet_staking::Pallet::<hydradx_runtime::Runtime>::get_position(1).unwrap();
		assert_eq!(stake_position.get_action_points(), 100);
		assert_lock(&ALICE.into(), 222_222 * UNITS, DEMOCRACY_ID);
	});
}

#[test]
fn increase_stake_should_work_when_processed_votes_are_removed() {
	TestNet::reset();
	Hydra::execute_with(|| {
		init_omnipool();
		assert_ok!(Staking::initialize_staking(RawOrigin::Root.into()));

		let staking_account = pallet_staking::Pallet::<hydradx_runtime::Runtime>::pot_account_id();
		assert_ok!(Currencies::update_balance(
			RawOrigin::Root.into(),
			staking_account,
			HDX,
			(10_000 * UNITS) as i128,
		));
		assert_ok!(Balances::force_set_balance(
			RawOrigin::Root.into(),
			ALICE.into(),
			1_000_000 * UNITS,
		));
		assert_ok!(Balances::force_set_balance(
			RawOrigin::Root.into(),
			BOB.into(),
			1_000_000 * UNITS,
		));
		let r = begin_referendum();
		assert_ok!(Staking::stake(
			hydradx_runtime::RuntimeOrigin::signed(BOB.into()),
			1_000_000 * UNITS
		));
		assert_ok!(Staking::stake(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			222_222 * UNITS
		));

		assert_ok!(Democracy::vote(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			r,
			AccountVote::Standard {
				vote: Vote {
					aye: true,
					conviction: Conviction::Locked6x,
				},
				balance: 1_000_000 * UNITS,
			}
		));

		assert_ok!(Democracy::vote(
			hydradx_runtime::RuntimeOrigin::signed(BOB.into()),
			r,
			AccountVote::Standard {
				vote: Vote {
					aye: false,
					conviction: Conviction::Locked6x,
				},
				balance: 1_000_000 * UNITS,
			}
		));
		end_referendum();
		fast_forward_to(DAYS);
		// Votes are processed
		assert_ok!(Staking::increase_stake(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			1,
			222_222 * UNITS
		));
		assert_ok!(Democracy::remove_vote(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			r
		));
		assert_ok!(Staking::increase_stake(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			1,
			222_222 * UNITS
		));
	});
}

#[test]
fn increase_stake_should_work_when_referendum_ongoing_and_votes_processed() {
	TestNet::reset();
	Hydra::execute_with(|| {
		init_omnipool();
		assert_ok!(Staking::initialize_staking(RawOrigin::Root.into()));

		let staking_account = pallet_staking::Pallet::<hydradx_runtime::Runtime>::pot_account_id();
		assert_ok!(Currencies::update_balance(
			RawOrigin::Root.into(),
			staking_account,
			HDX,
			(10_000 * UNITS) as i128,
		));
		assert_ok!(Balances::force_set_balance(
			RawOrigin::Root.into(),
			ALICE.into(),
			1_000_000 * UNITS,
		));
		assert_ok!(Balances::force_set_balance(
			RawOrigin::Root.into(),
			BOB.into(),
			1_000_000 * UNITS,
		));
		let r = begin_referendum();
		assert_ok!(Staking::stake(
			hydradx_runtime::RuntimeOrigin::signed(BOB.into()),
			1_000_000 * UNITS
		));
		assert_ok!(Staking::stake(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			222_222 * UNITS
		));

		assert_ok!(Democracy::vote(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			r,
			AccountVote::Standard {
				vote: Vote {
					aye: true,
					conviction: Conviction::Locked6x,
				},
				balance: 1_000_000 * UNITS,
			}
		));

		assert_ok!(Democracy::vote(
			hydradx_runtime::RuntimeOrigin::signed(BOB.into()),
			r,
			AccountVote::Standard {
				vote: Vote {
					aye: false,
					conviction: Conviction::Locked6x,
				},
				balance: 1_000_000 * UNITS,
			}
		));
		fast_forward_to(DAYS);
		// Votes are processed
		assert_ok!(Staking::increase_stake(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			1,
			222_222 * UNITS
		));
		assert!(
			pallet_staking::Pallet::<hydradx_runtime::Runtime>::processed_votes::<AccountId, u32>(ALICE.into(), r)
				.is_none()
		);
		assert_ok!(Staking::increase_stake(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			1,
			222_222 * UNITS
		));
	});
}

#[test]
fn voting_on_next_referenda_should_process_votes() {
	TestNet::reset();
	Hydra::execute_with(|| {
		init_omnipool();
		assert_ok!(Staking::initialize_staking(RawOrigin::Root.into()));

		let staking_account = pallet_staking::Pallet::<hydradx_runtime::Runtime>::pot_account_id();
		assert_ok!(Currencies::update_balance(
			RawOrigin::Root.into(),
			staking_account,
			HDX,
			(10_000 * UNITS) as i128,
		));
		assert_ok!(Balances::force_set_balance(
			RawOrigin::Root.into(),
			ALICE.into(),
			100_000_000 * UNITS,
		));
		assert_ok!(Balances::force_set_balance(
			RawOrigin::Root.into(),
			BOB.into(),
			1_000_000 * UNITS,
		));
		let r = begin_referendum();
		assert_ok!(Staking::stake(
			hydradx_runtime::RuntimeOrigin::signed(BOB.into()),
			1_000_000 * UNITS
		));
		assert_ok!(Staking::stake(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			222_222 * UNITS
		));

		assert_ok!(Democracy::vote(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			r,
			AccountVote::Standard {
				vote: Vote {
					aye: true,
					conviction: Conviction::Locked6x,
				},
				balance: 1_000_000 * UNITS,
			}
		));

		assert_ok!(Democracy::vote(
			hydradx_runtime::RuntimeOrigin::signed(BOB.into()),
			r,
			AccountVote::Standard {
				vote: Vote {
					aye: false,
					conviction: Conviction::Locked6x,
				},
				balance: 1_000_000 * UNITS,
			}
		));
		end_referendum();
		assert!(
			pallet_staking::Pallet::<hydradx_runtime::Runtime>::processed_votes::<AccountId, u32>(ALICE.into(), r)
				.is_none()
		);

		let r = begin_referendum() + 1;
		fast_forward_by(3 * DAYS);
		assert_ok!(Democracy::vote(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			r,
			AccountVote::Standard {
				vote: Vote {
					aye: true,
					conviction: Conviction::Locked6x,
				},
				balance: 1_000_000 * UNITS,
			}
		));
		assert!(
			pallet_staking::Pallet::<hydradx_runtime::Runtime>::processed_votes::<AccountId, u32>(ALICE.into(), 0)
				.is_some()
		);
	});
}

const DEMOCRACY_ID: LockIdentifier = *b"democrac";
fn assert_lock(who: &AccountId, amount: Balance, lock_id: LockIdentifier) {
	let locks = Balances::locks(who);
	let lock = locks.iter().find(|e| e.id == lock_id);

	assert_eq!(
		lock,
		Some(&pallet_balances::BalanceLock {
			id: lock_id,
			amount,
			reasons: pallet_balances::Reasons::All
		})
	);
}

fn assert_no_lock(who: &AccountId, lock_id: LockIdentifier) {
	let locks = Balances::locks(who);
	let lock = locks.iter().find(|e| e.id == lock_id);

	assert_eq!(lock, None);
}
