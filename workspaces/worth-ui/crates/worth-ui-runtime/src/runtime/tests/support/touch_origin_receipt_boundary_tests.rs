use crate::obligations::touch::{
    UiGraphTouchAspectPosture, UiGraphTouchAspects, UiGraphTouchDenial, UiGraphTouchOriginClass,
    UiGraphTouchTiming,
};
use crate::certification_support::{
    runtime_origin_fixture, WorthUiTouchOriginFixtureVariant,
};

#[test]
fn owner_derived_touch_receipts_cover_runtime_origin_lanes_without_target_remixing() {
    let fixture = runtime_origin_fixture(WorthUiTouchOriginFixtureVariant::Baseline);
    let graph = fixture.app.graph();
    let control_id = fixture.control_graph_node_identity();
    let region_id = fixture.region_graph_node_identity();
    let aspects = UiGraphTouchAspects::new().structural(UiGraphTouchAspectPosture::Read);

    let host_origin = graph
        .touches()
        .host_observation_receipt(fixture.runtime.inspect_active(), &fixture.inspection)
        .expect("host observation should admit touch origin from active runtime observation");
    let service_origin = graph
        .touches()
        .service_event_receipt(&fixture.frame_receipt, &fixture.inspection)
        .expect("service event should admit touch origin from ordinary frame receipt");
    let intent_origin = graph
        .touches()
        .intent_submission_receipt(&fixture.intent_candidate)
        .expect("intent submission should admit touch origin from replacement candidate");
    let diagnostic_origin = graph
        .touches()
        .diagnostic_only_report_receipt(&fixture.diagnostic_report, &fixture.inspection)
        .expect("diagnostic report should admit touch origin from inspected authored provenance");

    let host_touch = graph
        .touches()
        .from_node(
            host_origin.clone(),
            UiGraphTouchTiming::ReactiveObservation,
            control_id,
            aspects.clone(),
        )
        .expect("host observation touch should target authored control node");
    let service_touch = graph
        .touches()
        .from_node(
            service_origin.clone(),
            UiGraphTouchTiming::PostMutation,
            control_id,
            aspects.clone(),
        )
        .expect("service event touch should target authored control node");
    let intent_touch = graph
        .touches()
        .from_node(
            intent_origin.clone(),
            UiGraphTouchTiming::PreMutation,
            control_id,
            aspects.clone(),
        )
        .expect("intent submission touch should target authored control node");
    let diagnostic_touch = graph
        .touches()
        .from_node(
            diagnostic_origin.clone(),
            UiGraphTouchTiming::DiagnosticProjection,
            control_id,
            aspects.clone(),
        )
        .expect("diagnostic touch should target authored control node");

    assert_eq!(
        host_touch.origin().class(),
        UiGraphTouchOriginClass::HostObservation
    );
    assert_eq!(
        service_touch.origin().class(),
        UiGraphTouchOriginClass::ServiceEvent
    );
    assert_eq!(
        intent_touch.origin().class(),
        UiGraphTouchOriginClass::IntentSubmission
    );
    assert_eq!(
        diagnostic_touch.origin().class(),
        UiGraphTouchOriginClass::DiagnosticOnly
    );

    for (origin, origin_class) in [
        (host_origin, UiGraphTouchOriginClass::HostObservation),
        (service_origin, UiGraphTouchOriginClass::ServiceEvent),
        (intent_origin, UiGraphTouchOriginClass::IntentSubmission),
        (diagnostic_origin, UiGraphTouchOriginClass::DiagnosticOnly),
    ] {
        let denial = graph
            .touches()
            .from_node(
                origin,
                UiGraphTouchTiming::PostMutation,
                region_id,
                aspects.clone(),
            )
            .expect_err("owner-derived runtime origin must deny unrelated graph node remixing");

        assert!(matches!(
            denial,
            UiGraphTouchDenial::OriginDoesNotAuthorizeGraphNode {
                origin_class: denied_class,
                graph_node_identity,
            } if denied_class == origin_class && graph_node_identity == region_id
        ));
    }
}

#[test]
fn runtime_origin_receipts_deny_cross_plan_or_cross_artifact_owner_mixing() {
    let fixture = runtime_origin_fixture(WorthUiTouchOriginFixtureVariant::Baseline);
    let same_artifact_different_plan =
        runtime_origin_fixture(WorthUiTouchOriginFixtureVariant::SameArtifactExtraPlanHook);
    let different_artifact =
        runtime_origin_fixture(WorthUiTouchOriginFixtureVariant::OverlayArtifact);
    let graph = fixture.app.graph();

    assert_owner_mismatch(
        graph.touches().host_observation_receipt(
            fixture.runtime.inspect_active(),
            &same_artifact_different_plan.inspection,
        ),
        UiGraphTouchOriginClass::HostObservation,
    );
    assert_owner_mismatch(
        graph.touches().service_event_receipt(
            &fixture.frame_receipt,
            &same_artifact_different_plan.inspection,
        ),
        UiGraphTouchOriginClass::ServiceEvent,
    );
    assert_owner_mismatch(
        graph.touches().diagnostic_only_report_receipt(
            &fixture.diagnostic_report,
            &same_artifact_different_plan.inspection,
        ),
        UiGraphTouchOriginClass::DiagnosticOnly,
    );

    assert_owner_mismatch(
        graph.touches().host_observation_receipt(
            fixture.runtime.inspect_active(),
            &different_artifact.inspection,
        ),
        UiGraphTouchOriginClass::HostObservation,
    );
    assert_owner_mismatch(
        graph
            .touches()
            .service_event_receipt(&fixture.frame_receipt, &different_artifact.inspection),
        UiGraphTouchOriginClass::ServiceEvent,
    );
    assert_owner_mismatch(
        graph.touches().diagnostic_only_report_receipt(
            &fixture.diagnostic_report,
            &different_artifact.inspection,
        ),
        UiGraphTouchOriginClass::DiagnosticOnly,
    );
}

fn assert_owner_mismatch(
    result: Result<crate::obligations::touch::UiGraphTouchOriginWitness, UiGraphTouchDenial>,
    origin_class: UiGraphTouchOriginClass,
) {
    assert_eq!(
        result.expect_err("owner-derived runtime origin must deny mismatched owner pairing"),
        UiGraphTouchDenial::OriginOwnerMismatch { origin_class }
    );
}
