use super::query_plan_projection::{
    WorthGraphReadAccessSlicePlanProjection, WorthGraphReadAccessSlicePlanProjectionStatus,
};
use super::receipt_boundary::WorthGraphReadAccessSliceReceiptProjection;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessFirstVerticalSliceCounters {
    selected_slice_count: usize,
    query_plan_projection_attempt_count: usize,
    query_plan_admitted_count: usize,
    query_receipt_observed_count: usize,
    query_execution_gap_count: usize,
    local_graph_traversal_attempt_count: usize,
    local_adjacency_lookup_attempt_count: usize,
    local_broad_scan_attempt_count: usize,
    local_receipt_fabrication_attempt_count: usize,
}

impl WorthGraphReadAccessFirstVerticalSliceCounters {
    pub(crate) fn from_products(
        plan_projection: &WorthGraphReadAccessSlicePlanProjection,
        receipt_projection: &WorthGraphReadAccessSliceReceiptProjection,
    ) -> Self {
        Self {
            selected_slice_count: 1,
            query_plan_projection_attempt_count: 1,
            query_plan_admitted_count: usize::from(
                plan_projection.status()
                    == WorthGraphReadAccessSlicePlanProjectionStatus::QueryPlanAdmitted,
            ),
            query_receipt_observed_count: usize::from(
                receipt_projection.claims_graph_read_receipt(),
            ),
            query_execution_gap_count: usize::from(!receipt_projection.claims_graph_read_receipt()),
            local_graph_traversal_attempt_count: 0,
            local_adjacency_lookup_attempt_count: 0,
            local_broad_scan_attempt_count: 0,
            local_receipt_fabrication_attempt_count: 0,
        }
    }

    pub const fn selected_slice_count(&self) -> usize {
        self.selected_slice_count
    }

    pub const fn query_plan_projection_attempt_count(&self) -> usize {
        self.query_plan_projection_attempt_count
    }

    pub const fn query_plan_admitted_count(&self) -> usize {
        self.query_plan_admitted_count
    }

    pub const fn query_receipt_observed_count(&self) -> usize {
        self.query_receipt_observed_count
    }

    pub const fn query_execution_gap_count(&self) -> usize {
        self.query_execution_gap_count
    }

    pub const fn local_graph_traversal_attempt_count(&self) -> usize {
        self.local_graph_traversal_attempt_count
    }

    pub const fn local_adjacency_lookup_attempt_count(&self) -> usize {
        self.local_adjacency_lookup_attempt_count
    }

    pub const fn local_broad_scan_attempt_count(&self) -> usize {
        self.local_broad_scan_attempt_count
    }

    pub const fn local_receipt_fabrication_attempt_count(&self) -> usize {
        self.local_receipt_fabrication_attempt_count
    }
}
