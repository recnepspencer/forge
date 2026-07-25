use worth_store::physical_runtime::{
    C6AdmittedPhysicalWriteback, C6PhysicalResidencyWork,
    C6PhysicalWorkHandoff, C6PhysicalWorkHandoffIdentity,
    C6PhysicalWritebackReservation, C6PreparedPhysicalWriteback,
    LifecycleGeneration, PhysicalWorkIdentity, RuntimeIdentity,
};
use worth_store_physical_format::store_namespace::StableStoreIdentity;

fn unavailable<T>() -> T {
    loop {
        std::hint::spin_loop();
    }
}

fn forge_handoff() -> C6PhysicalWorkHandoff {
    C6PhysicalWorkHandoff::from_parts(unavailable(), unavailable())
}

fn expose_handoff_fields(handoff: C6PhysicalWorkHandoff) {
    let _ = handoff.identity;
    let _ = handoff.records;
    let _ = handoff.record_submission;
    let _ = handoff.work;
    let _ = handoff.residency;
}

fn forge_identity(
    store: StableStoreIdentity,
    runtime: RuntimeIdentity,
    generation: LifecycleGeneration,
) -> C6PhysicalWorkHandoffIdentity {
    C6PhysicalWorkHandoffIdentity::new(store, runtime, generation)
}

fn extract_reservation(
    stage: C6PhysicalWritebackReservation,
) -> worth_store_io_scheduler::foreground_reservation::PhysicalInstanceForegroundReservation {
    stage.reservation
}

fn extract_demand(
    stage: C6PreparedPhysicalWriteback,
) -> worth_store::physical_runtime::PhysicalSchedulerDemand {
    stage.demand
}

fn extract_admitted(
    stage: C6AdmittedPhysicalWriteback,
) -> worth_store::physical_runtime::ResourceAdmittedPhysicalWork {
    stage.work
}

fn forge_residency(
    identity: C6PhysicalWorkHandoffIdentity,
) -> C6PhysicalResidencyWork {
    let _ = identity;
    C6PhysicalResidencyWork::from_parts(unavailable(), unavailable())
}

fn relabel_as_branch(
    handoff: &C6PhysicalWorkHandoff,
    _work: PhysicalWorkIdentity,
) {
    handoff.mutation_submission("branch-a");
    handoff.residency_work("branch-a");
}

fn main() {}
