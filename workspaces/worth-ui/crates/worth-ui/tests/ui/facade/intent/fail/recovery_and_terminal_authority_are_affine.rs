use worth_ui::facade::{
    app::WorthUiActiveApplicationSession,
    intent::{
        UiIntentExecutionClockReading, UiIntentExecutionTransition, UiIntentRecoveryHandle,
        UiIntentRecoveryProgressOutcome,
    },
};

fn require_clone<T: Clone>() {}

fn recovery_authority_cannot_gain_clone() {
    require_clone::<UiIntentRecoveryHandle>();
}

fn terminal_settlement_cannot_gain_clone() {
    require_clone::<UiIntentExecutionTransition>();
}

fn recovery_authority_cannot_retry_twice(
    session: &mut WorthUiActiveApplicationSession,
    recovery: UiIntentRecoveryHandle,
) {
    let _ = session.retry_intent_recovery(recovery, UiIntentExecutionClockReading::at_tick(1));
    let _ = session.retry_intent_recovery(recovery, UiIntentExecutionClockReading::at_tick(2));
}

fn terminal_settlement_cannot_be_consumed_twice(transition: UiIntentExecutionTransition) {
    let _ = transition.into_recovery();
    let _ = transition.posture();
}

fn main() {}
