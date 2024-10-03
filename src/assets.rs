//! Assets pallet groupings wrapper

#[macro_export]
macro_rules! impl_openzeppelin_assets {
    ($t:ty) => {
        parameter_types! {
            pub const StringLimit: u32 = 50;
            pub const MetadataDepositBase: Balance = deposit(1, 68);
            pub const MetadataDepositPerByte: Balance = deposit(0, 1);
            pub const RemoveItemsLimit: u32 = 1000;
        }

        impl pallet_assets::Config for Runtime {
            type ApprovalDeposit = <$t as AssetsConfig>::ApprovalDeposit;
            type AssetAccountDeposit = <$t as AssetsConfig>::ApprovalDeposit;
            type AssetDeposit = <$t as AssetsConfig>::AssetDeposit;
            type AssetId = u32;
            type AssetIdParameter = parity_scale_codec::Compact<u32>;
            type Balance = Balance;
            #[cfg(feature = "runtime-benchmarks")]
            type BenchmarkHelper = ();
            type CallbackHandle = ();
            type CreateOrigin = <$t as AssetsConfig>::CreateOrigin;
            type Currency = Balances;
            type Extra = ();
            type ForceOrigin = <$t as AssetsConfig>::ForceOrigin;
            type Freezer = ();
            type MetadataDepositBase = MetadataDepositBase;
            type MetadataDepositPerByte = MetadataDepositPerByte;
            type RemoveItemsLimit = RemoveItemsLimit;
            type RuntimeEvent = RuntimeEvent;
            type StringLimit = StringLimit;
            /// Rerun benchmarks if you are making changes to runtime configuration.
            type WeightInfo = weights::pallet_assets::WeightInfo<Runtime>;
        }

        parameter_types! {
            /// Relay Chain `TransactionByteFee` / 10
            pub const TransactionByteFee: Balance = 10 * MICROCENTS;
            pub const OperationalFeeMultiplier: u8 = 5;
        }

        impl pallet_transaction_payment::Config for Runtime {
            /// There are two possible mechanisms available: slow and fast adjusting.
            /// With slow adjusting fees stay almost constant in short periods of time, changing only in long term.
            /// It may lead to long inclusion times during spikes, therefore tipping is enabled.
            /// With fast adjusting fees change rapidly, but fixed for all users at each block (no tipping)
            type FeeMultiplierUpdate = SlowAdjustingFeeUpdate<Self>;
            type LengthToFee = ConstantMultiplier<Balance, TransactionByteFee>;
            type OnChargeTransaction = pallet_transaction_payment::CurrencyAdapter<Balances, ()>;
            type OperationalFeeMultiplier = OperationalFeeMultiplier;
            type RuntimeEvent = RuntimeEvent;
            type WeightToFee = WeightToFee;
        }
    };
}