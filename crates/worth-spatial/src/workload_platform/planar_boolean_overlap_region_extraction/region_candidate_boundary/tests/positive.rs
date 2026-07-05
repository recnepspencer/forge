use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanOverlapRegionCandidateBoundaryBundle, PlanarBooleanOverlapRegionCandidateBoundaryInput,
};

use super::support::{
    admitted_pre_region_bundle, admitted_region_candidate_bundle, admitted_shared_area_bundle,
    boundary_only_outcome_set, boundary_only_region_candidate_bundle, region_candidate_graph,
    replayed_inputs,
};

#[test]
fn overlap_region_candidate_promotion_is_replay_stable() {
    let (canonical_shared, canonical_pre, replayed_shared, replayed_pre) = replayed_inputs();

    assert_eq!(
        canonical_pre.promote_overlap_region_candidates(&canonical_shared),
        replayed_pre.promote_overlap_region_candidates(&replayed_shared),
    );
}

#[test]
fn overlap_region_candidate_promotion_exposes_candidate_and_admitted_products() {
    let bundle = admitted_region_candidate_bundle(&region_candidate_graph());

    assert_eq!(bundle.overlap_region_candidates().rows().len(), 1);
    assert_eq!(bundle.admitted_overlap_regions().rows().len(), 1);
}

#[test]
fn overlap_region_candidate_bundle_is_the_ordinary_phase_eleven_lowering_surface() {
    let shared_area_bundle = admitted_shared_area_bundle(&region_candidate_graph());
    let pre_region_bundle = admitted_pre_region_bundle(&region_candidate_graph());
    let direct = PlanarBooleanOverlapRegionCandidateBoundaryBundle::admit(
        PlanarBooleanOverlapRegionCandidateBoundaryInput::new(&pre_region_bundle, &shared_area_bundle),
    )
    .expect("direct phase-eleven admission should succeed");
    let ordinary = pre_region_bundle
        .promote_overlap_region_candidates(&shared_area_bundle)
        .expect("ordinary phase-eleven seam should succeed");

    assert_eq!(ordinary, direct);
}

#[test]
fn boundary_only_outcomes_remain_products_but_not_admitted_regions() {
    let bundle = boundary_only_region_candidate_bundle();
    let row = &boundary_only_outcome_set(&bundle).rows()[0];

    assert_eq!(boundary_only_outcome_set(&bundle).rows().len(), 1);
    assert_eq!(bundle.overlap_region_candidates().rows().len(), 0);
    assert_eq!(bundle.admitted_overlap_regions().rows().len(), 0);
    assert!(!row.boundary_component_identities().is_empty());
    assert!(!row.boundary_segment_identities().is_empty());
    assert!(!row.source_loop_identities().is_empty());
}
