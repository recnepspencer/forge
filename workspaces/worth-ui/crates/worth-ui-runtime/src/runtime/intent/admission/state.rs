use super::{
    UiAdmittedIntent, UiCurrentIntentAdmissionCandidate, UiIntentAdmissionCancellationReason,
    UiIntentAdmissionCost, UiIntentAdmissionDecision, UiIntentAdmissionMetrics,
    UiIntentAdmissionSettlementPosture, UiIntentAdmissionSettlementReceipt,
    UiIntentAdmissionShutdownReport, UiIntentAdmissionStop, UiIntentAdmissionStopReason,
};

pub(crate) struct UiIntentAdmissionState {
    lineage: super::super::attempt_lineage::UiIntentAttemptLineageState,
    counters: UiIntentAdmissionCounters,
}

#[derive(Default)]
struct UiIntentAdmissionCounters {
    admitted: u64,
    released: u64,
    lifecycle_cancelled: u64,
    stopped: u64,
}

impl UiIntentAdmissionState {
    pub(crate) fn new() -> Self {
        Self {
            lineage: super::super::attempt_lineage::UiIntentAttemptLineageState::new(),
            counters: Default::default(),
        }
    }

    pub(crate) fn issue_lineage(&mut self) -> Option<super::super::UiIntentAttemptLineage> {
        self.lineage.issue()
    }

    pub(crate) fn reject<I: crate::capability::UiIntent>(
        &mut self,
        reason: UiIntentAdmissionStopReason,
        cost: UiIntentAdmissionCost,
    ) -> UiIntentAdmissionDecision<I> {
        self.stopped(reason, cost)
    }

    pub(crate) fn admit<I: crate::capability::UiIntent>(
        &mut self,
        candidate: UiCurrentIntentAdmissionCandidate,
        execution: &mut crate::runtime::intent_execution::UiIntentExecutionState,
    ) -> UiIntentAdmissionDecision<I> {
        let base_cost = admission_cost(&candidate);
        let Some(lineage) = candidate.lineage().or_else(|| self.lineage.issue()) else {
            return self.stopped(
                UiIntentAdmissionStopReason::AttemptLineageExhausted,
                base_cost,
            );
        };
        let commit = match execution.reserve_admission(candidate, lineage) {
            Ok(commit) => commit,
            Err(failure) => {
                let (reason, slots_inspected, occupancy_slots_inspected) = failure.into_parts();
                let cost = base_cost
                    .with_slots_inspected(slots_inspected)
                    .with_occupancy_slots_inspected(occupancy_slots_inspected);
                return self.stopped(reservation_stop_reason(reason), cost);
            }
        };
        let (identity, lease, slots_inspected, occupancy_slots_inspected) = commit.into_parts();
        let cost = base_cost
            .with_slots_inspected(slots_inspected)
            .with_occupancy_slots_inspected(occupancy_slots_inspected);
        self.counters.admitted = next(self.counters.admitted);
        UiIntentAdmissionDecision::Admitted(UiAdmittedIntent::new(identity, cost, lease))
    }

    pub(crate) fn release<I: crate::capability::UiIntent>(
        &mut self,
        admitted: UiAdmittedIntent<I>,
        execution: &mut crate::runtime::intent_execution::UiIntentExecutionState,
    ) -> UiIntentAdmissionSettlementReceipt {
        let receipt = execution.release_admission(admitted);
        if receipt.posture() == UiIntentAdmissionSettlementPosture::Released {
            self.counters.released = next(self.counters.released);
        }
        receipt
    }

    pub(crate) fn cancel_instance(
        &mut self,
        execution: &mut crate::runtime::intent_execution::UiIntentExecutionState,
        instance: worth_ui_host_contract::UiMountedInstanceIdentity,
    ) -> usize {
        self.record_lifecycle_cancellation(execution.cancel_instance(instance))
    }

    pub(crate) fn cancel_binding(
        &mut self,
        execution: &mut crate::runtime::intent_execution::UiIntentExecutionState,
        binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
    ) -> usize {
        self.record_lifecycle_cancellation(execution.cancel_binding(binding))
    }

    pub(crate) fn cancel_all(
        &mut self,
        execution: &mut crate::runtime::intent_execution::UiIntentExecutionState,
        reason: UiIntentAdmissionCancellationReason,
    ) -> usize {
        let cancelled = execution.cancel_all(reason);
        self.record_lifecycle_cancellation(cancelled)
    }

    pub(crate) fn shutdown(
        &mut self,
        execution: &mut crate::runtime::intent_execution::UiIntentExecutionState,
    ) -> (
        UiIntentAdmissionShutdownReport,
        crate::runtime::intent_execution::UiIntentExecutionShutdownReport,
    ) {
        let execution_report = execution.shutdown();
        self.record_lifecycle_cancellation(execution_report.reservation_backed_entries_disposed());
        (
            UiIntentAdmissionShutdownReport::new(
                execution_report.reservation_backed_entries_disposed(),
                self.metrics(execution),
            ),
            execution_report,
        )
    }

    pub(crate) fn metrics(
        &self,
        execution: &crate::runtime::intent_execution::UiIntentExecutionState,
    ) -> UiIntentAdmissionMetrics {
        let census: crate::runtime::intent_execution::UiIntentExecutionAdmissionCensus =
            execution.census();
        UiIntentAdmissionMetrics::new(super::metrics::UiIntentAdmissionMetricInput {
            active_attempts: census.active_attempts,
            active_occupancy: census.active_occupancy,
            retained_candidates: census.retained_candidates,
            retained_payloads: census.retained_payloads,
            retained_owner_references: census.retained_owner_references,
            admitted: self.counters.admitted,
            released: self.counters.released,
            lifecycle_cancelled: self.counters.lifecycle_cancelled,
            stopped: self.counters.stopped,
        })
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn reserve_occupancy_for_certification(
        &mut self,
        execution: &mut crate::runtime::intent_execution::UiIntentExecutionState,
        proof: super::super::operability::UiIntentOperabilityProof,
    ) -> Result<
        super::super::operability::UiIntentOccupancyReservation,
        super::super::operability::UiIntentOccupancyReservationDenial,
    > {
        execution.reserve_occupancy_for_certification(proof)
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn release_occupancy_for_certification(
        &mut self,
        execution: &mut crate::runtime::intent_execution::UiIntentExecutionState,
        reservation: super::super::operability::UiIntentOccupancyReservation,
    ) -> super::super::operability::UiIntentOccupancyReleasePosture {
        execution.release_occupancy_for_certification(reservation)
    }

    fn stopped<I: crate::capability::UiIntent>(
        &mut self,
        reason: UiIntentAdmissionStopReason,
        cost: UiIntentAdmissionCost,
    ) -> UiIntentAdmissionDecision<I> {
        self.counters.stopped = next(self.counters.stopped);
        UiIntentAdmissionDecision::Stopped(UiIntentAdmissionStop::new(reason, cost))
    }

    fn record_lifecycle_cancellation(&mut self, cancelled: usize) -> usize {
        self.counters.lifecycle_cancelled = self
            .counters
            .lifecycle_cancelled
            .checked_add(cancelled as u64)
            .expect("bounded admission cancellation accounting exhausted");
        cancelled
    }
}

fn reservation_stop_reason(
    reason: crate::runtime::intent_execution::UiIntentExecutionAdmissionReservationFailureReason,
) -> UiIntentAdmissionStopReason {
    match reason {
        crate::runtime::intent_execution::UiIntentExecutionAdmissionReservationFailureReason::Capacity(
            denial,
        ) => UiIntentAdmissionStopReason::ExecutionReservation(denial),
        crate::runtime::intent_execution::UiIntentExecutionAdmissionReservationFailureReason::Occupancy(
            denial,
        ) => occupancy_stop_reason(denial),
        crate::runtime::intent_execution::UiIntentExecutionAdmissionReservationFailureReason::ReservationIdentityExhausted => {
            UiIntentAdmissionStopReason::ReservationIdentityExhausted
        }
    }
}

fn occupancy_stop_reason(
    denial: super::super::operability::UiIntentOccupancyReservationDenial,
) -> UiIntentAdmissionStopReason {
    match denial {
        super::super::operability::UiIntentOccupancyReservationDenial::ScopeBecameOccupied => {
            UiIntentAdmissionStopReason::OccupancyChanged
        }
        super::super::operability::UiIntentOccupancyReservationDenial::CapacityExceeded {
            maximum,
        } => UiIntentAdmissionStopReason::OccupancyCapacityExceeded { maximum },
        super::super::operability::UiIntentOccupancyReservationDenial::ReservationIdentityExhausted => {
            UiIntentAdmissionStopReason::ReservationIdentityExhausted
        }
    }
}

fn admission_cost(candidate: &UiCurrentIntentAdmissionCandidate) -> UiIntentAdmissionCost {
    UiIntentAdmissionCost::prepared(
        candidate.route_resolution_cost(),
        candidate.payload_projection_cost(),
        candidate.decision().cost().selected_dependencies_visited(),
        candidate.currentness_checks(),
    )
}

fn next(value: u64) -> u64 {
    value
        .checked_add(1)
        .expect("bounded admission lifecycle accounting exhausted")
}
