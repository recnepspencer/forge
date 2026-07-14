use serde::Serialize;

use crate::data::handle::NodeId;
use crate::data::output::OutputIdentity;
use crate::data::resource::{
    ResourceBoundaryPerformanceEnvelope, ResourceLifecycleClass, ResourceOutputContinuity,
    ResourceRequestHandle,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum AsyncNodeDownstreamDependenceFact {
    LifecycleClass,
    CommittedOutput,
    OutputContinuity,
    ObservationBoundary,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AsyncNodeGateStateReport {
    node: NodeId,
    upstream_dependency_count: u32,
    downstream_subscriber_count: u32,
    lifecycle_class: ResourceLifecycleClass,
    active_request_handle: Option<ResourceRequestHandle>,
    committed_output_identity: Option<OutputIdentity>,
    output_continuity: Option<ResourceOutputContinuity>,
    latest_observation_match_count: u32,
    downstream_dependence_facts: Vec<AsyncNodeDownstreamDependenceFact>,
    gate_digest: String,
    performance: ResourceBoundaryPerformanceEnvelope,
}

impl AsyncNodeGateStateReport {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        node: NodeId,
        upstream_dependency_count: u32,
        downstream_subscriber_count: u32,
        lifecycle_class: ResourceLifecycleClass,
        active_request_handle: Option<ResourceRequestHandle>,
        committed_output_identity: Option<OutputIdentity>,
        output_continuity: Option<ResourceOutputContinuity>,
        latest_observation_match_count: u32,
        downstream_dependence_facts: Vec<AsyncNodeDownstreamDependenceFact>,
        gate_digest: String,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self {
            node,
            upstream_dependency_count,
            downstream_subscriber_count,
            lifecycle_class,
            active_request_handle,
            committed_output_identity,
            output_continuity,
            latest_observation_match_count,
            downstream_dependence_facts,
            gate_digest,
            performance,
        }
    }

    pub fn node(&self) -> NodeId {
        self.node
    }

    pub fn upstream_dependency_count(&self) -> u32 {
        self.upstream_dependency_count
    }

    pub fn downstream_subscriber_count(&self) -> u32 {
        self.downstream_subscriber_count
    }

    pub fn lifecycle_class(&self) -> ResourceLifecycleClass {
        self.lifecycle_class
    }

    pub fn active_request_handle(&self) -> Option<ResourceRequestHandle> {
        self.active_request_handle
    }

    pub fn committed_output_identity(&self) -> Option<&OutputIdentity> {
        self.committed_output_identity.as_ref()
    }

    pub fn output_continuity(&self) -> Option<ResourceOutputContinuity> {
        self.output_continuity
    }

    pub fn latest_observation_match_count(&self) -> u32 {
        self.latest_observation_match_count
    }

    pub fn downstream_dependence_facts(&self) -> &[AsyncNodeDownstreamDependenceFact] {
        &self.downstream_dependence_facts
    }

    pub fn gate_digest(&self) -> &str {
        &self.gate_digest
    }

    pub fn performance(&self) -> ResourceBoundaryPerformanceEnvelope {
        self.performance
    }
}
