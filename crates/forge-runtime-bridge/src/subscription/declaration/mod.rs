mod declaration;
mod slice_intent;

#[cfg(test)]
mod tests;

pub use declaration::BridgeSubscriptionDeclaration;
pub use slice_intent::{
    BridgeSubscriptionDeliveryIntentClass, NormalizedSubscriptionSliceIntent,
    NormalizedSubscriptionSliceIntentError, NormalizedSubscriptionSliceIntentErrorKind,
};
