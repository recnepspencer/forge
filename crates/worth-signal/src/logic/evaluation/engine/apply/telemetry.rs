use crate::data::graph::SignalGraph;

pub(super) fn record_dependency_input_reuse(graph: &mut SignalGraph) {
    graph.telemetry_mut().execution.dependency_input_reuse_count += 1;
}

pub(super) fn record_dependency_input_rebuild(graph: &mut SignalGraph) {
    graph
        .telemetry_mut()
        .execution
        .dependency_input_rebuild_count += 1;
}

pub(super) fn record_stable_shape_timing(
    graph: &mut SignalGraph,
    shape_handle_lookup_nanos: u128,
    previous_snapshot_fetch_nanos: u128,
    version_scan_nanos: u128,
    stable_proof_nanos: u128,
    version_delta_nanos: u128,
) {
    let execution = &mut graph.telemetry_mut().execution;
    execution.dependency_input_shape_handle_lookup_nanos += shape_handle_lookup_nanos;
    execution.dependency_input_previous_snapshot_fetch_nanos += previous_snapshot_fetch_nanos;
    execution.dependency_input_version_scan_nanos += version_scan_nanos;
    execution.dependency_input_stable_proof_nanos += stable_proof_nanos;
    execution.dependency_input_version_delta_nanos += version_delta_nanos;
    execution.dependency_input_stable_shape_count += 1;
}

pub(super) fn record_replacement_timing(
    graph: &mut SignalGraph,
    shape_handle_lookup_nanos: u128,
    previous_snapshot_fetch_nanos: u128,
    version_scan_nanos: u128,
    replacement_build_nanos: u128,
) {
    let execution = &mut graph.telemetry_mut().execution;
    execution.dependency_input_shape_handle_lookup_nanos += shape_handle_lookup_nanos;
    execution.dependency_input_previous_snapshot_fetch_nanos += previous_snapshot_fetch_nanos;
    execution.dependency_input_version_scan_nanos += version_scan_nanos;
    execution.dependency_input_replacement_build_nanos += replacement_build_nanos;
    execution.dependency_input_replacement_count += 1;
}

pub(super) fn record_storage_shape_proof(graph: &mut SignalGraph, stable_shape_proved: bool) {
    if stable_shape_proved {
        graph
            .telemetry_mut()
            .storage
            .stable_shape_snapshot_proof_count += 1;
    } else {
        graph
            .telemetry_mut()
            .storage
            .stable_shape_snapshot_proof_failure_count += 1;
    }
}
