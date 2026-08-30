use super::*;

pub(super) fn commit_measurement(
    runtime: &RelationalRuntime,
    run: impl FnOnce(&RelationalRuntime) -> CommitResult,
) -> PerfMeasurement {
    runtime.performance_access().reset_counters();
    let started_at = Instant::now();
    let outcome = run(runtime);
    let counters = runtime.performance_access().counters();
    let phase_timing = outcome.execution().phase_timing.clone();

    measurement_from(started_at, || {
        perf_metrics!({
            "changed_records": outcome.changed_records.len(),
            "commit_topology": format!("{:?}", outcome.structural_summary().commit_topology),
            "touched_partitions": outcome.structural_summary().touched_partitions.len(),
            "packet_count": outcome.complexity_delta().preparation_packet_count,
            "query_packet_count": outcome.complexity_delta().query_packet_count,
            "snapshot_pin_full_rebuilds": outcome.complexity_delta().snapshot_pin_full_rebuilds,
            "phase_timing": {
                "working_state_preparation_micros": phase_timing.working_state_preparation_micros,
                "invariant_pre_check_micros": phase_timing.invariant_pre_check_micros,
                "authoritative_mutation_micros": phase_timing.authoritative_mutation_micros,
                "history_resolution_micros": phase_timing.history_resolution_micros,
                "invariant_post_check_micros": phase_timing.invariant_post_check_micros,
                "artifact_assembly_micros": phase_timing.artifact_assembly_micros,
                "durable_append_micros": phase_timing.durable_append_micros,
                "publication_micros": phase_timing.publication_micros
            },
            "counters": counters,
        })
    })
}
