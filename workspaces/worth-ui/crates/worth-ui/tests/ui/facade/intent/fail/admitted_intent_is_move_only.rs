use worth_ui::facade::{
    app::WorthUiActiveApplicationSession,
    intent::{UiAdmittedIntent, UiIntent, UiIntentExecutionClockReading},
};

fn require_clone<T: Clone>() {}

fn admitted_handle_cannot_gain_clone<I: UiIntent>() {
    require_clone::<UiAdmittedIntent<I>>();
}

fn admitted_authority_cannot_dispatch_twice<I: UiIntent>(
    session: &mut WorthUiActiveApplicationSession,
    admitted: UiAdmittedIntent<I>,
) {
    let first = UiIntentExecutionClockReading::at_tick(0)
        .deadline_after_ticks(1)
        .unwrap();
    let second = UiIntentExecutionClockReading::at_tick(0)
        .deadline_after_ticks(2)
        .unwrap();
    let _ = session.dispatch_admitted_intent(admitted, first);
    let _ = session.dispatch_admitted_intent(admitted, second);
}

fn main() {}
