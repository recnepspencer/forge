use worth_ui::facade::interaction::UiHostInteractionIngressOutcome;
use worth_ui::facade::observation_report::{
    UiHostObservationFamily, UiHostObservationPayload, UiHostObservationReportDenial,
    UiHostPointerButtonTransition,
};

use super::super::super::interaction_world::InteractionWorld;
use super::super::assertions::applied;

#[test]
fn sequence_duplicate_reorder_skip_delay_and_overflow_are_exact() {
    let mut duplicate = InteractionWorld::canonical();
    let focused = UiHostObservationPayload::Focus { focused: true };
    assert!(matches!(
        duplicate.payload_at(1, 100, focused.clone()),
        UiHostInteractionIngressOutcome::Applied(_)
    ));
    assert!(matches!(
        duplicate.payload_at(1, 100, focused.clone()),
        UiHostInteractionIngressOutcome::Duplicate(_)
    ));
    assert!(matches!(
        duplicate.payload_at(2, 1, focused.clone()),
        UiHostInteractionIngressOutcome::Applied(_)
    ));
    let reordered =
        duplicate.payload_at(1, 200, UiHostObservationPayload::Focus { focused: false });
    assert_denial(
        reordered,
        UiHostObservationReportDenial::SequenceReordered,
        0,
    );
    let _ = duplicate.session.shutdown();

    let mut skipped = InteractionWorld::canonical();
    let _ = applied(skipped.button(1, 1, UiHostPointerButtonTransition::Pressed, [20, 20]));
    let gap = skipped.payload_at(3, 3, focused);
    assert_denial(gap, UiHostObservationReportDenial::SequenceGap, 1);
    assert_eq!(skipped.session.interaction_state().active_gestures(), 0);
    let _ = skipped.session.shutdown();

    let mut overflow = InteractionWorld::canonical();
    let before = overflow.pointer_button_overflow();
    assert_denial(
        before,
        UiHostObservationReportDenial::LosslessOverflow(UiHostObservationFamily::PointerButton),
        0,
    );
    let _ = overflow.session.shutdown();

    let mut during = InteractionWorld::canonical();
    let _ = applied(during.button(1, 1, UiHostPointerButtonTransition::Pressed, [20, 20]));
    let overflow = during.pointer_button_overflow();
    assert_denial(
        overflow,
        UiHostObservationReportDenial::LosslessOverflow(UiHostObservationFamily::PointerButton),
        1,
    );
    assert_eq!(during.session.interaction_state().active_gestures(), 0);
    let _ = during.session.shutdown();
}

fn assert_denial(
    outcome: UiHostInteractionIngressOutcome,
    expected: UiHostObservationReportDenial,
    settled_gestures: usize,
) {
    let UiHostInteractionIngressOutcome::Denied(denial) = outcome else {
        panic!("hostile sequence must be denied")
    };
    assert_eq!(denial.denial(), expected);
    assert_eq!(denial.settlement().settled_gestures(), settled_gestures);
}
