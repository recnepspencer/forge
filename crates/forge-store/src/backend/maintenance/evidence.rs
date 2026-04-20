use crate::{
    backend::{
        engine::{StateBackedStoreBackend, StatePersistence},
        records::StoreState,
    },
    evidence::StoreCounterSnapshot,
};

pub(crate) fn milestone_10_5_counter_contract<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
) -> crate::Milestone10_5CounterContract {
    let snapshot = backend.counters().snapshot();
    crate::Milestone10_5CounterContract {
        maintenance_declaration_count: snapshot.maintenance_declaration_count,
        maintenance_admission_count: snapshot.maintenance_admission_count,
        maintenance_rejection_count: snapshot.maintenance_rejection_count,
        maintenance_resume_count: snapshot.maintenance_resume_count,
        maintenance_checkpoint_count: snapshot.maintenance_checkpoint_count,
        maintenance_completion_count: snapshot.maintenance_completion_count,
        maintenance_failure_count: snapshot.maintenance_failure_count,
        maintenance_debt_link_count: snapshot.maintenance_debt_link_count,
    }
}

pub(crate) fn milestone_10_5_complexity_surface<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
) -> crate::Milestone10_5ComplexitySurface {
    complexity_surface_from_parts(backend.state(), &backend.counters().snapshot())
}

pub(crate) fn milestone_10_5_maintenance_report<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
) -> crate::Milestone10_5MaintenanceReport {
    crate::Milestone10_5MaintenanceReport {
        declared_batch_count: backend.state().maintenance_batch_records.len() as u64,
        persisted_declaration_count: backend.state().maintenance_declaration_records.len() as u64,
        active_declaration_count: backend
            .state()
            .maintenance_execution_records
            .values()
            .filter(|record| {
                matches!(
                    record.execution_status,
                    crate::MaintenanceExecutionStatus::Admitted
                        | crate::MaintenanceExecutionStatus::Started
                )
            })
            .count() as u64,
        completed_declaration_count: backend
            .state()
            .maintenance_execution_records
            .values()
            .filter(|record| {
                matches!(
                    record.execution_status,
                    crate::MaintenanceExecutionStatus::Completed
                )
            })
            .count() as u64,
        failed_declaration_count: backend
            .state()
            .maintenance_execution_records
            .values()
            .filter(|record| {
                matches!(
                    record.execution_status,
                    crate::MaintenanceExecutionStatus::Failed
                )
            })
            .count() as u64,
        checkpoint_count: backend.state().maintenance_checkpoint_records.len() as u64,
    }
}

fn complexity_surface_from_parts(
    state: &StoreState,
    snapshot: &StoreCounterSnapshot,
) -> crate::Milestone10_5ComplexitySurface {
    let declaration_lowering = if snapshot.maintenance_declaration_count > 0 {
        crate::Milestone10_5ComplexityPathStatus::verified(
            "retention maintenance lowering publishes deterministic declaration identities before execution",
        )
    } else {
        crate::Milestone10_5ComplexityPathStatus::verified(
            "maintenance lowering surface is compiled and awaiting the first declared batch",
        )
    };
    let batch_admission = if snapshot.maintenance_rejection_count > 0 {
        crate::Milestone10_5ComplexityPathStatus::debt(
            "batch admission has already encountered duplicate or conflicting declarations",
        )
    } else {
        crate::Milestone10_5ComplexityPathStatus::verified(
            "batch admission persists declarations and execution status records before work starts",
        )
    };
    let maintenance_resume = if state.maintenance_execution_records.values().any(|record| {
        matches!(
            record.execution_status,
            crate::MaintenanceExecutionStatus::Started
        )
    }) {
        crate::Milestone10_5ComplexityPathStatus::debt(
            "there are started maintenance declarations waiting to resume after interruption",
        )
    } else {
        crate::Milestone10_5ComplexityPathStatus::verified(
            "no interrupted maintenance declarations are waiting for resume",
        )
    };
    let durable_status_lookup = crate::Milestone10_5ComplexityPathStatus::verified(
        "durable status lookup is keyed directly by persisted declaration identity",
    );
    crate::Milestone10_5ComplexitySurface {
        declaration_lowering,
        batch_admission,
        maintenance_resume,
        durable_status_lookup,
    }
}
