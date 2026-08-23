use super::*;

#[test]
fn focus_has_independent_admission_and_report_accounting() {
    let mut state = presented_state();
    state.observe_window_event(&WindowEvent::Focused(true));
    let report = state.report();
    assert_eq!(
        report.family_count(UiNativeInputObservationEventFamily::Focus),
        1
    );
    assert_eq!(
        report.family_count(UiNativeInputObservationEventFamily::Pointer),
        0
    );
}
