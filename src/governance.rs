//! Implements the OpenZeppelin governance configuration for a Runtime.
//!
//! This macro sets up the necessary configurations for the following pallets:
//! - `pallet_sudo`
//! - `pallet_treasury`
//! - `pallet_conviction_voting`
//! - `pallet_whitelist`
//! - `pallet_custom_origins`
//! - `pallet_referenda`
//!
//! # Parameters
//! - `$t`: A type that implements the `GovernanceConfigFull` trait, providing the necessary associated types
//!   and configurations.
//!
//! # Important
//! Rerun benchmarks if making changes to runtime configuration, as weight calculations
//! may need to be updated.

#[macro_export]
macro_rules! impl_openzeppelin_governance {
    ($t:ty) => {
        impl pallet_sudo::Config for Runtime {
            type RuntimeCall = RuntimeCall;
            type RuntimeEvent = RuntimeEvent;
            type WeightInfo = <$t as GovernanceWeight>::Sudo;
        }

        #[cfg(feature = "runtime-benchmarks")]
        parameter_types! {
            pub LocationParents: u8 = 1;
            pub BenchmarkParaId: u8 = 0;
        }

        impl pallet_treasury::Config for Runtime {
            type AssetKind = AssetKind;
            type BalanceConverter = frame_support::traits::tokens::UnityAssetBalanceConversion;
            #[cfg(feature = "runtime-benchmarks")]
            type BenchmarkHelper = polkadot_runtime_common::impls::benchmarks::TreasuryArguments<
                LocationParents,
                BenchmarkParaId,
            >;
            type Beneficiary = Beneficiary;
            type BeneficiaryLookup = IdentityLookup<Self::Beneficiary>;
            type Burn = <$t as openzeppelin_pallet_abstractions::GovernanceConfigFull>::TreasuryBurn;
            type BurnDestination = <$t as openzeppelin_pallet_abstractions::GovernanceConfigFull>::TreasuryBurnDestination;
            type Currency = Balances;
            type MaxApprovals = <$t as openzeppelin_pallet_abstractions::GovernanceConfigFull>::TreasuryMaxApprovals;
            type PalletId = <$t as openzeppelin_pallet_abstractions::GovernanceConfigFull>::TreasuryPalletId;
            #[cfg(feature = "runtime-benchmarks")]
            type Paymaster = PayWithEnsure<TreasuryPaymaster, OpenHrmpChannel<BenchmarkParaId>>;
            #[cfg(not(feature = "runtime-benchmarks"))]
            type Paymaster = TreasuryPaymaster;
            type PayoutPeriod = <$t as openzeppelin_pallet_abstractions::GovernanceConfigFull>::TreasuryPayoutSpendPeriod;
            type RejectOrigin = <$t as openzeppelin_pallet_abstractions::GovernanceConfigFull>::TreasuryRejectOrigin;
            type RuntimeEvent = RuntimeEvent;
            type SpendFunds = <$t as openzeppelin_pallet_abstractions::GovernanceConfigFull>::TreasurySpendFunds;
            type SpendOrigin = <$t as openzeppelin_pallet_abstractions::GovernanceConfigFull>::TreasurySpendOrigin;
            type SpendPeriod = <$t as openzeppelin_pallet_abstractions::GovernanceConfigFull>::TreasurySpendPeriod;
            type WeightInfo = <$t as GovernanceWeight>::Treasury;
        }

        impl pallet_conviction_voting::Config for Runtime {
            type Currency = Balances;
            type MaxTurnout = frame_support::traits::tokens::currency::ActiveIssuanceOf<
                Balances,
                Self::AccountId,
            >;
            type MaxVotes = <$t as openzeppelin_pallet_abstractions::GovernanceConfigFull>::ConvictionMaxVotes;
            type Polls = Referenda;
            type RuntimeEvent = RuntimeEvent;
            type VoteLockingPeriod = <$t as openzeppelin_pallet_abstractions::GovernanceConfigFull>::ConvictionVoteLockingPeriod;
            type WeightInfo =<$t as GovernanceWeight>::ConvictionVoting;
        }

        impl pallet_whitelist::Config for Runtime {
            type DispatchWhitelistedOrigin = <$t as openzeppelin_pallet_abstractions::GovernanceConfigFull>::DispatchWhitelistedOrigin;
            type Preimages = Preimage;
            type RuntimeCall = RuntimeCall;
            type RuntimeEvent = RuntimeEvent;
            type WeightInfo = <$t as GovernanceWeight>::Whitelist;
            type WhitelistOrigin = <$t as openzeppelin_pallet_abstractions::GovernanceConfigFull>::WhitelistOrigin;
        }

        impl pallet_custom_origins::Config for Runtime {}

        parameter_types! {
            pub const MaxBalance: Balance = Balance::MAX;
        }
        pub type TreasurySpender = EitherOf<EnsureRootWithSuccess<AccountId, MaxBalance>, Spender>;

        impl pallet_referenda::Config for Runtime {
            type AlarmInterval = <$t as openzeppelin_pallet_abstractions::GovernanceConfigFull>::ReferendaAlarmInterval;
            type CancelOrigin = <$t as openzeppelin_pallet_abstractions::GovernanceConfigFull>::ReferendaCancelOrigin;
            type Currency = Balances;
            type KillOrigin = <$t as openzeppelin_pallet_abstractions::GovernanceConfigFull>::ReferendaKillOrigin;
            type MaxQueued = <$t as openzeppelin_pallet_abstractions::GovernanceConfigFull>::ReferendaMaxQueued;
            type Preimages = Preimage;
            type RuntimeCall = RuntimeCall;
            type RuntimeEvent = RuntimeEvent;
            type Scheduler = Scheduler;
            type Slash = <$t as openzeppelin_pallet_abstractions::GovernanceConfigFull>::ReferendaSlash;
            type SubmissionDeposit = <$t as openzeppelin_pallet_abstractions::GovernanceConfigFull>::ReferendaSubmissionDeposit;
            type SubmitOrigin = <$t as openzeppelin_pallet_abstractions::GovernanceConfigFull>::ReferendaSubmitOrigin;
            type Tally = pallet_conviction_voting::TallyOf<Runtime>;
            type Tracks = tracks::TracksInfo;
            type UndecidingTimeout = <$t as openzeppelin_pallet_abstractions::GovernanceConfigFull>::ReferendaUndecidingTimeout;
            type Votes = pallet_conviction_voting::VotesOf<Runtime>;
            type WeightInfo = <$t as GovernanceWeight>::Referenda;
        }
    };
}

pub const PALLET_NAMES: [(&str, &str); 6] = [
    ("Sudo", "pallet_sudo"),
    ("Treasury", "pallet_treasury"),
    ("ConvictionVoting", "pallet_conviction_voting"),
    ("Whitelist", "pallet_whitelist"),
    ("Origins", "pallet_custom_origins"),
    ("Referenda", "pallet_referenda"),
];
