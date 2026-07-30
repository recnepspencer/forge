use super::decision::{
    contender_label, ready_label, release_label, worker_label, EquivalentContenderIdentity,
    GateReleaseOrder, IndependentReadyWorkSelection, WorkerStartOrder,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::bounded_residency) struct ExecutedPrefetchSchedule {
    worker_start_order: WorkerStartOrder,
    ready_work_selection: IndependentReadyWorkSelection,
}

impl ExecutedPrefetchSchedule {
    pub(in crate::bounded_residency) const fn new(
        worker_start_order: WorkerStartOrder,
        ready_work_selection: IndependentReadyWorkSelection,
    ) -> Self {
        Self {
            worker_start_order,
            ready_work_selection,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::bounded_residency) struct ExecutedDuplicateFaultSchedule {
    contender_identity: EquivalentContenderIdentity,
    gate_release_order: GateReleaseOrder,
}

impl ExecutedDuplicateFaultSchedule {
    pub(in crate::bounded_residency) const fn new(
        contender_identity: EquivalentContenderIdentity,
        gate_release_order: GateReleaseOrder,
    ) -> Self {
        Self {
            contender_identity,
            gate_release_order,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct BoundedResidencyExecutedSchedule {
    prefetch: ExecutedPrefetchSchedule,
    duplicate_fault: ExecutedDuplicateFaultSchedule,
}

impl BoundedResidencyExecutedSchedule {
    pub(in crate::bounded_residency) const fn from_proofs(
        prefetch: ExecutedPrefetchSchedule,
        duplicate_fault: ExecutedDuplicateFaultSchedule,
    ) -> Self {
        Self {
            prefetch,
            duplicate_fault,
        }
    }

    pub(in crate::bounded_residency) fn encoded(self) -> String {
        [
            worker_label(self.prefetch.worker_start_order),
            contender_label(self.duplicate_fault.contender_identity),
            release_label(self.duplicate_fault.gate_release_order),
            ready_label(self.prefetch.ready_work_selection),
        ]
        .join(";")
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BoundedResidencyExecutedSchedule, EquivalentContenderIdentity,
        ExecutedDuplicateFaultSchedule, ExecutedPrefetchSchedule, GateReleaseOrder,
        IndependentReadyWorkSelection, WorkerStartOrder,
    };

    #[test]
    fn executed_schedule_is_assembled_only_from_minted_execution_receipts() {
        let executed = BoundedResidencyExecutedSchedule::from_proofs(
            ExecutedPrefetchSchedule::new(
                WorkerStartOrder::SecondThenFirst,
                IndependentReadyWorkSelection::SecondWorkerThenFirst,
            ),
            ExecutedDuplicateFaultSchedule::new(
                EquivalentContenderIdentity::SecondOwner,
                GateReleaseOrder::WaiterThenOwner,
            ),
        );
        let expected = "worker-start-order=second-then-first;\
             equivalent-contender-identity=second-owner;\
             gate-release-order=waiter-then-owner;\
             independent-ready-work-selection=second-worker-then-first";
        if executed.encoded() != expected {
            panic!("MUTANT_PREDICATE:executed-schedule-receipts-ignored");
        }
    }
}
