mod admission;
mod basis;
mod counters;
mod declaration;
mod declaration_family;
mod declaration_identity;
mod diagnostics;
mod family_registry;
mod lifecycle;
mod rejection;
mod replay;
mod signal_strategy;

pub use admission::{
    AdmittedBridgeSubscription, BridgeSubscriptionAdmissionRejection,
    BridgeSubscriptionAdmissionRejectionKind,
};
pub use basis::{
    BridgeSubscriptionBasisKind, BridgeSubscriptionBasisRequest,
    BridgeSubscriptionBasisResolutionFailure, BridgeSubscriptionBasisResolutionFailureKind,
    ValidatedSubscriptionBasisBinding,
};
pub use counters::BridgeSubscriptionCounters;
pub use declaration::{
    BridgeSubscriptionDeclaration, BridgeSubscriptionDeliveryIntentClass,
    NormalizedSubscriptionSliceIntent, NormalizedSubscriptionSliceIntentError,
    NormalizedSubscriptionSliceIntentErrorKind,
};
pub use declaration_family::{
    BridgeSubscriptionDeclarationFamily, BridgeSubscriptionDeclarationFamilyKind,
};
pub use declaration_identity::{
    BridgeAdmittedSubscriptionIdentity, BridgeSignalStrategyIdentity,
    BridgeSubscriptionBasisIdentity, BridgeSubscriptionDeclarationFamilyIdentity,
    BridgeSubscriptionDeclarationIdentity, BridgeSubscriptionFamilyRegistryIdentity,
    BridgeSubscriptionLifecycleIdentity, BridgeSubscriptionReplayIdentity,
};
pub use diagnostics::BridgeSubscriptionExplanation;
pub(crate) use family_registry::{
    freeze_subscription_family_registry, FrozenSubscriptionFamilyRegistration,
    FrozenSubscriptionFamilyRegistry,
};
#[cfg(test)]
pub(crate) use family_registry::phase_one_subscription_families;
pub use lifecycle::{
    BridgeSubscriptionActivationReady, BridgeSubscriptionDeactivated,
    BridgeSubscriptionLifecycleRecord, BridgeSubscriptionLifecycleStateKind,
};
pub use rejection::{
    BridgeSubscriptionDeclarationRejection, BridgeSubscriptionDeclarationRejectionKind,
};
pub use replay::{
    BridgeRetainedSubscriptionBundle, BridgeSubscriptionReplayMismatch,
    BridgeSubscriptionReplayMismatchKind, BridgeSubscriptionReplaySummary,
};
pub use signal_strategy::{BridgeSignalStrategyDescriptor, BridgeSignalStrategyKind};
