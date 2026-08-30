use worth_ui::facade::app::{
    UiMountedFrameOutcome, UiMountedFrameRequest, UiPresentationDeadline, WorthUi,
};

#[test]
fn query_free_headless_app_executes_without_optional_subsystem_ceremony() {
    let app = WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .freeze()
        .map(
            worth_ui_runtime::facade::entry::WorthUiCertificationApplicationTransition::activate_headless,
        )
        .expect("an empty Query-free application prepares");
    let mut session = app
        .launch()
        .expect("an empty Query-free application launches");
    assert!(session.runtime_service_resource_census().is_empty());
    assert!(session.why_portal_closed().is_none());
    assert!(session.why_focus_moved().is_none());
    assert!(session.why_focus_restoration_failed().is_none());
    assert!(session.why_motion_interrupted().is_none());
    assert!(session.why_scroll_owner().is_none());
    assert!(session.why_selection_dropped().is_none());
    assert!(session.why_command_won().is_none());
    let outcome = session
        .execute_mounted_frame(
            UiMountedFrameRequest::all_bound_surfaces(),
            UiPresentationDeadline::at_tick(1),
            0,
            |_| {},
        )
        .unwrap_or_else(|_| panic!("the ordinary mounted-frame route executes headlessly"));

    let posture = match outcome {
        UiMountedFrameOutcome::Published(receipt) => {
            let _ = receipt.cost_report();
            "published"
        }
        UiMountedFrameOutcome::Unchanged(receipt) => {
            let _ = receipt.cost_report();
            "unchanged"
        }
        UiMountedFrameOutcome::Reconciled(receipt) => {
            let _ = receipt.cost_report();
            "reconciled"
        }
        UiMountedFrameOutcome::RejectedBeforeEffects(rejection) => {
            let _ = rejection.cost_report();
            "rejected-before-effects"
        }
        UiMountedFrameOutcome::InFlight(in_flight) => {
            let _ = in_flight.cost_report();
            "in-flight"
        }
        UiMountedFrameOutcome::PresentationIndeterminate(indeterminate) => {
            let _ = indeterminate.cost_report();
            "presentation-indeterminate"
        }
        UiMountedFrameOutcome::RetentionDenied(rejection) => {
            let _ = rejection.frame().cost_report();
            "retention-denied"
        }
        UiMountedFrameOutcome::AdmissionDenied(rejection) => {
            let _ = rejection.frame().cost_report();
            "admission-denied"
        }
        UiMountedFrameOutcome::CompletionDenied(_) => "completion-denied",
        UiMountedFrameOutcome::Superseded(superseded) => {
            let _ = superseded.cost_report();
            "superseded"
        }
    };
    assert_eq!(
        posture, "rejected-before-effects",
        "an empty Query-free app should stop before host effects without setup ceremony"
    );
    let shutdown = session.shutdown();
    assert!(shutdown.runtime_service_resource_census().is_empty());
}
