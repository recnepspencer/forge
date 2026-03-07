//! Tier policy types for N-granular signal scheduling.

use crate::data::checkpoint::CheckpointBarrier;
use crate::data::comparator::VersionComparatorPolicy;

/// Dependency discovery mode for a tier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DependencyMode {
    /// Dependencies are declared externally and routed statically.
    Static,
    /// Dependencies are discovered during evaluation reads.
    AutoDiscovered,
}

/// Dirty propagation policy for a tier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DirtyPropagation {
    /// Collect dirty domains/nodes and flush at configured checkpoints.
    Batched,
    /// Push dirtiness immediately to downstream dependencies.
    Immediate,
}

/// Evaluation trigger policy for a tier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EvaluationTrigger {
    /// Evaluate when this checkpoint barrier is reached.
    Checkpoint(CheckpointBarrier),
    /// Evaluate lazily when downstream reads occur.
    LazyPull,
    /// Evaluate only when explicitly requested.
    OnDemand,
    /// Evaluate on an async scheduler (execution backend-defined).
    Async,
}

/// Tier policy for a caller-defined tier key.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TierPolicy<T: Copy + Ord> {
    pub tier: T,
    pub dependency_mode: DependencyMode,
    pub dirty_propagation: DirtyPropagation,
    pub evaluation_trigger: EvaluationTrigger,
    pub default_comparator: VersionComparatorPolicy,
}

impl<T: Copy + Ord> TierPolicy<T> {
    /// Create a new explicit tier policy.
    pub fn new(
        tier: T,
        dependency_mode: DependencyMode,
        dirty_propagation: DirtyPropagation,
        evaluation_trigger: EvaluationTrigger,
    ) -> Self {
        Self {
            tier,
            dependency_mode,
            dirty_propagation,
            evaluation_trigger,
            default_comparator: VersionComparatorPolicy::Exact,
        }
    }

    /// Override the tier-level default comparator policy.
    pub fn with_default_comparator(mut self, default_comparator: VersionComparatorPolicy) -> Self {
        self.default_comparator = default_comparator;
        self
    }
}

/// Legacy Forge tier labels retained for compatibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[deprecated(note = "Use caller-defined tier keys with TierPolicy<T>")]
pub enum EvaluationTier {
    Entity,
    Feature,
    Analysis,
}
