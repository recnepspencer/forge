#![cfg(test)]

use worth_ui_inspection::{
    UiInspectionMeasurementDenialPosture, UiInspectionMeasurementFailureSource,
    UiInspectionMeasurementQueryFactFamily, UiInspectionTarget,
};

use crate::evidence::measurement::projection::fact_test_support::{
    capability_report, entity_identity_projection_context, host_result_font_metrics,
};

use super::inspection_bridge::UiMeasurementInspectionEvidenceBundle;
use super::measurement_inspection_test_support::{
    graph_node_identity, measurement_detail, measurement_query, query_measurement_app_in_world,
};

#[test]
fn graph_node_measurement_inspection_reports_unavailable_fact_families_on_public_lane() {
    let (_, consumption, world_profile) =
        entity_identity_projection_context("measurement-inspection-missing-families");
    let host_report = capability_report(23);
    let host_result = host_result_font_metrics(
        71,
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
        Some(UiInspectionMeasurementDenialPosture::UnavailableFactFamilies {
            available_families,
            missing_families,
        }) => {
            assert!(available_families.is_empty());
            assert_eq!(
                missing_families.as_ref(),
                &[UiInspectionMeasurementQueryFactFamily::ScrollContentExtent]
            );
        }
        other => panic!("expected unavailable-fact-families denial, got {other:?}"),
    }
}
