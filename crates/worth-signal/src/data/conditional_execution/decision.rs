use crate::data::node::InstalledSignalConditionIdentity;
use crate::logic::evaluation::ConditionEvaluationContext;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstalledSignalConditionDecision {
    Eligible,
    Suppressed,
    Deferred,
}

pub trait InstalledSignalConditionResolver {
    fn resolve(
        &mut self,
        identity: InstalledSignalConditionIdentity,
        context: &ConditionEvaluationContext,
    ) -> Result<InstalledSignalConditionDecision, crate::data::error::SignalError>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalConditionalDecisionClass {
    ComputedChanged,
    ComputedRevertedClean,
    DependencyUnchanged,
    SuppressedBeforeCompute,
    DeferredByCondition,
    DeferredTemporal,
    DeferredOnDemand,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SignalConditionalDecisionCounters {
    pub dependency_version_checks: usize,
    pub condition_checks: usize,
    pub comparator_checks: usize,
    pub compute_contacts: usize,
    pub semantic_changes: usize,
    pub reuse_checks: usize,
}

/// Signal-owned operational evidence. Fields are private so labels, digests,
/// or resolver return values cannot be restamped into execution authority.
pub struct SignalConditionalDecisionEvidence {
    pub(crate) identity: String,
    pub(crate) graph_instance_id: u64,
    pub(crate) node: crate::data::handle::NodeId,
    pub(crate) snapshot_identity: String,
    pub(crate) execution_identity: String,
    pub(crate) attempt: u64,
    pub(crate) class: SignalConditionalDecisionClass,
    pub(crate) counters: SignalConditionalDecisionCounters,
    pub(crate) artifact_reuse_admitted: bool,
    pub(crate) condition: crate::data::node::EvaluationCondition,
    pub(crate) semantic_condition: super::SignalConditionalCondition,
    pub(crate) dependency_aspects: crate::data::aspect::AspectMask,
    pub(crate) trigger_aspects: crate::data::aspect::AspectMask,
    pub(crate) dependency_comparator: crate::data::comparator::VersionComparatorPolicy,
    pub(crate) output_comparator: crate::data::comparator::VersionComparatorPolicy,
    pub(crate) artifact_reuse: super::SignalConditionalArtifactReusePolicy,
    pub(super) _dependency_versions:
        Vec<super::dependency_versions::SignalConditionalDependencyVersion>,
    pub(super) _execution: super::execution_proof::SignalConditionalExecutedRecipe,
}

impl SignalConditionalDecisionEvidence {
    pub fn identity(&self) -> &str {
        &self.identity
    }
    pub const fn graph_instance_id(&self) -> u64 {
        self.graph_instance_id
    }
    pub const fn node(&self) -> crate::data::handle::NodeId {
        self.node
    }
    pub fn snapshot_identity(&self) -> &str {
        &self.snapshot_identity
    }
    pub fn execution_identity(&self) -> &str {
        &self.execution_identity
    }
    pub const fn attempt(&self) -> u64 {
        self.attempt
    }
    pub const fn class(&self) -> SignalConditionalDecisionClass {
        self.class
    }
    pub const fn counters(&self) -> SignalConditionalDecisionCounters {
        self.counters
    }
    pub const fn artifact_reuse_admitted(&self) -> bool {
        self.artifact_reuse_admitted
    }
    pub fn dependency_version_count(&self) -> usize {
        self._dependency_versions.len()
    }
    pub fn condition(&self) -> &crate::data::node::EvaluationCondition {
        &self.condition
    }
    pub fn semantic_condition(&self) -> &super::SignalConditionalCondition {
        &self.semantic_condition
    }
    pub const fn dependency_aspects(&self) -> crate::data::aspect::AspectMask {
        self.dependency_aspects
    }
    pub const fn trigger_aspects(&self) -> crate::data::aspect::AspectMask {
        self.trigger_aspects
    }
    pub fn dependency_comparator(&self) -> &crate::data::comparator::VersionComparatorPolicy {
        &self.dependency_comparator
    }
    pub fn output_comparator(&self) -> &crate::data::comparator::VersionComparatorPolicy {
        &self.output_comparator
    }
    pub fn artifact_reuse(&self) -> &super::SignalConditionalArtifactReusePolicy {
        &self.artifact_reuse
    }
}
