//! Weights per pallet grouping

pub trait SystemWeight {
    type DbWeight;
}

pub trait SystemWeightFull: SystemWeight {
    type Timestamp;
    type Scheduler;
    type Preimage;
    type Proxy;
    type Multisig;
    type ParachainSystem;
    type Balances;
    type Utility;
    type DbWeight;
}

impl<T: SystemWeight> SystemWeightFull for T {
    type Timestamp = ();
    type Scheduler = ();
    type Preimage = ();
    type Proxy = ();
    type Multisig = ();
    type ParachainSystem = ();
    type Balances = ();
    type Utility = ();
    type DbWeight = T::DbWeight;
}

pub trait ConsensusWeight {}

impl<T: ConsensusWeight> ConsensusWeightFull for T {
    type CollatorSelection = ();
    type Session = ();
}

pub trait ConsensusWeightFull: ConsensusWeight {
    type CollatorSelection;
    type Session;
}

pub trait AssetsWeight {}

impl<T: AssetsWeight> AssetsWeightFull for T {
    type Assets = ();
    type AssetManager = ();
    type TransactionPayment = ();
    type OrmlOracle = ();
    type OracleMembership = ();
    type AssetTxPayment = ();
}

pub trait AssetsWeightFull: AssetsWeight {
    type Assets;
    type AssetManager;
    type TransactionPayment;
    type OrmlOracle;
    type OracleMembership;
    type AssetTxPayment;
}

pub trait GovernanceWeight {}

impl<T: GovernanceWeight> GovernanceWeightFull for T {
    type Sudo = ();
    type Treasury = ();
    type ConvictionVoting = ();
    type Whitelist = ();
    type Referenda = ();
}

pub trait GovernanceWeightFull: GovernanceWeight {
    type Sudo;
    type Treasury;
    type ConvictionVoting;
    type Whitelist;
    type Referenda;
}

pub trait XcmWeight {}

impl<T: XcmWeight> XcmWeightFull for T {
    type MessageQueue = ();
    type XcmpQueue = ();
    type Xcm = ();
    type XcmWeightTrader = ();
    type XcmTransactor = ();
}

pub trait XcmWeightFull: XcmWeight {
    type MessageQueue;
    type XcmpQueue;
    type Xcm;
    type XcmWeightTrader;
    type XcmTransactor;
}

pub trait EvmWeight {}

impl<T: EvmWeight> EvmWeightFull for T {
    type Evm = ();
}

pub trait EvmWeightFull: EvmWeight {
    type Evm;
}

pub trait TanssiWeight {}

impl<T: TanssiWeight> TanssiWeightFull for T {
    type AuthorInherent = ();
    type AuthoritiesNoting = ();
}

pub trait TanssiWeightFull: TanssiWeight {
    type AuthorInherent;
    type AuthoritiesNoting;
}
