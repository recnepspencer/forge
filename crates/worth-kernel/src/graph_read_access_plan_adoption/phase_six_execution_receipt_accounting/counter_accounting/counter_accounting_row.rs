#[cfg(test)]
mod adversarial_counter_row;
mod receipt_counter_sources;
mod row_digest;
mod row_input;

use super::{
    WorthGraphReadAccessCallerOwnedWorkBreakdown, WorthGraphReadAccessCounterAccountingStatus,
    WorthGraphReadAccessSourceCounterProof,
};
use row_digest::counter_accounting_row_digest;
use row_input::WorthGraphReadAccessCounterAccountingRowInput;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessCounterAccountingRow {
    source_projection_digest: String,
    receipt_identity_digest: String,
    status: WorthGraphReadAccessCounterAccountingStatus,
    planned_access_step_count: usize,
    consumed_access_step_count: usize,
    executor_entry_count: usize,
    materialized_row_count: usize,
    ephemeral_allocation_count: usize,
    candidate_root_count: usize,
    touched_node_count: usize,
    touched_edge_count: usize,
    frontier_width: usize,
    visited_breadth: usize,
    dedup_breadth: usize,
    resident_byte_count: usize,
    fallback_count: usize,
    streaming_page_count: usize,
    streaming_emitted_row_count: usize,
    local_work_count: usize,
    source_counter_proof: WorthGraphReadAccessSourceCounterProof,
    caller_owned_work: WorthGraphReadAccessCallerOwnedWorkBreakdown,
    row_digest: String,
}

impl WorthGraphReadAccessCounterAccountingRow {
    pub(crate) fn new(input: WorthGraphReadAccessCounterAccountingRowInput) -> Self {
        let row_digest = counter_accounting_row_digest(&input);
        Self {
            source_projection_digest: input.source_projection_digest,
            receipt_identity_digest: input.receipt_identity_digest,
            status: input.status,
            planned_access_step_count: input.planned_access_step_count,
            consumed_access_step_count: input.consumed_access_step_count,
            executor_entry_count: input.executor_entry_count,
            materialized_row_count: input.materialized_row_count,
            ephemeral_allocation_count: input.ephemeral_allocation_count,
            candidate_root_count: input.candidate_root_count,
            touched_node_count: input.touched_node_count,
            touched_edge_count: input.touched_edge_count,
            frontier_width: input.frontier_width,
            visited_breadth: input.visited_breadth,
            dedup_breadth: input.dedup_breadth,
            resident_byte_count: input.resident_byte_count,
            fallback_count: input.fallback_count,
            streaming_page_count: input.streaming_page_count,
            streaming_emitted_row_count: input.streaming_emitted_row_count,
            local_work_count: input.local_work_count,
            source_counter_proof: input.source_counter_proof,
            caller_owned_work: input.caller_owned_work,
            row_digest,
        }
    }

    pub fn source_projection_digest(&self) -> &str {
        &self.source_projection_digest
    }

    pub fn receipt_identity_digest(&self) -> &str {
        &self.receipt_identity_digest
    }

    pub const fn status(&self) -> WorthGraphReadAccessCounterAccountingStatus {
        self.status
    }

    pub const fn planned_access_step_count(&self) -> usize {
        self.planned_access_step_count
    }

    pub const fn consumed_access_step_count(&self) -> usize {
        self.consumed_access_step_count
    }

    pub const fn executor_entry_count(&self) -> usize {
        self.executor_entry_count
    }

    pub const fn materialized_row_count(&self) -> usize {
        self.materialized_row_count
    }

    pub const fn ephemeral_allocation_count(&self) -> usize {
        self.ephemeral_allocation_count
    }

    pub const fn candidate_root_count(&self) -> usize {
        self.candidate_root_count
    }

    pub const fn touched_node_count(&self) -> usize {
        self.touched_node_count
    }

    pub const fn touched_edge_count(&self) -> usize {
        self.touched_edge_count
    }

    pub const fn frontier_width(&self) -> usize {
        self.frontier_width
    }

    pub const fn visited_breadth(&self) -> usize {
        self.visited_breadth
    }

    pub const fn dedup_breadth(&self) -> usize {
        self.dedup_breadth
    }

    pub const fn resident_byte_count(&self) -> usize {
        self.resident_byte_count
    }

    pub const fn fallback_count(&self) -> usize {
        self.fallback_count
    }

    pub const fn streaming_page_count(&self) -> usize {
        self.streaming_page_count
    }

    pub const fn streaming_emitted_row_count(&self) -> usize {
        self.streaming_emitted_row_count
    }

    pub const fn local_work_count(&self) -> usize {
        self.local_work_count
    }

    pub const fn caller_owned_work(&self) -> &WorthGraphReadAccessCallerOwnedWorkBreakdown {
        &self.caller_owned_work
    }

    pub const fn source_counter_proof(&self) -> &WorthGraphReadAccessSourceCounterProof {
        &self.source_counter_proof
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}
