use crate::workload_platform::planar_boolean_loop_reconstruction::test_support::LoopFixtureEntryOrder;
use crate::workload_platform::planar_boolean_overlap_region_extraction::arrangement_graph::cell_classification::tests::fixtures::{
    admitted_graph, inside_both_multi_boundary_graph, multi_cell_graph, permuted_multi_cell_graph,
};
use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanAreaOverlapComponentRow, PlanarBooleanAreaOverlapComponentSet,
    PlanarBooleanBoundaryContactClassificationBundle,
    PlanarBooleanBoundaryContactClassificationCounters, PlanarBooleanOverlapCellContainmentInput,
    PlanarBooleanOverlapCellContainmentMap, PlanarBooleanOverlapCellContainmentRow,
    PlanarBooleanOverlapCellWindingField, PlanarBooleanOverlapCellWindingFieldInput,
    PlanarBooleanOverlapCellWindingRow, PlanarBooleanOverlapIslandCandidateInput,
    PlanarBooleanOverlapIslandComponentBundle, PlanarBooleanPureBoundaryOnlyOutcomeRow,
    PlanarBooleanPureBoundaryOnlyOutcomeSet, PlanarBooleanSharedAreaAdmissionBundle,
    PlanarBooleanSharedBoundaryContactOutcomeRow, PlanarBooleanSharedBoundaryContactOutcomeSet,
};

pub(super) fn admitted_island_bundle(
    arrangement: &crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanCoplanarOverlapArrangementGraph,
) -> PlanarBooleanOverlapIslandComponentBundle {
    let containment = PlanarBooleanOverlapCellContainmentMap::admit(
        PlanarBooleanOverlapCellContainmentInput::from_arrangement(arrangement),
    )
    .expect("fixture arrangement should admit containment");
    let winding = PlanarBooleanOverlapCellWindingField::admit(
        PlanarBooleanOverlapCellWindingFieldInput::from_arrangement(arrangement, &containment),
    )
    .expect("fixture arrangement should admit winding");
    PlanarBooleanOverlapIslandComponentBundle::admit(
        PlanarBooleanOverlapIslandCandidateInput::from_cell_classification(
            arrangement,
            &containment,
            &winding,
        ),
    )
    .expect("fixture arrangement should admit island component bundle")
}

pub(super) fn admitted_boundary_contact_bundle(
    arrangement: &crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanCoplanarOverlapArrangementGraph,
) -> (
    PlanarBooleanBoundaryContactClassificationBundle,
    PlanarBooleanOverlapCellContainmentMap,
    PlanarBooleanOverlapCellWindingField,
) {
    let containment = PlanarBooleanOverlapCellContainmentMap::admit(
        PlanarBooleanOverlapCellContainmentInput::from_arrangement(arrangement),
    )
    .expect("fixture arrangement should admit containment");
    let winding = PlanarBooleanOverlapCellWindingField::admit(
        PlanarBooleanOverlapCellWindingFieldInput::from_arrangement(arrangement, &containment),
    )
    .expect("fixture arrangement should admit winding");
    let island_bundle = PlanarBooleanOverlapIslandComponentBundle::admit(
        PlanarBooleanOverlapIslandCandidateInput::from_cell_classification(
            arrangement,
            &containment,
            &winding,
        ),
    )
    .expect("fixture arrangement should admit island component bundle");
    let boundary_bundle = island_bundle
        .classify_boundary_contact_components()
        .expect("fixture island component bundle should admit boundary classification");

    (boundary_bundle, containment, winding)
}

pub(super) fn admitted_shared_area_bundle(
    arrangement: &crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanCoplanarOverlapArrangementGraph,
) -> PlanarBooleanSharedAreaAdmissionBundle {
    let (boundary_bundle, containment, winding) = admitted_boundary_contact_bundle(arrangement);
    boundary_bundle
        .admit_shared_area_components(&containment, &winding)
        .expect("fixture bundle should admit shared area classification")
}

pub(super) fn area_graph()
-> crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanCoplanarOverlapArrangementGraph {
    inside_both_multi_boundary_graph()
}

pub(super) fn boundary_graph()
-> crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanCoplanarOverlapArrangementGraph {
    multi_cell_graph()
}

pub(super) fn permuted_boundary_graph()
-> crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanCoplanarOverlapArrangementGraph {
    permuted_multi_cell_graph()
}

pub(super) fn replayed_real_arrangements() -> (
    crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanCoplanarOverlapArrangementGraph,
    crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanCoplanarOverlapArrangementGraph,
) {
    (
        admitted_graph(LoopFixtureEntryOrder::Canonical),
        admitted_graph(LoopFixtureEntryOrder::Replayed),
    )
}

pub(super) fn synthetic_mixed_boundary_bundle(
    overlapping_boundary_cell: bool,
) -> (
    PlanarBooleanBoundaryContactClassificationBundle,
    PlanarBooleanOverlapCellContainmentMap,
    PlanarBooleanOverlapCellWindingField,
) {
    let (boundary_bundle, containment, winding) = admitted_boundary_contact_bundle(&area_graph());
    let component = &boundary_bundle.area_overlap_components().rows()[0];
    let boundary_cell_identity = if overlapping_boundary_cell {
        component.cell_identities()[0].clone()
    } else {
        "synthetic-boundary-cell".to_string()
    };
    let shared_boundary_outcomes = PlanarBooleanSharedBoundaryContactOutcomeSet::new(
        "synthetic-shared-boundary-outcomes".to_string(),
        boundary_bundle.request_identity().to_string(),
        boundary_bundle.arrangement_graph_identity().to_string(),
        boundary_bundle.cell_set_identity().to_string(),
        boundary_bundle.ordering_basis_identity().to_string(),
        vec![PlanarBooleanSharedBoundaryContactOutcomeRow::new(
            "synthetic-shared-boundary-outcome".to_string(),
            component.island_identity().to_string(),
            component.neighborhood_identity().to_string(),
            "synthetic-boundary-contact-component".to_string(),
            vec![boundary_cell_identity],
            vec!["synthetic-boundary-component".to_string()],
            vec!["synthetic-boundary-segment".to_string()],
            vec!["synthetic-boundary-loop".to_string()],
        )],
    );
    let pure_boundary_only_outcomes = PlanarBooleanPureBoundaryOnlyOutcomeSet::new(
        "synthetic-pure-boundary-only-outcomes".to_string(),
        boundary_bundle.request_identity().to_string(),
        boundary_bundle.arrangement_graph_identity().to_string(),
        boundary_bundle.cell_set_identity().to_string(),
        boundary_bundle.ordering_basis_identity().to_string(),
        Vec::new(),
    );

    (
        PlanarBooleanBoundaryContactClassificationBundle::new(
            "synthetic-mixed-boundary-contact-classification".to_string(),
            shared_boundary_outcomes,
            pure_boundary_only_outcomes,
            boundary_bundle.area_overlap_components().clone(),
            PlanarBooleanBoundaryContactClassificationCounters::default(),
        ),
        containment,
        winding,
    )
}

pub(super) fn synthetic_boundary_only_promotion_bundle(
) -> (
    PlanarBooleanBoundaryContactClassificationBundle,
    PlanarBooleanOverlapCellContainmentMap,
    PlanarBooleanOverlapCellWindingField,
) {
    let (boundary_bundle, containment, winding) = admitted_boundary_contact_bundle(&boundary_graph());
    let boundary_only_row = &boundary_bundle.pure_boundary_only_outcomes().rows()[0];
    let promoted_area_components = PlanarBooleanAreaOverlapComponentSet::new(
        "synthetic-promoted-area-components".to_string(),
        boundary_bundle.request_identity().to_string(),
        boundary_bundle.arrangement_graph_identity().to_string(),
        boundary_bundle.cell_set_identity().to_string(),
        boundary_bundle.ordering_basis_identity().to_string(),
        vec![PlanarBooleanAreaOverlapComponentRow::new(
            "synthetic-promoted-area-component".to_string(),
            boundary_only_row.island_identity().to_string(),
            boundary_only_row.neighborhood_identity().to_string(),
            boundary_only_row.cell_identities().to_vec(),
            vec!["synthetic-promoted-boundary-component".to_string()],
            vec!["synthetic-promoted-boundary-segment".to_string()],
            vec!["synthetic-promoted-boundary-loop".to_string()],
        )],
    );

    (
        PlanarBooleanBoundaryContactClassificationBundle::new(
            "synthetic-boundary-only-promotion".to_string(),
            boundary_bundle.shared_boundary_contact_outcomes().clone(),
            boundary_bundle.pure_boundary_only_outcomes().clone(),
            promoted_area_components,
            boundary_bundle.counters(),
        ),
        containment,
        winding,
    )
}

pub(super) fn synthetic_incoherent_area_cell_proof_bundle(
) -> (
    PlanarBooleanBoundaryContactClassificationBundle,
    PlanarBooleanOverlapCellContainmentMap,
    PlanarBooleanOverlapCellWindingField,
) {
    let (boundary_bundle, containment, winding) = admitted_boundary_contact_bundle(&area_graph());
    let component = &boundary_bundle.area_overlap_components().rows()[0];
    let component_cell = component.cell_identities()[0].as_str();

    let rewritten_containment = containment
        .rows()
        .iter()
        .map(|row| {
            if row.cell_identity() == component_cell {
                PlanarBooleanOverlapCellContainmentRow::new(
                    row.cell_identity().to_string(),
                    row.arrangement_identity().to_string(),
                    row.neighborhood_identity().to_string(),
                    row.operand_side(),
                    Some("synthetic-foreign-island".to_string()),
                    vec!["synthetic-foreign-loop".to_string()],
                    row.evidence_kind(),
                )
            } else {
                row.clone()
            }
        })
        .collect::<Vec<_>>();
    let rewritten_winding = winding
        .rows()
        .iter()
        .map(|row| {
            if row.cell_identity() == component_cell {
                PlanarBooleanOverlapCellWindingRow::new(
                    row.cell_identity().to_string(),
                    row.arrangement_identity().to_string(),
                    row.neighborhood_identity().to_string(),
                    row.operand_side(),
                    Some("synthetic-foreign-island".to_string()),
                    vec!["synthetic-foreign-loop".to_string()],
                    row.evidence_kind(),
                    row.winding_number(),
                )
            } else {
                row.clone()
            }
        })
        .collect::<Vec<_>>();

    (
        boundary_bundle,
        PlanarBooleanOverlapCellContainmentMap::new(
            containment.containment_map_identity().to_string(),
            containment.request_identity().to_string(),
            containment.arrangement_graph_identity().to_string(),
            containment.cell_set_identity().to_string(),
            containment.ordering_basis_identity().to_string(),
            rewritten_containment,
            containment.counters(),
        ),
        PlanarBooleanOverlapCellWindingField::new(
            winding.winding_field_identity().to_string(),
            winding.request_identity().to_string(),
            winding.arrangement_graph_identity().to_string(),
            winding.cell_set_identity().to_string(),
            winding.ordering_basis_identity().to_string(),
            rewritten_winding,
            winding.counters(),
        ),
    )
}
