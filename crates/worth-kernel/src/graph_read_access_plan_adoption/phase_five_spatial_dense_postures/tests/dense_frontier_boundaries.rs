use crate::graph_read_access_plan_adoption::{
    WorthGraphReadAccessPlanAdoptionPostureKind, WorthGraphReadAccessResolvedPosture,
};

use super::super::bounded_execution::build_bounded_execution_contract;
use super::super::query_posture_projection::project_spatial_dense_postures;
use super::super::query_posture_projection::WorthGraphReadAccessSpatialDensePostureOutcome;
use super::super::slice_classification::{
    classify_unresolved_slices, WorthGraphReadAccessUnresolvedSliceKind,
};

#[test]
fn dense_frontier_read_requires_explicit_dense_posture() {
    let posture = WorthGraphReadAccessResolvedPosture::for_tests(
        "dense_frontier_broad_boolean_requirement",
        WorthGraphReadAccessPlanAdoptionPostureKind::PersistentIndexRequired,
    );

    let slices = classify_unresolved_slices(&[posture]);
    let projections = project_spatial_dense_postures(&slices);
    let contract = build_bounded_execution_contract(&projections);

    assert_eq!(
        slices[0].kind(),
        WorthGraphReadAccessUnresolvedSliceKind::DenseFrontierRead
    );
    assert_eq!(contract.dense_or_broad_row_count(), 1);
    assert_eq!(contract.unbounded_ephemeral_index_count(), 0);
    assert_eq!(
        projections[0].outcome(),
        WorthGraphReadAccessSpatialDensePostureOutcome::RequiredQueryPosture
    );
}

#[test]
fn bounded_ephemeral_identity_text_does_not_create_dense_frontier_authority() {
    let posture = WorthGraphReadAccessResolvedPosture::for_tests(
        "dense_frontier_broad_boolean_requirement",
        WorthGraphReadAccessPlanAdoptionPostureKind::BoundedEphemeralIndexAdmitted,
    );

    let slices = classify_unresolved_slices(&[posture]);
    let projections = project_spatial_dense_postures(&slices);
    let contract = build_bounded_execution_contract(&projections);

    assert_eq!(
        slices[0].kind(),
        WorthGraphReadAccessUnresolvedSliceKind::KernelGraphRead
    );
    assert_eq!(contract.dense_or_broad_row_count(), 0);
    assert_eq!(contract.unbounded_ephemeral_index_count(), 0);
    assert_eq!(
        projections[0].outcome(),
        WorthGraphReadAccessSpatialDensePostureOutcome::AdmittedPlanRequiresExecutionReceipt
    );
    assert_eq!(projections[0].query_plan_digest(), None);
    assert_eq!(
        projections[0].claims_graph_read_receipt(),
        false,
        "bounded ephemeral posture cannot become receipt proof without counters"
    );
}
