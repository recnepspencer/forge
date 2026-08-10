use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ResourcePolicyCompatibilityClass {
    ExactDescriptorMatch,
    CompatibleParameterExpansion,
    CompatibleRetentionNarrowing,
    CompatibleDiagnosticsRichnessChange,
    MissingDescriptor,
    VersionIncompatible,
    ParameterDigestDrift,
    DecisionSemanticsDrift,
}

impl ResourcePolicyCompatibilityClass {
    pub fn is_compatible(self) -> bool {
        matches!(
            self,
            Self::ExactDescriptorMatch
                | Self::CompatibleParameterExpansion
                | Self::CompatibleRetentionNarrowing
                | Self::CompatibleDiagnosticsRichnessChange
        )
    }

    pub(super) fn as_str(self) -> &'static str {
        match self {
            Self::ExactDescriptorMatch => "exact-descriptor-match",
            Self::CompatibleParameterExpansion => "compatible-parameter-expansion",
            Self::CompatibleRetentionNarrowing => "compatible-retention-narrowing",
            Self::CompatibleDiagnosticsRichnessChange => "compatible-diagnostics-richness-change",
            Self::MissingDescriptor => "missing-descriptor",
            Self::VersionIncompatible => "version-incompatible",
            Self::ParameterDigestDrift => "parameter-digest-drift",
            Self::DecisionSemanticsDrift => "decision-semantics-drift",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ResourcePolicyRestoreCompatibilityDenialClass {
    MissingDescriptor,
    VersionIncompatible,
    ParameterDigestDrift,
    DecisionSemanticsDrift,
    ReplayPolicyDisallowsCompatibleDrift,
    MultipleIncompatibilities,
}
