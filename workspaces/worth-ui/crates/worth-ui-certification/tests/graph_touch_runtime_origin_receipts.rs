use worth_ui::facade::graph::{
    UiGraphTouchAspectPosture, UiGraphTouchAspects, UiGraphTouchDenial, UiGraphTouchOriginClass,
    UiGraphTouchTiming,
};
use worth_ui_test_support::{runtime_origin_fixture, WorthUiTouchOriginFixtureVariant};

#[test]
fn certification_runtime_origins_require_real_owner_receipts_and_deny_unrelated_targets() {
    let fixture = runtime_origin_fixture(WorthUiTouchOriginFixtureVariant::Baseline);
    let graph = fixture.app.graph();
    let control_id = fixture.control_graph_node_identity();
    let region_id = fixture.region_graph_node_identity();
    let aspects = UiGraphTouchAspects::new().structural(UiGraphTouchAspectPosture::Read);

    for (origin_class, origin) in [
        (
            UiGraphTouchOriginClass::HostObservation,
            graph
                .touches()
                .host_observation_receipt(fixture.runtime.inspect_active(), &fixture.inspection)
                .expect("host observation should admit from active runtime observation"),
        ),
        (
            UiGraphTouchOriginClass::ServiceEvent,
            graph
                .touches()
                .service_event_receipt(&fixture.frame_receipt, &fixture.inspection)
                .expect("service event should admit from ordinary frame receipt"),
        ),
        (
            UiGraphTouchOriginClass::IntentSubmission,
            graph
                .touches()
                .intent_submission_receipt(&fixture.intent_candidate)
                .expect("intent submission should admit from replacement candidate"),
        ),
        (
            UiGraphTouchOriginClass::DiagnosticOnly,
            graph
                .touches()
                .diagnostic_only_report_receipt(&fixture.diagnostic_report, &fixture.inspection)
                .expect("diagnostic-only should admit from diagnostic report and inspection"),
        ),
    ] {
        let admitted = graph
            .touches()
            .from_node(
                origin.clone(),
                runtime_timing(origin_class),
                control_id,
                aspects.clone(),
            )
            .expect("runtime-origin touch should target the authored control node");
        assert_eq!(admitted.origin().class(), origin_class);

        let denial = graph
            .touches()
            .from_node(
                origin,
                UiGraphTouchTiming::PostMutation,
                region_id,
                aspects.clone(),
            )
            .expect_err("runtime-origin touch must deny unrelated graph targets");
        assert!(matches!(
            denial,
            UiGraphTouchDenial::OriginDoesNotAuthorizeGraphNode {
                origin_class: denied_class,
                graph_node_identity,
            } if denied_class == origin_class && graph_node_identity == region_id
        ));
    }
}

fn runtime_timing(origin_class: UiGraphTouchOriginClass) -> UiGraphTouchTiming {
    match origin_class {
        UiGraphTouchOriginClass::HostObservation => UiGraphTouchTiming::ReactiveObservation,
        UiGraphTouchOriginClass::ServiceEvent => UiGraphTouchTiming::PostMutation,
        UiGraphTouchOriginClass::IntentSubmission => UiGraphTouchTiming::PreMutation,
        UiGraphTouchOriginClass::DiagnosticOnly => UiGraphTouchTiming::DiagnosticProjection,
        UiGraphTouchOriginClass::DeclarationChange | UiGraphTouchOriginClass::QueryFactChange => {
            unreachable!("only runtime-origin classes are covered here")
        }
    }
}
