use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanOverlapRegionIdentityLineageBundle, PlanarBooleanOverlapRegionIdentityLineageInput,
};

use super::support::{
    boundary_only_bundle, canonical_graph, canonical_identity_map, identity_bundle,
    equivalent_multi_row_canonical_bundles_with_reversed_order, payload_permuted_canonical_bundle,
    replayed_inputs, same_row_count_distinct_identity_bundle,
};

#[test]
fn overlap_region_identity_lineage_is_replay_stable() {
    let (canonical, replayed) = replayed_inputs();

    assert_eq!(
        canonical.mint_overlap_region_identity_lineage(),
        replayed.mint_overlap_region_identity_lineage(),
    );
}

#[test]
fn admitted_regions_receive_stable_identity_rows() {
    let bundle = identity_bundle(&canonical_graph());
    let row = &canonical_identity_map(&bundle).rows()[0];

    assert!(!row.region_identity().is_empty());
    assert!(!row.canonical_boundary_segment_identities().is_empty());
    assert!(!row.canonical_source_loop_identities().is_empty());
}

#[test]
fn persistent_name_propagation_and_subshape_signature_are_typed_products() {
    let bundle = identity_bundle(&canonical_graph());

    assert!(!bundle.persistent_name_propagation_map().rows().is_empty());
    assert!(!bundle.subshape_signature_map().rows().is_empty());
}

#[test]
fn phase_thirteen_bundle_is_the_ordinary_lowering_surface() {
    let canonical = super::support::canonical_bundle(&canonical_graph());
    let direct = PlanarBooleanOverlapRegionIdentityLineageBundle::admit(
        PlanarBooleanOverlapRegionIdentityLineageInput::new(&canonical),
    )
    .expect("direct phase-thirteen admission should succeed");
    let ordinary = canonical
        .mint_overlap_region_identity_lineage()
        .expect("ordinary phase-thirteen seam should succeed");

    assert_eq!(ordinary, direct);
}

#[test]
fn boundary_only_outcomes_receive_identity_without_area_winding_authority() {
    let bundle = boundary_only_bundle()
        .mint_overlap_region_identity_lineage()
        .expect("boundary-only canonical bundle should admit identity minting");
    let row = &bundle.overlap_region_identity_map().rows()[0];

    assert_eq!(row.area_overlap_component_identity(), None);
    assert_eq!(row.canonical_winding_sign(), None);
}

#[test]
fn identity_lineage_uses_canonical_proof_not_unordered_payload_membership() {
    let ordinary = super::support::canonical_bundle(&canonical_graph())
        .mint_overlap_region_identity_lineage()
        .expect("ordinary canonical bundle should admit identity minting");
    let payload_permuted = payload_permuted_canonical_bundle(&canonical_graph())
        .mint_overlap_region_identity_lineage()
        .expect("payload permutation should not perturb identity minting");

    assert_eq!(ordinary, payload_permuted);
}

#[test]
fn typed_product_identities_change_when_same_cardinality_truth_changes() {
    let ordinary = identity_bundle(&canonical_graph());
    let distinct = same_row_count_distinct_identity_bundle(&canonical_graph())
        .mint_overlap_region_identity_lineage()
        .expect("same-cardinality distinct canonical truth should still admit identity minting");

    assert_ne!(
        ordinary.overlap_region_identity_map().map_identity(),
        distinct.overlap_region_identity_map().map_identity(),
    );
    assert_ne!(
        ordinary.persistent_name_propagation_map().map_identity(),
        distinct.persistent_name_propagation_map().map_identity(),
    );
    assert_ne!(
        ordinary.subshape_signature_map().map_identity(),
        distinct.subshape_signature_map().map_identity(),
    );
}

#[test]
fn multi_row_identity_products_ignore_equivalent_canonical_row_order_variation() {
    let (forward, reversed) =
        equivalent_multi_row_canonical_bundles_with_reversed_order(&canonical_graph());

    let forward_identity = forward
        .mint_overlap_region_identity_lineage()
        .expect("forward ordered canonical bundle should admit identity minting");
    let reversed_identity = reversed
        .mint_overlap_region_identity_lineage()
        .expect("reversed ordered canonical bundle should admit identity minting");

    assert_eq!(forward_identity, reversed_identity);
}
