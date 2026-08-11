use crate::facade::{
    Aspect, NodeId, NodeState, SignalBranchHandle, SignalGraph, SignalRuntimePolicy,
};

use super::workflow_truth::ReferenceModel;

pub(super) struct InvariantReport {
    pub(super) step_index: usize,
    pub(super) errors: Vec<String>,
}

pub(super) fn assert_runtime_invariants(
    graph: &SignalGraph,
    model: &ReferenceModel,
    current_branch: SignalBranchHandle,
    node_a: NodeId,
    aspect_a: Aspect,
    node_b: NodeId,
    aspect_b: Aspect,
    policy: SignalRuntimePolicy,
) -> InvariantReport {
    let mut errors = Vec::new();
    for node in graph.live_node_ids() {
        let entry = graph.get_entry(node).unwrap();
        if !matches!(entry.get_state(), NodeState::Clean) && entry.get_dirty_aspects().is_empty() {
            errors.push(format!("dirty node {node} has an empty dirty-aspect mask"));
        }
        for dependency in graph.dependencies_of(node).unwrap() {
            if !graph.is_alive(dependency.source()) {
                errors.push(format!(
                    "node {node} depends on stale upstream {}",
                    dependency.source()
                ));
                continue;
            }
            if !graph
                .subscribers_of(dependency.source())
                .unwrap()
                .contains(&node)
            {
                errors.push(format!(
                    "subscriber index missing back-edge {} -> {node}",
                    dependency.source()
                ));
            }
        }
        for subscriber in graph.subscribers_of(node).unwrap() {
            if !graph.is_alive(*subscriber) {
                errors.push(format!(
                    "node {node} points at stale subscriber {subscriber}"
                ));
                continue;
            }
            let has_backref = graph
                .dependencies_of(*subscriber)
                .unwrap()
                .iter()
                .any(|dependency| dependency.source() == node);
            if !has_backref {
                errors.push(format!(
                    "dependency index missing back-edge {node} -> {subscriber}"
                ));
            }
        }
    }

    let expected = model.branch(current_branch.id);
    let actual_a = graph
        .get_entry(node_a)
        .unwrap()
        .get_aspect_version()
        .get(aspect_a);
    let actual_b = graph
        .get_entry(node_b)
        .unwrap()
        .get_aspect_version()
        .get(aspect_b);
    if actual_a != expected.a {
        errors.push(format!(
            "branch `{}` expected aspect A version {}, got {}",
            current_branch.name, expected.a, actual_a
        ));
    }
    if actual_b != expected.b {
        errors.push(format!(
            "branch `{}` expected aspect B version {}, got {}",
            current_branch.name, expected.b, actual_b
        ));
    }
    if expected.head_snapshot.is_some()
        && graph
            .observe()
            .branch_head_snapshot_id(current_branch.id)
            .is_none()
    {
        errors.push(format!(
            "branch `{}` lost its head snapshot metadata",
            current_branch.name,
        ));
    }
    if graph.observe().recent_execution_history_diagnostics().len()
        > policy.retention_budget.history_limit
    {
        errors.push(format!(
            "history retention exceeded policy: {} > {}",
            graph.observe().recent_execution_history_diagnostics().len(),
            policy.retention_budget.history_limit
        ));
    }

    InvariantReport {
        step_index: 0,
        errors,
    }
}
