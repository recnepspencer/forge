use crate::data::dependency::{
    CommittedSnapshotUpdate, DependencyEdge, DependencyInputScan, DependencySnapshot,
    ReplacementSnapshotUpdate, SnapshotDeltaRecord, SnapshotShapeHandle, StableShapeSnapshotBasis,
    VersionOnlySnapshotUpdate, VersionVector,
};
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::logic::evaluation::{DependencyInputContext, EffectDependencyInputs};

use super::telemetry;

pub(super) fn resolve_effect_dependency_inputs(
    graph: &mut SignalGraph,
    node: NodeId,
    dependency_inputs: Option<EffectDependencyInputs>,
) -> Result<EffectDependencyInputs, SignalError> {
    match dependency_inputs {
        Some(inputs) if dependency_inputs_match_graph(graph, node, &inputs)? => {
            telemetry::record_dependency_input_reuse(graph);
            Ok(inputs)
        }
        Some(_) => {
            telemetry::record_dependency_input_rebuild(graph);
            build_effect_dependency_inputs(graph, node)
        }
        None => build_effect_dependency_inputs(graph, node),
    }
}

fn dependency_inputs_match_graph(
    graph: &SignalGraph,
    node: NodeId,
    dependency_inputs: &EffectDependencyInputs,
) -> Result<bool, SignalError> {
    let (dependency_set_id, dependency_snapshot_id) = graph.node_dependency_ids(node)?;
    Ok(
        dependency_inputs.context.dependency_set_id == dependency_set_id
            && dependency_inputs.context.dependency_snapshot_id == dependency_snapshot_id,
    )
}

fn build_effect_dependency_inputs(
    graph: &mut SignalGraph,
    node: NodeId,
) -> Result<EffectDependencyInputs, SignalError> {
    let (dependency_set_id, dependency_snapshot_id) = graph.node_dependency_ids(node)?;
    let context = DependencyInputContext {
        dependency_set_id,
        dependency_snapshot_id,
    };
    graph.refresh_runtime_dependencies_of(node)?;
    let dependencies = graph.current_runtime_dependencies_of(node)?.to_vec();
    build_effect_dependency_inputs_for_dependencies(graph, node, context, dependencies.as_slice())
}

pub(crate) fn collect_effect_dependency_inputs_iter<I>(
    graph: &mut SignalGraph,
    nodes: I,
) -> Result<Vec<EffectDependencyInputs>, SignalError>
where
    I: IntoIterator<Item = NodeId>,
{
    nodes
        .into_iter()
        .map(|node| build_effect_dependency_inputs(graph, node))
        .collect()
}

pub(crate) fn build_effect_dependency_inputs_for_dependencies(
    graph: &mut SignalGraph,
    node: NodeId,
    context: DependencyInputContext,
    dependencies: &[DependencyEdge],
) -> Result<EffectDependencyInputs, SignalError> {
    let shape_handle_lookup_start = crate::clock::RuntimeInstant::now();
    let previous_shape_handle =
        graph.dependency_snapshot_shape_handle(context.dependency_snapshot_id);
    let shape_handle_lookup_nanos = shape_handle_lookup_start.elapsed().as_nanos();
    let previous_snapshot_fetch_start = crate::clock::RuntimeInstant::now();
    let previous_snapshot = graph.get_dep_snapshot(node)?.clone();
    let previous_snapshot_fetch_nanos = previous_snapshot_fetch_start.elapsed().as_nanos();
    let shape_scan = scan_dependency_shape(graph, dependencies, previous_snapshot.entries())?;
    let (stable_shape_proved, inputs) = {
        if shape_scan.shape_stable
            && shape_scan.matched_entry_count == previous_snapshot.entries().len()
        {
            let inputs = build_stable_shape_dependency_inputs(
                graph,
                node,
                context,
                &previous_snapshot,
                previous_shape_handle,
                shape_scan.matched_entry_count,
                shape_scan.stable_shape_versions,
                shape_scan.changes,
                shape_handle_lookup_nanos,
                previous_snapshot_fetch_nanos,
                shape_scan.version_scan_nanos,
            )?;
            (true, inputs)
        } else {
            // `runtime_dependencies_of(node)` must preserve canonical dependency order
            // by `DependencyEdge::sort_key()`. Snapshot reuse and delta detection rely
            // on stable ordering between the current dependency view and the prior
            // snapshot entries.
            let inputs = build_replacement_dependency_inputs(
                graph,
                node,
                context,
                &previous_snapshot,
                dependencies,
                shape_handle_lookup_nanos,
                previous_snapshot_fetch_nanos,
                shape_scan.version_scan_nanos,
            )?;
            (false, inputs)
        }
    };
    telemetry::record_storage_shape_proof(graph, stable_shape_proved);
    Ok(inputs)
}

struct DependencyShapeScan {
    shape_stable: bool,
    matched_entry_count: usize,
    changes: u32,
    stable_shape_versions: Vec<u64>,
    version_scan_nanos: u128,
}

fn scan_dependency_shape(
    graph: &mut SignalGraph,
    dependencies: &[DependencyEdge],
    previous_entries: &[crate::data::dependency::DependencySnapshotEntry],
) -> Result<DependencyShapeScan, SignalError> {
    let mut matched_entry_count = 0usize;
    let mut shape_stable = dependencies.len() == previous_entries.len();
    let mut changes = 0_u32;
    let mut stable_shape_versions = Vec::with_capacity(dependencies.len());
    let version_scan_start = crate::clock::RuntimeInstant::now();
    for dep in dependencies {
        let source = dep.source();
        let aspect = dep.aspect();
        let Some(previous_entry) = previous_entries.get(matched_entry_count) else {
            shape_stable = false;
            break;
        };
        if !graph.is_alive(source) {
            shape_stable = false;
            break;
        }

        let version = graph.node_version_for_scope(source, aspect, dep.scope_ref())?;
        stable_shape_versions.push(version);
        if previous_entry.sort_key() != dep.sort_key() {
            shape_stable = false;
            break;
        }
        if previous_entry.cached_version != version {
            changes += 1;
        }
        matched_entry_count += 1;
    }
    Ok(DependencyShapeScan {
        shape_stable,
        matched_entry_count,
        changes,
        stable_shape_versions,
        version_scan_nanos: version_scan_start.elapsed().as_nanos(),
    })
}

fn build_stable_shape_dependency_inputs(
    graph: &mut SignalGraph,
    node: NodeId,
    context: DependencyInputContext,
    previous_snapshot: &DependencySnapshot,
    previous_shape_handle: SnapshotShapeHandle,
    previous_entry_count: usize,
    stable_shape_versions: Vec<u64>,
    changes: u32,
    shape_handle_lookup_nanos: u128,
    previous_snapshot_fetch_nanos: u128,
    version_scan_nanos: u128,
) -> Result<EffectDependencyInputs, SignalError> {
    let stable_proof_start = crate::clock::RuntimeInstant::now();
    let scan = DependencyInputScan::stable_shape(
        node,
        context.dependency_snapshot_id,
        previous_entry_count,
        stable_shape_versions.len(),
        stable_shape_versions,
    );
    let basis = StableShapeSnapshotBasis::prove(&scan, previous_shape_handle).ok_or_else(|| {
        SignalError::internal("stable-shape dependency scan failed to produce a proof")
    })?;
    let versions = VersionVector::from_scan(&basis, &scan);
    let stable_proof_nanos = stable_proof_start.elapsed().as_nanos();
    let version_delta_start = crate::clock::RuntimeInstant::now();
    let snapshot_delta = SnapshotDeltaRecord::for_version_update(
        node,
        previous_snapshot,
        scan.stable_shape_versions(),
    );
    let dependency_snapshot_update = CommittedSnapshotUpdate::VersionOnly(
        VersionOnlySnapshotUpdate::from_basis_and_versions(basis, versions),
    );
    let version_delta_nanos = version_delta_start.elapsed().as_nanos();
    telemetry::record_stable_shape_timing(
        graph,
        shape_handle_lookup_nanos,
        previous_snapshot_fetch_nanos,
        version_scan_nanos,
        stable_proof_nanos,
        version_delta_nanos,
    );
    Ok(EffectDependencyInputs {
        context,
        snapshot_delta,
        dependency_snapshot_update,
        meaningful_input_changes: changes,
    })
}

fn build_replacement_dependency_inputs(
    graph: &mut SignalGraph,
    node: NodeId,
    context: DependencyInputContext,
    previous_snapshot: &DependencySnapshot,
    dependencies: &[DependencyEdge],
    shape_handle_lookup_nanos: u128,
    previous_snapshot_fetch_nanos: u128,
    version_scan_nanos: u128,
) -> Result<EffectDependencyInputs, SignalError> {
    let replacement_build_start = crate::clock::RuntimeInstant::now();
    let (snapshot, changes) =
        build_replacement_dependency_snapshot(graph, dependencies, previous_snapshot)?;

    let replacement_snapshot =
        crate::data::dependency::SharedDependencySnapshot::new(snapshot.clone());
    let snapshot_delta =
        SnapshotDeltaRecord::between(node, previous_snapshot, &replacement_snapshot);
    let dependency_snapshot_update = CommittedSnapshotUpdate::Replace(
        ReplacementSnapshotUpdate::from_snapshot(snapshot, graph.dependency_snapshot_shapes_mut()),
    );
    let replacement_build_nanos = replacement_build_start.elapsed().as_nanos();
    telemetry::record_replacement_timing(
        graph,
        shape_handle_lookup_nanos,
        previous_snapshot_fetch_nanos,
        version_scan_nanos,
        replacement_build_nanos,
    );
    Ok(EffectDependencyInputs {
        context,
        snapshot_delta,
        dependency_snapshot_update,
        meaningful_input_changes: changes,
    })
}

fn build_replacement_dependency_snapshot(
    graph: &mut SignalGraph,
    dependencies: &[DependencyEdge],
    previous_snapshot: &DependencySnapshot,
) -> Result<(DependencySnapshot, u32), SignalError> {
    let mut snapshot = DependencySnapshot::empty();
    let snapshot_entries = previous_snapshot.entries();
    let mut snapshot_index = 0usize;
    let mut changes = 0_u32;
    for dep in dependencies {
        let source = dep.source();
        let aspect = dep.aspect();
        if graph.is_alive(source) {
            let ver = graph.node_version_for_scope(source, aspect, dep.scope_ref())?;
            snapshot.record(source, aspect, ver, dep.scope_ref().cloned());
            while snapshot_index < snapshot_entries.len()
                && snapshot_entries[snapshot_index].sort_key() < dep.sort_key()
            {
                snapshot_index += 1;
            }
            if snapshot_index < snapshot_entries.len()
                && snapshot_entries[snapshot_index].sort_key() == dep.sort_key()
            {
                if snapshot_entries[snapshot_index].cached_version != ver {
                    changes += 1;
                }
                snapshot_index += 1;
            }
        } else {
            changes += 1;
        }
    }
    Ok((snapshot, changes))
}
