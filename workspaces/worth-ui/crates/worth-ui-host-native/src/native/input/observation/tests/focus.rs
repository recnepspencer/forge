use super::*;

#[test]
fn focus_has_independent_admission_and_report_accounting() {
    let mut state = presented_state();
    let presentation = state
        .report()
        .last_completed_presentation()
        .expect("completed presentation");
    state.observe_window_event(&WindowEvent::Focused(true));
    let report = state.report();
    assert_eq!(
        report.family_count(UiNativeInputObservationEventFamily::WindowFocus),
        1
    );
    assert_eq!(
        report.family_count(UiNativeInputObservationEventFamily::Pointer),
        0
    );
    let batches = state.drain(HOST_SESSION).into_batches();
    assert!(matches!(
        batches[0].reports()[0].payload(),
        UiHostObservationPayload::WindowFocus { surface, focused: true }
            if *surface == presentation.host_surface()
    ));
}
