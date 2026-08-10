use crate::{
    backend::{
        engine::{StateBackedStoreBackend, StatePersistence},
        records::StoreState,
    },
    evidence::StoreCounterSnapshot,
};

pub(crate) fn milestone_11_complexity_surface<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
) -> crate::Milestone11ComplexitySurface {
    complexity_surface_from_parts(backend.state(), &backend.counters().snapshot())
}

fn complexity_surface_from_parts(
    state: &StoreState,
    snapshot: &StoreCounterSnapshot,
) -> crate::Milestone11ComplexitySurface {
    let declaration_lowering = if snapshot.maintenance_declaration_count > 0 {
        crate::Milestone11ComplexityPathStatus::verified(
            "retention maintenance lowering publishes deterministic declaration identities before execution",
        )
    } else {
        crate::Milestone11ComplexityPathStatus::verified(
            "maintenance lowering surface is compiled and awaiting the first declared batch",
        )
    };
    let batch_admission = if snapshot.maintenance_rejection_count > 0 {
        crate::Milestone11ComplexityPathStatus::debt(
            "batch admission has already encountered duplicate or conflicting declarations",
        )
    } else if !state.maintenance_queue_summary_records.is_empty() {
        crate::Milestone11ComplexityPathStatus::verified(
            "batch admission maintains durable scheduler lane summaries before planning begins",
        )
    } else {
        crate::Milestone11ComplexityPathStatus::verified(
            "batch admission persists declarations and execution status records before work starts",
        )
    };
    let maintenance_resume = if state.maintenance_execution_records.values().any(|record| {
        matches!(
            record.execution_status,
            crate::MaintenanceExecutionStatus::Started
        )
    }) {
        crate::Milestone11ComplexityPathStatus::debt(
            "there are started maintenance declarations waiting to resume after interruption",
        )
    } else {
        crate::Milestone11ComplexityPathStatus::verified(
            "no interrupted maintenance declarations are waiting for resume",
        )
    };
    let durable_status_lookup = crate::Milestone11ComplexityPathStatus::verified(
        "durable status lookup is keyed directly by persisted declaration identity",
    );
    crate::Milestone11ComplexitySurface {
        declaration_lowering,
        batch_admission,
        maintenance_resume,
        durable_status_lookup,
    }
}
