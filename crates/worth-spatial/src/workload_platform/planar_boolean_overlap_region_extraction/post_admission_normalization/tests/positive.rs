use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanOverlapRegionCanonicalWindingSourceKind, PlanarBooleanPostAdmissionNormalizationBundle,
    PlanarBooleanPostAdmissionNormalizationInput,
};

use super::support::{
    admitted_post_admission_bundle, admitted_region_candidate_bundle, boundary_only_region_candidate_bundle,
    canonical_winding_graph, canonical_winding_set, payload_permuted_region_candidate_bundle,
    replayed_inputs,
};

#[test]
fn post_admission_canonical_winding_is_replay_stable() {
    let (canonical, replayed) = replayed_inputs();

    assert_eq!(
        canonical.normalize_post_admission_canonical_winding(),
        replayed.normalize_post_admission_canonical_winding(),
    );
}

#[test]
fn admitted_regions_receive_canonical_winding_products() {
    let bundle = admitted_post_admission_bundle(&canonical_winding_graph());
    let row = &canonical_winding_set(&bundle).rows()[0];

    assert_eq!(row.source_kind(), PlanarBooleanOverlapRegionCanonicalWindingSourceKind::AdmittedRegion);
    assert!(row.canonical_winding_sign().is_some());
    assert!(!row.canonical_boundary_segment_identities().is_empty());
    assert!(!row.canonical_source_loop_identities().is_empty());
}

#[test]
fn boundary_only_outcomes_are_carried_into_the_canonical_product_without_area_winding() {
    let bundle = boundary_only_region_candidate_bundle()
        .normalize_post_admission_canonical_winding()
        .expect("boundary-only fixture should still canonicalize");
    let row = &canonical_winding_set(&bundle).rows()[0];

    assert_eq!(row.source_kind(), PlanarBooleanOverlapRegionCanonicalWindingSourceKind::BoundaryOnlyOutcome);
    assert_eq!(row.canonical_winding_sign(), None);
    assert_eq!(row.canonical_operand_side(), None);
}

#[test]
fn post_admission_bundle_is_the_ordinary_phase_twelve_lowering_surface() {
    let region_candidate_bundle = admitted_region_candidate_bundle(&canonical_winding_graph());
    let direct = PlanarBooleanPostAdmissionNormalizationBundle::admit(
        PlanarBooleanPostAdmissionNormalizationInput::new(&region_candidate_bundle),
    )
    .expect("direct phase-twelve admission should succeed");
    let ordinary = region_candidate_bundle
        .normalize_post_admission_canonical_winding()
        .expect("ordinary phase-twelve seam should succeed");

    assert_eq!(ordinary, direct);
}

#[test]
fn canonical_winding_consumes_explicit_ordered_witness_not_payload_vector_order() {
    let ordinary = admitted_region_candidate_bundle(&canonical_winding_graph())
        .normalize_post_admission_canonical_winding()
        .expect("ordinary admitted bundle should canonicalize");
    let payload_permuted = payload_permuted_region_candidate_bundle(&canonical_winding_graph())
        .normalize_post_admission_canonical_winding()
        .expect("payload permutation should not perturb canonical witness output");

    assert_eq!(ordinary, payload_permuted);
}
