use serde::{Deserialize, Serialize};

use crate::diagnostics::profile::DiagnosticsProfile;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArtifactRetentionPolicy {
    Retain,
    Reconstruct,
    Omit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArtifactMaterializationMode {
    Retained,
    Reconstructed,
    Unavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReplayDetailPolicy {
    Minimal,
    Standard,
    Forensic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SemanticRetentionPolicy {
    Minimal,
    Development,
    Forensic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParallelAdmissionPolicy {
    pub operational_min_parallel_tasks: usize,
    pub development_min_parallel_tasks: usize,
    pub forensic_min_parallel_tasks: usize,
    pub full_parallel_min_tasks: usize,
}

impl Default for ParallelAdmissionPolicy {
    fn default() -> Self {
        Self {
            operational_min_parallel_tasks: 2,
            development_min_parallel_tasks: 4,
            forensic_min_parallel_tasks: 8,
            full_parallel_min_tasks: 8,
        }
    }
}

impl ParallelAdmissionPolicy {
    pub fn min_parallel_tasks_for(self, profile: DiagnosticsProfile) -> usize {
        match profile {
            DiagnosticsProfile::Operational => self.operational_min_parallel_tasks,
            DiagnosticsProfile::Development => self.development_min_parallel_tasks,
            DiagnosticsProfile::Forensic => self.forensic_min_parallel_tasks,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignalRuntimePolicy {
    pub profile: DiagnosticsProfile,
    pub history_limit: usize,
    pub detail_limit: usize,
    pub retain_history_details: bool,
    pub retain_flow_explanation: bool,
    pub retain_latest_failure_context: bool,
    pub retain_stage_details: bool,
    pub capture_forensic_failure_context: bool,
    pub explanation_retention: ArtifactRetentionPolicy,
    pub provenance_retention: ArtifactRetentionPolicy,
    pub replay_detail: ReplayDetailPolicy,
    pub semantic_retention: SemanticRetentionPolicy,
    pub parallel_admission: ParallelAdmissionPolicy,
}

impl Default for SignalRuntimePolicy {
    fn default() -> Self {
        Self::operational()
    }
}

impl SignalRuntimePolicy {
    pub fn kernel() -> Self {
        Self::forensic().with_parallel_admission(ParallelAdmissionPolicy {
            operational_min_parallel_tasks: 4,
            development_min_parallel_tasks: 8,
            forensic_min_parallel_tasks: 16,
            full_parallel_min_tasks: 16,
        })
    }

    pub fn fintech() -> Self {
        Self::development()
            .with_replay_detail(ReplayDetailPolicy::Forensic)
            .with_parallel_admission(ParallelAdmissionPolicy {
                operational_min_parallel_tasks: 4,
                development_min_parallel_tasks: 8,
                forensic_min_parallel_tasks: 12,
                full_parallel_min_tasks: 12,
            })
    }

    pub fn game_engine() -> Self {
        Self::operational().with_parallel_admission(ParallelAdmissionPolicy {
            operational_min_parallel_tasks: 2,
            development_min_parallel_tasks: 4,
            forensic_min_parallel_tasks: 8,
            full_parallel_min_tasks: 8,
        })
    }

    pub fn operational() -> Self {
        Self {
            profile: DiagnosticsProfile::Operational,
            history_limit: DiagnosticsProfile::Operational.history_limit(),
            detail_limit: DiagnosticsProfile::Operational.detail_limit(),
            retain_history_details: false,
            retain_flow_explanation: false,
            retain_latest_failure_context: false,
            retain_stage_details: false,
            capture_forensic_failure_context: false,
            explanation_retention: ArtifactRetentionPolicy::Reconstruct,
            provenance_retention: ArtifactRetentionPolicy::Reconstruct,
            replay_detail: ReplayDetailPolicy::Minimal,
            semantic_retention: SemanticRetentionPolicy::Minimal,
            parallel_admission: ParallelAdmissionPolicy::default(),
        }
    }

    pub fn development() -> Self {
        Self {
            profile: DiagnosticsProfile::Development,
            history_limit: DiagnosticsProfile::Development.history_limit(),
            detail_limit: DiagnosticsProfile::Development.detail_limit(),
            retain_history_details: true,
            retain_flow_explanation: true,
            retain_latest_failure_context: true,
            retain_stage_details: true,
            capture_forensic_failure_context: false,
            explanation_retention: ArtifactRetentionPolicy::Retain,
            provenance_retention: ArtifactRetentionPolicy::Retain,
            replay_detail: ReplayDetailPolicy::Standard,
            semantic_retention: SemanticRetentionPolicy::Development,
            parallel_admission: ParallelAdmissionPolicy::default(),
        }
    }

    pub fn forensic() -> Self {
        Self {
            profile: DiagnosticsProfile::Forensic,
            history_limit: DiagnosticsProfile::Forensic.history_limit(),
            detail_limit: DiagnosticsProfile::Forensic.detail_limit(),
            retain_history_details: true,
            retain_flow_explanation: true,
            retain_latest_failure_context: true,
            retain_stage_details: true,
            capture_forensic_failure_context: true,
            explanation_retention: ArtifactRetentionPolicy::Retain,
            provenance_retention: ArtifactRetentionPolicy::Retain,
            replay_detail: ReplayDetailPolicy::Forensic,
            semantic_retention: SemanticRetentionPolicy::Forensic,
            parallel_admission: ParallelAdmissionPolicy::default(),
        }
    }

    pub fn from_profile(profile: DiagnosticsProfile) -> Self {
        match profile {
            DiagnosticsProfile::Operational => Self::operational(),
            DiagnosticsProfile::Development => Self::development(),
            DiagnosticsProfile::Forensic => Self::forensic(),
        }
    }

    pub fn with_explanation_retention(mut self, retention: ArtifactRetentionPolicy) -> Self {
        self.explanation_retention = retention;
        self
    }

    pub fn with_provenance_retention(mut self, retention: ArtifactRetentionPolicy) -> Self {
        self.provenance_retention = retention;
        self
    }

    pub fn with_replay_detail(mut self, replay_detail: ReplayDetailPolicy) -> Self {
        self.replay_detail = replay_detail;
        self
    }

    pub fn with_semantic_retention(mut self, semantic_retention: SemanticRetentionPolicy) -> Self {
        self.semantic_retention = semantic_retention;
        self
    }

    pub fn with_parallel_admission(mut self, parallel_admission: ParallelAdmissionPolicy) -> Self {
        self.parallel_admission = parallel_admission;
        self
    }

    pub fn retains_explanation_facts(self) -> bool {
        matches!(self.explanation_retention, ArtifactRetentionPolicy::Retain)
    }

    pub fn retains_provenance_facts(self) -> bool {
        matches!(self.provenance_retention, ArtifactRetentionPolicy::Retain)
    }

    pub fn can_reconstruct_explanation(self) -> bool {
        !matches!(self.explanation_retention, ArtifactRetentionPolicy::Omit)
    }

    pub fn can_reconstruct_provenance(self) -> bool {
        !matches!(self.provenance_retention, ArtifactRetentionPolicy::Omit)
    }
}

pub type DiagnosticsPolicy = SignalRuntimePolicy;
