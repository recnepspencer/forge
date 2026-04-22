#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeSubscriptionBundleFieldState {
    Present,
    NotExercised,
    RejectedBeforeProduced,
    UnavailableBecausePriorArtifactMissing,
    UnavailableBecauseSchemaIncompatible,
}

impl BridgeSubscriptionBundleFieldState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Present => "present",
            Self::NotExercised => "not_exercised",
            Self::RejectedBeforeProduced => "rejected_before_produced",
            Self::UnavailableBecausePriorArtifactMissing => {
                "unavailable_because_prior_artifact_missing"
            }
            Self::UnavailableBecauseSchemaIncompatible => "unavailable_because_schema_incompatible",
        }
    }
}
