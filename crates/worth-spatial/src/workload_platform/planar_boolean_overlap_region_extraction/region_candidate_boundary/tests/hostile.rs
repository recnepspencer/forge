use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanDeniedOverlapRegionCandidateKind::{
        ContradictoryPromotionPostureDenied, MissingNormalizationDenied,
        MixedBoundaryAreaRequiresFurtherDecompositionDenied,
    },
    PlanarBooleanOverlapRegionCandidateBoundaryDenialKind,
};

use super::support::{
    admitted_shared_area_bundle, denied_candidate_set, duplicate_normalization_bundle,
    missing_normalization_bundle, mixed_boundary_disjoint_shared_area_bundle,
    mixed_boundary_shared_area_bundle, orphan_normalization_bundle, region_candidate_graph,
};

#[test]
fn missing_normalization_yields_a_denied_candidate_before_region_admission() {
    let shared_area_bundle = admitted_shared_area_bundle(&region_candidate_graph());
    let pre_region_bundle = missing_normalization_bundle(&shared_area_bundle);
    let bundle = pre_region_bundle
        .promote_overlap_region_candidates(&shared_area_bundle)
        .expect("missing normalization should deny locally, not fail the whole phase");

    assert_eq!(bundle.overlap_region_candidates().rows().len(), 0);
    assert_eq!(bundle.admitted_overlap_regions().rows().len(), 0);
    assert_eq!(denied_candidate_set(&bundle).rows().len(), 1);
    assert_eq!(
        denied_candidate_set(&bundle).rows()[0].denial_kind(),
        MissingNormalizationDenied
    );
}

#[test]
fn mixed_boundary_area_rows_stay_denied_candidates() {
    let shared_area_bundle = mixed_boundary_shared_area_bundle(&region_candidate_graph());
    let pre_region_bundle = super::support::admitted_pre_region_bundle(&region_candidate_graph());
    let bundle = pre_region_bundle
        .promote_overlap_region_candidates(&shared_area_bundle)
        .expect("mixed boundary area should deny locally, not fail the whole phase");

    assert_eq!(bundle.overlap_region_candidates().rows().len(), 1);
    assert_eq!(bundle.admitted_overlap_regions().rows().len(), 1);
    assert_eq!(denied_candidate_set(&bundle).rows().len(), 1);
    assert_eq!(
        denied_candidate_set(&bundle).rows()[0].denial_kind(),
        MixedBoundaryAreaRequiresFurtherDecompositionDenied
    );
}

#[test]
fn disjoint_mixed_boundary_area_rows_do_not_block_honest_shared_area_promotion() {
    let shared_area_bundle = mixed_boundary_disjoint_shared_area_bundle(&region_candidate_graph());
    let pre_region_bundle = super::support::admitted_pre_region_bundle(&region_candidate_graph());
    let bundle = pre_region_bundle
        .promote_overlap_region_candidates(&shared_area_bundle)
        .expect("disjoint mixed boundary area should not block honest shared-area promotion");

    assert_eq!(bundle.overlap_region_candidates().rows().len(), 1);
    assert_eq!(bundle.admitted_overlap_regions().rows().len(), 1);
    assert_eq!(denied_candidate_set(&bundle).rows().len(), 1);
    assert_eq!(
        denied_candidate_set(&bundle).rows()[0].denial_kind(),
        MixedBoundaryAreaRequiresFurtherDecompositionDenied
    );
}

#[test]
fn duplicate_normalization_rows_deny_contradictory_promotion_posture() {
    let shared_area_bundle = admitted_shared_area_bundle(&region_candidate_graph());
    let pre_region_bundle = duplicate_normalization_bundle(&shared_area_bundle);
    let bundle = pre_region_bundle
        .promote_overlap_region_candidates(&shared_area_bundle)
        .expect("duplicate normalization rows should deny locally, not fail the whole phase");

    assert_eq!(bundle.overlap_region_candidates().rows().len(), 0);
    assert_eq!(bundle.admitted_overlap_regions().rows().len(), 0);
    assert_eq!(denied_candidate_set(&bundle).rows().len(), 1);
    assert_eq!(
        denied_candidate_set(&bundle).rows()[0].denial_kind(),
        ContradictoryPromotionPostureDenied
    );
}

#[test]
fn orphan_normalization_rows_fail_the_phase_boundary() {
    let shared_area_bundle = admitted_shared_area_bundle(&region_candidate_graph());
    let pre_region_bundle = orphan_normalization_bundle(&shared_area_bundle);
    let denial = pre_region_bundle
        .promote_overlap_region_candidates(&shared_area_bundle)
        .expect_err("orphan normalization rows must fail the phase boundary");

    assert_eq!(
        denial.kind(),
        PlanarBooleanOverlapRegionCandidateBoundaryDenialKind::NormalizationSharedAreaMismatchDenied
    );
}
