use super::super::super::stable_digest;
use super::WorthGraphReadAccessCounterAccountingRowInput;

pub(crate) fn counter_accounting_row_digest(
    input: &WorthGraphReadAccessCounterAccountingRowInput,
) -> String {
    stable_digest(&[
        "worth_graph_read_access_counter_accounting_row_v1".to_string(),
        format!("source:{}", input.source_projection_digest),
        format!("identity:{}", input.receipt_identity_digest),
        format!("status:{}", input.status.as_str()),
        format!("planned_steps:{}", input.planned_access_step_count),
        format!("consumed_steps:{}", input.consumed_access_step_count),
        format!("executor_entry:{}", input.executor_entry_count),
        format!("materialized_rows:{}", input.materialized_row_count),
        format!("ephemeral_allocations:{}", input.ephemeral_allocation_count),
        format!("candidate_roots:{}", input.candidate_root_count),
        format!("touched_nodes:{}", input.touched_node_count),
        format!("touched_edges:{}", input.touched_edge_count),
        format!("frontier_width:{}", input.frontier_width),
        format!("visited_breadth:{}", input.visited_breadth),
        format!("dedup_breadth:{}", input.dedup_breadth),
        format!("resident_bytes:{}", input.resident_byte_count),
        format!("fallback_count:{}", input.fallback_count),
        format!("streaming_pages:{}", input.streaming_page_count),
        format!("streaming_rows:{}", input.streaming_emitted_row_count),
        format!("local_work:{}", input.local_work_count),
        format!(
            "source_counter_proof:{}",
            input.source_counter_proof.proof_digest()
        ),
        format!("caller_work:{}", input.caller_owned_work.breakdown_digest()),
    ])
}
