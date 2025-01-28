//! Implements the OpenZeppelin assets configuration for a Runtime.
//!
//! This macro sets up the necessary configurations for the following pallets:
//! - `pallet_assets`
//! - `pallet_transaction_payment`
//! - `pallet_asset_manager`
//! - `orml_oracle`
//! - `pallet_asset_tx_payment`
//! - `pallet_membership`
//!
//! # Parameters
//! - `$t`: A type that implements the `AssetsConfig` trait, providing the necessary associated types
//!   and configurations.
//!
//! # Important
//! Rerun benchmarks if making changes to runtime configuration, as weight calculations
//! may need to be updated.

#[macro_export]
macro_rules! impl_openzeppelin_assets {
    ($t:ty) => {
        // Constants for assets configuration
        parameter_types! {
            // The maximum length of a name or symbol stored on-chain.
            pub const StringLimit: u32 = 50;
            // The basic amount of funds that must be reserved when adding metadata to your asset.
            pub const MetadataDepositBase: Balance = deposit(1, 68);
            // The additional funds that must be reserved for the number of bytes you store in your metadata.
            pub const MetadataDepositPerByte: Balance = deposit(0, 1);
            // Maximum number of items that can be removed in a single operation.
            pub const RemoveItemsLimit: u32 = 1000;
        }

        // Helper struct and implementation for runtime benchmarks
        // Only enabled when the `runtime-benchmarks` feature is active
        pallet_assets::runtime_benchmarks_enabled! {
            pub struct BenchmarkHelper;
            impl<AssetIdParameter> pallet_assets::BenchmarkHelper<AssetIdParameter> for BenchmarkHelper
            where
                AssetIdParameter: From<<$t as AssetsConfig>::AssetId>,
            {
                fn create_asset_id_parameter(id: u32) -> AssetIdParameter {
                    (id as <$t as AssetsConfig>::AssetId).into()
                }
            }
        }

        impl pallet_assets::Config for Runtime {
            // The amount of funds that must be reserved when creating a new approval.
            type ApprovalDeposit = <$t as AssetsConfig>::ApprovalDeposit;
            // The amount of funds that must be reserved for a non-provider asset account to be maintained.
            type AssetAccountDeposit = <$t as AssetsConfig>::AssetAccountDeposit;
            // The basic amount of funds that must be reserved for an asset.
            type AssetDeposit = <$t as AssetsConfig>::AssetDeposit;
            // Identifier for the class of asset.
            type AssetId = <$t as AssetsConfig>::AssetId;
            // Wrapper around `AssetId` to use in dispatchable call signatures.
            type AssetIdParameter = parity_scale_codec::Compact<<$t as AssetsConfig>::AssetId>;
            // The units in which we record balances.
            type Balance = Balance;
            #[cfg(feature = "runtime-benchmarks")]
            type BenchmarkHelper = BenchmarkHelper;
            type CallbackHandle = ();
            // Standard asset class creation is only allowed if the origin attempting it and the
            // asset class are in this set.
            type CreateOrigin = <$t as AssetsConfig>::CreateOrigin;
            type Currency = Balances;
            type Extra = ();
            // The origin which may forcibly create or destroy an asset or otherwise alter privileged
	    // attributes.
            type ForceOrigin = <$t as AssetsConfig>::ForceOrigin;
            type Freezer = ();
            type MetadataDepositBase = MetadataDepositBase;
            type MetadataDepositPerByte = MetadataDepositPerByte;
            type RemoveItemsLimit = RemoveItemsLimit;
            // The overarching event type
            type RuntimeEvent = RuntimeEvent;
            type StringLimit = StringLimit;
            type WeightInfo = <$t as AssetsWeight>::Assets;
        }

        parameter_types! {
            // Relay Chain `TransactionByteFee` / 10
            pub const TransactionByteFee: Balance = 10 * MICROCENTS;
            pub const OperationalFeeMultiplier: u8 = 5;
        }

        impl pallet_transaction_payment::Config for Runtime {
            // Fees stay almost constant over the short term and adjust slowly over time.
            // Spikes in transaction volume in the short term lead to long transaction inclusion times so tipping is allowed
            // to enable prioritization in proportion to tip amount.
            type FeeMultiplierUpdate = SlowAdjustingFeeUpdate<Self>;
            // Convert a length value into a deductible fee based on the currency type.
            type LengthToFee = ConstantMultiplier<Balance, TransactionByteFee>;
            // Handler for withdrawing, refunding and depositing the transaction fee.
            type OnChargeTransaction = pallet_transaction_payment::FungibleAdapter<Balances, ()>;
            // A fee multiplier for `Operational` extrinsics to compute "virtual tip" to boost their
	        // `priority`
            type OperationalFeeMultiplier = OperationalFeeMultiplier;
            type RuntimeEvent = RuntimeEvent;
            type WeightToFee = <$t as AssetsConfig>::WeightToFee;
        }

        impl pallet_asset_manager::Config for Runtime {
            type AssetId = AssetId;
            type AssetRegistrar = <$t as AssetsConfig>::AssetRegistrar;
            type AssetRegistrarMetadata = <$t as AssetsConfig>::AssetRegistrarMetadata;
            type Balance = Balance;
            type ForeignAssetModifierOrigin = <$t as AssetsConfig>::ForeignAssetModifierOrigin;
            type ForeignAssetType = <$t as AssetsConfig>::AssetType;
            type RuntimeEvent = RuntimeEvent;
            type WeightInfo = <$t as AssetsWeight>::AssetManager;
        }

        pub struct AssetConverter;

        impl frame_support::traits::tokens::ConversionToAssetBalance<Balance, <$t as AssetsConfig>::AssetId, Balance> for AssetConverter {
            type Error = sp_runtime::transaction_validity::InvalidTransaction;
        
            fn to_asset_balance(balance: Balance, asset_id: <$t as AssetsConfig>::AssetId) -> Result<Balance, Self::Error> {
                let funding_asset_price = Oracle::get(&asset_id)
                    .ok_or(sp_runtime::transaction_validity::InvalidTransaction::Payment)?;
                // FIXME: check if timestamp on oracle data is outdated.
        
                use sp_arithmetic::FixedPointNumber;
                let price = funding_asset_price.value.reciprocal().ok_or(sp_runtime::transaction_validity::InvalidTransaction::Payment)?;
                
                Ok(price.saturating_mul_int(balance))
            }
        }
        
        type BalanceOf<T> = <<T as pallet_transaction_payment::Config>::OnChargeTransaction as pallet_transaction_payment::OnChargeTransaction<T>>::Balance;
        type AssetIdOf<T> = <<T as pallet_asset_tx_payment::Config>::Fungibles as frame_support::traits::fungibles::Inspect<parachains_common::impls::AccountIdOf<T>>>::AssetId;
        type AssetBalanceOf<T> =
                <<T as pallet_asset_tx_payment::Config>::Fungibles as frame_support::traits::fungibles::Inspect<<T as frame_system::Config>::AccountId>>::Balance;
        
        /// Implements the asset transaction for a balance to asset converter (implementing
        /// [`ConversionToAssetBalance`]) and 2 credit handlers (implementing [`HandleCredit`]).
        ///
        /// First handler does the fee, second the tip.
        pub struct TxFeeFungiblesAdapter<Converter, FeeCreditor, TipCreditor>(
            sp_std::marker::PhantomData<(Converter, FeeCreditor, TipCreditor)>,
        );
        
        /// Default implementation for a runtime instantiating this pallet, a balance to asset converter and
        /// a credit handler.
        impl<Runtime, Converter, FeeCreditor, TipCreditor> pallet_asset_tx_payment::OnChargeAssetTransaction<Runtime>
            for TxFeeFungiblesAdapter<Converter, FeeCreditor, TipCreditor>
        where
            Runtime: pallet_asset_tx_payment::Config,
            Runtime::Fungibles: frame_support::traits::fungibles::Inspect<parachains_common::impls::AccountIdOf<Runtime>, AssetId = <$t as AssetsConfig>::AssetId>,
            Converter: frame_support::traits::tokens::ConversionToAssetBalance<BalanceOf<Runtime>, AssetIdOf<Runtime>, AssetBalanceOf<Runtime>>,
            FeeCreditor: pallet_asset_tx_payment::HandleCredit<Runtime::AccountId, Runtime::Fungibles>,
            TipCreditor: pallet_asset_tx_payment::HandleCredit<Runtime::AccountId, Runtime::Fungibles>,
        {
            // Note: We stick to `v3::MultiLocation`` because `v4::Location`` doesn't implement `Copy`.
            type AssetId = xcm::v3::MultiLocation;
            type Balance = BalanceOf<Runtime>;
            type LiquidityInfo = frame_support::traits::fungibles::Credit<Runtime::AccountId, Runtime::Fungibles>;
        
            /// Note: The `fee` already includes the `tip`.
            fn withdraw_fee(
                who: &Runtime::AccountId,
                _call: &Runtime::RuntimeCall,
                _info: &sp_runtime::traits::DispatchInfoOf<Runtime::RuntimeCall>,
                asset_id: Self::AssetId,
                fee: Self::Balance,
                _tip: Self::Balance,
            ) -> Result<Self::LiquidityInfo, sp_runtime::transaction_validity::TransactionValidityError> {
                use sp_runtime::traits::Zero;
                // We don't know the precision of the underlying asset. Because the converted fee could be
                // less than one (e.g. 0.5) but gets rounded down by integer division we introduce a minimum
                // fee.
                let asset_id: AssetId = AssetType::Xcm(asset_id).into();
                let min_converted_fee = if fee.is_zero() { sp_runtime::traits::Zero::zero() } else { sp_runtime::traits::One::one() };
                let converted_fee = Converter::to_asset_balance(fee, asset_id.clone())
                    .map_err(|_| sp_runtime::transaction_validity::TransactionValidityError::from(sp_runtime::transaction_validity::InvalidTransaction::Payment))?
                    .max(min_converted_fee);
                let can_withdraw =
                    <Runtime::Fungibles as frame_support::traits::fungibles::Inspect<Runtime::AccountId>>::can_withdraw(asset_id.clone(), who, converted_fee);
                if can_withdraw != frame_support::traits::tokens::WithdrawConsequence::Success {
                    return Err(sp_runtime::transaction_validity::InvalidTransaction::Payment.into())
                }
                <Runtime::Fungibles as frame_support::traits::fungibles::Balanced<Runtime::AccountId>>::withdraw(
                    asset_id,
                    who,
                    converted_fee,
                    frame_support::traits::tokens::Precision::Exact,
                    frame_support::traits::tokens::Preservation::Protect,
                    frame_support::traits::tokens::Fortitude::Polite,
                )
                .map_err(|_| sp_runtime::transaction_validity::TransactionValidityError::from(sp_runtime::transaction_validity::InvalidTransaction::Payment))
            }
        
            /// Note: The `corrected_fee` already includes the `tip`.
            fn correct_and_deposit_fee(
                who: &Runtime::AccountId,
                _dispatch_info: &sp_runtime::traits::DispatchInfoOf<Runtime::RuntimeCall>,
                _post_info: &sp_runtime::traits::PostDispatchInfoOf<Runtime::RuntimeCall>,
                corrected_fee: Self::Balance,
                tip: Self::Balance,
                paid: Self::LiquidityInfo,
            ) -> Result<(AssetBalanceOf<Runtime>, AssetBalanceOf<Runtime>), sp_runtime::transaction_validity::TransactionValidityError> {
                use sp_runtime::traits::Zero;
                let min_converted_fee = if corrected_fee.is_zero() { sp_runtime::traits::Zero::zero() } else { sp_runtime::traits::One::one() };
                // Convert the corrected fee and tip into the asset used for payment.
                let converted_fee = Converter::to_asset_balance(corrected_fee, paid.asset())
                    .map_err(|_| -> sp_runtime::transaction_validity::TransactionValidityError { sp_runtime::transaction_validity::InvalidTransaction::Payment.into() })?
                    .max(min_converted_fee);
                let converted_tip = Converter::to_asset_balance(tip, paid.asset())
                    .map_err(|_| -> sp_runtime::transaction_validity::TransactionValidityError { sp_runtime::transaction_validity::InvalidTransaction::Payment.into() })?;
        
                // Calculate how much refund we should return.
                let (final_fee, refund) = paid.split(converted_fee);
                // Split the tip from the fee
                let (final_tip, final_fee_minus_tip) = final_fee.split(converted_tip);
        
                let _ = <Runtime::Fungibles as frame_support::traits::fungibles::Balanced<Runtime::AccountId>>::resolve(who, refund);
        
                FeeCreditor::handle_credit(final_fee_minus_tip);
                TipCreditor::handle_credit(final_tip);
        
                Ok((converted_fee, converted_tip))
            }
        }
        
        pub struct CreditFungiblesToAccount<AccountId, Assets, Account>(sp_std::marker::PhantomData<(AccountId, Assets, Account)>);
        impl<AccountId, Assets: frame_support::traits::fungibles::Balanced<AccountId>, Account: frame_support::pallet_prelude::Get<AccountId>>
            pallet_asset_tx_payment::HandleCredit<AccountId, Assets> for CreditFungiblesToAccount<AccountId, Assets, Account>
        {
            fn handle_credit(credit: frame_support::traits::fungibles::Credit<AccountId, Assets>) {
                let payee: AccountId = Account::get();
                let _ = <Assets as frame_support::traits::fungibles::Balanced<AccountId>>::resolve(&payee, credit);
            }
        }
        
        pub type OnCharge = TxFeeFungiblesAdapter<
            AssetConverter,
            CreditFungiblesToAccount<
                <$t as AssetsConfig>::AccountId, 
                crate::Assets, 
                <$t as AssetsConfig>::FungiblesToAccount
            >, 
            parachains_common::impls::AssetsToBlockAuthor<
                Runtime,
                ()
            >, 
        >;
        
        impl pallet_asset_tx_payment::Config for Runtime {
            type Fungibles = crate::Assets;
            type OnChargeAssetTransaction = OnCharge;
            type RuntimeEvent = RuntimeEvent;
        }
        
        parameter_types! {
            pub const MinimumCount: u32 = 5;
            pub const ExpiresIn: u64 = 1000 * 60 * 60; // 1 hours
            pub const MaxFeedValues: u32 = 10; // max 10 values allowd to feed in one call.
            pub RootOperatorAccountId: AccountId = AccountId::from([0xffu8; 32]);
        }
        
        #[cfg(feature = "runtime-benchmarks")]
        pub struct BenchmarkHelper;
        #[cfg(feature = "runtime-benchmarks")]
        impl orml_oracle::BenchmarkHelper<<$t as AssetsConfig>::AssetId, sp_runtime::FixedU128, MaxFeedValues> for BenchmarkHelper {
            fn get_currency_id_value_pairs() -> sp_runtime::BoundedVec<(AssetId, sp_runtime::FixedU128), MaxFeedValues> {
                sp_runtime::BoundedVec::default()
            }
        }
        
        impl orml_oracle::Config for Runtime {
            type RuntimeEvent = RuntimeEvent;
            type OnNewData = ();
            type CombineData = orml_oracle::DefaultCombineData<
                Runtime, 
                MinimumCount, 
                ExpiresIn,
                ()
            >;
            type Time = <$t as AssetsConfig>::Timestamp;
            type OracleKey = <$t as AssetsConfig>::AssetId;
            type OracleValue = sp_runtime::FixedU128;
            type RootOperatorAccountId = RootOperatorAccountId;
            type Members = OracleMembership;
            type MaxHasDispatchedSize = ConstU32<20>;
            type WeightInfo = <$t as AssetsWeight>::Oracle;
            type MaxFeedValues = MaxFeedValues;
            #[cfg(feature = "runtime-benchmarks")]
            type BenchmarkHelper = BenchmarkHelper;
        }
        
        parameter_types! {
            pub const MaxMembers: u32 = 30;
        }
        
        impl pallet_membership::Config for Runtime {
            type RuntimeEvent = RuntimeEvent;
            type AddOrigin = EnsureRoot<AccountId>;
            type RemoveOrigin = EnsureRoot<AccountId>;
            type SwapOrigin = EnsureRoot<AccountId>;
            type ResetOrigin = EnsureRoot<AccountId>;
            type PrimeOrigin = EnsureRoot<AccountId>;
            type MembershipInitialized = ();
            type MembershipChanged = Oracle;
            type MaxMembers = MaxMembers;
            type WeightInfo = <$t as AssetsWeight>::OracleMembership;
        }
        
    };
}

pub const PALLET_NAMES: [(&str, &str); 6] = [
    ("Assets", "pallet_assets"),
    ("TransactionPayment", "pallet_transaction_payment"),
    ("AssetManager", "pallet_asset_manager"),
    ("Oracle", "orml_oracle"),
    ("AssetTxPayment", "pallet_asset_tx_payment"),
    ("OracleMembership", "pallet_membership"),
];
