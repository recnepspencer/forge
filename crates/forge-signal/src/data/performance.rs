use serde::{Deserialize, Serialize};

use crate::data::comparator::VersionComparatorPolicy;
use crate::data::telemetry::{
    CheckpointTelemetry, EvaluationTelemetry, ExecutionTelemetry, InvalidationTelemetry,
    PlannerTelemetry, RuntimeTelemetry, StorageTelemetry, TransactionTelemetry,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum IdentityBasis {
    #[default]
    AspectVersion,
    OutputIdentity,
    ContinuityToken,
    OutputIdentityAndContinuity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum SuppressionBasis {
    #[default]
    Never,
    OutputIdentity,
    ContinuityToken,
    ComparatorMatch,
    OutputIdentityAndComparator,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum CanonicalDependencyOrder {
    #[default]
    SourceAspectScope,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComparatorBasis {
    TierDefault,
    Exact,
    Tolerance,
    OutputIdentity,
    Custom,
}

impl Default for ComparatorBasis {
    fn default() -> Self {
        Self::TierDefault
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct EquivalenceContract {
    pub identity_basis: IdentityBasis,
    pub suppression_basis: SuppressionBasis,
    pub canonical_dependency_order: CanonicalDependencyOrder,
    pub comparator_basis: ComparatorBasis,
}

impl EquivalenceContract {
    pub fn for_comparator_override(comparator: &VersionComparatorPolicy) -> Self {
        let comparator_basis = match comparator {
            VersionComparatorPolicy::Exact => ComparatorBasis::Exact,
            VersionComparatorPolicy::Tolerance { .. } => ComparatorBasis::Tolerance,
            VersionComparatorPolicy::OutputIdentity => ComparatorBasis::OutputIdentity,
            VersionComparatorPolicy::Custom { .. } => ComparatorBasis::Custom,
        };
        let suppression_basis = match comparator {
            VersionComparatorPolicy::OutputIdentity => {
                SuppressionBasis::OutputIdentityAndComparator
            }
            _ => SuppressionBasis::ComparatorMatch,
        };
        let identity_basis = match comparator {
            VersionComparatorPolicy::OutputIdentity => IdentityBasis::OutputIdentity,
            _ => IdentityBasis::AspectVersion,
        };
        Self {
            identity_basis,
            suppression_basis,
            canonical_dependency_order: CanonicalDependencyOrder::SourceAspectScope,
            comparator_basis,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum PathClass {
    #[default]
    Operational,
    Rich,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum MaintenanceMode {
    IncrementalOnly,
    RebuildAllowed,
    #[default]
    DensityAdaptive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ArtifactPolicyClass {
    #[default]
    OperationalMinimal,
    DevelopmentRetained,
    ForensicReconstructable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum AuthorityPolicy {
    AuthoritativeOnly,
    #[default]
    SpeculativeThenReconcile,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ResolvedExecutionStrategy {
    #[default]
    SparseIncremental,
    DenseStageBatched,
    FullGraphPass,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ResolvedMaintenanceStrategy {
    Incremental,
    Rebuild,
    #[default]
    DensityAdaptive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PerformanceEnforcementLayer {
    CompileTime,
    RuntimePolicy,
    CounterTest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompileTimePerformanceContract {
    pub equivalence: EquivalenceContract,
    pub path_class: PathClass,
    pub maintenance_mode: MaintenanceMode,
    pub artifact_policy: ArtifactPolicyClass,
    pub authority_policy: AuthorityPolicy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResolvedPerformancePolicy {
    pub path_class: PathClass,
    pub artifact_policy: ArtifactPolicyClass,
    pub execution_strategy: ResolvedExecutionStrategy,
    pub maintenance_strategy: ResolvedMaintenanceStrategy,
    pub authority_policy: AuthorityPolicy,
}

#[derive(Debug, Clone, Copy)]
pub struct PerformanceCounterSurface<'a> {
    pub evaluation: &'a EvaluationTelemetry,
    pub invalidation: &'a InvalidationTelemetry,
    pub transaction: &'a TransactionTelemetry,
    pub planner: &'a PlannerTelemetry,
    pub execution: &'a ExecutionTelemetry,
    pub storage: &'a StorageTelemetry,
    pub checkpoint: &'a CheckpointTelemetry,
}

impl RuntimeTelemetry {
    pub fn performance_counter_surface(&self) -> PerformanceCounterSurface<'_> {
        PerformanceCounterSurface {
            evaluation: &self.evaluation,
            invalidation: &self.invalidation,
            transaction: &self.transaction,
            planner: &self.planner,
            execution: &self.execution,
            storage: &self.storage,
            checkpoint: &self.checkpoint,
        }
    }
}
