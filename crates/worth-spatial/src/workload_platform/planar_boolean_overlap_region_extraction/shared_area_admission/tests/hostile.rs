use crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanSharedAreaAdmissionDenialKind;

use super::support::{
    synthetic_boundary_only_promotion_bundle, synthetic_incoherent_area_cell_proof_bundle,
    synthetic_mixed_boundary_bundle,
};

#[test]
fn shared_area_admission_rejects_mixed_islands_that_still_overlap_boundary_locality() {
    let (boundary_bundle, containment, winding) = synthetic_mixed_boundary_bundle(true);
    let denial = boundary_bundle
        .admit_shared_area_components(&containment, &winding)
        .expect_err("mixed island with overlapping boundary and area cells must deny");

    assert_eq!(
        denial.kind(),
        PlanarBooleanSharedAreaAdmissionDenialKind::MixedBoundaryAreaRequiresCellDecompositionDenied
    );
}

#[test]
fn shared_area_admission_rejects_boundary_only_promotion_into_positive_area() {
    let (boundary_bundle, containment, winding) = synthetic_boundary_only_promotion_bundle();
    let denial = boundary_bundle
        .admit_shared_area_components(&containment, &winding)
        .expect_err("pure-boundary-only islands cannot be promoted into shared-area admission");

    assert_eq!(
        denial.kind(),
        PlanarBooleanSharedAreaAdmissionDenialKind::ContradictoryIslandComponentMembershipDenied
    );
}

#[test]
fn shared_area_admission_rejects_incoherent_cell_proof_even_when_cell_ids_coincide() {
    let (boundary_bundle, containment, winding) = synthetic_incoherent_area_cell_proof_bundle();
    let denial = boundary_bundle
        .admit_shared_area_components(&containment, &winding)
        .expect_err("phase nine must deny cell-id coincidence without matching island-local containment and winding proof");

    assert_eq!(
        denial.kind(),
        PlanarBooleanSharedAreaAdmissionDenialKind::AreaComponentMissingSupportingCellProofDenied
    );
}
