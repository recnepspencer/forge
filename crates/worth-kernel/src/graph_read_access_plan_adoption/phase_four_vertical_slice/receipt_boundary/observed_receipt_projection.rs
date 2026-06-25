#[cfg(test)]
use super::super::execution_binding::WorthGraphReadAccessExecutedVerticalSlice;
#[cfg(test)]
use super::super::query_plan_projection::WorthGraphReadAccessSlicePlanProjection;
#[cfg(test)]
use super::super::slice_selection::WorthGraphReadAccessSelectedVerticalSlice;
#[cfg(test)]
use super::super::stable_digest;
#[cfg(test)]
use super::{WorthGraphReadAccessSliceReceiptProjection, WorthGraphReadAccessSliceReceiptStatus};

#[cfg(test)]
pub(crate) fn query_receipt_observed_projection(
    selected_slice: &WorthGraphReadAccessSelectedVerticalSlice,
    plan_projection: &WorthGraphReadAccessSlicePlanProjection,
    executed_slice: &WorthGraphReadAccessExecutedVerticalSlice,
) -> WorthGraphReadAccessSliceReceiptProjection {
    let status = WorthGraphReadAccessSliceReceiptStatus::QueryReceiptObserved;
    let projection_digest = stable_digest(&[
        "worth_graph_read_access_slice_receipt_projection_v1".to_string(),
        format!("slice:{}", selected_slice.slice_digest()),
        format!("plan_projection:{}", plan_projection.projection_digest()),
        format!("status:{}", status.as_str()),
        format!(
            "requirement_row:{}",
            executed_slice.requirement_row_digest()
        ),
        format!(
            "declared_read_family:{}",
            executed_slice.declared_read_family_identity_digest()
        ),
        format!(
            "executed_read_family:{}",
            executed_slice.executed_read_family_digest()
        ),
        format!("admitted_plan:{}", executed_slice.admitted_plan_digest()),
        format!(
            "query_admission:{}",
            executed_slice.query_admission_digest()
        ),
        format!(
            "query_requirement_set:{}",
            executed_slice.query_requirement_set_digest()
        ),
        format!("consumption:{}", executed_slice.plan_consumption_digest()),
        format!("candidate_roots:{}", executed_slice.candidate_root_count()),
        format!("touched_nodes:{}", executed_slice.touched_node_count()),
        format!("touched_edges:{}", executed_slice.touched_edge_count()),
        format!("frontier_width:{}", executed_slice.frontier_width()),
        format!("visited_breadth:{}", executed_slice.visited_breadth()),
        format!("dedup_breadth:{}", executed_slice.dedup_breadth()),
        format!("resident_bytes:{}", executed_slice.resident_byte_count()),
        format!(
            "execution_basis:{}",
            executed_slice.execution_basis_digest()
        ),
        format!("executor_entries:{}", executed_slice.executor_entry_count()),
        format!(
            "materialized_rows:{}",
            executed_slice.materialized_row_count()
        ),
    ]);
    WorthGraphReadAccessSliceReceiptProjection {
        selected_slice_digest: selected_slice.slice_digest().to_string(),
        plan_projection_digest: plan_projection.projection_digest().to_string(),
        status,
        plan_consumption_digest: Some(executed_slice.plan_consumption_digest().to_string()),
        execution_basis: executed_slice.execution_basis_digest().to_string(),
        requirement_row_digest: Some(executed_slice.requirement_row_digest().to_string()),
        declared_read_family_identity_digest: Some(
            executed_slice
                .declared_read_family_identity_digest()
                .to_string(),
        ),
        executed_read_family_digest: Some(executed_slice.executed_read_family_digest().to_string()),
        admitted_plan_digest: Some(executed_slice.admitted_plan_digest().to_string()),
        query_admission_digest: Some(executed_slice.query_admission_digest().to_string()),
        query_requirement_set_digest: Some(
            executed_slice.query_requirement_set_digest().to_string(),
        ),
        candidate_root_count: executed_slice.candidate_root_count(),
        touched_node_count: executed_slice.touched_node_count(),
        touched_edge_count: executed_slice.touched_edge_count(),
        frontier_width: executed_slice.frontier_width(),
        visited_breadth: executed_slice.visited_breadth(),
        dedup_breadth: executed_slice.dedup_breadth(),
        resident_byte_count: executed_slice.resident_byte_count(),
        executor_entry_count: executed_slice.executor_entry_count(),
        materialized_row_count: executed_slice.materialized_row_count(),
        local_strategy_recompute_count: executed_slice.local_strategy_recompute_count(),
        local_edge_scan_count: executed_slice.local_edge_scan_count(),
        local_neighbor_lookup_count: executed_slice.local_neighbor_lookup_count(),
        persistent_artifact_bypass_count: executed_slice.persistent_artifact_bypass_count(),
        fallback_count: executed_slice.fallback_count(),
        required_query_surface: None,
        required_worth_surface: None,
        existing_worth_execution_surface: None,
        blocker: None,
        projection_digest,
    }
}
