use serde::{Deserialize, Serialize};

use crate::data::aspect::AspectMask;
use crate::data::handle::NodeId;
use crate::data::node::{EvaluationCondition, NodeState};
use crate::data::resource::{
    ResourceBoundaryPerformanceEnvelope, ResourceLifecycleClass, ResourcePolicyDigest,
    ResourceRequestAdmissionReport, ResourceRevalidationReport,
};
use crate::data::temporal::TemporalPreviousValueReference;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AsyncNodeAdmissionClass {
    AdmittedNewLineage,
    RefreshEligibleNoNewLineage,
    BlockedByCondition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AsyncNodeConditionBlockClass {
    ContractAspectMismatch,
    AspectFilterMismatch,
    PartitionScopeMismatch,
    DeltaThresholdNotCrossed,
    TemporalConditionNotReady,
    PreviousValueReferenceDrifted,
    CustomConditionResolverRequired,
    CustomConditionRejected,
    DependencyNotReady,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct AsyncNodeAdmissionClassification {
    node: NodeId,
    node_state: NodeState,
    lifecycle_class: ResourceLifecycleClass,
    condition: EvaluationCondition,
    class: AsyncNodeAdmissionClass,
    condition_block_class: Option<AsyncNodeConditionBlockClass>,
    dirty_aspects: AspectMask,
    dirty_partition_scope_count: u32,
    contract_partition_scope_count: u32,
    max_dependency_delta: u64,
    requires_clean_dependencies: bool,
    previous_value_reference: Option<TemporalPreviousValueReference>,
    decision_digest: ResourcePolicyDigest,
    performance: ResourceBoundaryPerformanceEnvelope,
}

impl AsyncNodeAdmissionClassification {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        node: NodeId,
        node_state: NodeState,
        lifecycle_class: ResourceLifecycleClass,
        condition: EvaluationCondition,
        class: AsyncNodeAdmissionClass,
        condition_block_class: Option<AsyncNodeConditionBlockClass>,
        dirty_aspects: AspectMask,
        dirty_partition_scope_count: u32,
        contract_partition_scope_count: u32,
        max_dependency_delta: u64,
        requires_clean_dependencies: bool,
        previous_value_reference: Option<TemporalPreviousValueReference>,
        decision_digest: ResourcePolicyDigest,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self {
            node,
            node_state,
            lifecycle_class,
            condition,
            class,
            condition_block_class,
            dirty_aspects,
            dirty_partition_scope_count,
            contract_partition_scope_count,
            max_dependency_delta,
            requires_clean_dependencies,
            previous_value_reference,
            decision_digest,
            performance,
        }
    }

    pub fn node(&self) -> NodeId {
        self.node
    }

    pub fn node_state(&self) -> NodeState {
        self.node_state
    }

    pub fn lifecycle_class(&self) -> ResourceLifecycleClass {
        self.lifecycle_class
    }

    pub fn condition(&self) -> &EvaluationCondition {
        &self.condition
    }

    pub fn class(&self) -> AsyncNodeAdmissionClass {
        self.class
    }

    pub fn condition_block_class(&self) -> Option<AsyncNodeConditionBlockClass> {
        self.condition_block_class
    }

    pub fn dirty_aspects(&self) -> AspectMask {
        self.dirty_aspects
    }

    pub fn dirty_partition_scope_count(&self) -> u32 {
        self.dirty_partition_scope_count
    }

    pub fn contract_partition_scope_count(&self) -> u32 {
        self.contract_partition_scope_count
    }

    pub fn max_dependency_delta(&self) -> u64 {
        self.max_dependency_delta
    }

    pub fn requires_clean_dependencies(&self) -> bool {
        self.requires_clean_dependencies
    }

    pub fn previous_value_reference(&self) -> Option<&TemporalPreviousValueReference> {
        self.previous_value_reference.as_ref()
    }

    pub fn decision_digest(&self) -> &ResourcePolicyDigest {
        &self.decision_digest
    }

    pub fn performance(&self) -> ResourceBoundaryPerformanceEnvelope {
        self.performance
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct AsyncNodeRequestAdmissionReport {
    classification: AsyncNodeAdmissionClassification,
    resource_admission: Option<ResourceRequestAdmissionReport>,
}

impl AsyncNodeRequestAdmissionReport {
    pub(crate) fn blocked(classification: AsyncNodeAdmissionClassification) -> Self {
        Self {
            classification,
            resource_admission: None,
        }
    }

    pub(crate) fn admitted(
        classification: AsyncNodeAdmissionClassification,
        resource_admission: ResourceRequestAdmissionReport,
    ) -> Self {
        Self {
            classification,
            resource_admission: Some(resource_admission),
        }
    }

    pub fn classification(&self) -> &AsyncNodeAdmissionClassification {
        &self.classification
    }

    pub fn resource_admission(&self) -> Option<&ResourceRequestAdmissionReport> {
        self.resource_admission.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct AsyncNodeRevalidationReport {
    classification: AsyncNodeAdmissionClassification,
    resource_revalidation: Option<ResourceRevalidationReport>,
}

impl AsyncNodeRevalidationReport {
    pub(crate) fn blocked(classification: AsyncNodeAdmissionClassification) -> Self {
        Self {
            classification,
            resource_revalidation: None,
        }
    }

    pub(crate) fn revalidated(
        classification: AsyncNodeAdmissionClassification,
        resource_revalidation: ResourceRevalidationReport,
    ) -> Self {
        Self {
            classification,
            resource_revalidation: Some(resource_revalidation),
        }
    }

    pub fn classification(&self) -> &AsyncNodeAdmissionClassification {
        &self.classification
    }

    pub fn resource_revalidation(&self) -> Option<&ResourceRevalidationReport> {
        self.resource_revalidation.as_ref()
    }
}
