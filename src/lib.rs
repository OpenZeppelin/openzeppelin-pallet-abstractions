#![cfg_attr(not(feature = "std"), no_std)]

pub mod assets;
pub mod consensus;
pub mod evm;
pub mod governance;
pub mod system;
pub mod tanssi;
pub mod weights;
pub mod xcm;
pub use crate::weights::*;
use frame_support::traits::{ConstU32, Get, OnTimestampSet};
use sp_version::RuntimeVersion;

pub trait SystemConfig: SystemWeight {
    type AccountId;
    type Lookup;
    type SS58Prefix;
    type Version: Get<RuntimeVersion>;
    type ExistentialDeposit;
    type ScheduleOrigin;
    type PreimageOrigin;
    type ProxyType;
    type ConsensusHook;
    type SlotDuration;
    type OnTimestampSet: OnTimestampSet<u64>;
}

impl<T: SystemConfig> SystemConfigFull for T {
    type AccountId = T::AccountId;
    type Lookup = T::Lookup;
    type SS58Prefix = T::SS58Prefix;
    type Version = T::Version;
    type ExistentialDeposit = T::ExistentialDeposit;
    type ScheduleOrigin = T::ScheduleOrigin;
    type PreimageOrigin = T::PreimageOrigin;
    type ProxyType = T::ProxyType;
    type ConsensusHook = T::ConsensusHook;
    type SlotDuration = T::SlotDuration;
    type OnTimestampSet = T::OnTimestampSet;
    type MaxConsumers = ConstU32<16>;
    type MaxSignatories = ConstU32<100>;
    type MaxPendingProxies = ConstU32<32>;
    type MaxProxies = ConstU32<32>;
    type MaxFreezes = ConstU32<0>;
    type MaxLocks = ConstU32<50>;
    type MaxReserves = ConstU32<50>;
}

pub trait SystemConfigFull: SystemConfig {
    type AccountId;
    type Lookup;
    type SS58Prefix;
    type Version: Get<RuntimeVersion>;
    type ExistentialDeposit;
    type ScheduleOrigin;
    type PreimageOrigin;
    type ProxyType;
    type ConsensusHook;
    type SlotDuration;
    type OnTimestampSet: OnTimestampSet<u64>;
    type MaxConsumers;
    type MaxSignatories;
    type MaxPendingProxies;
    type MaxProxies;
    type MaxFreezes;
    type MaxLocks;
    type MaxReserves;
}

pub trait ConsensusConfig: ConsensusWeight {
    type CollatorSelectionUpdateOrigin;
}

impl<T: ConsensusConfig> ConsensusConfigFull for T {
    type DisabledValidators = ();
    type MaxAuthorities = ConstU32<100_000>;
    type MaxCandidates = ConstU32<100>;
    type MaxInvulnerables = ConstU32<20>;
    type MinEligibleCollators = ConstU32<4>;
    type CollatorSelectionUpdateOrigin = T::CollatorSelectionUpdateOrigin;
}

pub trait ConsensusConfigFull: ConsensusConfig {
    type CollatorSelectionUpdateOrigin;
    type DisabledValidators;
    type MaxAuthorities;
    type MaxCandidates;
    type MaxInvulnerables;
    type MinEligibleCollators;
}

pub trait AssetsConfig: AssetsWeight {
    type ApprovalDeposit;
    type AssetAccountDeposit;
    type AssetDeposit;
    type AssetId;
    type AssetType;
    type CreateOrigin;
    type ForceOrigin;
    type ForeignAssetModifierOrigin;
    type AssetRegistrar;
    type AssetRegistrarMetadata;
    type WeightToFee;

    type Timestamp;
    type AccountId;
    type FungiblesToAccount;
    type RootOperatorAccountId;
    type AssetsToBlockAuthor;
}

pub trait GovernanceConfig: GovernanceWeight {
    type TreasuryInteriorLocation;
    type TreasuryPalletId;
    type TreasurySpendPeriod;
    type TreasuryPayoutSpendPeriod;
    type TreasuryRejectOrigin;
    type TreasurySpendOrigin;
    type ConvictionVoteLockingPeriod;
    type DispatchWhitelistedOrigin;
    type WhitelistOrigin;
    type ReferendaAlarmInterval;
    type ReferendaCancelOrigin;
    type ReferendaKillOrigin;
    type ReferendaSlash;
    type ReferendaSubmissionDeposit;
    type ReferendaSubmitOrigin;
    type ReferendaUndecidingTimeout;
}

impl<T: GovernanceConfig> GovernanceConfigFull for T {
    type TreasuryBurn = ();
    type TreasurySpendFunds = ();
    type TreasuryBurnDestination = ();
    type TreasuryMaxApprovals = ConstU32<100>;
    type ConvictionMaxVotes = ConstU32<512>;
    type ReferendaMaxQueued = ConstU32<20>;
    type ReferendaCancelOrigin = T::ReferendaCancelOrigin;
    type ReferendaKillOrigin = T::ReferendaKillOrigin;
    type ReferendaSlash = T::ReferendaSlash;
    type ReferendaSubmissionDeposit = T::ReferendaSubmissionDeposit;
    type ReferendaSubmitOrigin = T::ReferendaSubmitOrigin;
    type ReferendaUndecidingTimeout = T::ReferendaUndecidingTimeout;
    type TreasuryInteriorLocation = T::TreasuryInteriorLocation;
    type TreasuryPalletId = T::TreasuryPalletId;
    type TreasurySpendPeriod = T::TreasurySpendPeriod;
    type TreasuryPayoutSpendPeriod = T::TreasuryPayoutSpendPeriod;
    type TreasuryRejectOrigin = T::TreasuryRejectOrigin;
    type TreasurySpendOrigin = T::TreasurySpendOrigin;
    type ConvictionVoteLockingPeriod = T::ConvictionVoteLockingPeriod;
    type DispatchWhitelistedOrigin = T::DispatchWhitelistedOrigin;
    type WhitelistOrigin = T::WhitelistOrigin;
    type ReferendaAlarmInterval = T::ReferendaAlarmInterval;
}

pub trait GovernanceConfigFull: GovernanceConfig {
    type TreasuryBurn;
    type TreasurySpendFunds;
    type TreasuryBurnDestination;
    type TreasuryMaxApprovals;
    type TreasuryInteriorLocation;
    type TreasuryPalletId;
    type TreasurySpendPeriod;
    type TreasuryPayoutSpendPeriod;
    type TreasuryRejectOrigin;
    type TreasurySpendOrigin;
    type ConvictionVoteLockingPeriod;
    type ConvictionMaxVotes;
    type DispatchWhitelistedOrigin;
    type WhitelistOrigin;
    type ReferendaAlarmInterval;
    type ReferendaCancelOrigin;
    type ReferendaKillOrigin;
    type ReferendaMaxQueued;
    type ReferendaSlash;
    type ReferendaSubmissionDeposit;
    type ReferendaSubmitOrigin;
    type ReferendaUndecidingTimeout;
}

pub trait XcmConfig: XcmWeight {
    type LocationToAccountId;
    type LocalOriginToLocation;
    type AssetTransactors;
    type XcmOriginToTransactDispatchOrigin;
    type FeeManager;
    type Trader;
    type Reserves;
    type MessageQueueHeapSize;
    type MessageQueueMaxStale;
    type MessageQueueServiceWeight;
    type XcmpQueueControllerOrigin;
    type XcmpQueueMaxInboundSuspended;
    type XcmAdminOrigin;
    type MaxAssetsForTransfer;
    type ParachainMinFee;
    type XtokensReserveProviders;
    type AccountIdToLocation;
    type BaseXcmWeight;
    type CurrencyId;
    type CurrencyIdToLocation;
    type DerivativeAddressRegistrationOrigin;
    type HrmpManipulatorOrigin;
    type HrmpOpenOrigin;
    type MaxHrmpRelayFee;
    type TransactorReserveProvider;
    type SelfLocation;
    type SovereignAccountDispatcherOrigin;
    type Transactors;
    type UniversalLocation;
    type XcmWeigher;
    type XcmSender;
    type AddSupportedAssetOrigin;
    type AssetFeesFilter;
    type EditSupportedAssetOrigin;
    type SelfReserve;
    type RelayLocation;
    type PauseSupportedAssetOrigin;
    type RemoveSupportedAssetOrigin;
    type ResumeSupportedAssetOrigin;
    type WeightToFee;
    type XcmFeesAccount;
}

impl<T: XcmConfig> XcmConfigFull for T {
    type MaxActiveOutboundChannels = ConstU32<128>;
    type MaxPageSize = ConstU32<{ 1 << 16 }>;
    type LocationToAccountId = T::LocationToAccountId;
    type LocalOriginToLocation = T::LocalOriginToLocation;
    type AssetTransactors = T::AssetTransactors;
    type XcmOriginToTransactDispatchOrigin = T::XcmOriginToTransactDispatchOrigin;
    type FeeManager = T::FeeManager;
    type Trader = T::Trader;
    type Reserves = T::Reserves;
    type MessageQueueHeapSize = T::MessageQueueHeapSize;
    type MessageQueueMaxStale = T::MessageQueueMaxStale;
    type MessageQueueServiceWeight = T::MessageQueueServiceWeight;
    type XcmpQueueControllerOrigin = T::XcmpQueueControllerOrigin;
    type XcmpQueueMaxInboundSuspended = T::XcmpQueueMaxInboundSuspended;
    type XcmAdminOrigin = T::XcmAdminOrigin;
    type MaxAssetsForTransfer = T::MaxAssetsForTransfer;
    type ParachainMinFee = T::ParachainMinFee;
    type XtokensReserveProviders = T::XtokensReserveProviders;
    type AccountIdToLocation = T::AccountIdToLocation;
    type BaseXcmWeight = T::BaseXcmWeight;
    type CurrencyId = T::CurrencyId;
    type CurrencyIdToLocation = T::CurrencyIdToLocation;
    type DerivativeAddressRegistrationOrigin = T::DerivativeAddressRegistrationOrigin;
    type HrmpManipulatorOrigin = T::HrmpManipulatorOrigin;
    type HrmpOpenOrigin = T::HrmpOpenOrigin;
    type MaxHrmpRelayFee = T::MaxHrmpRelayFee;
    type TransactorReserveProvider = T::TransactorReserveProvider;
    type SelfLocation = T::SelfLocation;
    type SovereignAccountDispatcherOrigin = T::SovereignAccountDispatcherOrigin;
    type Transactors = T::Transactors;
    type UniversalLocation = T::UniversalLocation;
    type XcmWeigher = T::XcmWeigher;
    type XcmSender = T::XcmSender;
    type AddSupportedAssetOrigin = T::AddSupportedAssetOrigin;
    type AssetFeesFilter = T::AssetFeesFilter;
    type EditSupportedAssetOrigin = T::EditSupportedAssetOrigin;
    type SelfReserve = T::SelfReserve;
    type RelayLocation = T::RelayLocation;
    type PauseSupportedAssetOrigin = T::PauseSupportedAssetOrigin;
    type RemoveSupportedAssetOrigin = T::RemoveSupportedAssetOrigin;
    type ResumeSupportedAssetOrigin = T::ResumeSupportedAssetOrigin;
    type WeightToFee = T::WeightToFee;
    type XcmFeesAccount = T::XcmFeesAccount;
}

pub trait XcmConfigFull: XcmWeight {
    type LocationToAccountId;
    type LocalOriginToLocation;
    type AssetTransactors;
    type XcmOriginToTransactDispatchOrigin;
    type FeeManager;
    type Trader;
    type Reserves;
    type MessageQueueHeapSize;
    type MessageQueueMaxStale;
    type MessageQueueServiceWeight;
    type XcmpQueueControllerOrigin;
    type XcmpQueueMaxInboundSuspended;
    type XcmAdminOrigin;
    type MaxActiveOutboundChannels;
    type MaxPageSize;
    type MaxAssetsForTransfer;
    type ParachainMinFee;
    type XtokensReserveProviders;
    type AccountIdToLocation;
    type BaseXcmWeight;
    type CurrencyId;
    type CurrencyIdToLocation;
    type DerivativeAddressRegistrationOrigin;
    type HrmpManipulatorOrigin;
    type HrmpOpenOrigin;
    type MaxHrmpRelayFee;
    type TransactorReserveProvider;
    type SelfLocation;
    type SovereignAccountDispatcherOrigin;
    type Transactors;
    type UniversalLocation;
    type XcmWeigher;
    type XcmSender;
    type AddSupportedAssetOrigin;
    type AssetFeesFilter;
    type EditSupportedAssetOrigin;
    type SelfReserve;
    type RelayLocation;
    type PauseSupportedAssetOrigin;
    type RemoveSupportedAssetOrigin;
    type ResumeSupportedAssetOrigin;
    type WeightToFee;
    type XcmFeesAccount;
}

pub trait EvmConfig: EvmWeight {
    type AddressMapping;
    type FindAuthor;
    type CallOrigin;
    type WithdrawOrigin;
    type PrecompilesType;
    type PrecompilesValue;
    type Erc20XcmBridgeTransferGasLimit;
    type LocationToH160;
}

pub trait TanssiConfig: TanssiWeight {
    type AuthorInherent;
    type AuthoritiesNothing;
}

#[test]
fn example() {
    assert_eq!(1 + 1, 2);
}
