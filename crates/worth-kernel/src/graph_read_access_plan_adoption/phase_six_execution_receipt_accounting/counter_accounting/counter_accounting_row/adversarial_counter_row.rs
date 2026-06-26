use super::{
    WorthGraphReadAccessCounterAccountingRow, WorthGraphReadAccessCounterAccountingRowInput,
};

impl WorthGraphReadAccessCounterAccountingRow {
    pub(crate) fn with_receipt_identity_digest_for_tests(
        &self,
        receipt_identity_digest: impl Into<String>,
    ) -> Self {
        Self::new(WorthGraphReadAccessCounterAccountingRowInput {
            source_projection_digest: self.source_projection_digest.clone(),
            receipt_identity_digest: receipt_identity_digest.into(),
            status: self.status,
            planned_access_step_count: self.planned_access_step_count,
            consumed_access_step_count: self.consumed_access_step_count,
            executor_entry_count: self.executor_entry_count,
            materialized_row_count: self.materialized_row_count,
            ephemeral_allocation_count: self.ephemeral_allocation_count,
            candidate_root_count: self.candidate_root_count,
            touched_node_count: self.touched_node_count,
            touched_edge_count: self.touched_edge_count,
            frontier_width: self.frontier_width,
            visited_breadth: self.visited_breadth,
            dedup_breadth: self.dedup_breadth,
            resident_byte_count: self.resident_byte_count,
            fallback_count: self.fallback_count,
            streaming_page_count: self.streaming_page_count,
            streaming_emitted_row_count: self.streaming_emitted_row_count,
            local_work_count: self.local_work_count,
            source_counter_proof: self.source_counter_proof.clone(),
            caller_owned_work: self.caller_owned_work.clone(),
        })
    }
}
