use super::super::{
    WorthGraphReadAccessCallerOwnedWorkBreakdown, WorthGraphReadAccessCounterAccountingStatus,
    WorthGraphReadAccessSourceCounterProof,
};

pub(crate) struct WorthGraphReadAccessCounterAccountingRowInput {
    pub(crate) source_projection_digest: String,
    pub(crate) receipt_identity_digest: String,
    pub(crate) status: WorthGraphReadAccessCounterAccountingStatus,
    pub(crate) planned_access_step_count: usize,
    pub(crate) consumed_access_step_count: usize,
    pub(crate) executor_entry_count: usize,
    pub(crate) materialized_row_count: usize,
    pub(crate) ephemeral_allocation_count: usize,
    pub(crate) candidate_root_count: usize,
    pub(crate) touched_node_count: usize,
    pub(crate) touched_edge_count: usize,
    pub(crate) frontier_width: usize,
    pub(crate) visited_breadth: usize,
    pub(crate) dedup_breadth: usize,
    pub(crate) resident_byte_count: usize,
    pub(crate) fallback_count: usize,
    pub(crate) streaming_page_count: usize,
    pub(crate) streaming_emitted_row_count: usize,
    pub(crate) local_work_count: usize,
    pub(crate) source_counter_proof: WorthGraphReadAccessSourceCounterProof,
    pub(crate) caller_owned_work: WorthGraphReadAccessCallerOwnedWorkBreakdown,
}
