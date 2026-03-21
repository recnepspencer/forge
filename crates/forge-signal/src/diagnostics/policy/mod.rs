use serde::{Deserialize, Serialize};

pub mod profile;

use self::profile::DiagnosticsProfile;
use crate::data::node::{ArtifactPolicyClass, AuthorityPolicy, PathClass};
use crate::data::performance::ResolvedPerformancePolicy;
use crate::logic::planner::{ResolvedExecutionStrategy, ResolvedMaintenanceStrategy};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArtifactRetentionPolicy {
    Retain,
    Reconstruct,
    Omit,
}

impl ArtifactRetentionPolicy {
    pub fn description(self) -> &'static str {
        match self {
            Self::Retain => "retain eagerly in runtime state",
            Self::Reconstruct => "reconstruct deterministically on demand",
            Self::Omit => "omit unless a richer policy is configured",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArtifactMaterializationMode {
    Retained,
    Reconstructed,
    Unavailable,
}

impl Default for ArtifactMaterializationMode {
    fn default() -> Self {
        Self::Reconstructed
    }
}

impl ArtifactMaterializationMode {
    pub fn message(self) -> &'static str {
        match self {
            Self::Retained => "artifact was retained eagerly by the active runtime policy",
            Self::Reconstructed => {
                "artifact was reconstructed deterministically because the active runtime policy does not retain it eagerly"
            }
            Self::Unavailable => {
                "artifact is unavailable under the active runtime policy; choose a richer policy or use replay/native truth instead"
            }
        }
    }
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
pub enum SnapshotRestoreLineageMode {
    CompactGlobal,
    PerNode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum FrontierTracingPolicy {
    #[default]
    SummaryOnly,
    RetainWaveRecords,
    FullForensic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum FrontierPropagationPolicy {
    #[default]
    CanonicalFrontier,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum FrontierCyclePolicy {
    #[default]
    ReachableCycleCheck,
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
    pub snapshot_restore_lineage_mode: SnapshotRestoreLineageMode,
    #[serde(default)]
    pub frontier_tracing_policy: FrontierTracingPolicy,
    #[serde(default)]
    pub frontier_propagation_policy: FrontierPropagationPolicy,
    #[serde(default)]
    pub frontier_cycle_policy: FrontierCyclePolicy,
    pub parallel_admission: ParallelAdmissionPolicy,
}

impl Default for SignalRuntimePolicy {
    fn default() -> Self {
        Self::operational()
    }
}

impl SignalRuntimePolicy {
    pub fn default_path_class(self) -> PathClass {
        match self.profile {
            DiagnosticsProfile::Operational => PathClass::Operational,
            DiagnosticsProfile::Development | DiagnosticsProfile::Forensic => PathClass::Rich,
        }
    }

    pub fn default_artifact_policy_class(self) -> ArtifactPolicyClass {
        match (
            self.profile,
            self.explanation_retention,
            self.provenance_retention,
        ) {
            (
                DiagnosticsProfile::Development,
                ArtifactRetentionPolicy::Retain,
                ArtifactRetentionPolicy::Retain,
            ) => ArtifactPolicyClass::DevelopmentRetained,
            (DiagnosticsProfile::Forensic, _, _) => ArtifactPolicyClass::ForensicReconstructable,
            _ => ArtifactPolicyClass::OperationalMinimal,
        }
    }

    pub fn default_execution_strategy(self) -> ResolvedExecutionStrategy {
        match self.profile {
            DiagnosticsProfile::Operational => ResolvedExecutionStrategy::SparseIncremental,
            DiagnosticsProfile::Development | DiagnosticsProfile::Forensic => {
                ResolvedExecutionStrategy::DenseStageBatched
            }
        }
    }

    pub fn default_maintenance_strategy(self) -> ResolvedMaintenanceStrategy {
        match self.profile {
            DiagnosticsProfile::Operational => ResolvedMaintenanceStrategy::DensityAdaptive,
            DiagnosticsProfile::Development => ResolvedMaintenanceStrategy::Incremental,
            DiagnosticsProfile::Forensic => ResolvedMaintenanceStrategy::Rebuild,
        }
    }

    pub fn default_authority_policy(self) -> AuthorityPolicy {
        match self.profile {
            DiagnosticsProfile::Operational
            | DiagnosticsProfile::Development
            | DiagnosticsProfile::Forensic => AuthorityPolicy::SpeculativeThenReconcile,
        }
    }

    pub fn resolve_performance_policy(self) -> ResolvedPerformancePolicy {
        ResolvedPerformancePolicy {
            path_class: self.default_path_class(),
            artifact_policy: self.default_artifact_policy_class(),
            execution_strategy: self.default_execution_strategy(),
            maintenance_strategy: self.default_maintenance_strategy(),
            authority_policy: self.default_authority_policy(),
        }
    }

    pub fn web_development() -> Self {
        Self::operational().with_parallel_admission(ParallelAdmissionPolicy {
            operational_min_parallel_tasks: 4,
            development_min_parallel_tasks: 8,
            forensic_min_parallel_tasks: 12,
            full_parallel_min_tasks: 16,
        })
    }

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
            snapshot_restore_lineage_mode: SnapshotRestoreLineageMode::CompactGlobal,
            frontier_tracing_policy: FrontierTracingPolicy::SummaryOnly,
            frontier_propagation_policy: FrontierPropagationPolicy::CanonicalFrontier,
            frontier_cycle_policy: FrontierCyclePolicy::ReachableCycleCheck,
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
            snapshot_restore_lineage_mode: SnapshotRestoreLineageMode::CompactGlobal,
            frontier_tracing_policy: FrontierTracingPolicy::RetainWaveRecords,
            frontier_propagation_policy: FrontierPropagationPolicy::CanonicalFrontier,
            frontier_cycle_policy: FrontierCyclePolicy::ReachableCycleCheck,
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
            snapshot_restore_lineage_mode: SnapshotRestoreLineageMode::PerNode,
            frontier_tracing_policy: FrontierTracingPolicy::FullForensic,
            frontier_propagation_policy: FrontierPropagationPolicy::CanonicalFrontier,
            frontier_cycle_policy: FrontierCyclePolicy::ReachableCycleCheck,
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

    pub fn with_snapshot_restore_lineage_mode(mut self, mode: SnapshotRestoreLineageMode) -> Self {
        self.snapshot_restore_lineage_mode = mode;
        self
    }

    pub fn with_parallel_admission(mut self, parallel_admission: ParallelAdmissionPolicy) -> Self {
        self.parallel_admission = parallel_admission;
        self
    }

    pub fn with_history_limit(mut self, history_limit: usize) -> Self {
        self.history_limit = history_limit.max(1);
        self
    }

    pub fn with_detail_limit(mut self, detail_limit: usize) -> Self {
        self.detail_limit = detail_limit.max(1);
        self
    }

    pub fn with_history_details(mut self, retain_history_details: bool) -> Self {
        self.retain_history_details = retain_history_details;
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

    pub fn explanation_behavior_summary(self) -> &'static str {
        self.explanation_retention.description()
    }

    pub fn provenance_behavior_summary(self) -> &'static str {
        self.provenance_retention.description()
    }
}

pub type DiagnosticsPolicy = SignalRuntimePolicy;
