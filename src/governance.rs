//! Governance pallet groupings wrapper

#[macro_export]
macro_rules! impl_openzeppelin_governance {
    ($t:ty) => {
        impl pallet_sudo::Config for Runtime {
            type RuntimeCall = RuntimeCall;
            type RuntimeEvent = RuntimeEvent;
            /// Rerun benchmarks if you are making changes to runtime configuration.
            type WeightInfo = weights::pallet_sudo::WeightInfo<Runtime>;
        }

        #[cfg(feature = "runtime-benchmarks")]
        parameter_types! {
            pub LocationParents: u8 = 1;
            pub BenchmarkParaId: u8 = 0;
        }

        parameter_types! {
            pub TreasuryAccount: AccountId = Treasury::account_id();
        }

        impl pallet_treasury::Config for Runtime {
            type ApproveOrigin = <$t as GovernanceConfig>::TreasuryApproveOrigin;
            type AssetKind = AssetKind;
            type BalanceConverter = frame_support::traits::tokens::UnityAssetBalanceConversion;
            #[cfg(feature = "runtime-benchmarks")]
            type BenchmarkHelper = polkadot_runtime_common::impls::benchmarks::TreasuryArguments<
                LocationParents,
                BenchmarkParaId,
            >;
            type Beneficiary = Beneficiary;
            type BeneficiaryLookup = IdentityLookup<Self::Beneficiary>;
            type Burn = <$t as GovernanceConfig>::TreasuryBurn;
            type BurnDestination = <$t as GovernanceConfig>::TreasuryBurnDestination;
            type Currency = Balances;
            type MaxApprovals = <$t as GovernanceConfig>::TreasuryMaxApprovals;
            type OnSlash = <$t as GovernanceConfig>::TreasuryOnSlash;
            type PalletId = <$t as GovernanceConfig>::TreasuryPalletId;
            // TODO:
            #[cfg(feature = "runtime-benchmarks")]
            type Paymaster = PayWithEnsure<TreasuryPaymaster, OpenHrmpChannel<BenchmarkParaId>>;
            #[cfg(not(feature = "runtime-benchmarks"))]
            type Paymaster = TreasuryPaymaster;
            type PayoutPeriod = <$t as GovernanceConfig>::TreasuryPayoutSpendPeriod;
            type ProposalBond = <$t as GovernanceConfig>::TreasuryProposalBond;
            type ProposalBondMaximum = <$t as GovernanceConfig>::TreasuryProposalBondMaximum;
            type ProposalBondMinimum = <$t as GovernanceConfig>::TreasuryProposalBondMinimum;
            type RejectOrigin = <$t as GovernanceConfig>::TreasuryRejectOrigin;
            type RuntimeEvent = RuntimeEvent;
            type SpendFunds = <$t as GovernanceConfig>::TreasurySpendFunds;
            type SpendOrigin = <$t as GovernanceConfig>::TreasurySpendOrigin;
            type SpendPeriod = <$t as GovernanceConfig>::TreasurySpendPeriod;
            /// Rerun benchmarks if you are making changes to runtime configuration.
            type WeightInfo = weights::pallet_treasury::WeightInfo<Runtime>;
        }
    };
}