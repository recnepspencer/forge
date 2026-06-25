use worth_ui::facade::{WorthUiAuthoredLiveViewDocument, WorthUiLiveViewProjectionAdmissionDenial};

use super::support::{invalid_action_source, prepared_app_with_live_view_source};

#[test]
fn invalid_action_projection_values_batch_in_source_order() {
    let app = prepared_app_with_live_view_source(invalid_action_source());
    let live_view = app.live_view_state_proof().expect("state binding admits");
    let document =
        WorthUiAuthoredLiveViewDocument::parse(&invalid_action_source()).expect("source parses");
    let declaration = document
        .declaration("validation.live_view.primitive_proof")
        .expect("live view declaration exists");
    let report = app
        .workbench()
        .runtime()
        .admit_authored_live_view_projections(&live_view, declaration)
        .expect_err("invalid action projections deny");
    let codes = report
        .denials()
        .iter()
        .map(WorthUiLiveViewProjectionAdmissionDenial::code)
        .collect::<Vec<_>>();
    assert_eq!(
        codes,
        vec![
            "live_view_readiness.invalid_id",
            "live_view_readiness.unknown_required_binding",
            "live_view_payload.unsupported_shape",
            "live_view_interaction.unsupported_kind",
            "live_view_interaction.unsupported_effect",
            "live_view_interaction.unknown_readiness",
            "live_view_interaction.unknown_payload",
        ]
    );
    assert_eq!(report.counters().readiness_count(), 1);
    assert_eq!(report.counters().payload_count(), 1);
    assert_eq!(report.counters().interaction_count(), 1);
    assert_eq!(report.counters().denial_count(), 7);
    assert!(report.denial_set_digest() > 0);
}
