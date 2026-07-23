#![cfg(test)]

use worth_ui_inspection::{
    UiInspectionMeasurementBasisInput, UiInspectionMeasurementBasisPosture,
    UiInspectionMeasurementChildIntrinsicSource, UiInspectionMeasurementDenialPosture,
    UiInspectionMeasurementEvidenceSlot, UiInspectionMeasurementFailureSource,
    UiInspectionMeasurementGenerationCompatibility, UiInspectionMeasurementNeighborhoodClassHint,
    UiInspectionScope, UiInspectionScopeSupportRow, UiInspectionSupportWorld,
};

use super::fact_test_support::{
    capability_report, display_field_projection_context, host_result_font_metrics,
    host_result_viewport_extent, scroll_viewport_policy, synthetic_declaration_identity,
};
use crate::evidence::{
    admit_measurement_basis, consume_declared_measurement_projection_facts,
    project_measurement_inspection_view, MeasurementEvidenceInput,
};
use crate::graph::UiGraphNodeIdentity;

#[test]
fn measurement_inspection_view_reports_query_fact_failure_without_flattening() {
    let declaration_identity = synthetic_declaration_identity("measurement-inspection-query");
    let policy = scroll_viewport_policy();
    let support_report = supported_measurement_report();
    let host_report = capability_report(11);
    let font_metrics = host_result_font_metrics(
        41,
        &host_report,
        worth_ui_inspection::UiEvidenceAuthorityGeneration::new(7),
    );
    let basis = admit_measurement_basis(
        declaration_identity,
        UiGraphNodeIdentity::new(9),
        crate::graph::UiGraphWorldProfile::authoritative(),
        worth_ui_inspection::UiEvidenceAuthorityGeneration::new(7),
        &policy,
        &[
            MeasurementEvidenceInput::host_capability_report(&host_report),
            MeasurementEvidenceInput::host_measurement_result(&font_metrics),
        ],
    );

    let view = project_measurement_inspection_view(support_report, Some(&basis));

    assert_eq!(
        view.failure_source(),
        Some(UiInspectionMeasurementFailureSource::QueryFacts)
    );
    match view.denial_posture() {
        Some(UiInspectionMeasurementDenialPosture::MissingEvidence { slot }) => {
            assert_eq!(
                *slot,
                UiInspectionMeasurementEvidenceSlot::QueryProjectionFactReceipt
            );
        }
        other => panic!("expected query fact denial, got {other:?}"),
    }
    assert_eq!(view.basis_inputs().len(), 2);
    assert_eq!(view.dependency_lineage().len(), 1);
}

#[test]
fn measurement_inspection_view_preserves_generation_mismatch() {
    let declaration_identity = synthetic_declaration_identity("measurement-inspection-stale");
    let policy = scroll_viewport_policy();
    let support_report = supported_measurement_report();
    let (prerequisites, consumption, world_profile) =
        display_field_projection_context("measurement-inspection-stale");
    let projection_receipt = consume_declared_measurement_projection_facts(
        declaration_identity.clone(),
        worth_ui_inspection::UiEvidenceAuthorityGeneration::new(3),
        &policy,
        prerequisites,
        &consumption,
    )
    .expect("projection receipt should admit");
    let host_report = capability_report(17);
    let font_metrics = host_result_font_metrics(
        51,
        &host_report,
        worth_ui_inspection::UiEvidenceAuthorityGeneration::new(4),
    );
    let basis = admit_measurement_basis(
        declaration_identity,
        UiGraphNodeIdentity::new(11),
        world_profile,
        worth_ui_inspection::UiEvidenceAuthorityGeneration::new(4),
        &policy,
        &[
            MeasurementEvidenceInput::settled_query_fact(&projection_receipt),
            MeasurementEvidenceInput::host_capability_report(&host_report),
            MeasurementEvidenceInput::host_measurement_result(&font_metrics),
        ],
    );

    let view = project_measurement_inspection_view(support_report, Some(&basis));

    assert_eq!(
        view.failure_source(),
        Some(UiInspectionMeasurementFailureSource::CompatibilityMismatch)
    );
    match view.generation_compatibility() {
        Some(UiInspectionMeasurementGenerationCompatibility::StaleQueryFactReceipt {
            expected,
            observed,
        }) => {
            assert_eq!((*expected, *observed), (4, 3));
        }
        other => panic!("expected stale query fact compatibility, got {other:?}"),
    }
}

#[test]
fn measurement_inspection_view_materializes_successful_basis_lineage() {
    let declaration_identity = synthetic_declaration_identity("measurement-inspection-success");
    let policy = scroll_viewport_policy();
    let support_report = supported_measurement_report();
    let (prerequisites, consumption, world_profile) =
        display_field_projection_context("measurement-inspection-success");
    let projection_receipt = consume_declared_measurement_projection_facts(
        declaration_identity.clone(),
        worth_ui_inspection::UiEvidenceAuthorityGeneration::new(5),
        &policy,
        prerequisites,
        &consumption,
    )
    .expect("projection receipt should admit");
    let host_report = capability_report(23);
    let font_metrics = host_result_font_metrics(
        81,
        &host_report,
        worth_ui_inspection::UiEvidenceAuthorityGeneration::new(5),
    );
    let viewport_extent = host_result_viewport_extent(
        82,
        &host_report,
        worth_ui_inspection::UiEvidenceAuthorityGeneration::new(5),
    );
    let basis = admit_measurement_basis(
        declaration_identity,
        UiGraphNodeIdentity::new(17),
        world_profile,
        worth_ui_inspection::UiEvidenceAuthorityGeneration::new(5),
        &policy,
        &[
            MeasurementEvidenceInput::settled_query_fact(&projection_receipt),
            MeasurementEvidenceInput::host_capability_report(&host_report),
            MeasurementEvidenceInput::host_measurement_result(&font_metrics),
            MeasurementEvidenceInput::host_measurement_result(&viewport_extent),
        ],
    );

    let view = project_measurement_inspection_view(support_report, Some(&basis));

    assert_eq!(view.failure_source(), None);
    assert_eq!(
        view.basis_posture(),
        Some(UiInspectionMeasurementBasisPosture::QueryAndHost)
    );
    assert_eq!(view.basis_inputs().len(), 4);
    assert!(!view.dependency_lineage().is_empty());
    assert_eq!(
        view.neighborhood_class_hint(),
        Some(UiInspectionMeasurementNeighborhoodClassHint::ViewportDependency)
    );
    assert_eq!(
        view.generation_compatibility(),
        Some(&UiInspectionMeasurementGenerationCompatibility::Compatible)
    );
    assert!(view.denial_posture().is_none());
}

#[test]
fn measurement_inspection_view_preserves_child_intrinsic_identity() {
    let declaration_identity = synthetic_declaration_identity("measurement-inspection-child");
    let policy = scroll_viewport_policy();
    let support_report = supported_measurement_report();
    let child_node = UiGraphNodeIdentity::new(31);
    let (prerequisites, consumption, world_profile) =
        display_field_projection_context("measurement-inspection-child");
    let projection_receipt = consume_declared_measurement_projection_facts(
        declaration_identity.clone(),
        worth_ui_inspection::UiEvidenceAuthorityGeneration::new(9),
        &policy,
        prerequisites,
        &consumption,
    )
    .expect("projection receipt should admit");
    let basis = admit_measurement_basis(
        declaration_identity,
        UiGraphNodeIdentity::new(29),
        world_profile,
        worth_ui_inspection::UiEvidenceAuthorityGeneration::new(9),
        &policy,
        &[MeasurementEvidenceInput::child_query_projection_fact(
            child_node,
            &projection_receipt,
        )],
    );

    let view = project_measurement_inspection_view(support_report, Some(&basis));

    assert_eq!(view.basis_inputs().len(), 1);
    match &view.basis_inputs()[0] {
        UiInspectionMeasurementBasisInput::ChildIntrinsicMeasurement {
            contributor_graph_node_identity_digest,
            source,
            ..
        } => {
            assert_eq!(*contributor_graph_node_identity_digest, child_node.digest());
            assert_eq!(
                *source,
                UiInspectionMeasurementChildIntrinsicSource::QueryProjectionFact
            );
        }
        other => panic!("expected child intrinsic basis input, got {other:?}"),
    }
}

fn supported_measurement_report() -> worth_ui_inspection::UiInspectionSupportReport {
    let rows = [UiInspectionScopeSupportRow::supported(
        "inspection",
        UiInspectionScope::Measurement,
        UiInspectionSupportWorld::Authoritative,
    )];
    worth_ui_inspection::UiInspectionSupportReport::from_scope_rows(
        UiInspectionScope::Measurement,
        &rows,
    )
}
