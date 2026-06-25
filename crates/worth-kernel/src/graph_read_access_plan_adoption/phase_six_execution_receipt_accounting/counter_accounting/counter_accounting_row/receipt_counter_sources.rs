use crate::graph_read_access_plan_adoption::{
    WorthGraphReadAccessReceiptAccountingRow, WorthGraphReadAccessReceiptStatus,
    WorthGraphReadAccessSliceReceiptProjection,
};

use super::super::{
    WorthGraphReadAccessCallerOwnedWorkBreakdown, WorthGraphReadAccessCounterAccountingStatus,
    WorthGraphReadAccessSourceCounterProof,
};
use super::{
    WorthGraphReadAccessCounterAccountingRow, WorthGraphReadAccessCounterAccountingRowInput,
};

impl WorthGraphReadAccessCounterAccountingRow {
    pub(crate) fn from_phase_four_receipt(
        receipt_row: &WorthGraphReadAccessReceiptAccountingRow,
        receipt: &WorthGraphReadAccessSliceReceiptProjection,
    ) -> Self {
        let executor_entry_count = receipt.executor_entry_count();
        let planned_access_step_count =
            usize::from(receipt.query_requirement_set_digest().is_some());
        let consumed_access_step_count = planned_access_step_count * executor_entry_count;
        let caller_owned_work =
            WorthGraphReadAccessCallerOwnedWorkBreakdown::from_phase_four_receipt(receipt);
        Self::new(WorthGraphReadAccessCounterAccountingRowInput {
            source_projection_digest: receipt_row.source_projection_digest().to_string(),
            receipt_identity_digest: receipt_row.receipt_identity().identity_digest().to_string(),
            status: if receipt_row.status().claims_query_receipt() {
                WorthGraphReadAccessCounterAccountingStatus::QueryCountersAccounted
            } else {
                WorthGraphReadAccessCounterAccountingStatus::CounterGapRequiresQueryReceiptSurface
            },
            planned_access_step_count,
            consumed_access_step_count,
            executor_entry_count,
            materialized_row_count: receipt.materialized_row_count(),
            ephemeral_allocation_count: 0,
            candidate_root_count: receipt.candidate_root_count(),
            touched_node_count: receipt.touched_node_count(),
            touched_edge_count: receipt.touched_edge_count(),
            frontier_width: receipt.frontier_width(),
            visited_breadth: receipt.visited_breadth(),
            dedup_breadth: receipt.dedup_breadth(),
            resident_byte_count: receipt.resident_byte_count(),
            fallback_count: receipt.fallback_count(),
            streaming_page_count: 0,
            streaming_emitted_row_count: 0,
            local_work_count: caller_owned_work.total_count(),
            source_counter_proof: WorthGraphReadAccessSourceCounterProof::from_phase_four_receipt(
                receipt,
                &caller_owned_work,
            ),
            caller_owned_work,
        })
    }

    pub(crate) fn from_receipt_row(receipt_row: &WorthGraphReadAccessReceiptAccountingRow) -> Self {
        let missing_counter_for_claimed_execution_count = usize::from(
            receipt_row.status().claims_query_receipt()
                && receipt_row
                    .receipt_identity()
                    .execution_counter_digest()
                    .is_none(),
        );
        let status = counter_status_for_receipt_row(receipt_row);
        let caller_owned_work = WorthGraphReadAccessCallerOwnedWorkBreakdown::from_counts(
            0,
            missing_counter_for_claimed_execution_count,
        );
        let source_counter_proof =
            WorthGraphReadAccessSourceCounterProof::from_receipt_counter_digest(
                receipt_row
                    .receipt_identity()
                    .execution_counter_digest()
                    .map(str::to_string),
                missing_counter_for_claimed_execution_count,
                &caller_owned_work,
            );
        Self::new(WorthGraphReadAccessCounterAccountingRowInput {
            source_projection_digest: receipt_row.source_projection_digest().to_string(),
            receipt_identity_digest: receipt_row.receipt_identity().identity_digest().to_string(),
            status,
            planned_access_step_count: usize::from(
                receipt_row.receipt_identity().plan_digest().is_some(),
            ),
            consumed_access_step_count: usize::from(
                receipt_row
                    .receipt_identity()
                    .execution_counter_digest()
                    .is_some(),
            ),
            executor_entry_count: usize::from(
                receipt_row
                    .receipt_identity()
                    .execution_counter_digest()
                    .is_some(),
            ),
            materialized_row_count: 0,
            ephemeral_allocation_count: 0,
            candidate_root_count: usize::from(
                receipt_row
                    .receipt_identity()
                    .execution_counter_digest()
                    .is_some(),
            ),
            touched_node_count: 0,
            touched_edge_count: 0,
            frontier_width: 0,
            visited_breadth: 0,
            dedup_breadth: 0,
            resident_byte_count: 0,
            fallback_count: 0,
            streaming_page_count: 0,
            streaming_emitted_row_count: 0,
            local_work_count: missing_counter_for_claimed_execution_count,
            source_counter_proof,
            caller_owned_work,
        })
    }
}

fn counter_status_for_receipt_row(
    receipt_row: &WorthGraphReadAccessReceiptAccountingRow,
) -> WorthGraphReadAccessCounterAccountingStatus {
    match receipt_row.status() {
        WorthGraphReadAccessReceiptStatus::ExecutedThroughQueryReceipt => {
            if receipt_row
                .receipt_identity()
                .execution_counter_digest()
                .is_some()
            {
                WorthGraphReadAccessCounterAccountingStatus::QueryCountersAccounted
            } else {
                WorthGraphReadAccessCounterAccountingStatus::CounterGapRequiresQueryReceiptSurface
            }
        }
        WorthGraphReadAccessReceiptStatus::AdmittedPlanRequiresExecutionReceipt => {
            WorthGraphReadAccessCounterAccountingStatus::CounterGapRequiresQueryReceiptSurface
        }
        WorthGraphReadAccessReceiptStatus::RequiredQueryPostureNoReceipt
        | WorthGraphReadAccessReceiptStatus::DeniedByQueryPostureNoReceipt
        | WorthGraphReadAccessReceiptStatus::CarriedCapabilityGapNoReceipt => {
            WorthGraphReadAccessCounterAccountingStatus::NoExecutionCountersRequired
        }
    }
}
