use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanBoundaryContactClassificationBundle, PlanarBooleanBoundaryContactClassificationInput,
};

use super::support::{
    admitted_bundle, admitted_from_arrangement_area, admitted_from_arrangement_boundary,
    admitted_island_bundle, permuted_boundary_bundle, replayed_real_bundles,
};

#[test]
fn boundary_contact_classification_is_replay_stable_for_real_partition_products() {
    let (canonical, replayed) = replayed_real_bundles();
    let canonical_bundle = admitted_bundle(&canonical);
    let replayed_bundle = admitted_bundle(&replayed);

    assert_eq!(canonical_bundle, replayed_bundle);
}

#[test]
fn boundary_contact_classification_is_stable_under_benign_order_variation() {
    let (canonical, permuted) = permuted_boundary_bundle();
    let canonical_bundle = admitted_bundle(&canonical);
    let permuted_bundle = admitted_bundle(&permuted);

    assert_eq!(canonical_bundle, permuted_bundle);
}

#[test]
fn boundary_contact_classification_keeps_shared_boundary_and_pure_boundary_only_typed_separately() {
    let boundary_bundle = admitted_from_arrangement_boundary();
    assert_eq!(boundary_bundle.shared_boundary_contact_outcomes().rows().len(), 2);
    assert_eq!(boundary_bundle.pure_boundary_only_outcomes().rows().len(), 1);

    let area_bundle = admitted_from_arrangement_area();
    assert!(area_bundle.shared_boundary_contact_outcomes().rows().is_empty());
    assert!(area_bundle.pure_boundary_only_outcomes().rows().is_empty());
}

#[test]
fn boundary_contact_classification_bundle_is_the_ordinary_phase_eight_lowering_surface() {
    let island_bundle = admitted_island_bundle(&super::support::boundary_graph());
    let direct = PlanarBooleanBoundaryContactClassificationBundle::admit(
        PlanarBooleanBoundaryContactClassificationInput::new(
            island_bundle.overlap_islands(),
            island_bundle.boundary_contact_components(),
            island_bundle.area_overlap_components(),
        ),
    )
    .expect("phase-seven partition should admit direct phase-eight classification");
    let ordinary = admitted_bundle(&island_bundle);

    assert_eq!(ordinary, direct);
}
