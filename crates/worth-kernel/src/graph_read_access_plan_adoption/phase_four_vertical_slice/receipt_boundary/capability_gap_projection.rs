use super::super::query_plan_projection::WorthGraphReadAccessSlicePlanProjection;
use super::super::slice_selection::WorthGraphReadAccessSelectedVerticalSlice;
use super::super::stable_digest;
use super::execution_basis::missing_execution_basis_label;
use super::receipt_gap::missing_worth_execution_binding_blocker;
use super::{WorthGraphReadAccessSliceReceiptProjection, WorthGraphReadAccessSliceReceiptStatus};

pub(crate) fn query_execution_capability_gap_projection(
    selected_slice: &WorthGraphReadAccessSelectedVerticalSlice,
    plan_projection: &WorthGraphReadAccessSlicePlanProjection,
) -> WorthGraphReadAccessSliceReceiptProjection {
    let status = WorthGraphReadAccessSliceReceiptStatus::QueryExecutionCapabilityGap;
    let required_query_surface =
        "ForgeQueryReadResult::receipt().graph_read_access_plan_consumption()";
    let required_worth_surface =
        "WorthGraphReadAccessSelectedVerticalSlice -> ForgeQueryReadFamily execution binding";
    let existing_worth_execution_surface =
        "crate::construction::query_access_planning::execute_planned_construction_query_access";
    let blocker = missing_worth_execution_binding_blocker();
    let execution_basis = missing_execution_basis_label();
    let projection_digest = stable_digest(&[
        "worth_graph_read_access_slice_receipt_projection_v1".to_string(),
        format!("slice:{}", selected_slice.slice_digest()),
        format!("plan_projection:{}", plan_projection.projection_digest()),
        format!("status:{}", status.as_str()),
        format!("required_query_surface:{required_query_surface}"),
        format!("required_worth_surface:{required_worth_surface}"),
        format!("existing_worth_execution_surface:{existing_worth_execution_surface}"),
        format!("execution_basis:{execution_basis}"),
        format!("blocker:{blocker}"),
    ]);
    WorthGraphReadAccessSliceReceiptProjection {
        selected_slice_digest: selected_slice.slice_digest().to_string(),
        plan_projection_digest: plan_projection.projection_digest().to_string(),
        status,
        plan_consumption_digest: None,
        execution_basis: execution_basis.to_string(),
        requirement_row_digest: None,
        declared_read_family_identity_digest: None,
        executed_read_family_digest: None,
        admitted_plan_digest: None,
        query_admission_digest: None,
        query_requirement_set_digest: None,
        candidate_root_count: 0,
        touched_node_count: 0,
        touched_edge_count: 0,
        frontier_width: 0,
        visited_breadth: 0,
        dedup_breadth: 0,
        resident_byte_count: 0,
        executor_entry_count: 0,
        materialized_row_count: 0,
        local_strategy_recompute_count: 0,
        local_edge_scan_count: 0,
        local_neighbor_lookup_count: 0,
        persistent_artifact_bypass_count: 0,
        fallback_count: 0,
        required_query_surface: Some(required_query_surface.to_string()),
        required_worth_surface: Some(required_worth_surface.to_string()),
        existing_worth_execution_surface: Some(existing_worth_execution_surface.to_string()),
        blocker: Some(blocker.to_string()),
        projection_digest,
    }
}
