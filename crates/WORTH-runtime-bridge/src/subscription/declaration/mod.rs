mod declaration;
mod slice_intent;

#[cfg(test)]
mod tests;

pub use declaration::BridgeSubscriptionDeclaration;
pub(crate) use slice_intent::{
    subscription_slice_target_identity, BridgeSubscriptionSliceTargetIdentity,
};
pub use slice_intent::{
    BridgeSubscriptionDeliveryIntentClass, NormalizedSubscriptionSliceIntent,
    NormalizedSubscriptionSliceIntentError, NormalizedSubscriptionSliceIntentErrorKind,
};
