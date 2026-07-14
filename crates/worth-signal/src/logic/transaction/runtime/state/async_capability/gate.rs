use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::data::async_node::{AsyncNodeDownstreamDependenceFact, AsyncNodeGateStateReport};
use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::data::resource::{
    ResourceBoundaryPerformanceEnvelope, ResourceLifecycleClass, ResourceNodeId,
};

use super::SignalRuntime;

impl<D, I, E, Ctx, T> SignalRuntime<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn async_node_gate_state_report(
        &mut self,
        node: NodeId,
    ) -> Result<AsyncNodeGateStateReport, SignalError> {
        self.ensure_live_async_node_owner(node, "read async node gate state")?;
        if self.async_node_capability_bundle_for_node(node).is_none() {
            self.telemetry
                .resource
                .resource_undeclared_owner_denial_count += 1;
            return Err(SignalError::invalid_input(format!(
                "cannot read async node gate state for undeclared node {node}"
            )));
        }

        let resource_node = ResourceNodeId::from_node(node);
        let lifecycle_class = self
            .resource
            .lifecycle_summary_for_node(resource_node)
            .map(|summary| summary.lifecycle())
            .unwrap_or(ResourceLifecycleClass::Unrequested);
        let active_request_handle = self.resource.active_request_handle_for_node(resource_node);
        let committed_output_identity = self
            .graph
            .observe()
            .runtime_artifact_warm(node)?
            .and_then(|warm| warm.output_identity.clone());
        let output_continuity = self
            .resource
            .observed_resource_node_state(resource_node)
            .and_then(|state| state.output_continuity());
        let latest_observation_match_count = self
            .observe()
            .latest_observation_summary()
            .map(|summary| {
                summary
                    .boundary_events
                    .iter()
                    .filter(|event| event.matched_nodes.contains(node))
                    .count() as u32
            })
            .unwrap_or(0);
        let gate_digest = async_gate_digest(&AsyncNodeGateDigestBasis {
            node,
            lifecycle_class,
            active_request_identity: active_request_handle
                .as_ref()
                .map(|handle| (handle.request_id().get(), handle.generation().get())),
            latest_observation_match_count,
            upstream_dependency_count: self.graph.dependencies_of(node)?.len() as u32,
            downstream_subscriber_count: self.graph.subscribers_of(node)?.len() as u32,
            committed_output_identity: committed_output_identity.as_ref().map(|id| id.as_str()),
            output_continuity,
        });
        let upstream_dependency_count = self.graph.dependencies_of(node)?.len() as u32;
        let downstream_subscriber_count = self.graph.subscribers_of(node)?.len() as u32;
        let performance = ResourceBoundaryPerformanceEnvelope::async_node_gate_state(
            upstream_dependency_count,
            downstream_subscriber_count,
        );
        self.telemetry
            .resource
            .record_boundary_performance_envelope(performance);
        Ok(AsyncNodeGateStateReport::new(
            node,
            upstream_dependency_count,
            downstream_subscriber_count,
            lifecycle_class,
            active_request_handle,
            committed_output_identity,
            output_continuity,
            latest_observation_match_count,
            vec![
                AsyncNodeDownstreamDependenceFact::LifecycleClass,
                AsyncNodeDownstreamDependenceFact::CommittedOutput,
                AsyncNodeDownstreamDependenceFact::OutputContinuity,
                AsyncNodeDownstreamDependenceFact::ObservationBoundary,
            ],
            gate_digest,
            performance,
        ))
    }
}

#[derive(Serialize)]
struct AsyncNodeGateDigestBasis<'a> {
    node: NodeId,
    lifecycle_class: ResourceLifecycleClass,
    active_request_identity: Option<(u64, u64)>,
    latest_observation_match_count: u32,
    upstream_dependency_count: u32,
    downstream_subscriber_count: u32,
    committed_output_identity: Option<&'a str>,
    output_continuity: Option<crate::data::resource::ResourceOutputContinuity>,
}

fn async_gate_digest<T: Serialize>(basis: &T) -> String {
    let bytes = serde_json::to_vec(basis).expect("async gate digest serialization");
    let digest = Sha256::digest(bytes);
    format!("{digest:x}")
}
