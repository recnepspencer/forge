use super::{presented_state, HOST_SESSION};
use winit::event::{DeviceId, MouseScrollDelta, TouchPhase, WindowEvent};
use worth_ui_host_contract::UiHostObservationPayload;

#[test]
fn qualified_line_wheel_is_canonical_and_does_not_suppress_later_input() {
    let mut state = presented_state();
    state.observe_window_event(&WindowEvent::MouseWheel {
        device_id: DeviceId::dummy(),
        delta: MouseScrollDelta::LineDelta(1.0, -2.0),
        phase: TouchPhase::Moved,
    });
    state.observe_window_event(&WindowEvent::Focused(true));
    let batches = state.drain(HOST_SESSION).into_batches();
    assert_eq!(batches.len(), 2);
    assert!(matches!(
        batches[0].reports()[0].payload(),
        UiHostObservationPayload::ScrollDelta {
            x_subpixels: 40_000,
            y_subpixels: -80_000,
        }
    ));
    assert!(matches!(
        batches[1].reports()[0].payload(),
        UiHostObservationPayload::Focus { focused: true }
    ));
    assert_eq!(batches[0].reports()[0].sequence().value(), 1);
    assert_eq!(batches[1].reports()[0].sequence().value(), 2);
    let report = state.report();
    assert_eq!(report.terminal_stop(), None);
    assert_eq!(
        report.last_vertical_scroll().map(|scroll| (
            scroll.sequence(),
            scroll.x_subpixels(),
            scroll.y_subpixels()
        )),
        Some((1, 40_000, -80_000))
    );
    assert_eq!(
        report.last_horizontal_scroll().map(|scroll| (
            scroll.sequence(),
            scroll.x_subpixels(),
            scroll.y_subpixels()
        )),
        Some((1, 40_000, -80_000))
    );
}
