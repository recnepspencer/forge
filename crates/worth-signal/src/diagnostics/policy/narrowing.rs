use super::definition::{
    ReconstructionBudget, ReplayDetailPolicy, RetentionBudget, SemanticRetentionPolicy,
};
use super::materialization::ArtifactRetentionPolicy;
use crate::diagnostics::profile::DiagnosticsTier;

impl RetentionBudget {
    pub fn operational() -> Self {
        Self {
            history_limit: super::definition::HistoryLimit::new(4),
            detail_limit: super::definition::DetailLimit::new(16),
            retain_history_details: false,
            retain_flow_explanation: false,
            retain_latest_failure_context: false,
            retain_stage_details: false,
            capture_forensic_failure_context: false,
            explanation_retention: ArtifactRetentionPolicy::Reconstruct,
            provenance_retention: ArtifactRetentionPolicy::Reconstruct,
            replay_detail: ReplayDetailPolicy::Minimal,
            semantic_detail: SemanticRetentionPolicy::Minimal,
        }
    }

    pub fn development() -> Self {
        Self {
            history_limit: super::definition::HistoryLimit::new(16),
            detail_limit: super::definition::DetailLimit::new(64),
            retain_history_details: true,
            retain_flow_explanation: true,
            retain_latest_failure_context: true,
            retain_stage_details: true,
            capture_forensic_failure_context: false,
            explanation_retention: ArtifactRetentionPolicy::Retain,
            provenance_retention: ArtifactRetentionPolicy::Retain,
            replay_detail: ReplayDetailPolicy::Standard,
            semantic_detail: SemanticRetentionPolicy::Development,
        }
    }

    pub fn forensic() -> Self {
        Self {
            history_limit: super::definition::HistoryLimit::new(64),
            detail_limit: super::definition::DetailLimit::new(256),
            retain_history_details: true,
            retain_flow_explanation: true,
            retain_latest_failure_context: true,
            retain_stage_details: true,
            capture_forensic_failure_context: true,
            explanation_retention: ArtifactRetentionPolicy::Retain,
            provenance_retention: ArtifactRetentionPolicy::Retain,
            replay_detail: ReplayDetailPolicy::Forensic,
            semantic_detail: SemanticRetentionPolicy::Forensic,
        }
    }

    pub fn for_tier(tier: DiagnosticsTier) -> Self {
        match tier {
            DiagnosticsTier::Operational => Self::operational(),
            DiagnosticsTier::Development => Self::development(),
            DiagnosticsTier::Forensic => Self::forensic(),
        }
    }
}

impl Default for RetentionBudget {
    fn default() -> Self {
        Self::operational()
    }
}

impl ReconstructionBudget {
    pub fn operational() -> Self {
        Self {
            allow_explanation_reconstruction: true,
            allow_provenance_reconstruction: true,
            allow_replay_reconstruction: false,
            allow_certification_materialization: false,
        }
    }

    pub fn development() -> Self {
        Self {
            allow_explanation_reconstruction: true,
            allow_provenance_reconstruction: true,
            allow_replay_reconstruction: false,
            allow_certification_materialization: true,
        }
    }

    pub fn forensic() -> Self {
        Self {
            allow_explanation_reconstruction: true,
            allow_provenance_reconstruction: true,
            allow_replay_reconstruction: true,
            allow_certification_materialization: true,
        }
    }

    pub fn for_tier(tier: DiagnosticsTier) -> Self {
        match tier {
            DiagnosticsTier::Operational => Self::operational(),
            DiagnosticsTier::Development => Self::development(),
            DiagnosticsTier::Forensic => Self::forensic(),
        }
    }
}
