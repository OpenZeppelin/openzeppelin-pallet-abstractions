#![cfg_attr(not(feature = "std"), no_std)]

mod impl_system;

use frame_support::pallet_prelude::Get;
use sp_version::RuntimeVersion;

pub trait SystemConfig {
    type AccountId;
    type SS58Prefix;
    type Version: Get<RuntimeVersion>;
}
