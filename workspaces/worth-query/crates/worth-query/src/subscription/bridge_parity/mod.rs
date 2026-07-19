mod explanation;
mod identities;
mod support;
#[cfg(test)]
mod validation;
mod witness;

#[cfg(test)]
pub use explanation::explain_query_subscription_bridge_parity;
pub use explanation::{
    QuerySubscriptionBridgeParityComparison, QuerySubscriptionBridgeParityExplanation,
};
pub use support::{
    BridgeParityReceipt, QuerySubscriptionBridgeParityClass, QuerySubscriptionBridgeParityCounters,
    QuerySubscriptionBridgeParityError, QuerySubscriptionBridgeParityFailure,
    QuerySubscriptionBridgeParityFailureKind, SubscriptionBridgeParityWidth,
};
pub use witness::{
    build_query_subscription_manual_bridge_witness, BridgeWitnessAssemblyPosture,
    QuerySubscriptionManualBridgeWitness,
};
