use crate::identity::{BridgeIdentity, InvalidationIdentityTag, SubscriptionSliceIdentityTag};

pub type BridgeInvalidationIdentity = BridgeIdentity<InvalidationIdentityTag>;
pub type BridgeSubscriptionSliceIdentity = BridgeIdentity<SubscriptionSliceIdentityTag>;
