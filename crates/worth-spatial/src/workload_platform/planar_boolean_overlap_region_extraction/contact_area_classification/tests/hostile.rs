use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanAreaOverlapComponentRow, PlanarBooleanAreaOverlapComponentSet,
    PlanarBooleanBoundaryContactClassificationBundle, PlanarBooleanBoundaryContactClassificationDenialKind,
    PlanarBooleanBoundaryContactClassificationInput,
};

use super::support::permuted_boundary_bundle;

#[test]
fn boundary_contact_classification_rejects_mixed_islands_hidden_as_pure_boundary_only() {
    let island_bundle = permuted_boundary_bundle().0;
    let forged_area_component_set =
        PlanarBooleanAreaOverlapComponentSet::new(
            "forged-area-components".to_string(),
            island_bundle.overlap_islands().request_identity().to_string(),
            island_bundle.overlap_islands().arrangement_graph_identity().to_string(),
            island_bundle.overlap_islands().cell_set_identity().to_string(),
            island_bundle.overlap_islands().ordering_basis_identity().to_string(),
            vec![PlanarBooleanAreaOverlapComponentRow::new(
                "forged-area-component".to_string(),
                island_bundle.overlap_islands().rows()[0].island_identity().to_string(),
                island_bundle.overlap_islands().rows()[0].neighborhood_identity().to_string(),
                vec!["forged-area-cell".to_string()],
                vec!["forged-area-boundary".to_string()],
                vec!["forged-area-segment".to_string()],
                vec!["forged-area-loop".to_string()],
            )],
        );

    let denial = PlanarBooleanBoundaryContactClassificationBundle::admit(
        PlanarBooleanBoundaryContactClassificationInput::new(
            island_bundle.overlap_islands(),
            island_bundle.boundary_contact_components(),
            &forged_area_component_set,
        ),
    )
    .expect_err("mixed island hidden behind pure-boundary-only rhetoric must deny");

    assert_eq!(
        denial.kind(),
        PlanarBooleanBoundaryContactClassificationDenialKind::MixedBoundaryAreaRequiresCellDecompositionDenied
    );
}
