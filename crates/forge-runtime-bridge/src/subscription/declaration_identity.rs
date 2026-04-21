use crate::identity::{
    AdmittedSubscriptionIdentityTag, BridgeIdentity, SubscriptionBasisIdentityTag,
    SubscriptionDeclarationFamilyIdentityTag, SubscriptionDeclarationIdentityTag,
    SubscriptionFamilyRegistryIdentityTag, SubscriptionLifecycleIdentityTag,
    SubscriptionReplayIdentityTag, SubscriptionSignalStrategyIdentityTag,
};

pub type BridgeSubscriptionDeclarationFamilyIdentity =
    BridgeIdentity<SubscriptionDeclarationFamilyIdentityTag>;
pub type BridgeSubscriptionDeclarationIdentity = BridgeIdentity<SubscriptionDeclarationIdentityTag>;
pub type BridgeSubscriptionFamilyRegistryIdentity =
    BridgeIdentity<SubscriptionFamilyRegistryIdentityTag>;
pub type BridgeSubscriptionBasisIdentity = BridgeIdentity<SubscriptionBasisIdentityTag>;
pub type BridgeSignalStrategyIdentity = BridgeIdentity<SubscriptionSignalStrategyIdentityTag>;
pub type BridgeAdmittedSubscriptionIdentity = BridgeIdentity<AdmittedSubscriptionIdentityTag>;
pub type BridgeSubscriptionLifecycleIdentity = BridgeIdentity<SubscriptionLifecycleIdentityTag>;
pub type BridgeSubscriptionReplayIdentity = BridgeIdentity<SubscriptionReplayIdentityTag>;
