#[cfg(test)]
use crate::construction::query_access_planning::PrimitiveConstructionConsumedQueryAccess;

#[cfg(test)]
use super::super::errors::{
    WorthGraphReadAccessFirstVerticalSliceError, WorthGraphReadAccessFirstVerticalSliceErrorKind,
};
#[cfg(test)]
use super::super::slice_selection::WorthGraphReadAccessSelectedVerticalSlice;
#[cfg(test)]
use super::super::stable_digest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessExecutedVerticalSlice {
    selected_slice_digest: String,
    source_requirement_record_digest: String,
    requirement_row_digest: String,
    declared_read_family_identity_digest: String,
    executed_read_family_digest: String,
    admitted_plan_digest: String,
    query_admission_digest: String,
    query_requirement_set_digest: String,
    plan_consumption_digest: String,
    execution_basis_digest: String,
    candidate_root_count: usize,
    touched_node_count: usize,
    touched_edge_count: usize,
    frontier_width: usize,
    visited_breadth: usize,
    dedup_breadth: usize,
    resident_byte_count: usize,
    executor_entry_count: usize,
    materialized_row_count: usize,
    local_strategy_recompute_count: usize,
    local_edge_scan_count: usize,
    local_neighbor_lookup_count: usize,
    persistent_artifact_bypass_count: usize,
    fallback_count: usize,
    execution_digest: String,
}

#[cfg(test)]
pub(crate) fn bind_selected_slice_to_construction_execution(
    selected_slice: &WorthGraphReadAccessSelectedVerticalSlice,
    consumed_access: &PrimitiveConstructionConsumedQueryAccess,
) -> Result<WorthGraphReadAccessExecutedVerticalSlice, WorthGraphReadAccessFirstVerticalSliceError>
{
    let declared_read_family_identity_digest = selected_slice
        .read_family_identity_digest()
        .ok_or_else(|| missing_execution_binding_error())?;
    let requirement_row_digest = selected_slice
        .requirement_row_digest()
        .ok_or_else(|| missing_execution_binding_error())?;
    let receipt = consumed_access.receipt();
    let execution_basis_digest = stable_digest(&[
        "worth_graph_read_access_phase_four_execution_basis_v1".to_string(),
        format!("slice:{}", selected_slice.slice_digest()),
        format!("surface:{}", receipt.surface().as_str()),
        format!("family:{}", receipt.family_digest()),
        format!("plan:{}", receipt.plan_digest()),
        format!("admission:{}", receipt.admission_digest()),
        format!("requirement_set:{}", receipt.requirement_set_digest()),
    ]);
    let execution_digest = stable_digest(&[
        "worth_graph_read_access_executed_vertical_slice_v1".to_string(),
        format!("slice:{}", selected_slice.slice_digest()),
        format!(
            "source_requirement:{}",
            selected_slice.source_requirement_record_digest()
        ),
        format!("requirement_row:{requirement_row_digest}"),
        format!("declared_read_family:{declared_read_family_identity_digest}"),
        format!("executed_read_family:{}", receipt.family_digest()),
        format!("plan:{}", receipt.plan_digest()),
        format!("admission:{}", receipt.admission_digest()),
        format!("requirement_set:{}", receipt.requirement_set_digest()),
        format!("consumption:{}", receipt.plan_consumption_digest()),
        format!("candidate_roots:{}", receipt.candidate_root_count()),
        format!("touched_nodes:{}", receipt.touched_node_count()),
        format!("touched_edges:{}", receipt.touched_edge_count()),
        format!("frontier_width:{}", receipt.frontier_width()),
        format!("visited_breadth:{}", receipt.visited_breadth()),
        format!("dedup_breadth:{}", receipt.dedup_breadth()),
        format!("resident_bytes:{}", receipt.resident_byte_count()),
        format!("fallback_count:{}", receipt.fallback_count()),
        format!("basis:{execution_basis_digest}"),
    ]);
    Ok(WorthGraphReadAccessExecutedVerticalSlice {
        selected_slice_digest: selected_slice.slice_digest().to_string(),
        source_requirement_record_digest: selected_slice
            .source_requirement_record_digest()
            .to_string(),
        requirement_row_digest: requirement_row_digest.to_string(),
        declared_read_family_identity_digest: declared_read_family_identity_digest.to_string(),
        executed_read_family_digest: receipt.family_digest().to_string(),
        admitted_plan_digest: receipt.plan_digest().to_string(),
        query_admission_digest: receipt.admission_digest().to_string(),
        query_requirement_set_digest: receipt.requirement_set_digest().to_string(),
        plan_consumption_digest: receipt.plan_consumption_digest().to_string(),
        execution_basis_digest,
        candidate_root_count: receipt.candidate_root_count(),
        touched_node_count: receipt.touched_node_count(),
        touched_edge_count: receipt.touched_edge_count(),
        frontier_width: receipt.frontier_width(),
        visited_breadth: receipt.visited_breadth(),
        dedup_breadth: receipt.dedup_breadth(),
        resident_byte_count: receipt.resident_byte_count(),
        executor_entry_count: receipt.executor_entry_count(),
        materialized_row_count: receipt.materialized_row_count(),
        local_strategy_recompute_count: receipt.strategy_recompute_count(),
        local_edge_scan_count: receipt.edge_scan_count(),
        local_neighbor_lookup_count: receipt.per_result_neighbor_lookup_count(),
        persistent_artifact_bypass_count: receipt.persistent_artifact_bypass_count(),
        fallback_count: receipt.fallback_count(),
        execution_digest,
    })
}

#[cfg(test)]
impl WorthGraphReadAccessExecutedVerticalSlice {
    pub fn requirement_row_digest(&self) -> &str {
        &self.requirement_row_digest
    }

    pub fn declared_read_family_identity_digest(&self) -> &str {
        &self.declared_read_family_identity_digest
    }

    pub fn executed_read_family_digest(&self) -> &str {
        &self.executed_read_family_digest
    }

    pub fn admitted_plan_digest(&self) -> &str {
        &self.admitted_plan_digest
    }

    pub fn query_admission_digest(&self) -> &str {
        &self.query_admission_digest
    }

    pub fn query_requirement_set_digest(&self) -> &str {
        &self.query_requirement_set_digest
    }

    pub fn plan_consumption_digest(&self) -> &str {
        &self.plan_consumption_digest
    }

    pub fn execution_basis_digest(&self) -> &str {
        &self.execution_basis_digest
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

    pub const fn executor_entry_count(&self) -> usize {
        self.executor_entry_count
    }

    pub const fn materialized_row_count(&self) -> usize {
        self.materialized_row_count
    }

    pub const fn local_strategy_recompute_count(&self) -> usize {
        self.local_strategy_recompute_count
    }

    pub const fn local_edge_scan_count(&self) -> usize {
        self.local_edge_scan_count
    }

    pub const fn local_neighbor_lookup_count(&self) -> usize {
        self.local_neighbor_lookup_count
    }

    pub const fn persistent_artifact_bypass_count(&self) -> usize {
        self.persistent_artifact_bypass_count
    }

    pub const fn fallback_count(&self) -> usize {
        self.fallback_count
    }

    pub const fn no_caller_owned_graph_work(&self) -> bool {
        self.local_strategy_recompute_count == 0
            && self.local_edge_scan_count == 0
            && self.local_neighbor_lookup_count == 0
            && self.persistent_artifact_bypass_count == 0
    }
}

#[cfg(test)]
const fn missing_execution_binding_error() -> WorthGraphReadAccessFirstVerticalSliceError {
    WorthGraphReadAccessFirstVerticalSliceError::new(
        WorthGraphReadAccessFirstVerticalSliceErrorKind::MissingExecutionBindingIdentity,
    )
}
