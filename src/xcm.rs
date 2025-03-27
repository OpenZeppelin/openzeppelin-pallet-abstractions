//! Implements the OpenZeppelin XCM configuration for a Runtime.
//!
//! This macro sets up the necessary configurations for the following pallets:
//! - `pallet_message_queue`
//! - `cumulus_pallet_xcmp_queue`
//! - `pallet_xcm`
//! - `cumulus_pallet_xcm`
//! - `pallet_xcm_weight_trader`
//! - `orml_xtokens`
//! - `pallet_xcm_transactor`
//!
//! # Parameters
//! - `$t`: A type that implements the `XcmConfigFull` trait, providing the necessary associated types
//!   and configurations for cross-chain messaging functionality.
//!
//! # Important
//! Rerun benchmarks if making changes to runtime configuration, as weight calculations
//! may need to be updated.

#[macro_export]
macro_rules! impl_openzeppelin_xcm {
    ($t:ty) => {
        impl pallet_message_queue::Config for Runtime {
            type HeapSize = <$t as openzeppelin_pallet_abstractions::XcmConfigFull>::MessageQueueHeapSize;
            type IdleMaxServiceWeight = <$t as openzeppelin_pallet_abstractions::XcmConfigFull>::MessageQueueServiceWeight;
            type MaxStale = <$t as openzeppelin_pallet_abstractions::XcmConfigFull>::MessageQueueMaxStale;
            #[cfg(feature = "runtime-benchmarks")]
            type MessageProcessor = pallet_message_queue::mock_helpers::NoopMessageProcessor<
                cumulus_primitives_core::AggregateMessageOrigin,
            >;
            #[cfg(not(feature = "runtime-benchmarks"))]
            type MessageProcessor = ProcessXcmMessage<
                AggregateMessageOrigin,
                xcm_executor::XcmExecutor<XcmExecutorConfig>,
                RuntimeCall,
            >;
            // The XCMP queue pallet is only ever able to handle the `Sibling(ParaId)` origin:
            type QueueChangeHandler = NarrowOriginToSibling<XcmpQueue>;
            type QueuePausedQuery = NarrowOriginToSibling<XcmpQueue>;
            type RuntimeEvent = RuntimeEvent;
            type ServiceWeight = <$t as openzeppelin_pallet_abstractions::XcmConfigFull>::MessageQueueServiceWeight;
            type Size = u32;
            type WeightInfo = <$t as XcmWeight>::MessageQueue;
        }

        parameter_types! {
            /// The asset ID for the asset that we use to pay for message delivery fees.
            pub FeeAssetId: cumulus_primitives_core::AssetId = cumulus_primitives_core::AssetId(Location::parent());
            /// The base fee for the message delivery fees. Kusama is based for the reference.
            pub const ToSiblingBaseDeliveryFee: u128 = CENTS.saturating_mul(3);
        }

        impl cumulus_pallet_xcmp_queue::Config for Runtime {
            type ChannelInfo = ParachainSystem;
            type ControllerOrigin = <$t as openzeppelin_pallet_abstractions::XcmConfigFull>::XcmpQueueControllerOrigin;
            type ControllerOriginConverter = XcmOriginToTransactDispatchOrigin;
            type MaxActiveOutboundChannels = <$t as openzeppelin_pallet_abstractions::XcmConfigFull>::MaxActiveOutboundChannels;
            type MaxInboundSuspended = <$t as openzeppelin_pallet_abstractions::XcmConfigFull>::XcmpQueueMaxInboundSuspended;
            type MaxPageSize = <$t as openzeppelin_pallet_abstractions::XcmConfigFull>::MaxPageSize;
            /// Ensure that this value is not set to null (or NoPriceForMessageDelivery) to prevent spamming
            type PriceForSiblingDelivery = PriceForSiblingParachainDelivery;
            type RuntimeEvent = RuntimeEvent;
            type VersionWrapper = ();
            type WeightInfo = <$t as XcmWeight>::XcmpQueue;
            // Enqueue XCMP messages from siblings for later processing.
            type XcmpQueue =
                TransformOrigin<MessageQueue, AggregateMessageOrigin, ParaId, ParaIdToSibling>;
        }

        parameter_types! {
            pub PlaceholderAccount: AccountId = PolkadotXcm::check_account();
            pub UniversalLocation: InteriorLocation = Parachain(ParachainInfo::parachain_id().into()).into();
        }

        parameter_types! {
            // One XCM operation is 1_000_000_000 weight - almost certainly a conservative estimate.
            pub const UnitWeightCost: Weight = Weight::from_parts(1_000_000_000, 64 * 1024);
            pub const MaxInstructions: u32 = 100;
            pub const MaxAssetsIntoHolding: u32 = 64;
        }

        pub struct ParentOrParentsExecutivePlurality;
        impl Contains<Location> for ParentOrParentsExecutivePlurality {
            fn contains(location: &Location) -> bool {
                matches!(location.unpack(), (1, []) | (1, [Plurality { id: BodyId::Executive, .. }]))
            }
        }

        pub type Barrier = TrailingSetTopicAsId<
            DenyThenTry<
                DenyReserveTransferToRelayChain,
                (
                    TakeWeightCredit,
                    WithComputedOrigin<
                        (
                            AllowTopLevelPaidExecutionFrom<Everything>,
                            AllowExplicitUnpaidExecutionFrom<ParentOrParentsExecutivePlurality>,
                            // ^^^ Parent and its exec plurality get free execution
                        ),
                        UniversalLocation,
                        ConstU32<8>,
                    >,
                ),
            >,
        >;

        pub struct XcmExecutorConfig;
        impl xcm_executor::Config for XcmExecutorConfig {
            type Aliasers = Nothing;
            type AssetClaims = PolkadotXcm;
            type AssetExchanger = ();
            type AssetLocker = ();
            // How to withdraw and deposit an asset.
            type AssetTransactor = <$t as openzeppelin_pallet_abstractions::XcmConfigFull>::AssetTransactors;
            type AssetTrap = PolkadotXcm;
            type Barrier = Barrier;
            type CallDispatcher = RuntimeCall;
            /// When changing this config, keep in mind, that you should collect fees.
            type FeeManager = <$t as openzeppelin_pallet_abstractions::XcmConfigFull>::FeeManager;
            type HrmpChannelAcceptedHandler = ();
            type HrmpChannelClosingHandler = ();
            type HrmpNewChannelOpenRequestHandler = ();
            /// Please, keep these two configs (`IsReserve` and `IsTeleporter`) mutually exclusive
            type IsReserve = <$t as openzeppelin_pallet_abstractions::XcmConfigFull>::Reserves;
            type IsTeleporter = ();
            type MaxAssetsIntoHolding = MaxAssetsIntoHolding;
            type MessageExporter = ();
            type OriginConverter = <$t as openzeppelin_pallet_abstractions::XcmConfigFull>::XcmOriginToTransactDispatchOrigin;
            type PalletInstancesInfo = AllPalletsWithSystem;
            type ResponseHandler = PolkadotXcm;
            type RuntimeCall = RuntimeCall;
            type SafeCallFilter = Everything;
            type SubscriptionService = PolkadotXcm;
            type Trader = <$t as openzeppelin_pallet_abstractions::XcmConfigFull>::Trader;
            type TransactionalProcessor = FrameTransactionalProcessor;
            type UniversalAliases = Nothing;
            // Teleporting is disabled.
            type UniversalLocation = UniversalLocation;
            type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
            type XcmRecorder = PolkadotXcm;
            type XcmSender = XcmRouter;
        }

        /// The means for routing XCM messages which are not for local execution into
        /// the right message queues.
        pub type XcmRouter = WithUniqueTopic<(
            // Two routers - use UMP to communicate with the relay chain:
            cumulus_primitives_utility::ParentAsUmp<ParachainSystem, (), ()>,
            // ..and XCMP to communicate with the sibling chains.
            XcmpQueue,
        )>;

        parameter_types! {
            pub const MaxLockers: u32 = 8;
            pub const MaxRemoteLockConsumers: u32 = 0;
        }

        impl pallet_xcm::Config for Runtime {
            type AdminOrigin = <$t as openzeppelin_pallet_abstractions::XcmConfigFull>::XcmAdminOrigin;
            // ^ Override for AdvertisedXcmVersion default
            type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
            type Currency = Balances;
            type CurrencyMatcher = ();
            type ExecuteXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, <$t as openzeppelin_pallet_abstractions::XcmConfigFull>::LocalOriginToLocation>;
            type MaxLockers = MaxLockers;
            type MaxRemoteLockConsumers = MaxRemoteLockConsumers;
            type RemoteLockConsumerIdentifier = ();
            type RuntimeCall = RuntimeCall;
            type RuntimeEvent = RuntimeEvent;
            type RuntimeOrigin = RuntimeOrigin;
            type SendXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, <$t as openzeppelin_pallet_abstractions::XcmConfigFull>::LocalOriginToLocation>;
            type SovereignAccountOf = <$t as openzeppelin_pallet_abstractions::XcmConfigFull>::LocationToAccountId;
            type TrustedLockers = ();
            type UniversalLocation = UniversalLocation;
            type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
            type WeightInfo = <$t as XcmWeight>::Xcm;
            #[cfg(feature = "runtime-benchmarks")]
            type XcmExecuteFilter = Everything;
            #[cfg(not(feature = "runtime-benchmarks"))]
            type XcmExecuteFilter = Nothing;
            // ^ Disable dispatchable execute on the XCM pallet.
            // Needs to be `Everything` for local testing.
            type XcmExecutor = XcmExecutor<XcmExecutorConfig>;
            type XcmReserveTransferFilter = Everything;
            type XcmRouter = XcmRouter;
            type XcmTeleportFilter = Nothing;

            const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;
        }

        impl cumulus_pallet_xcm::Config for Runtime {
            type RuntimeEvent = RuntimeEvent;
            type XcmExecutor = XcmExecutor<XcmExecutorConfig>;
        }

        impl pallet_xcm_weight_trader::Config for Runtime {
            type AccountIdToLocation = <$t as openzeppelin_pallet_abstractions::XcmConfigFull>::AccountIdToLocation;
            type AddSupportedAssetOrigin = <$t as openzeppelin_pallet_abstractions::XcmConfigFull>::AddSupportedAssetOrigin;
            type AssetLocationFilter = <$t as openzeppelin_pallet_abstractions::XcmConfigFull>::AssetFeesFilter;
            type AssetTransactor = <$t as openzeppelin_pallet_abstractions::XcmConfigFull>::AssetTransactors;
            type Balance = Balance;
            type EditSupportedAssetOrigin = <$t as openzeppelin_pallet_abstractions::XcmConfigFull>::EditSupportedAssetOrigin;
            type NativeLocation = <$t as openzeppelin_pallet_abstractions::XcmConfigFull>::SelfReserve;
            #[cfg(feature = "runtime-benchmarks")]
            type NotFilteredLocation = <$t as openzeppelin_pallet_abstractions::XcmConfigFull>::RelayLocation;
            type PauseSupportedAssetOrigin = <$t as openzeppelin_pallet_abstractions::XcmConfigFull>::PauseSupportedAssetOrigin;
            type RemoveSupportedAssetOrigin = <$t as openzeppelin_pallet_abstractions::XcmConfigFull>::RemoveSupportedAssetOrigin;
            type ResumeSupportedAssetOrigin = <$t as openzeppelin_pallet_abstractions::XcmConfigFull>::ResumeSupportedAssetOrigin;
            type RuntimeEvent = RuntimeEvent;
            type WeightInfo = <$t as XcmWeight>::XcmWeightTrader;
            type WeightToFee = <$t as openzeppelin_pallet_abstractions::XcmConfigFull>::WeightToFee;
            type XcmFeesAccount = <$t as openzeppelin_pallet_abstractions::XcmConfigFull>::XcmFeesAccount;
        }

        impl orml_xtokens::Config for Runtime {
            type AccountIdToLocation = <$t as openzeppelin_pallet_abstractions::XcmConfigFull>::AccountIdToLocation;
            type Balance = Balance;
            type BaseXcmWeight = <$t as openzeppelin_pallet_abstractions::XcmConfigFull>::BaseXcmWeight;
            type CurrencyId = <$t as openzeppelin_pallet_abstractions::XcmConfigFull>::CurrencyId;
            type CurrencyIdConvert = <$t as openzeppelin_pallet_abstractions::XcmConfigFull>::CurrencyIdToLocation;
            type LocationsFilter = Everything;
            type MaxAssetsForTransfer = <$t as openzeppelin_pallet_abstractions::XcmConfigFull>::MaxAssetsForTransfer;
            type MinXcmFee = <$t as openzeppelin_pallet_abstractions::XcmConfigFull>::ParachainMinFee;
            type RateLimiter = ();
            type RateLimiterId = ();
            type ReserveProvider = <$t as openzeppelin_pallet_abstractions::XcmConfigFull>::XtokensReserveProviders;
            type RuntimeEvent = RuntimeEvent;
            type SelfLocation = <$t as openzeppelin_pallet_abstractions::XcmConfigFull>::SelfLocation;
            type UniversalLocation = <$t as openzeppelin_pallet_abstractions::XcmConfigFull>::UniversalLocation;
            type Weigher = <$t as openzeppelin_pallet_abstractions::XcmConfigFull>::XcmWeigher;
            type XcmExecutor = XcmExecutor<XcmExecutorConfig>;
        }

        impl pallet_xcm_transactor::Config for Runtime {
            type AccountIdToLocation = <$t as openzeppelin_pallet_abstractions::XcmConfigFull>::AccountIdToLocation;
            type AssetTransactor = <$t as openzeppelin_pallet_abstractions::XcmConfigFull>::AssetTransactors;
            type Balance = Balance;
            type BaseXcmWeight = <$t as openzeppelin_pallet_abstractions::XcmConfigFull>::BaseXcmWeight;
            type CurrencyId = <$t as openzeppelin_pallet_abstractions::XcmConfigFull>::CurrencyId;
            type CurrencyIdToLocation = <$t as openzeppelin_pallet_abstractions::XcmConfigFull>::CurrencyIdToLocation;
            type DerivativeAddressRegistrationOrigin = <$t as openzeppelin_pallet_abstractions::XcmConfigFull>::DerivativeAddressRegistrationOrigin;
            type HrmpManipulatorOrigin = <$t as openzeppelin_pallet_abstractions::XcmConfigFull>::HrmpManipulatorOrigin;
            type HrmpOpenOrigin = <$t as openzeppelin_pallet_abstractions::XcmConfigFull>::HrmpOpenOrigin;
            type MaxHrmpFee = xcm_builder::Case<<$t as openzeppelin_pallet_abstractions::XcmConfigFull>::MaxHrmpRelayFee>;
            type ReserveProvider = <$t as openzeppelin_pallet_abstractions::XcmConfigFull>::TransactorReserveProvider;
            type RuntimeEvent = RuntimeEvent;
            type SelfLocation = <$t as openzeppelin_pallet_abstractions::XcmConfigFull>::SelfLocation;
            type SovereignAccountDispatcherOrigin = <$t as openzeppelin_pallet_abstractions::XcmConfigFull>::SovereignAccountDispatcherOrigin;
            type Transactor = <$t as openzeppelin_pallet_abstractions::XcmConfigFull>::Transactors;
            type UniversalLocation = <$t as openzeppelin_pallet_abstractions::XcmConfigFull>::UniversalLocation;
            type Weigher = <$t as openzeppelin_pallet_abstractions::XcmConfigFull>::XcmWeigher;
            type WeightInfo = <$t as XcmWeight>::XcmTransactor;
            type XcmSender = <$t as openzeppelin_pallet_abstractions::XcmConfigFull>::XcmSender;
        }
    };
}

pub const PALLET_NAMES: [(&str, &str); 7] = [
    ("MessageQueue", "pallet_message_queue"),
    ("XcmpQueue", "cumulus_pallet_xcmp_queue"),
    ("PolkadotXcm", "pallet_xcm"),
    ("CumulusXcm", "cumulus_pallet_xcm"),
    ("XcmWeightTrader", "pallet_xcm_weight_trader"),
    ("XTokens", "orml_xtokens"),
    ("XcmTransactor", "pallet_xcm_transactor"),
];
