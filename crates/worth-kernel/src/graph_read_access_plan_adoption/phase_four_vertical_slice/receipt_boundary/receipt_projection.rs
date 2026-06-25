#[cfg(test)]
use super::super::execution_binding::WorthGraphReadAccessExecutedVerticalSlice;
use super::super::query_plan_projection::{
    WorthGraphReadAccessSlicePlanProjection, WorthGraphReadAccessSlicePlanProjectionStatus,
};
use super::super::slice_selection::WorthGraphReadAccessSelectedVerticalSlice;
use super::capability_gap_projection::query_execution_capability_gap_projection;
#[cfg(test)]
use super::observed_receipt_projection::query_receipt_observed_projection;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthGraphReadAccessSliceReceiptStatus {
    QueryReceiptObserved,
    QueryExecutionCapabilityGap,
}

impl WorthGraphReadAccessSliceReceiptStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QueryReceiptObserved => "query_receipt_observed",
            Self::QueryExecutionCapabilityGap => "query_execution_capability_gap",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessSliceReceiptProjection {
    pub(crate) selected_slice_digest: String,
    pub(crate) plan_projection_digest: String,
    pub(crate) status: WorthGraphReadAccessSliceReceiptStatus,
    pub(crate) plan_consumption_digest: Option<String>,
    pub(crate) execution_basis: String,
    pub(crate) requirement_row_digest: Option<String>,
    pub(crate) declared_read_family_identity_digest: Option<String>,
    pub(crate) executed_read_family_digest: Option<String>,
    pub(crate) admitted_plan_digest: Option<String>,
    pub(crate) query_admission_digest: Option<String>,
    pub(crate) query_requirement_set_digest: Option<String>,
    pub(crate) candidate_root_count: usize,
    pub(crate) touched_node_count: usize,
    pub(crate) touched_edge_count: usize,
    pub(crate) frontier_width: usize,
    pub(crate) visited_breadth: usize,
    pub(crate) dedup_breadth: usize,
    pub(crate) resident_byte_count: usize,
    pub(crate) executor_entry_count: usize,
    pub(crate) materialized_row_count: usize,
    pub(crate) local_strategy_recompute_count: usize,
    pub(crate) local_edge_scan_count: usize,
    pub(crate) local_neighbor_lookup_count: usize,
    pub(crate) persistent_artifact_bypass_count: usize,
    pub(crate) fallback_count: usize,
    pub(crate) required_query_surface: Option<String>,
    pub(crate) required_worth_surface: Option<String>,
    pub(crate) existing_worth_execution_surface: Option<String>,
    pub(crate) blocker: Option<String>,
    pub(crate) projection_digest: String,
}

pub(crate) fn project_receipt_for_plan_projection(
    selected_slice: &WorthGraphReadAccessSelectedVerticalSlice,
    plan_projection: &WorthGraphReadAccessSlicePlanProjection,
) -> WorthGraphReadAccessSliceReceiptProjection {
    match plan_projection.status() {
        WorthGraphReadAccessSlicePlanProjectionStatus::QueryPlanAdmitted => {
            query_execution_capability_gap_projection(selected_slice, plan_projection)
        }
        WorthGraphReadAccessSlicePlanProjectionStatus::MissingQueryReadFamilyArtifactForExecution => {
            query_execution_capability_gap_projection(selected_slice, plan_projection)
        }
    }
}

#[cfg(test)]
pub(crate) fn project_receipt_for_executed_slice(
    selected_slice: &WorthGraphReadAccessSelectedVerticalSlice,
    plan_projection: &WorthGraphReadAccessSlicePlanProjection,
    executed_slice: &WorthGraphReadAccessExecutedVerticalSlice,
) -> WorthGraphReadAccessSliceReceiptProjection {
    query_receipt_observed_projection(selected_slice, plan_projection, executed_slice)
}

impl WorthGraphReadAccessSliceReceiptProjection {
    pub fn selected_slice_digest(&self) -> &str {
        &self.selected_slice_digest
    }

    pub fn plan_projection_digest(&self) -> &str {
        &self.plan_projection_digest
    }

    pub const fn status(&self) -> WorthGraphReadAccessSliceReceiptStatus {
        self.status
    }

    pub fn plan_consumption_digest(&self) -> Option<&str> {
        self.plan_consumption_digest.as_deref()
    }

    pub fn execution_basis(&self) -> &str {
        &self.execution_basis
    }

    pub fn requirement_row_digest(&self) -> Option<&str> {
        self.requirement_row_digest.as_deref()
    }

    pub fn declared_read_family_identity_digest(&self) -> Option<&str> {
        self.declared_read_family_identity_digest.as_deref()
    }

    pub fn executed_read_family_digest(&self) -> Option<&str> {
        self.executed_read_family_digest.as_deref()
    }

    pub fn admitted_plan_digest(&self) -> Option<&str> {
        self.admitted_plan_digest.as_deref()
    }

    pub fn query_admission_digest(&self) -> Option<&str> {
        self.query_admission_digest.as_deref()
    }

    pub fn query_requirement_set_digest(&self) -> Option<&str> {
        self.query_requirement_set_digest.as_deref()
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

    pub fn required_query_surface(&self) -> Option<&str> {
        self.required_query_surface.as_deref()
    }

    pub fn required_worth_surface(&self) -> Option<&str> {
        self.required_worth_surface.as_deref()
    }

    pub fn existing_worth_execution_surface(&self) -> Option<&str> {
        self.existing_worth_execution_surface.as_deref()
    }

    pub fn blocker(&self) -> Option<&str> {
        self.blocker.as_deref()
    }

    pub fn projection_digest(&self) -> &str {
        &self.projection_digest
    }

    pub const fn claims_access_plan_consumption(&self) -> bool {
        matches!(
            self.status,
            WorthGraphReadAccessSliceReceiptStatus::QueryReceiptObserved
        )
    }

    pub const fn claims_graph_read_execution(&self) -> bool {
        matches!(
            self.status,
            WorthGraphReadAccessSliceReceiptStatus::QueryReceiptObserved
        )
    }

    pub const fn claims_graph_read_receipt(&self) -> bool {
        matches!(
            self.status,
            WorthGraphReadAccessSliceReceiptStatus::QueryReceiptObserved
        )
    }

    #[cfg(test)]
    pub(crate) fn with_adversarial_caller_owned_work_for_tests(&self) -> Self {
        let mut mutated = self.clone();
        mutated.local_neighbor_lookup_count += 1;
        mutated.projection_digest = super::super::stable_digest(&[
            "worth_graph_read_access_slice_receipt_projection_test_mutation_v1".to_string(),
            format!("source:{}", self.projection_digest),
            "mutation:caller_owned_neighbor_lookup".to_string(),
            format!(
                "local_neighbor_lookup:{}",
                mutated.local_neighbor_lookup_count
            ),
        ]);
        mutated
    }

    #[cfg(test)]
    pub(crate) fn with_adversarial_plan_digest_for_tests(
        &self,
        admitted_plan_digest: impl Into<String>,
    ) -> Self {
        let mut mutated = self.clone();
        mutated.admitted_plan_digest = Some(admitted_plan_digest.into());
        mutated.projection_digest = super::super::stable_digest(&[
            "worth_graph_read_access_slice_receipt_projection_test_mutation_v1".to_string(),
            format!("source:{}", self.projection_digest),
            "mutation:admitted_plan_digest".to_string(),
            format!(
                "admitted_plan:{}",
                mutated.admitted_plan_digest.as_deref().unwrap_or("none")
            ),
        ]);
        mutated
    }

    #[cfg(test)]
    pub(crate) fn with_adversarial_touched_authority_for_tests(
        &self,
        selected_slice_digest: impl Into<String>,
    ) -> Self {
        let mut mutated = self.clone();
        mutated.selected_slice_digest = selected_slice_digest.into();
        mutated.projection_digest = super::super::stable_digest(&[
            "worth_graph_read_access_slice_receipt_projection_test_mutation_v1".to_string(),
            format!("source:{}", self.projection_digest),
            "mutation:selected_slice_digest".to_string(),
            format!("selected_slice:{}", mutated.selected_slice_digest),
        ]);
        mutated
    }
}
