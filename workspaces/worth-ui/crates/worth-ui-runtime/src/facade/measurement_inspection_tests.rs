#![cfg(test)]

use worth_ui_inspection::{
    UiInspectionMeasurementBasisPosture, UiInspectionMeasurementDenialPosture,
    UiInspectionMeasurementEvidenceSlot, UiInspectionMeasurementFailureSource,
    UiInspectionMeasurementGenerationCompatibility, UiInspectionMeasurementNeighborhoodClassHint,
    UiInspectionMeasurementQueryUnsupportedReason, UiInspectionSupportStatus, UiInspectionTarget,
};

use crate::evidence::measurement::projection::fact_test_support::{
    capability_report, display_field_projection_context, host_result_font_metrics,
    host_result_scroll_container_viewport, host_result_viewport_extent,
};

use super::inspection_bridge::UiMeasurementInspectionEvidenceBundle;
use super::measurement_inspection_test_support::{
    direct_measurement_view_for_graph_node, graph_node_identity, host_measurement_app,
    measurement_app_in_world, measurement_detail, measurement_query,
    query_measurement_app_in_world, query_measurement_package, repeated_instance_app,
};

#[test]
fn declared_surface_measurement_inspection_materializes_typed_detail() {
    let app = host_measurement_app();
    let query = measurement_query(UiInspectionTarget::declared_surface(
        "app/measurement_inspection.wui",
        0,
    ));

    let receipt = app.inspect(query);
    let view = measurement_detail(receipt.evidence_slice().expect("measurement slice"));

    assert_eq!(
        receipt.support_report().map(|report| report.status()),
        Some(UiInspectionSupportStatus::Supported)
    );
    assert_eq!(
        view.failure_source(),
        Some(UiInspectionMeasurementFailureSource::HostEvidence)
    );
    match view.denial_posture() {
        Some(UiInspectionMeasurementDenialPosture::MissingEvidence { slot }) => {
            assert_eq!(
                *slot,
                UiInspectionMeasurementEvidenceSlot::HostCapabilityReport
            );
        }
        other => panic!("expected host capability denial, got {other:?}"),
    }
}

#[test]
fn graph_node_measurement_inspection_reports_query_fact_failure_on_public_lane() {
    let (_, _, world_profile) = display_field_projection_context("measurement-inspection-query");
    let host_report = capability_report(11);
    let host_result = host_result_font_metrics(
        41,
        &host_report,
        worth_ui_inspection::UiEvidenceAuthorityGeneration::new(1),
    );
    let app = query_measurement_app_in_world(
        world_profile,
        Some(
            UiMeasurementInspectionEvidenceBundle::declared_surface(
                "app/measurement_inspection.wui",
                0,
            )
            .with_host_capability_report(host_report)
            .with_host_measurement_results([host_result]),
        ),
    );
    let graph_node_identity = graph_node_identity(&app);

    let receipt = app.inspect(measurement_query(UiInspectionTarget::graph_node_identity(
        graph_node_identity.digest(),
    )));
    let view = measurement_detail(receipt.evidence_slice().expect("measurement slice"));

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
}

#[test]
fn graph_node_measurement_inspection_success_matches_direct_runtime_projection() {
    let (_, consumption, world_profile) = display_field_projection_context("measurement-success");
    let host_report = capability_report(29);
    let host_result = host_result_font_metrics(
        91,
        &host_report,
        worth_ui_inspection::UiEvidenceAuthorityGeneration::new(1),
    );
    let scroll_container_viewport = host_result_scroll_container_viewport(
        92,
        &host_report,
        worth_ui_inspection::UiEvidenceAuthorityGeneration::new(1),
    );
    let viewport_extent = host_result_viewport_extent(
        93,
        &host_report,
        worth_ui_inspection::UiEvidenceAuthorityGeneration::new(1),
    );
    let bundle = UiMeasurementInspectionEvidenceBundle::declared_surface(
        "app/measurement_inspection.wui",
        0,
    )
    .with_query_authority(consumption)
    .with_host_capability_report(host_report)
    .with_host_measurement_results([host_result, scroll_container_viewport, viewport_extent]);
    let app = query_measurement_app_in_world(world_profile, Some(bundle.clone()));
    let graph_node_identity = graph_node_identity(&app);

    let receipt = app.inspect(measurement_query(UiInspectionTarget::graph_node_identity(
        graph_node_identity.digest(),
    )));
    let public_view = measurement_detail(receipt.evidence_slice().expect("measurement slice"));
    let direct_view = direct_measurement_view_for_graph_node(&app, &bundle, graph_node_identity);

    assert_eq!(public_view, &direct_view);
    assert_eq!(public_view.failure_source(), None);
    assert_eq!(
        public_view.basis_posture(),
        Some(UiInspectionMeasurementBasisPosture::QueryAndHost)
    );
    assert_eq!(public_view.basis_inputs().len(), 5);
    assert!(!public_view.dependency_lineage().is_empty());
    assert_eq!(
        public_view.neighborhood_class_hint(),
        Some(UiInspectionMeasurementNeighborhoodClassHint::ScrollContainerDependency)
    );
    assert_eq!(
        public_view.generation_compatibility(),
        Some(&UiInspectionMeasurementGenerationCompatibility::Compatible)
    );
    assert!(public_view.denial_posture().is_none());
}

#[test]
fn graph_node_measurement_inspection_reports_compatibility_on_public_lane() {
    let (_, consumption, world_profile) =
        display_field_projection_context("measurement-inspection-compatibility");
    let host_report = capability_report(17);
    let stale_host_result = host_result_font_metrics(
        51,
        &host_report,
        worth_ui_inspection::UiEvidenceAuthorityGeneration::new(2),
    );
    let app = query_measurement_app_in_world(
        world_profile,
        Some(
            UiMeasurementInspectionEvidenceBundle::declared_surface(
                "app/measurement_inspection.wui",
                0,
            )
            .with_query_authority(consumption)
            .with_host_capability_report(host_report)
            .with_host_measurement_results([stale_host_result]),
        ),
    );
    let graph_node_identity = graph_node_identity(&app);

    let receipt = app.inspect(measurement_query(UiInspectionTarget::graph_node_identity(
        graph_node_identity.digest(),
    )));
    let view = measurement_detail(receipt.evidence_slice().expect("measurement slice"));

    assert_eq!(
        view.failure_source(),
        Some(UiInspectionMeasurementFailureSource::CompatibilityMismatch)
    );
    match view.generation_compatibility() {
        Some(UiInspectionMeasurementGenerationCompatibility::StaleHostEvidence {
            expected,
            observed,
        }) => assert_eq!((*expected, *observed), (1, 2)),
        other => panic!("expected stale host evidence, got {other:?}"),
    }
    assert_eq!(view.basis_inputs().len(), 3);
}

#[test]
fn graph_node_measurement_inspection_reports_unsupported_query_posture_on_public_lane() {
    let (_, consumption, _) =
        display_field_projection_context("measurement-inspection-unsupported");
    let host_report = capability_report(19);
    let host_result = host_result_font_metrics(
        61,
        &host_report,
        worth_ui_inspection::UiEvidenceAuthorityGeneration::new(1),
    );
    let app = measurement_app_in_world(
        query_measurement_package(),
        crate::graph::UiGraphWorldProfile::authoritative(),
        Some(
            UiMeasurementInspectionEvidenceBundle::declared_surface(
                "app/measurement_inspection.wui",
                0,
            )
            .with_query_authority(consumption)
            .with_host_capability_report(host_report)
            .with_host_measurement_results([host_result]),
        ),
    );
    let graph_node_identity = graph_node_identity(&app);

    let receipt = app.inspect(measurement_query(UiInspectionTarget::graph_node_identity(
        graph_node_identity.digest(),
    )));
    let view = measurement_detail(receipt.evidence_slice().expect("measurement slice"));

    assert_eq!(
        view.failure_source(),
        Some(UiInspectionMeasurementFailureSource::QueryFacts)
    );
    match view.denial_posture() {
        Some(UiInspectionMeasurementDenialPosture::UnsupportedQueryPosture { reason }) => {
            assert_eq!(
                *reason,
                UiInspectionMeasurementQueryUnsupportedReason::MissingQueryPrerequisites
            );
        }
        other => panic!("expected unsupported query posture, got {other:?}"),
    }
}

#[test]
fn declared_surface_measurement_inspection_reports_ambiguous_instances_narrowly() {
    let app = repeated_instance_app();
    let receipt = app.inspect(measurement_query(UiInspectionTarget::declared_surface(
        "app/measurement_inspection.wui",
        0,
    )));
    let slice = receipt.evidence_slice().expect("measurement slice");
    let view = measurement_detail(slice);

    assert_eq!(
        view.failure_source(),
        Some(UiInspectionMeasurementFailureSource::DeclarationPosture)
    );
    match view.denial_posture() {
        Some(UiInspectionMeasurementDenialPosture::AmbiguousGraphNodeInstances {
            instance_count,
        }) => assert_eq!(*instance_count, 2),
        other => panic!("expected ambiguous-instance denial, got {other:?}"),
    }
    assert_eq!(slice.refs().len(), 1);
    assert!(view.basis_inputs().is_empty());
}
