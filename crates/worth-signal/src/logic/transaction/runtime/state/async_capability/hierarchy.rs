use std::collections::{BTreeSet, VecDeque};

use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::data::async_node::{
    AsyncNodeHierarchyCancellationReport, AsyncNodeHierarchyReplaySummary,
};
use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::data::resource::{
    ResourceBoundaryPerformanceEnvelope, ResourceCancellationReason, ResourceLifecycleClass,
    ResourceNodeId, ResourceRequestHandle,
};

use super::SignalRuntime;

impl<D, I, E, Ctx, T> SignalRuntime<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn async_node_hierarchy_replay_summary(
        &mut self,
        root: NodeId,
    ) -> Result<AsyncNodeHierarchyReplaySummary, SignalError> {
        self.ensure_live_async_node_owner(root, "read async node hierarchy replay summary")?;
        if self.async_node_capability_bundle_for_node(root).is_none() {
            self.with_resource_telemetry(|telemetry| {
                telemetry.resource_undeclared_owner_denial_count += 1
            });
            return Err(SignalError::invalid_input(format!(
                "cannot read async node hierarchy replay summary for undeclared root {root}"
            )));
        }

        let members = self.collect_async_hierarchy_nodes(root)?;
        let active_request_handles = members
            .iter()
            .filter_map(|node| {
                self.resource
                    .active_request_handle_for_node(ResourceNodeId::from_node(*node))
            })
            .collect::<Vec<_>>();
        let lifecycle_rows = members
            .iter()
            .map(|node| {
                (
                    *node,
                    self.resource
                        .lifecycle_summary_for_node(ResourceNodeId::from_node(*node))
                        .map(|summary| summary.lifecycle())
                        .unwrap_or(ResourceLifecycleClass::Unrequested),
                )
            })
            .collect::<Vec<_>>();
        let hierarchy_depth = async_hierarchy_depth(self, root)?;
        let performance = ResourceBoundaryPerformanceEnvelope::async_node_hierarchy_replay(
            members.len() as u32,
            active_request_handles.len() as u32,
            hierarchy_depth,
        );
        self.with_resource_telemetry(|telemetry| {
            telemetry.record_boundary_performance_envelope(performance)
        });
        let lifecycle_digest = async_hierarchy_digest(&lifecycle_rows);
        let active_request_identities = active_request_handles
            .iter()
            .map(|handle| (handle.request_id().get(), handle.generation().get()))
            .collect::<Vec<_>>();
        let in_flight_digest = async_hierarchy_digest(&active_request_identities);
        let replay_digest = async_hierarchy_digest(&AsyncHierarchyReplayDigestBasis {
            root,
            members: &members,
            lifecycle_rows: &lifecycle_rows,
            active_request_identities: &active_request_identities,
        });
        Ok(AsyncNodeHierarchyReplaySummary::new(
            root,
            members,
            active_request_handles,
            hierarchy_depth,
            lifecycle_digest,
            in_flight_digest,
            replay_digest,
            performance,
        ))
    }

    pub fn cancel_async_node_request(
        &mut self,
        handle: ResourceRequestHandle,
        reason: ResourceCancellationReason,
    ) -> Result<AsyncNodeHierarchyCancellationReport, SignalError> {
        let root_node = self
            .in_flight_resource_request(handle)
            .map(|request| request.node().node())
            .ok_or_else(|| {
                SignalError::invalid_input(format!(
                    "cannot cancel async node request for unknown handle {}",
                    handle.request_id().get()
                ))
            })?;
        let cancellation = self.cancel_resource_request(handle, reason)?;
        let affected_nodes = cancellation
            .dependent_propagation()
            .map(|propagation| -> Vec<NodeId> {
                let mut nodes = Vec::with_capacity(1 + propagation.cancelled_dependents().len());
                nodes.push(root_node);
                for cancelled in propagation.cancelled_dependents() {
                    if let Some(node) = self
                        .in_flight_resource_request(cancelled.handle())
                        .map(|request| request.node().node())
                    {
                        nodes.push(node);
                    }
                }
                nodes
            })
            .unwrap_or_else(|| vec![root_node]);
        let propagated_hierarchy_width = affected_nodes.len().saturating_sub(1) as u32;
        self.with_resource_telemetry(|telemetry| {
            telemetry.async_node_hierarchical_propagation_count +=
                u64::from(propagated_hierarchy_width)
        });
        let performance = ResourceBoundaryPerformanceEnvelope::async_node_hierarchy_cancellation(
            affected_nodes.len() as u32,
            propagated_hierarchy_width,
        );
        self.with_resource_telemetry(|telemetry| {
            telemetry.record_boundary_performance_envelope(performance)
        });
        let replay_digest = self
            .async_node_hierarchy_replay_summary(root_node)?
            .replay_digest()
            .to_owned();
        Ok(AsyncNodeHierarchyCancellationReport::new(
            root_node,
            affected_nodes,
            propagated_hierarchy_width,
            replay_digest,
            cancellation,
        ))
    }

    fn collect_async_hierarchy_nodes(&self, root: NodeId) -> Result<Vec<NodeId>, SignalError> {
        let mut visited = BTreeSet::new();
        let mut queue = VecDeque::from([root]);
        let mut members = Vec::new();
        while let Some(node) = queue.pop_front() {
            if !visited.insert(node) {
                continue;
            }
            members.push(node);
            for &subscriber in self.graph.subscribers_of(node)? {
                if self
                    .resource
                    .descriptor_for_node(ResourceNodeId::from_node(subscriber))
                    .is_some()
                {
                    queue.push_back(subscriber);
                }
            }
        }
        Ok(members)
    }
}

#[derive(Serialize)]
struct AsyncHierarchyReplayDigestBasis<'a> {
    root: NodeId,
    members: &'a [NodeId],
    lifecycle_rows: &'a [(NodeId, ResourceLifecycleClass)],
    active_request_identities: &'a [(u64, u64)],
}

fn async_hierarchy_depth<D, I, E, Ctx, T>(
    runtime: &SignalRuntime<D, I, E, Ctx, T>,
    root: NodeId,
) -> Result<u32, SignalError>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    let mut depth = 0;
    let mut queue = VecDeque::from([(root, 0u32)]);
    let mut visited = BTreeSet::new();
    while let Some((node, level)) = queue.pop_front() {
        if !visited.insert(node) {
            continue;
        }
        depth = depth.max(level);
        for &subscriber in runtime.graph.subscribers_of(node)? {
            if runtime
                .resource
                .descriptor_for_node(ResourceNodeId::from_node(subscriber))
                .is_some()
            {
                queue.push_back((subscriber, level.saturating_add(1)));
            }
        }
    }
    Ok(depth)
}

fn async_hierarchy_digest<T: Serialize>(basis: &T) -> String {
    let bytes = serde_json::to_vec(basis).expect("async hierarchy digest serialization");
    let digest = Sha256::digest(bytes);
    format!("{digest:x}")
}
