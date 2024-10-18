// This file is part of HydraDX.

// Copyright (C) 2020-2023  Intergalactic, Limited (GIB).
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.


//! Autogenerated weights for `pallet_omnipool_liquidity_mining`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-09-19, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bench-bot`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// target/release/hydradx
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --wasm-execution=compiled
// --pallet=pallet-omnipool-liquidity-mining
// --extrinsic=*
// --template=scripts/pallet-weight-template.hbs
// --output=./weights/pallet_omnipool_liquidity_mining.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_omnipool_liquidity_mining.
pub trait WeightInfo {
	fn create_global_farm() -> Weight;
	fn update_global_farm() -> Weight;
	fn terminate_global_farm() -> Weight;
	fn create_yield_farm() -> Weight;
	fn update_yield_farm() -> Weight;
	fn stop_yield_farm() -> Weight;
	fn resume_yield_farm() -> Weight;
	fn terminate_yield_farm() -> Weight;
	fn deposit_shares() -> Weight;
	fn redeposit_shares() -> Weight;
	fn claim_rewards() -> Weight;
	fn withdraw_shares() -> Weight;	
	fn join_farms(c: u32) -> Weight;	
	fn add_liquidity_and_join_farms(c: u32) -> Weight;
}

/// Weights for pallet_omnipool_liquidity_mining using the hydraDX node and recommended hardware.
impl WeightInfo for () {
	/// Storage: `AssetRegistry::Assets` (r:2 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `OmnipoolWarehouseLM::FarmSequencer` (r:1 w:1)
	/// Proof: `OmnipoolWarehouseLM::FarmSequencer` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:0 w:1)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `OmnipoolWarehouseLM::GlobalFarm` (r:0 w:1)
	/// Proof: `OmnipoolWarehouseLM::GlobalFarm` (`max_values`: None, `max_size`: Some(205), added: 2680, mode: `MaxEncodedLen`)
	fn create_global_farm() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `557`
		//  Estimated: `6196`
		// Minimum execution time: 58_000_000 picoseconds.
		Weight::from_parts(63_000_000, 6196)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	/// Storage: `OmnipoolWarehouseLM::GlobalFarm` (r:1 w:1)
	/// Proof: `OmnipoolWarehouseLM::GlobalFarm` (`max_values`: None, `max_size`: Some(205), added: 2680, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn update_global_farm() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `573`
		//  Estimated: `3670`
		// Minimum execution time: 13_000_000 picoseconds.
		Weight::from_parts(14_000_000, 3670)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `OmnipoolWarehouseLM::GlobalFarm` (r:1 w:1)
	/// Proof: `OmnipoolWarehouseLM::GlobalFarm` (`max_values`: None, `max_size`: Some(205), added: 2680, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:1 w:1)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	fn terminate_global_farm() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `964`
		//  Estimated: `6196`
		// Minimum execution time: 59_000_000 picoseconds.
		Weight::from_parts(59_000_000, 6196)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: `Omnipool::Assets` (r:1 w:0)
	/// Proof: `Omnipool::Assets` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// Storage: `OmnipoolWarehouseLM::GlobalFarm` (r:1 w:1)
	/// Proof: `OmnipoolWarehouseLM::GlobalFarm` (`max_values`: None, `max_size`: Some(205), added: 2680, mode: `MaxEncodedLen`)
	/// Storage: `OmnipoolWarehouseLM::ActiveYieldFarm` (r:1 w:1)
	/// Proof: `OmnipoolWarehouseLM::ActiveYieldFarm` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:1 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Oracles` (r:2 w:0)
	/// Proof: `EmaOracle::Oracles` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	/// Storage: `OmnipoolWarehouseLM::FarmSequencer` (r:1 w:1)
	/// Proof: `OmnipoolWarehouseLM::FarmSequencer` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `OmnipoolWarehouseLM::YieldFarm` (r:0 w:1)
	/// Proof: `OmnipoolWarehouseLM::YieldFarm` (`max_values`: None, `max_size`: Some(198), added: 2673, mode: `MaxEncodedLen`)
	fn create_yield_farm() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2337`
		//  Estimated: `6294`
		// Minimum execution time: 88_000_000 picoseconds.
		Weight::from_parts(89_000_000, 6294)
			.saturating_add(RocksDbWeight::get().reads(9_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
	/// Storage: `Omnipool::Assets` (r:1 w:0)
	/// Proof: `Omnipool::Assets` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// Storage: `OmnipoolWarehouseLM::ActiveYieldFarm` (r:1 w:0)
	/// Proof: `OmnipoolWarehouseLM::ActiveYieldFarm` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// Storage: `OmnipoolWarehouseLM::YieldFarm` (r:1 w:1)
	/// Proof: `OmnipoolWarehouseLM::YieldFarm` (`max_values`: None, `max_size`: Some(198), added: 2673, mode: `MaxEncodedLen`)
	/// Storage: `OmnipoolWarehouseLM::GlobalFarm` (r:1 w:1)
	/// Proof: `OmnipoolWarehouseLM::GlobalFarm` (`max_values`: None, `max_size`: Some(205), added: 2680, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:1 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Oracles` (r:2 w:0)
	/// Proof: `EmaOracle::Oracles` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	fn update_yield_farm() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2507`
		//  Estimated: `6294`
		// Minimum execution time: 91_000_000 picoseconds.
		Weight::from_parts(92_000_000, 6294)
			.saturating_add(RocksDbWeight::get().reads(9_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: `OmnipoolWarehouseLM::ActiveYieldFarm` (r:1 w:1)
	/// Proof: `OmnipoolWarehouseLM::ActiveYieldFarm` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// Storage: `OmnipoolWarehouseLM::YieldFarm` (r:1 w:1)
	/// Proof: `OmnipoolWarehouseLM::YieldFarm` (`max_values`: None, `max_size`: Some(198), added: 2673, mode: `MaxEncodedLen`)
	/// Storage: `OmnipoolWarehouseLM::GlobalFarm` (r:1 w:1)
	/// Proof: `OmnipoolWarehouseLM::GlobalFarm` (`max_values`: None, `max_size`: Some(205), added: 2680, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:1 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Oracles` (r:2 w:0)
	/// Proof: `EmaOracle::Oracles` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	fn stop_yield_farm() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2146`
		//  Estimated: `6294`
		// Minimum execution time: 86_000_000 picoseconds.
		Weight::from_parts(88_000_000, 6294)
			.saturating_add(RocksDbWeight::get().reads(8_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	/// Storage: `Omnipool::Assets` (r:1 w:0)
	/// Proof: `Omnipool::Assets` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// Storage: `OmnipoolWarehouseLM::ActiveYieldFarm` (r:1 w:1)
	/// Proof: `OmnipoolWarehouseLM::ActiveYieldFarm` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// Storage: `OmnipoolWarehouseLM::YieldFarm` (r:1 w:1)
	/// Proof: `OmnipoolWarehouseLM::YieldFarm` (`max_values`: None, `max_size`: Some(198), added: 2673, mode: `MaxEncodedLen`)
	/// Storage: `OmnipoolWarehouseLM::GlobalFarm` (r:1 w:1)
	/// Proof: `OmnipoolWarehouseLM::GlobalFarm` (`max_values`: None, `max_size`: Some(205), added: 2680, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:1 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Oracles` (r:2 w:0)
	/// Proof: `EmaOracle::Oracles` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	fn resume_yield_farm() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2543`
		//  Estimated: `6294`
		// Minimum execution time: 89_000_000 picoseconds.
		Weight::from_parts(91_000_000, 6294)
			.saturating_add(RocksDbWeight::get().reads(9_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	/// Storage: `OmnipoolWarehouseLM::ActiveYieldFarm` (r:1 w:0)
	/// Proof: `OmnipoolWarehouseLM::ActiveYieldFarm` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// Storage: `OmnipoolWarehouseLM::GlobalFarm` (r:1 w:1)
	/// Proof: `OmnipoolWarehouseLM::GlobalFarm` (`max_values`: None, `max_size`: Some(205), added: 2680, mode: `MaxEncodedLen`)
	/// Storage: `OmnipoolWarehouseLM::YieldFarm` (r:1 w:1)
	/// Proof: `OmnipoolWarehouseLM::YieldFarm` (`max_values`: None, `max_size`: Some(198), added: 2673, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn terminate_yield_farm() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `924`
		//  Estimated: `6196`
		// Minimum execution time: 54_000_000 picoseconds.
		Weight::from_parts(56_000_000, 6196)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: `Uniques::Asset` (r:2 w:2)
	/// Proof: `Uniques::Asset` (`max_values`: None, `max_size`: Some(146), added: 2621, mode: `MaxEncodedLen`)
	/// Storage: `Omnipool::Positions` (r:1 w:0)
	/// Proof: `Omnipool::Positions` (`max_values`: None, `max_size`: Some(100), added: 2575, mode: `MaxEncodedLen`)
	/// Storage: `Omnipool::Assets` (r:1 w:0)
	/// Proof: `Omnipool::Assets` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// Storage: `OmnipoolWarehouseLM::YieldFarm` (r:1 w:1)
	/// Proof: `OmnipoolWarehouseLM::YieldFarm` (`max_values`: None, `max_size`: Some(198), added: 2673, mode: `MaxEncodedLen`)
	/// Storage: `OmnipoolWarehouseLM::GlobalFarm` (r:1 w:1)
	/// Proof: `OmnipoolWarehouseLM::GlobalFarm` (`max_values`: None, `max_size`: Some(205), added: 2680, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:1 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Oracles` (r:4 w:0)
	/// Proof: `EmaOracle::Oracles` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	/// Storage: `OmnipoolWarehouseLM::DepositSequencer` (r:1 w:1)
	/// Proof: `OmnipoolWarehouseLM::DepositSequencer` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::Class` (r:2 w:1)
	/// Proof: `Uniques::Class` (`max_values`: None, `max_size`: Some(190), added: 2665, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::CollectionMaxSupply` (r:1 w:0)
	/// Proof: `Uniques::CollectionMaxSupply` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::Account` (r:0 w:3)
	/// Proof: `Uniques::Account` (`max_values`: None, `max_size`: Some(112), added: 2587, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::ItemPriceOf` (r:0 w:1)
	/// Proof: `Uniques::ItemPriceOf` (`max_values`: None, `max_size`: Some(113), added: 2588, mode: `MaxEncodedLen`)
	/// Storage: `OmnipoolLiquidityMining::OmniPositionId` (r:0 w:1)
	/// Proof: `OmnipoolLiquidityMining::OmniPositionId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `OmnipoolWarehouseLM::Deposit` (r:0 w:1)
	/// Proof: `OmnipoolWarehouseLM::Deposit` (`max_values`: None, `max_size`: Some(385), added: 2860, mode: `MaxEncodedLen`)
	fn deposit_shares() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4072`
		//  Estimated: `11598`
		// Minimum execution time: 148_000_000 picoseconds.
		Weight::from_parts(152_000_000, 11598)
			.saturating_add(RocksDbWeight::get().reads(17_u64))
			.saturating_add(RocksDbWeight::get().writes(14_u64))
	}
	/// Storage: `Uniques::Asset` (r:2 w:0)
	/// Proof: `Uniques::Asset` (`max_values`: None, `max_size`: Some(146), added: 2621, mode: `MaxEncodedLen`)
	/// Storage: `OmnipoolLiquidityMining::OmniPositionId` (r:1 w:0)
	/// Proof: `OmnipoolLiquidityMining::OmniPositionId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Omnipool::Positions` (r:1 w:0)
	/// Proof: `Omnipool::Positions` (`max_values`: None, `max_size`: Some(100), added: 2575, mode: `MaxEncodedLen`)
	/// Storage: `Omnipool::Assets` (r:1 w:0)
	/// Proof: `Omnipool::Assets` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// Storage: `OmnipoolWarehouseLM::Deposit` (r:1 w:1)
	/// Proof: `OmnipoolWarehouseLM::Deposit` (`max_values`: None, `max_size`: Some(385), added: 2860, mode: `MaxEncodedLen`)
	/// Storage: `OmnipoolWarehouseLM::YieldFarm` (r:1 w:1)
	/// Proof: `OmnipoolWarehouseLM::YieldFarm` (`max_values`: None, `max_size`: Some(198), added: 2673, mode: `MaxEncodedLen`)
	/// Storage: `OmnipoolWarehouseLM::GlobalFarm` (r:1 w:1)
	/// Proof: `OmnipoolWarehouseLM::GlobalFarm` (`max_values`: None, `max_size`: Some(205), added: 2680, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:1 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Oracles` (r:4 w:0)
	/// Proof: `EmaOracle::Oracles` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	fn redeposit_shares() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4381`
		//  Estimated: `11598`
		// Minimum execution time: 125_000_000 picoseconds.
		Weight::from_parts(129_000_000, 11598)
			.saturating_add(RocksDbWeight::get().reads(15_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	fn claim_rewards() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_000_000 picoseconds.
		Weight::from_parts(2_000_000, 0)
	}
	/// Storage: `Uniques::Asset` (r:2 w:2)
	/// Proof: `Uniques::Asset` (`max_values`: None, `max_size`: Some(146), added: 2621, mode: `MaxEncodedLen`)
	/// Storage: `OmnipoolLiquidityMining::OmniPositionId` (r:1 w:1)
	/// Proof: `OmnipoolLiquidityMining::OmniPositionId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Omnipool::Positions` (r:1 w:0)
	/// Proof: `Omnipool::Positions` (`max_values`: None, `max_size`: Some(100), added: 2575, mode: `MaxEncodedLen`)
	/// Storage: `OmnipoolWarehouseLM::Deposit` (r:1 w:1)
	/// Proof: `OmnipoolWarehouseLM::Deposit` (`max_values`: None, `max_size`: Some(385), added: 2860, mode: `MaxEncodedLen`)
	/// Storage: `OmnipoolWarehouseLM::YieldFarm` (r:1 w:1)
	/// Proof: `OmnipoolWarehouseLM::YieldFarm` (`max_values`: None, `max_size`: Some(198), added: 2673, mode: `MaxEncodedLen`)
	/// Storage: `OmnipoolWarehouseLM::GlobalFarm` (r:1 w:1)
	/// Proof: `OmnipoolWarehouseLM::GlobalFarm` (`max_values`: None, `max_size`: Some(205), added: 2680, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:1 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:3 w:3)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Oracles` (r:2 w:0)
	/// Proof: `EmaOracle::Oracles` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::Class` (r:2 w:1)
	/// Proof: `Uniques::Class` (`max_values`: None, `max_size`: Some(190), added: 2665, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::Account` (r:0 w:3)
	/// Proof: `Uniques::Account` (`max_values`: None, `max_size`: Some(112), added: 2587, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::ItemPriceOf` (r:0 w:2)
	/// Proof: `Uniques::ItemPriceOf` (`max_values`: None, `max_size`: Some(113), added: 2588, mode: `MaxEncodedLen`)
	fn withdraw_shares() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3940`
		//  Estimated: `8799`
		// Minimum execution time: 191_000_000 picoseconds.
		Weight::from_parts(197_000_000, 8799)
			.saturating_add(RocksDbWeight::get().reads(15_u64))
			.saturating_add(RocksDbWeight::get().writes(15_u64))
	}
	/// Storage: `Uniques::Asset` (r:2 w:2)
	/// Proof: `Uniques::Asset` (`max_values`: None, `max_size`: Some(146), added: 2621, mode: `MaxEncodedLen`)
	/// Storage: `Omnipool::Positions` (r:1 w:0)
	/// Proof: `Omnipool::Positions` (`max_values`: None, `max_size`: Some(100), added: 2575, mode: `MaxEncodedLen`)
	/// Storage: `Omnipool::Assets` (r:1 w:0)
	/// Proof: `Omnipool::Assets` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// Storage: `OmnipoolWarehouseLM::YieldFarm` (r:5 w:5)
	/// Proof: `OmnipoolWarehouseLM::YieldFarm` (`max_values`: None, `max_size`: Some(198), added: 2673, mode: `MaxEncodedLen`)
	/// Storage: `OmnipoolWarehouseLM::GlobalFarm` (r:5 w:5)
	/// Proof: `OmnipoolWarehouseLM::GlobalFarm` (`max_values`: None, `max_size`: Some(205), added: 2680, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Oracles` (r:2 w:0)
	/// Proof: `EmaOracle::Oracles` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	/// Storage: `OmnipoolWarehouseLM::DepositSequencer` (r:1 w:1)
	/// Proof: `OmnipoolWarehouseLM::DepositSequencer` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::Class` (r:2 w:1)
	/// Proof: `Uniques::Class` (`max_values`: None, `max_size`: Some(190), added: 2665, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::CollectionMaxSupply` (r:1 w:0)
	/// Proof: `Uniques::CollectionMaxSupply` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::Account` (r:0 w:3)
	/// Proof: `Uniques::Account` (`max_values`: None, `max_size`: Some(112), added: 2587, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::ItemPriceOf` (r:0 w:1)
	/// Proof: `Uniques::ItemPriceOf` (`max_values`: None, `max_size`: Some(113), added: 2588, mode: `MaxEncodedLen`)
	/// Storage: `OmnipoolLiquidityMining::OmniPositionId` (r:0 w:1)
	/// Proof: `OmnipoolLiquidityMining::OmniPositionId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `OmnipoolWarehouseLM::Deposit` (r:0 w:1)
	/// Proof: `OmnipoolWarehouseLM::Deposit` (`max_values`: None, `max_size`: Some(385), added: 2860, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[1, 5]`.
	fn join_farms(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2751 + c * (335 ±0)`
		//  Estimated: `6320 + c * (2680 ±0)`
		// Minimum execution time: 86_000_000 picoseconds.
		Weight::from_parts(63_894_217, 6320)
			// Standard Error: 282_573
			.saturating_add(Weight::from_parts(24_545_151, 0).saturating_mul(c.into()))
			.saturating_add(RocksDbWeight::get().reads(10_u64))
			.saturating_add(RocksDbWeight::get().reads((2_u64).saturating_mul(c.into())))
			.saturating_add(RocksDbWeight::get().writes(10_u64))
			.saturating_add(RocksDbWeight::get().writes((2_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 2680).saturating_mul(c.into()))
	}
	/// Storage: `Tokens::Accounts` (r:3 w:3)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `Omnipool::Assets` (r:1 w:1)
	/// Proof: `Omnipool::Assets` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Oracles` (r:3 w:0)
	/// Proof: `EmaOracle::Oracles` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	/// Storage: `Omnipool::HubAssetImbalance` (r:1 w:1)
	/// Proof: `Omnipool::HubAssetImbalance` (`max_values`: Some(1), `max_size`: Some(17), added: 512, mode: `MaxEncodedLen`)
	/// Storage: `Omnipool::NextPositionId` (r:1 w:1)
	/// Proof: `Omnipool::NextPositionId` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::Asset` (r:2 w:2)
	/// Proof: `Uniques::Asset` (`max_values`: None, `max_size`: Some(146), added: 2621, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::Class` (r:2 w:2)
	/// Proof: `Uniques::Class` (`max_values`: None, `max_size`: Some(190), added: 2665, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::CollectionMaxSupply` (r:2 w:0)
	/// Proof: `Uniques::CollectionMaxSupply` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	/// Storage: `Router::SkipEd` (r:1 w:0)
	/// Proof: `Router::SkipEd` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:1 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:2 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:2 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::TotalIssuance` (r:1 w:1)
	/// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Accumulator` (r:1 w:1)
	/// Proof: `EmaOracle::Accumulator` (`max_values`: Some(1), `max_size`: Some(5921), added: 6416, mode: `MaxEncodedLen`)
	/// Storage: `CircuitBreaker::LiquidityAddLimitPerAsset` (r:1 w:0)
	/// Proof: `CircuitBreaker::LiquidityAddLimitPerAsset` (`max_values`: None, `max_size`: Some(29), added: 2504, mode: `MaxEncodedLen`)
	/// Storage: `CircuitBreaker::AllowedAddLiquidityAmountPerAsset` (r:1 w:1)
	/// Proof: `CircuitBreaker::AllowedAddLiquidityAmountPerAsset` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `CircuitBreaker::LiquidityRemoveLimitPerAsset` (r:1 w:0)
	/// Proof: `CircuitBreaker::LiquidityRemoveLimitPerAsset` (`max_values`: None, `max_size`: Some(29), added: 2504, mode: `MaxEncodedLen`)
	/// Storage: `CircuitBreaker::AllowedRemoveLiquidityAmountPerAsset` (r:1 w:1)
	/// Proof: `CircuitBreaker::AllowedRemoveLiquidityAmountPerAsset` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `OmnipoolWarehouseLM::YieldFarm` (r:5 w:5)
	/// Proof: `OmnipoolWarehouseLM::YieldFarm` (`max_values`: None, `max_size`: Some(198), added: 2673, mode: `MaxEncodedLen`)
	/// Storage: `OmnipoolWarehouseLM::GlobalFarm` (r:5 w:5)
	/// Proof: `OmnipoolWarehouseLM::GlobalFarm` (`max_values`: None, `max_size`: Some(205), added: 2680, mode: `MaxEncodedLen`)
	/// Storage: `OmnipoolWarehouseLM::DepositSequencer` (r:1 w:1)
	/// Proof: `OmnipoolWarehouseLM::DepositSequencer` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AccountCurrencyMap` (r:0 w:1)
	/// Proof: `MultiTransactionPayment::AccountCurrencyMap` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::Account` (r:0 w:3)
	/// Proof: `Uniques::Account` (`max_values`: None, `max_size`: Some(112), added: 2587, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::ItemPriceOf` (r:0 w:1)
	/// Proof: `Uniques::ItemPriceOf` (`max_values`: None, `max_size`: Some(113), added: 2588, mode: `MaxEncodedLen`)
	/// Storage: `OmnipoolLiquidityMining::OmniPositionId` (r:0 w:1)
	/// Proof: `OmnipoolLiquidityMining::OmniPositionId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Omnipool::Positions` (r:0 w:1)
	/// Proof: `Omnipool::Positions` (`max_values`: None, `max_size`: Some(100), added: 2575, mode: `MaxEncodedLen`)
	/// Storage: `OmnipoolWarehouseLM::Deposit` (r:0 w:1)
	/// Proof: `OmnipoolWarehouseLM::Deposit` (`max_values`: None, `max_size`: Some(385), added: 2860, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[1, 5]`.
	fn add_liquidity_and_join_farms(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4558 + c * (335 ±0)`
		//  Estimated: `8946 + c * (2680 ±0)`
		// Minimum execution time: 219_000_000 picoseconds.
		Weight::from_parts(201_456_775, 8946)
			// Standard Error: 443_301
			.saturating_add(Weight::from_parts(23_820_327, 0).saturating_mul(c.into()))
			.saturating_add(RocksDbWeight::get().reads(29_u64))
			.saturating_add(RocksDbWeight::get().reads((2_u64).saturating_mul(c.into())))
			.saturating_add(RocksDbWeight::get().writes(24_u64))
			.saturating_add(RocksDbWeight::get().writes((2_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 2680).saturating_mul(c.into()))
	}
}
