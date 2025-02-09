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

//! Mocks for the vesting module.

#![cfg(test)]

use super::*;
use crate::{self as vesting};
use frame_support::{ord_parameter_types, parameter_types};
use frame_system::EnsureSignedBy;

use sp_core::H256;
use sp_runtime::{traits::IdentityLookup, BuildStorage};

type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
	pub enum Test {
		System: frame_system::{Pallet, Call, Config<T>, Storage, Event<T>},
		PalletBalances: pallet_balances::{Pallet, Call, Config<T>, Storage, Event<T>},
		Vesting: vesting::{Pallet, Call, Storage, Event<T>, Config<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
}

pub type AccountId = u128;
impl frame_system::Config for Test {
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type BlockWeights = ();
	type BlockLength = ();
	type SS58Prefix = ();
	type Hash = H256;
	type Hashing = ::sp_runtime::traits::BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type Block = Block;
	type DbWeight = ();
	type BaseCallFilter = frame_support::traits::Everything;
	type OnSetCode = ();
	type SystemWeightInfo = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
	type Nonce = u32;
}

type Balance = u64;

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
	pub const MaxLocks: u32 = 50;
}

impl pallet_balances::Config for Test {
	type Balance = Balance;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ExistentialDeposit;
	type MaxLocks = MaxLocks;
	type AccountStore = frame_system::Pallet<Test>;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type WeightInfo = ();
	type FreezeIdentifier = [u8; 8];
	type MaxHolds = ();
	type MaxFreezes = ();
	type RuntimeHoldReason = ();
}

ord_parameter_types! {
	pub const ALICE: AccountId = 1;
	pub const BOB: AccountId = 2;
	pub const CancelOrigin: AccountId = 42;
}

parameter_types! {
	pub static MaxSchedule: u32 = 2;
}

impl Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Currency = PalletBalances;
	type CancelOrigin = EnsureSignedBy<CancelOrigin, AccountId>;
	type MaxSchedule = MaxSchedule;
	type WeightInfo = ();
	type BlockNumberProvider = frame_system::Pallet<Test>;
}

pub(crate) fn balances(who: &AccountId) -> (Balance, Balance) {
	(
		PalletBalances::free_balance(who),
		PalletBalances::free_balance(who) - PalletBalances::usable_balance(who),
	)
}

pub(crate) fn context_events() -> Vec<pallet::Event<Test>> {
	System::events()
		.into_iter()
		.filter_map(|r| {
			if let RuntimeEvent::Vesting(inner) = r.event {
				Some(inner)
			} else {
				None
			}
		})
		.collect::<Vec<_>>()
}

#[derive(Default)]
pub struct ExtBuilder {
	endowed_accounts: Vec<(AccountId, Balance)>,
}

impl ExtBuilder {
	pub fn balances(mut self, endowed_accounts: Vec<(AccountId, Balance)>) -> Self {
		self.endowed_accounts = endowed_accounts;
		self
	}

	pub fn one_hundred_for_alice(self) -> Self {
		self.balances(vec![(ALICE::get(), 100)])
	}

	pub fn build(self) -> sp_io::TestExternalities {
		sp_tracing::try_init_simple();

		let mut storage = frame_system::GenesisConfig::<Test>::default()
			.build_storage()
			.unwrap_or_else(|err| {
				panic!(
					"new_test_ext:[{:#?}] - FrameSystem GenesisConfig Err:[{:#?}]!!!",
					line!(),
					err
				)
			});

		pallet_balances::GenesisConfig::<Test> {
			balances: self.endowed_accounts.into_iter().collect::<Vec<_>>(),
		}
		.assimilate_storage(&mut storage)
		.unwrap_or_else(|err| {
			panic!(
				"new_test_ext:[{:#?}] - pallet_balances GenesisConfig Err:[{:#?}]!!!",
				line!(),
				err
			)
		});

		let mut ext = sp_io::TestExternalities::from(storage);

		ext.execute_with(|| {
			System::set_block_number(1);
		});

		ext
	}
}
