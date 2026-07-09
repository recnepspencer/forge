use std::fmt;

use serde::{Deserialize, Serialize};

pub mod profile;

use self::profile::DiagnosticsTier;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticsAvailability {
    RetainedAvailable,
    ReconstructedAvailable,
    OmittedByTier,
    DeniedByBudget,
    UnavailableNotRetained,
    UnavailableNotReconstructable,
}

impl Default for DiagnosticsAvailability {
    fn default() -> Self {
        Self::UnavailableNotRetained
    }
}

impl DiagnosticsAvailability {
    pub fn is_available(self) -> bool {
        matches!(self, Self::RetainedAvailable | Self::ReconstructedAvailable)
    }

    pub fn is_reconstructed(self) -> bool {
        matches!(self, Self::ReconstructedAvailable)
    }

    pub fn message(self) -> &'static str {
        match self {
            Self::RetainedAvailable => {
                "artifact detail was available from retained diagnostics state"
            }
            Self::ReconstructedAvailable => {
                "artifact detail was reconstructed through explicit cold materialization"
            }
            Self::OmittedByTier => "artifact detail is omitted by the active diagnostics tier",
            Self::DeniedByBudget => {
                "artifact detail was denied by the active reconstruction budget"
            }
            Self::UnavailableNotRetained => {
                "artifact detail is not retained in the active diagnostics envelope"
            }
            Self::UnavailableNotReconstructable => {
                "artifact detail is not reconstructable under the active diagnostics policy"
            }
        }
    }
}

impl fmt::Display for DiagnosticsAvailability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RetainedAvailable => write!(f, "RetainedAvailable"),
            Self::ReconstructedAvailable => write!(f, "ReconstructedAvailable"),
            Self::OmittedByTier => write!(f, "OmittedByTier"),
            Self::DeniedByBudget => write!(f, "DeniedByBudget"),
            Self::UnavailableNotRetained => write!(f, "UnavailableNotRetained"),
            Self::UnavailableNotReconstructable => write!(f, "UnavailableNotReconstructable"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct HistoryLimit(usize);

impl HistoryLimit {
    pub fn new(value: usize) -> Self {
        Self(value.max(1))
    }

    pub fn get(self) -> usize {
        self.0
    }

    pub fn max(self, other: usize) -> usize {
        self.0.max(other)
    }
}

impl fmt::Display for HistoryLimit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct DetailLimit(usize);

impl DetailLimit {
    pub fn new(value: usize) -> Self {
        Self(value.max(1))
    }

    pub fn get(self) -> usize {
        self.0
    }

    pub fn max(self, other: usize) -> usize {
        self.0.max(other)
    }
}

impl fmt::Display for DetailLimit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::ops::Mul<usize> for HistoryLimit {
    type Output = usize;

    fn mul(self, rhs: usize) -> Self::Output {
        self.0 * rhs
    }
}

impl PartialEq<usize> for HistoryLimit {
    fn eq(&self, other: &usize) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<usize> for HistoryLimit {
    fn partial_cmp(&self, other: &usize) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialEq<HistoryLimit> for usize {
    fn eq(&self, other: &HistoryLimit) -> bool {
        *self == other.0
    }
}

impl PartialOrd<HistoryLimit> for usize {
    fn partial_cmp(&self, other: &HistoryLimit) -> Option<std::cmp::Ordering> {
        self.partial_cmp(&other.0)
    }
}

impl PartialEq<usize> for DetailLimit {
    fn eq(&self, other: &usize) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<usize> for DetailLimit {
    fn partial_cmp(&self, other: &usize) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialEq<DetailLimit> for usize {
    fn eq(&self, other: &DetailLimit) -> bool {
        *self == other.0
    }
}

impl PartialOrd<DetailLimit> for usize {
    fn partial_cmp(&self, other: &DetailLimit) -> Option<std::cmp::Ordering> {
        self.partial_cmp(&other.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrdinaryAccessLane;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetainedForensicAccessLane;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExplicitColdAccessLane;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetentionBudget {
    pub history_limit: HistoryLimit,
    pub detail_limit: DetailLimit,
    pub retain_history_details: bool,
    pub retain_flow_explanation: bool,
    pub retain_latest_failure_context: bool,
    pub retain_stage_details: bool,
    pub capture_forensic_failure_context: bool,
    pub explanation_retention: ArtifactRetentionPolicy,
    pub provenance_retention: ArtifactRetentionPolicy,
    pub replay_detail: ReplayDetailPolicy,
    pub semantic_detail: SemanticRetentionPolicy,
}

impl RetentionBudget {
    pub fn operational() -> Self {
        Self {
            history_limit: HistoryLimit::new(4),
            detail_limit: DetailLimit::new(16),
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
            history_limit: HistoryLimit::new(16),
            detail_limit: DetailLimit::new(64),
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
            history_limit: HistoryLimit::new(64),
            detail_limit: DetailLimit::new(256),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReconstructionBudget {
    pub allow_explanation_reconstruction: bool,
    pub allow_provenance_reconstruction: bool,
    pub allow_replay_reconstruction: bool,
    pub allow_certification_materialization: bool,
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
    pub fn min_parallel_tasks_for(self, tier: DiagnosticsTier) -> usize {
        match tier {
            DiagnosticsTier::Operational => self.operational_min_parallel_tasks,
            DiagnosticsTier::Development => self.development_min_parallel_tasks,
            DiagnosticsTier::Forensic => self.forensic_min_parallel_tasks,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignalRuntimePolicy {
    pub tier: DiagnosticsTier,
    pub retention_budget: RetentionBudget,
    pub reconstruction_budget: ReconstructionBudget,
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
        Self::development()
    }
}

impl SignalRuntimePolicy {
    pub fn default_path_class(self) -> PathClass {
        match self.tier {
            DiagnosticsTier::Operational => PathClass::Operational,
            DiagnosticsTier::Development | DiagnosticsTier::Forensic => PathClass::Rich,
        }
    }

    pub fn default_artifact_policy_class(self) -> ArtifactPolicyClass {
        match (
            self.tier,
            self.retention_budget.explanation_retention,
            self.retention_budget.provenance_retention,
        ) {
            (
                DiagnosticsTier::Development,
                ArtifactRetentionPolicy::Retain,
                ArtifactRetentionPolicy::Retain,
            ) => ArtifactPolicyClass::DevelopmentRetained,
            (DiagnosticsTier::Forensic, _, _) => ArtifactPolicyClass::ForensicReconstructable,
            _ => ArtifactPolicyClass::OperationalMinimal,
        }
    }

    pub fn default_execution_strategy(self) -> ResolvedExecutionStrategy {
        match self.tier {
            DiagnosticsTier::Operational => ResolvedExecutionStrategy::SparseIncremental,
            DiagnosticsTier::Development | DiagnosticsTier::Forensic => {
                ResolvedExecutionStrategy::DenseStageBatched
            }
        }
    }

    pub fn default_maintenance_strategy(self) -> ResolvedMaintenanceStrategy {
        match self.tier {
            DiagnosticsTier::Operational => ResolvedMaintenanceStrategy::DensityAdaptive,
            DiagnosticsTier::Development => ResolvedMaintenanceStrategy::Incremental,
            DiagnosticsTier::Forensic => ResolvedMaintenanceStrategy::Rebuild,
        }
    }

    pub fn default_authority_policy(self) -> AuthorityPolicy {
        AuthorityPolicy::SpeculativeThenReconcile
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

    pub fn for_tier(tier: DiagnosticsTier) -> Self {
        let snapshot_restore_lineage_mode = match tier {
            DiagnosticsTier::Forensic => SnapshotRestoreLineageMode::PerNode,
            DiagnosticsTier::Operational | DiagnosticsTier::Development => {
                SnapshotRestoreLineageMode::CompactGlobal
            }
        };
        let frontier_tracing_policy = match tier {
            DiagnosticsTier::Operational => FrontierTracingPolicy::SummaryOnly,
            DiagnosticsTier::Development => FrontierTracingPolicy::RetainWaveRecords,
            DiagnosticsTier::Forensic => FrontierTracingPolicy::FullForensic,
        };

        Self {
            tier,
            retention_budget: RetentionBudget::for_tier(tier),
            reconstruction_budget: ReconstructionBudget::for_tier(tier),
            snapshot_restore_lineage_mode,
            frontier_tracing_policy,
            frontier_propagation_policy: FrontierPropagationPolicy::CanonicalFrontier,
            frontier_cycle_policy: FrontierCyclePolicy::ReachableCycleCheck,
            parallel_admission: ParallelAdmissionPolicy::default(),
        }
    }

    pub fn operational() -> Self {
        Self::for_tier(DiagnosticsTier::Operational)
    }

    pub fn development() -> Self {
        Self::for_tier(DiagnosticsTier::Development)
    }

    pub fn forensic() -> Self {
        Self::for_tier(DiagnosticsTier::Forensic)
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
        let mut policy = Self::development();
        policy.retention_budget.replay_detail = ReplayDetailPolicy::Forensic;
        policy.with_parallel_admission(ParallelAdmissionPolicy {
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

    pub fn with_explanation_retention(mut self, retention: ArtifactRetentionPolicy) -> Self {
        self.retention_budget.explanation_retention = retention;
        self
    }

    pub fn with_provenance_retention(mut self, retention: ArtifactRetentionPolicy) -> Self {
        self.retention_budget.provenance_retention = retention;
        self
    }

    pub fn with_replay_detail(mut self, replay_detail: ReplayDetailPolicy) -> Self {
        self.retention_budget.replay_detail = replay_detail;
        self
    }

    pub fn with_semantic_retention(mut self, semantic_retention: SemanticRetentionPolicy) -> Self {
        self.retention_budget.semantic_detail = semantic_retention;
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
        self.retention_budget.history_limit = HistoryLimit::new(history_limit);
        self
    }

    pub fn with_detail_limit(mut self, detail_limit: usize) -> Self {
        self.retention_budget.detail_limit = DetailLimit::new(detail_limit);
        self
    }

    pub fn with_history_details(mut self, retain_history_details: bool) -> Self {
        self.retention_budget.retain_history_details = retain_history_details;
        self
    }

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
