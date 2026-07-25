use worth_store::physical_runtime::ServingPhysicalRuntime;

fn independently_borrow_c6_capabilities(serving: &ServingPhysicalRuntime) {
    let handoff = serving.c6_physical_work_handoff();

    let reader = handoff.record_reads();
    let record_submission = handoff.record_submissions();
    let physical_reads = handoff.read_submission();
    let physical_mutations = handoff.mutation_submission();
    let observation = handoff.observation();
    let residency = handoff.residency_work();

    let _ = (
        handoff.identity(),
        reader.store_identity(),
        record_submission,
        physical_reads,
        physical_mutations,
        observation,
        residency.counters(),
        handoff.recovery_obligations(),
        handoff.recovery_evidence_damaged(),
    );
}

fn main() {}
