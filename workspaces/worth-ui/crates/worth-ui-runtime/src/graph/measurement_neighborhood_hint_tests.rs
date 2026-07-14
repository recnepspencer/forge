use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use crate::evidence::measurement::projection::fact_test_support::{
    capability_report, display_field_projection_context, host_result_font_metrics,
    host_result_viewport_extent, host_result_viewport_extent_with_value, scroll_viewport_policy,
    synthetic_declaration_identity,
};
use crate::evidence::{
    admit_measurement_basis, consume_declared_measurement_projection_facts,
    MeasurementEvidenceInput, UiMeasurementNeighborhoodClassHint,
};
use crate::graph::{UiGraphMeasurementNeighborhoodHint, UiGraphNodeIdentity};

#[test]
fn graph_measurement_neighborhood_hint_stays_basis_derived_and_node_local() {
    let generation = UiEvidenceAuthorityGeneration::new(17);
    let declaration_identity = synthetic_declaration_identity("graph-neighborhood");
    let (prerequisites, attempt, world_profile) =
        display_field_projection_context("graph-neighborhood");
    let receipt = consume_declared_measurement_projection_facts(
        declaration_identity.clone(),
        generation,
        &scroll_viewport_policy(),
        prerequisites,
        &attempt,
    )
    .expect("query receipt should admit");
    let capability_report = capability_report(77);
    let basis = admit_measurement_basis(
        declaration_identity,
        UiGraphNodeIdentity::new(201),
        world_profile,
        generation,
        &scroll_viewport_policy(),
        &[
            MeasurementEvidenceInput::query_projection_fact(&receipt),
            MeasurementEvidenceInput::host_capability_report(&capability_report),
            MeasurementEvidenceInput::host_measurement_result(&host_result_font_metrics(
                11,
                &capability_report,
                generation,
            )),
            MeasurementEvidenceInput::host_measurement_result(&host_result_viewport_extent(
                12,
                &capability_report,
                generation,
            )),
        ],
    );

    let hint = UiGraphMeasurementNeighborhoodHint::from_basis(&basis);

    assert_eq!(hint.graph_node_identity(), UiGraphNodeIdentity::new(201));
    assert_eq!(hint.basis_identity_digest(), basis.identity_digest());
    assert_eq!(
        hint.world_identity_digest(),
        basis.world_profile().identity_digest()
    );
    assert_eq!(hint.dependency_map(), basis.dependency_map());
    assert_eq!(
        hint.neighborhood_class_hint(),
        UiMeasurementNeighborhoodClassHint::ViewportDependency
    );
}

#[test]
fn graph_measurement_neighborhood_hint_identity_changes_when_dependency_map_changes() {
    let generation = UiEvidenceAuthorityGeneration::new(17);
    let declaration_identity = synthetic_declaration_identity("graph-neighborhood-drift");
    let (prerequisites, attempt, world_profile) =
        display_field_projection_context("graph-neighborhood-drift");
    let receipt = consume_declared_measurement_projection_facts(
        declaration_identity.clone(),
        generation,
        &scroll_viewport_policy(),
        prerequisites,
        &attempt,
    )
    .expect("query receipt should admit");
    let capability_report = capability_report(77);
    let stable_font_metrics = host_result_font_metrics(13, &capability_report, generation);
    let narrow_viewport =
        host_result_viewport_extent_with_value(14, &capability_report, generation, 100.0, 50.0);
    let wide_viewport =
        host_result_viewport_extent_with_value(14, &capability_report, generation, 240.0, 50.0);

    let narrow_basis = admit_measurement_basis(
        declaration_identity.clone(),
        UiGraphNodeIdentity::new(202),
        world_profile.clone(),
        generation,
        &scroll_viewport_policy(),
        &[
            MeasurementEvidenceInput::query_projection_fact(&receipt),
            MeasurementEvidenceInput::host_capability_report(&capability_report),
            MeasurementEvidenceInput::host_measurement_result(&stable_font_metrics),
            MeasurementEvidenceInput::host_measurement_result(&narrow_viewport),
        ],
    );
    let wide_basis = admit_measurement_basis(
        declaration_identity,
        UiGraphNodeIdentity::new(202),
        world_profile,
        generation,
        &scroll_viewport_policy(),
        &[
            MeasurementEvidenceInput::query_projection_fact(&receipt),
            MeasurementEvidenceInput::host_capability_report(&capability_report),
            MeasurementEvidenceInput::host_measurement_result(&stable_font_metrics),
            MeasurementEvidenceInput::host_measurement_result(&wide_viewport),
        ],
    );

    let narrow_hint = UiGraphMeasurementNeighborhoodHint::from_basis(&narrow_basis);
    let wide_hint = UiGraphMeasurementNeighborhoodHint::from_basis(&wide_basis);

    assert_ne!(
        narrow_basis.dependency_map().identity_digest(),
        wide_basis.dependency_map().identity_digest()
    );
    assert_ne!(narrow_hint.identity_digest(), wide_hint.identity_digest());
    assert_ne!(narrow_hint.dependency_map(), wide_hint.dependency_map());
    assert_eq!(
        narrow_hint.neighborhood_class_hint(),
        UiMeasurementNeighborhoodClassHint::ViewportDependency
    );
    assert_eq!(
        wide_hint.neighborhood_class_hint(),
        UiMeasurementNeighborhoodClassHint::ViewportDependency
    );
}
