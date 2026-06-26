use forge_query::facade::ForgeQueryGraphObligationSupportLane;

use super::super::{
    primitive_construction_birth_touch_descriptor, primitive_construction_graph_obligation_catalog,
    primitive_construction_graph_obligation_selector_coverage,
    primitive_construction_graph_obligation_support_matrix,
    primitive_construction_graph_obligation_support_pin,
    primitive_construction_phase_eighteen_family_count_gap,
    PHASE_EIGHTEEN_SPEC_PRIMITIVE_FAMILY_COUNT,
};
use crate::construction::request::PRIMITIVE_CONSTRUCTION_FAMILIES;

#[test]
fn kernel_construction_catalog_covers_current_primitive_family_set_exactly() {
    let catalog = primitive_construction_graph_obligation_catalog();
    let catalog_families = catalog
        .rows()
        .iter()
        .map(|row| row.family())
        .collect::<Vec<_>>();

    assert_eq!(catalog_families, PRIMITIVE_CONSTRUCTION_FAMILIES);
    assert_eq!(catalog.rows().len(), PRIMITIVE_CONSTRUCTION_FAMILIES.len());
    assert_eq!(PHASE_EIGHTEEN_SPEC_PRIMITIVE_FAMILY_COUNT, 7);
    assert_eq!(primitive_construction_phase_eighteen_family_count_gap(), 1);
    assert!(catalog
        .rows()
        .iter()
        .all(|row| row.descriptor_source().contains("compose")));
}

#[test]
fn kernel_construction_support_pin_matches_phase_eighteen_support_matrix() {
    primitive_construction_graph_obligation_support_pin()
        .evaluate(&primitive_construction_graph_obligation_support_matrix())
        .expect("primitive construction birth support pin should match matrix");
}

#[test]
fn kernel_construction_selector_coverage_matches_registration_selector() {
    let coverage = primitive_construction_graph_obligation_selector_coverage();
    let catalog = primitive_construction_graph_obligation_catalog();
    let selector_digest = primitive_construction_birth_touch_descriptor()
        .expect("descriptor")
        .descriptor_digest()
        .to_string();

    assert_eq!(coverage.row_count(), PRIMITIVE_CONSTRUCTION_FAMILIES.len());
    assert!(catalog.rows().iter().all(|row| {
        row.registration().support_posture().lane()
            == ForgeQueryGraphObligationSupportLane::GraphComposition
    }));
    assert!(selector_digest.starts_with("forge.query.evidence"));
}
