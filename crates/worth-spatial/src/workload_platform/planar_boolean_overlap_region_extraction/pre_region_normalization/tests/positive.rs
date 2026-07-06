use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanPreRegionNormalizationBundle, PlanarBooleanPreRegionNormalizationInput,
};

use super::support::{
    admitted_pre_region_bundle, admitted_shared_area_bundle,
    chain_lineage_map_with_unrelated_conflict, operand_permuted_chain_lineage_map,
    replayed_shared_area_bundles, shared_area_graph, synthetic_chain_lineage_map,
};

#[test]
fn pre_region_normalization_is_replay_stable_for_real_shared_area_products() {
    let (canonical, replayed) = replayed_shared_area_bundles();
    let canonical_chain_map = synthetic_chain_lineage_map(&canonical, false, false, false);
    let replayed_chain_map = synthetic_chain_lineage_map(&replayed, false, false, false);

    assert_eq!(
        canonical.normalize_pre_region_coincidence(&canonical_chain_map),
        replayed.normalize_pre_region_coincidence(&replayed_chain_map),
    );
}

#[test]
fn pre_region_normalization_has_a_typed_product_before_region_promotion() {
    let normalization = admitted_pre_region_bundle(&shared_area_graph());
    assert_eq!(
        normalization
            .opposite_sense_overlap_normalizations()
            .rows()
            .len(),
        1
    );
}

#[test]
fn pre_region_normalization_bundle_is_the_ordinary_phase_ten_lowering_surface() {
    let shared_area_bundle = admitted_shared_area_bundle(&shared_area_graph());
    let chain_lineage_map = synthetic_chain_lineage_map(&shared_area_bundle, false, false, false);
    let direct = PlanarBooleanPreRegionNormalizationBundle::admit(
        PlanarBooleanPreRegionNormalizationInput::new(&shared_area_bundle, &chain_lineage_map),
    )
    .expect("phase-nine bundle should admit direct phase-ten normalization");
    let ordinary = shared_area_bundle
        .normalize_pre_region_coincidence(&chain_lineage_map)
        .expect("ordinary phase-ten seam should admit normalization");

    assert_eq!(ordinary, direct);
}

#[test]
fn canonical_orientation_is_stable_across_reversed_source_edge_sense_and_operand_order() {
    let shared_area_bundle = admitted_shared_area_bundle(&shared_area_graph());
    let canonical = shared_area_bundle
        .normalize_pre_region_coincidence(&synthetic_chain_lineage_map(
            &shared_area_bundle,
            false,
            false,
            false,
        ))
        .expect("fixture bundle should admit canonical normalization");
    let reversed = shared_area_bundle
        .normalize_pre_region_coincidence(&synthetic_chain_lineage_map(
            &shared_area_bundle,
            true,
            false,
            false,
        ))
        .expect("reversed source-edge sense should still normalize identically");
    let permuted = shared_area_bundle
        .normalize_pre_region_coincidence(&operand_permuted_chain_lineage_map(&shared_area_bundle))
        .expect("operand-order variation should still normalize identically");

    assert_eq!(canonical, reversed);
    assert_eq!(canonical, permuted);
}

#[test]
fn pre_region_normalization_ignores_unrelated_lineage_conflict_outside_component_local_witness() {
    let shared_area_bundle = admitted_shared_area_bundle(&shared_area_graph());
    let canonical = shared_area_bundle
        .normalize_pre_region_coincidence(&synthetic_chain_lineage_map(
            &shared_area_bundle,
            false,
            false,
            false,
        ))
        .expect("fixture bundle should admit canonical normalization");
    let with_unrelated_conflict = shared_area_bundle
        .normalize_pre_region_coincidence(&chain_lineage_map_with_unrelated_conflict(
            &shared_area_bundle,
        ))
        .expect("unrelated lineage conflict should not perturb localized normalization");

    assert_eq!(canonical, with_unrelated_conflict);
}

#[test]
fn pre_region_normalization_exposes_downstream_proof_surfaces() {
    let normalization = admitted_pre_region_bundle(&shared_area_graph());
    let set = normalization.opposite_sense_overlap_normalizations();
    let row = &set.rows()[0];

    assert!(!set.normalization_set_identity().is_empty());
    assert!(!set.request_identity().is_empty());
    assert!(!set.arrangement_graph_identity().is_empty());
    assert!(!set.cell_set_identity().is_empty());
    assert!(!set.ordering_basis_identity().is_empty());
    assert!(!row.chain_identities().is_empty());
    assert!(!row.fragment_identities().is_empty());
    assert!(!row.boundary_roles().is_empty());
}
