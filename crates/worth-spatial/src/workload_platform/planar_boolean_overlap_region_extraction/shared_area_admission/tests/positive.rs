use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanOverlapCellContainmentInput, PlanarBooleanOverlapCellContainmentMap,
    PlanarBooleanOverlapCellWindingField, PlanarBooleanOverlapCellWindingFieldInput,
    PlanarBooleanSharedAreaAdmissionBundle, PlanarBooleanSharedAreaAdmissionInput,
};

use super::support::{
    admitted_boundary_contact_bundle, admitted_island_bundle, admitted_shared_area_bundle,
    area_graph, replayed_real_arrangements, synthetic_mixed_boundary_bundle,
};

#[test]
fn shared_area_admission_is_replay_stable_for_real_partition_products() {
    let (canonical, replayed) = replayed_real_arrangements();
    let canonical_bundle = admitted_shared_area_bundle(&canonical);
    let replayed_bundle = admitted_shared_area_bundle(&replayed);

    assert_eq!(canonical_bundle, replayed_bundle);
}

#[test]
fn shared_area_admission_keeps_shared_area_and_mixed_boundary_area_typed_separately() {
    let area_bundle = admitted_shared_area_bundle(&area_graph());
    assert_eq!(area_bundle.shared_area_admission_outcomes().rows().len(), 1);
    assert!(area_bundle.mixed_boundary_area_outcomes().rows().is_empty());
}

#[test]
fn shared_area_admission_bundle_is_the_ordinary_phase_nine_lowering_surface() {
    let arrangement = area_graph();
    let island_bundle = admitted_island_bundle(&arrangement);
    let boundary_bundle = island_bundle
        .classify_boundary_contact_components()
        .expect("fixture bundle should admit boundary classification");
    let containment = PlanarBooleanOverlapCellContainmentMap::admit(
        PlanarBooleanOverlapCellContainmentInput::from_arrangement(&arrangement),
    )
    .expect("fixture arrangement should admit containment");
    let winding = PlanarBooleanOverlapCellWindingField::admit(
        PlanarBooleanOverlapCellWindingFieldInput::from_arrangement(&arrangement, &containment),
    )
    .expect("fixture arrangement should admit winding");
    let direct = PlanarBooleanSharedAreaAdmissionBundle::admit(
        PlanarBooleanSharedAreaAdmissionInput::new(&boundary_bundle, &containment, &winding),
    )
    .expect("phase-eight bundle should admit direct phase-nine classification");
    let ordinary = boundary_bundle
        .admit_shared_area_components(&containment, &winding)
        .expect("ordinary phase-nine seam should admit shared area classification");

    assert_eq!(ordinary, direct);
}

#[test]
fn shared_area_admission_emits_mixed_boundary_area_for_separable_mixed_islands() {
    let (boundary_bundle, containment, winding) = synthetic_mixed_boundary_bundle(false);
    let mixed_bundle = boundary_bundle
        .admit_shared_area_components(&containment, &winding)
        .expect("disjoint mixed island should remain typed rather than denied");

    assert!(mixed_bundle.shared_area_admission_outcomes().rows().is_empty());
    assert_eq!(mixed_bundle.mixed_boundary_area_outcomes().rows().len(), 1);
}

#[test]
fn shared_area_admission_rows_expose_carried_lineage_for_downstream_consumers() {
    let area_bundle = admitted_shared_area_bundle(&area_graph());
    let shared_row = &area_bundle.shared_area_admission_outcomes().rows()[0];

    assert!(!shared_row.boundary_component_identities().is_empty());
    assert!(!shared_row.boundary_segment_identities().is_empty());
    assert!(!shared_row.source_loop_identities().is_empty());

    let (boundary_bundle, containment, winding) = synthetic_mixed_boundary_bundle(false);
    let mixed_bundle = boundary_bundle
        .admit_shared_area_components(&containment, &winding)
        .expect("disjoint mixed island should remain typed rather than denied");
    let mixed_row = &mixed_bundle.mixed_boundary_area_outcomes().rows()[0];

    assert!(!mixed_row.neighborhood_identity().is_empty());
    assert!(!mixed_row.cell_identities().is_empty());
}
