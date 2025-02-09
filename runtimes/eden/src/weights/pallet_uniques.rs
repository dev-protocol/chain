/*
 * This file is part of the Nodle Chain distributed at https://github.com/NodleCode/chain
 * Copyright (C) 2020-2022  Nodle International
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

//! Autogenerated weights for pallet_uniques
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-12-08, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `chain-bench-b606df9f`, CPU: `AMD EPYC 7B13`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/nodle-parachain
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_uniques
// --extrinsic=*
// --wasm-execution=compiled
// --template=./.maintain/external_pallet_weights.hbs
// --output=runtimes/eden/src/weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight}};
use core::marker::PhantomData;

/// Weight functions for `pallet_uniques`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_uniques::WeightInfo for WeightInfo<T> {
	// Storage: `Uniques::Class` (r:1 w:1)
	// Proof: `Uniques::Class` (`max_values`: None, `max_size`: Some(178), added: 2653, mode: `MaxEncodedLen`)
	// Storage: `Uniques::ClassAccount` (r:0 w:1)
	// Proof: `Uniques::ClassAccount` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	fn create() -> Weight {
		// Minimum execution time: 44_670 nanoseconds.
		Weight::from_parts(45_830_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: `Uniques::Class` (r:1 w:1)
	// Proof: `Uniques::Class` (`max_values`: None, `max_size`: Some(178), added: 2653, mode: `MaxEncodedLen`)
	// Storage: `Uniques::ClassAccount` (r:0 w:1)
	// Proof: `Uniques::ClassAccount` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	fn force_create() -> Weight {
		// Minimum execution time: 20_310 nanoseconds.
		Weight::from_parts(21_040_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: `Uniques::Class` (r:1 w:1)
	// Proof: `Uniques::Class` (`max_values`: None, `max_size`: Some(178), added: 2653, mode: `MaxEncodedLen`)
	// Storage: `Uniques::Asset` (r:1001 w:1000)
	// Proof: `Uniques::Asset` (`max_values`: None, `max_size`: Some(122), added: 2597, mode: `MaxEncodedLen`)
	// Storage: `Uniques::InstanceMetadataOf` (r:1000 w:1000)
	// Proof: `Uniques::InstanceMetadataOf` (`max_values`: None, `max_size`: Some(187), added: 2662, mode: `MaxEncodedLen`)
	// Storage: `Uniques::Attribute` (r:1000 w:1000)
	// Proof: `Uniques::Attribute` (`max_values`: None, `max_size`: Some(364), added: 2839, mode: `MaxEncodedLen`)
	// Storage: `Uniques::ClassAccount` (r:0 w:1)
	// Proof: `Uniques::ClassAccount` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	// Storage: `Uniques::ClassMetadataOf` (r:0 w:1)
	// Proof: `Uniques::ClassMetadataOf` (`max_values`: None, `max_size`: Some(167), added: 2642, mode: `MaxEncodedLen`)
	// Storage: `Uniques::Account` (r:0 w:1000)
	// Proof: `Uniques::Account` (`max_values`: None, `max_size`: Some(88), added: 2563, mode: `MaxEncodedLen`)
	// Storage: `Uniques::CollectionMaxSupply` (r:0 w:1)
	// Proof: `Uniques::CollectionMaxSupply` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[0, 1000]`.
	/// The range of component `m` is `[0, 1000]`.
	/// The range of component `a` is `[0, 1000]`.
	fn destroy(n: u32, m: u32, a: u32, ) -> Weight {
		// Minimum execution time: 2_764_170 nanoseconds.
		Weight::from_parts(2_790_440_000_u64, 0)
			// Standard Error: 25_587
			.saturating_add(Weight::from_parts(9_979_963_u64, 0).saturating_mul(n as u64))
			// Standard Error: 25_587
			.saturating_add(Weight::from_parts(263_483_u64, 0).saturating_mul(m as u64))
			// Standard Error: 25_587
			.saturating_add(Weight::from_parts(327_596_u64, 0).saturating_mul(a as u64))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(m as u64)))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(a as u64)))
			.saturating_add(T::DbWeight::get().writes(4_u64))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(m as u64)))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(a as u64)))
	}
	// Storage: `Uniques::Asset` (r:1 w:1)
	// Proof: `Uniques::Asset` (`max_values`: None, `max_size`: Some(122), added: 2597, mode: `MaxEncodedLen`)
	// Storage: `Uniques::Class` (r:1 w:1)
	// Proof: `Uniques::Class` (`max_values`: None, `max_size`: Some(178), added: 2653, mode: `MaxEncodedLen`)
	// Storage: `Uniques::CollectionMaxSupply` (r:1 w:0)
	// Proof: `Uniques::CollectionMaxSupply` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	// Storage: `Uniques::Account` (r:0 w:1)
	// Proof: `Uniques::Account` (`max_values`: None, `max_size`: Some(88), added: 2563, mode: `MaxEncodedLen`)
	fn mint() -> Weight {
		// Minimum execution time: 52_870 nanoseconds.
		Weight::from_parts(54_250_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: `Uniques::Class` (r:1 w:1)
	// Proof: `Uniques::Class` (`max_values`: None, `max_size`: Some(178), added: 2653, mode: `MaxEncodedLen`)
	// Storage: `Uniques::Asset` (r:1 w:1)
	// Proof: `Uniques::Asset` (`max_values`: None, `max_size`: Some(122), added: 2597, mode: `MaxEncodedLen`)
	// Storage: `Uniques::Account` (r:0 w:1)
	// Proof: `Uniques::Account` (`max_values`: None, `max_size`: Some(88), added: 2563, mode: `MaxEncodedLen`)
	// Storage: `Uniques::ItemPriceOf` (r:0 w:1)
	// Proof: `Uniques::ItemPriceOf` (`max_values`: None, `max_size`: Some(89), added: 2564, mode: `MaxEncodedLen`)
	fn burn() -> Weight {
		// Minimum execution time: 55_580 nanoseconds.
		Weight::from_parts(56_260_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	// Storage: `Uniques::Class` (r:1 w:0)
	// Proof: `Uniques::Class` (`max_values`: None, `max_size`: Some(178), added: 2653, mode: `MaxEncodedLen`)
	// Storage: `Uniques::Asset` (r:1 w:1)
	// Proof: `Uniques::Asset` (`max_values`: None, `max_size`: Some(122), added: 2597, mode: `MaxEncodedLen`)
	// Storage: `Uniques::Account` (r:0 w:2)
	// Proof: `Uniques::Account` (`max_values`: None, `max_size`: Some(88), added: 2563, mode: `MaxEncodedLen`)
	// Storage: `Uniques::ItemPriceOf` (r:0 w:1)
	// Proof: `Uniques::ItemPriceOf` (`max_values`: None, `max_size`: Some(89), added: 2564, mode: `MaxEncodedLen`)
	fn transfer() -> Weight {
		// Minimum execution time: 39_750 nanoseconds.
		Weight::from_parts(40_670_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	// Storage: `Uniques::Class` (r:1 w:1)
	// Proof: `Uniques::Class` (`max_values`: None, `max_size`: Some(178), added: 2653, mode: `MaxEncodedLen`)
	// Storage: `Uniques::Asset` (r:5000 w:5000)
	// Proof: `Uniques::Asset` (`max_values`: None, `max_size`: Some(122), added: 2597, mode: `MaxEncodedLen`)
	/// The range of component `i` is `[0, 5000]`.
	fn redeposit(i: u32, ) -> Weight {
		// Minimum execution time: 20_650 nanoseconds.
		Weight::from_parts(21_209_000_u64, 0)
			// Standard Error: 13_652
			.saturating_add(Weight::from_parts(25_338_219_u64, 0).saturating_mul(i as u64))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(i as u64)))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(i as u64)))
	}
	// Storage: `Uniques::Asset` (r:1 w:1)
	// Proof: `Uniques::Asset` (`max_values`: None, `max_size`: Some(122), added: 2597, mode: `MaxEncodedLen`)
	// Storage: `Uniques::Class` (r:1 w:0)
	// Proof: `Uniques::Class` (`max_values`: None, `max_size`: Some(178), added: 2653, mode: `MaxEncodedLen`)
	fn freeze() -> Weight {
		// Minimum execution time: 26_191 nanoseconds.
		Weight::from_parts(27_410_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `Uniques::Asset` (r:1 w:1)
	// Proof: `Uniques::Asset` (`max_values`: None, `max_size`: Some(122), added: 2597, mode: `MaxEncodedLen`)
	// Storage: `Uniques::Class` (r:1 w:0)
	// Proof: `Uniques::Class` (`max_values`: None, `max_size`: Some(178), added: 2653, mode: `MaxEncodedLen`)
	fn thaw() -> Weight {
		// Minimum execution time: 26_110 nanoseconds.
		Weight::from_parts(26_960_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `Uniques::Class` (r:1 w:1)
	// Proof: `Uniques::Class` (`max_values`: None, `max_size`: Some(178), added: 2653, mode: `MaxEncodedLen`)
	fn freeze_collection() -> Weight {
		// Minimum execution time: 19_480 nanoseconds.
		Weight::from_parts(20_020_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `Uniques::Class` (r:1 w:1)
	// Proof: `Uniques::Class` (`max_values`: None, `max_size`: Some(178), added: 2653, mode: `MaxEncodedLen`)
	fn thaw_collection() -> Weight {
		// Minimum execution time: 19_020 nanoseconds.
		Weight::from_parts(19_770_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `Uniques::OwnershipAcceptance` (r:1 w:1)
	// Proof: `Uniques::OwnershipAcceptance` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	// Storage: `Uniques::Class` (r:1 w:1)
	// Proof: `Uniques::Class` (`max_values`: None, `max_size`: Some(178), added: 2653, mode: `MaxEncodedLen`)
	// Storage: `Uniques::ClassAccount` (r:0 w:2)
	// Proof: `Uniques::ClassAccount` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	fn transfer_ownership() -> Weight {
		// Minimum execution time: 31_120 nanoseconds.
		Weight::from_parts(31_810_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	// Storage: `Uniques::Class` (r:1 w:1)
	// Proof: `Uniques::Class` (`max_values`: None, `max_size`: Some(178), added: 2653, mode: `MaxEncodedLen`)
	fn set_team() -> Weight {
		// Minimum execution time: 19_940 nanoseconds.
		Weight::from_parts(20_800_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `Uniques::Class` (r:1 w:1)
	// Proof: `Uniques::Class` (`max_values`: None, `max_size`: Some(178), added: 2653, mode: `MaxEncodedLen`)
	// Storage: `Uniques::ClassAccount` (r:0 w:1)
	// Proof: `Uniques::ClassAccount` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	fn force_item_status() -> Weight {
		// Minimum execution time: 24_100 nanoseconds.
		Weight::from_parts(24_790_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: `Uniques::Class` (r:1 w:1)
	// Proof: `Uniques::Class` (`max_values`: None, `max_size`: Some(178), added: 2653, mode: `MaxEncodedLen`)
	// Storage: `Uniques::InstanceMetadataOf` (r:1 w:0)
	// Proof: `Uniques::InstanceMetadataOf` (`max_values`: None, `max_size`: Some(187), added: 2662, mode: `MaxEncodedLen`)
	// Storage: `Uniques::Attribute` (r:1 w:1)
	// Proof: `Uniques::Attribute` (`max_values`: None, `max_size`: Some(364), added: 2839, mode: `MaxEncodedLen`)
	fn set_attribute() -> Weight {
		// Minimum execution time: 59_820 nanoseconds.
		Weight::from_parts(60_870_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: `Uniques::Class` (r:1 w:1)
	// Proof: `Uniques::Class` (`max_values`: None, `max_size`: Some(178), added: 2653, mode: `MaxEncodedLen`)
	// Storage: `Uniques::InstanceMetadataOf` (r:1 w:0)
	// Proof: `Uniques::InstanceMetadataOf` (`max_values`: None, `max_size`: Some(187), added: 2662, mode: `MaxEncodedLen`)
	// Storage: `Uniques::Attribute` (r:1 w:1)
	// Proof: `Uniques::Attribute` (`max_values`: None, `max_size`: Some(364), added: 2839, mode: `MaxEncodedLen`)
	fn clear_attribute() -> Weight {
		// Minimum execution time: 56_080 nanoseconds.
		Weight::from_parts(56_950_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: `Uniques::Class` (r:1 w:1)
	// Proof: `Uniques::Class` (`max_values`: None, `max_size`: Some(178), added: 2653, mode: `MaxEncodedLen`)
	// Storage: `Uniques::InstanceMetadataOf` (r:1 w:1)
	// Proof: `Uniques::InstanceMetadataOf` (`max_values`: None, `max_size`: Some(187), added: 2662, mode: `MaxEncodedLen`)
	fn set_metadata() -> Weight {
		// Minimum execution time: 44_000 nanoseconds.
		Weight::from_parts(44_830_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: `Uniques::Class` (r:1 w:1)
	// Proof: `Uniques::Class` (`max_values`: None, `max_size`: Some(178), added: 2653, mode: `MaxEncodedLen`)
	// Storage: `Uniques::InstanceMetadataOf` (r:1 w:1)
	// Proof: `Uniques::InstanceMetadataOf` (`max_values`: None, `max_size`: Some(187), added: 2662, mode: `MaxEncodedLen`)
	fn clear_metadata() -> Weight {
		// Minimum execution time: 44_780 nanoseconds.
		Weight::from_parts(45_830_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: `Uniques::Class` (r:1 w:1)
	// Proof: `Uniques::Class` (`max_values`: None, `max_size`: Some(178), added: 2653, mode: `MaxEncodedLen`)
	// Storage: `Uniques::ClassMetadataOf` (r:1 w:1)
	// Proof: `Uniques::ClassMetadataOf` (`max_values`: None, `max_size`: Some(167), added: 2642, mode: `MaxEncodedLen`)
	fn set_collection_metadata() -> Weight {
		// Minimum execution time: 45_690 nanoseconds.
		Weight::from_parts(46_660_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: `Uniques::Class` (r:1 w:0)
	// Proof: `Uniques::Class` (`max_values`: None, `max_size`: Some(178), added: 2653, mode: `MaxEncodedLen`)
	// Storage: `Uniques::ClassMetadataOf` (r:1 w:1)
	// Proof: `Uniques::ClassMetadataOf` (`max_values`: None, `max_size`: Some(167), added: 2642, mode: `MaxEncodedLen`)
	fn clear_collection_metadata() -> Weight {
		// Minimum execution time: 44_650 nanoseconds.
		Weight::from_parts(45_380_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `Uniques::Class` (r:1 w:0)
	// Proof: `Uniques::Class` (`max_values`: None, `max_size`: Some(178), added: 2653, mode: `MaxEncodedLen`)
	// Storage: `Uniques::Asset` (r:1 w:1)
	// Proof: `Uniques::Asset` (`max_values`: None, `max_size`: Some(122), added: 2597, mode: `MaxEncodedLen`)
	fn approve_transfer() -> Weight {
		// Minimum execution time: 26_710 nanoseconds.
		Weight::from_parts(27_660_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `Uniques::Class` (r:1 w:0)
	// Proof: `Uniques::Class` (`max_values`: None, `max_size`: Some(178), added: 2653, mode: `MaxEncodedLen`)
	// Storage: `Uniques::Asset` (r:1 w:1)
	// Proof: `Uniques::Asset` (`max_values`: None, `max_size`: Some(122), added: 2597, mode: `MaxEncodedLen`)
	fn cancel_approval() -> Weight {
		// Minimum execution time: 26_540 nanoseconds.
		Weight::from_parts(27_600_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `Uniques::OwnershipAcceptance` (r:1 w:1)
	// Proof: `Uniques::OwnershipAcceptance` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	fn set_accept_ownership() -> Weight {
		// Minimum execution time: 22_140 nanoseconds.
		Weight::from_parts(22_820_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `Uniques::CollectionMaxSupply` (r:1 w:1)
	// Proof: `Uniques::CollectionMaxSupply` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	// Storage: `Uniques::Class` (r:1 w:0)
	// Proof: `Uniques::Class` (`max_values`: None, `max_size`: Some(178), added: 2653, mode: `MaxEncodedLen`)
	fn set_collection_max_supply() -> Weight {
		// Minimum execution time: 23_420 nanoseconds.
		Weight::from_parts(24_150_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `Uniques::Asset` (r:1 w:0)
	// Proof: `Uniques::Asset` (`max_values`: None, `max_size`: Some(122), added: 2597, mode: `MaxEncodedLen`)
	// Storage: `Uniques::ItemPriceOf` (r:0 w:1)
	// Proof: `Uniques::ItemPriceOf` (`max_values`: None, `max_size`: Some(89), added: 2564, mode: `MaxEncodedLen`)
	fn set_price() -> Weight {
		// Minimum execution time: 22_810 nanoseconds.
		Weight::from_parts(23_860_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `Uniques::Asset` (r:1 w:1)
	// Proof: `Uniques::Asset` (`max_values`: None, `max_size`: Some(122), added: 2597, mode: `MaxEncodedLen`)
	// Storage: `Uniques::ItemPriceOf` (r:1 w:1)
	// Proof: `Uniques::ItemPriceOf` (`max_values`: None, `max_size`: Some(89), added: 2564, mode: `MaxEncodedLen`)
	// Storage: `Uniques::Class` (r:1 w:0)
	// Proof: `Uniques::Class` (`max_values`: None, `max_size`: Some(178), added: 2653, mode: `MaxEncodedLen`)
	// Storage: `Uniques::Account` (r:0 w:2)
	// Proof: `Uniques::Account` (`max_values`: None, `max_size`: Some(88), added: 2563, mode: `MaxEncodedLen`)
	fn buy_item() -> Weight {
		// Minimum execution time: 53_150 nanoseconds.
		Weight::from_parts(54_180_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
}
