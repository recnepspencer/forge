use worth_runtime_bridge::facade::BridgeOwnedSignalRuntime;

use super::{
    authoritative_clock_progression::AuthoritativeClockProgress, ErasedClockObservationReceipt,
    WorthQueryConditionalTruthBasis, WorthQueryInstalledTemporalOperation,
};
use crate::domain_computation::primary_graph::conditional_operation::{
    application_operation_reentry::WorthQueryTemporalReentryCounts,
    signal_decision_reentry::{
        evaluate_due_wake, retained_decision_counts, WorthQueryRetainedConditionalDecision,
    },
};

impl<Binding, Reconstruction, Execution, Clock, Input>
    WorthQueryInstalledTemporalOperation<Binding, Reconstruction, Execution, Clock, Input>
{
    pub(super) fn retain_due(
        &mut self,
        accepted: worth_runtime_bridge::facade::BridgeManagedClockAcceptedObservation,
        bridge: &mut BridgeOwnedSignalRuntime,
        truth: &WorthQueryConditionalTruthBasis,
        authoritative_deliveries: &[worth_runtime_bridge::facade::BridgeGranularInvalidationDelivery],
    ) -> ErasedClockObservationReceipt {
        let sequence = accepted.sequence();
        let observed_coordinate = accepted.observed_coordinate();
        let due = accepted.into_due();
        let due_wake_count = due.wakes().len();
        let due_work_remaining = due.due_work_remaining();
        let evaluated = due.into_wakes().into_iter().map(|wake| {
            let triggering_correspondence = authoritative_deliveries
                .iter()
                .map(|delivery| delivery.correspondence_receipt())
                .find(|receipt| {
                    receipt.change_set().changes().iter().any(|change| {
                        change.relational_record_identity() == Some(wake.source_record_identity())
                    })
                });
            evaluate_due_wake(
                bridge,
                wake,
                &self.lowering,
                &self.runtime_binding_identity,
                self.runtime_capability_identity,
                truth,
                triggering_correspondence,
            )
        });
        self.retained_wakes.extend(evaluated);
        let decisions = retained_decision_counts(&self.retained_wakes);
        ErasedClockObservationReceipt {
            sequence,
            observed_coordinate,
            due_wake_count,
            due_work_remaining,
            authoritative_commit_count: 0,
            authoritative_work_remaining: false,
            retained_due_wake_count: self.retained_wakes.len(),
            retained_eligible_wake_count: decisions.eligible,
            retained_suppressed_wake_count: decisions.suppressed,
            retained_deferred_wake_count: decisions.deferred,
            retained_failed_wake_count: decisions.failed,
            committed_operation_count: 0,
            already_committed_operation_count: 0,
            failed_operation_count: 0,
            indeterminate_operation_count: 0,
            snapshot_capacity_backpressure: None,
            retention_capacity_backpressure: false,
            execution_provenance: Vec::new(),
            granular_invalidations: Vec::new(),
        }
    }

    pub(super) fn complete_clock_receipt(
        &mut self,
        mut receipt: ErasedClockObservationReceipt,
        counts: WorthQueryTemporalReentryCounts,
        authoritative: AuthoritativeClockProgress,
    ) -> ErasedClockObservationReceipt {
        receipt.due_work_remaining |= authoritative.work_remaining;
        receipt.authoritative_commit_count = authoritative.commit_count;
        receipt.authoritative_work_remaining = authoritative.work_remaining;
        receipt.granular_invalidations = authoritative.granular_invalidations;
        self.committed_operation_count = self
            .committed_operation_count
            .saturating_add(counts.committed);
        self.already_committed_operation_count = self
            .already_committed_operation_count
            .saturating_add(counts.already_committed);
        self.failed_operation_count = self.failed_operation_count.saturating_add(counts.failed);
        self.indeterminate_operation_count = self
            .indeterminate_operation_count
            .saturating_add(counts.indeterminate);
        receipt.execution_provenance =
            super::super::execution_provenance::execution_provenance(&self.retained_wakes);
        self.retained_wakes.retain(|wake| {
            !matches!(
                wake.decision,
                WorthQueryRetainedConditionalDecision::OperationCommitted(_)
                    | WorthQueryRetainedConditionalDecision::OperationAlreadyCommitted(_)
            )
        });
        let decisions = retained_decision_counts(&self.retained_wakes);
        receipt.retained_due_wake_count = self.retained_wakes.len();
        receipt.retained_eligible_wake_count = decisions.eligible;
        receipt.retained_suppressed_wake_count = decisions.suppressed;
        receipt.retained_deferred_wake_count = decisions.deferred;
        receipt.retained_failed_wake_count = decisions.failed;
        receipt.committed_operation_count = self.committed_operation_count;
        receipt.already_committed_operation_count = self.already_committed_operation_count;
        receipt.failed_operation_count = self.failed_operation_count;
        receipt.indeterminate_operation_count = self.indeterminate_operation_count;
        receipt.snapshot_capacity_backpressure = counts.snapshot_capacity_backpressure;
        receipt.retention_capacity_backpressure = counts.retention_capacity_backpressure;
        receipt
    }
}
