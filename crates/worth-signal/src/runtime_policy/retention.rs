use super::definition::SignalRuntimePolicy;
use super::resolved::InstalledSignalRuntimePolicy;
use crate::diagnostics::policy::ArtifactRetentionPolicy;

impl SignalRuntimePolicy {
    pub fn retains_explanation_facts(self) -> bool {
        matches!(
            self.retention_budget.explanation_retention,
            ArtifactRetentionPolicy::Retain
        )
    }

    pub fn retains_provenance_facts(self) -> bool {
        matches!(
            self.retention_budget.provenance_retention,
            ArtifactRetentionPolicy::Retain
        )
    }

    pub fn can_reconstruct_explanation(self) -> bool {
        !matches!(
            self.retention_budget.explanation_retention,
            ArtifactRetentionPolicy::Omit
        ) && self.reconstruction_budget.allow_explanation_reconstruction
    }

    pub fn can_reconstruct_provenance(self) -> bool {
        !matches!(
            self.retention_budget.provenance_retention,
            ArtifactRetentionPolicy::Omit
        ) && self.reconstruction_budget.allow_provenance_reconstruction
    }

    pub fn explanation_behavior_summary(self) -> &'static str {
        self.retention_budget.explanation_retention.description()
    }

    pub fn provenance_behavior_summary(self) -> &'static str {
        self.retention_budget.provenance_retention.description()
    }
}

impl InstalledSignalRuntimePolicy {
    pub fn retains_explanation_facts(&self) -> bool {
        matches!(
            self.retention_budget().explanation_retention,
            ArtifactRetentionPolicy::Retain
        )
    }

    pub fn retains_provenance_facts(&self) -> bool {
        matches!(
            self.retention_budget().provenance_retention,
            ArtifactRetentionPolicy::Retain
        )
    }
}
