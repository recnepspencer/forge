use super::bounded_execution::WorthGraphReadAccessBoundedExecutionContract;
use super::query_posture_projection::{
    WorthGraphReadAccessSpatialDensePostureOutcome,
    WorthGraphReadAccessSpatialDensePostureProjection,
};
use super::slice_classification::WorthGraphReadAccessUnresolvedSliceKind;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessSpatialDensePostureCounters {
    unresolved_slice_count: usize,
    spatial_slice_count: usize,
    dense_frontier_slice_count: usize,
    broad_boolean_slice_count: usize,
    kernel_slice_count: usize,
    unknown_covered_slice_count: usize,
    required_posture_count: usize,
    denied_posture_count: usize,
    carried_gap_count: usize,
    admitted_plan_requires_receipt_count: usize,
    receipt_claim_count: usize,
    dense_or_broad_row_count: usize,
    unbounded_ephemeral_index_count: usize,
    scalarized_caller_loop_count: usize,
}

impl WorthGraphReadAccessSpatialDensePostureCounters {
    pub(crate) fn from_products(
        projections: &[WorthGraphReadAccessSpatialDensePostureProjection],
        bounded_contract: &WorthGraphReadAccessBoundedExecutionContract,
        scalarized_caller_loop_count: usize,
    ) -> Self {
        Self {
            unresolved_slice_count: projections.len(),
            spatial_slice_count: count_slice_kind(
                projections,
                WorthGraphReadAccessUnresolvedSliceKind::SpatialGraphRead,
            ),
            dense_frontier_slice_count: count_slice_kind(
                projections,
                WorthGraphReadAccessUnresolvedSliceKind::DenseFrontierRead,
            ),
            broad_boolean_slice_count: count_slice_kind(
                projections,
                WorthGraphReadAccessUnresolvedSliceKind::BroadBooleanPredicateRead,
            ),
            kernel_slice_count: count_slice_kind(
                projections,
                WorthGraphReadAccessUnresolvedSliceKind::KernelGraphRead,
            ),
            unknown_covered_slice_count: count_slice_kind(
                projections,
                WorthGraphReadAccessUnresolvedSliceKind::UnknownCoveredGraphRead,
            ),
            required_posture_count: projections
                .iter()
                .filter(|projection| {
                    projection.outcome()
                        == WorthGraphReadAccessSpatialDensePostureOutcome::RequiredQueryPosture
                })
                .count(),
            denied_posture_count: projections
                .iter()
                .filter(|projection| {
                    projection.outcome()
                        == WorthGraphReadAccessSpatialDensePostureOutcome::DeniedByQueryPosture
                })
                .count(),
            carried_gap_count: projections
                .iter()
                .filter(|projection| {
                    projection.outcome()
                        == WorthGraphReadAccessSpatialDensePostureOutcome::CarriedCapabilityGap
                })
                .count(),
            admitted_plan_requires_receipt_count: projections
                .iter()
                .filter(|projection| {
                    projection.outcome()
                        == WorthGraphReadAccessSpatialDensePostureOutcome::AdmittedPlanRequiresExecutionReceipt
                })
                .count(),
            receipt_claim_count: projections
                .iter()
                .filter(|projection| projection.claims_graph_read_receipt())
                .count(),
            dense_or_broad_row_count: bounded_contract.dense_or_broad_row_count(),
            unbounded_ephemeral_index_count: bounded_contract.unbounded_ephemeral_index_count(),
            scalarized_caller_loop_count,
        }
    }

    pub const fn unresolved_slice_count(&self) -> usize {
        self.unresolved_slice_count
    }

    pub const fn spatial_slice_count(&self) -> usize {
        self.spatial_slice_count
    }

    pub const fn dense_frontier_slice_count(&self) -> usize {
        self.dense_frontier_slice_count
    }

    pub const fn broad_boolean_slice_count(&self) -> usize {
        self.broad_boolean_slice_count
    }

    pub const fn kernel_slice_count(&self) -> usize {
        self.kernel_slice_count
    }

    pub const fn unknown_covered_slice_count(&self) -> usize {
        self.unknown_covered_slice_count
    }

    pub const fn required_posture_count(&self) -> usize {
        self.required_posture_count
    }

    pub const fn denied_posture_count(&self) -> usize {
        self.denied_posture_count
    }

    pub const fn carried_gap_count(&self) -> usize {
        self.carried_gap_count
    }

    pub const fn admitted_plan_requires_receipt_count(&self) -> usize {
        self.admitted_plan_requires_receipt_count
    }

    pub const fn receipt_claim_count(&self) -> usize {
        self.receipt_claim_count
    }

    pub const fn dense_or_broad_row_count(&self) -> usize {
        self.dense_or_broad_row_count
    }

    pub const fn unbounded_ephemeral_index_count(&self) -> usize {
        self.unbounded_ephemeral_index_count
    }

    pub const fn scalarized_caller_loop_count(&self) -> usize {
        self.scalarized_caller_loop_count
    }
}

fn count_slice_kind(
    projections: &[WorthGraphReadAccessSpatialDensePostureProjection],
    kind: WorthGraphReadAccessUnresolvedSliceKind,
) -> usize {
    projections
        .iter()
        .filter(|projection| projection.slice_kind() == kind)
        .count()
}
