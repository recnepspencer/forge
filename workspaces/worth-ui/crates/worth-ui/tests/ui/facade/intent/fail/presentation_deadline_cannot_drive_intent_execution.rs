use worth_ui::facade::{
    app::{UiPresentationDeadline, WorthUiActiveApplicationSession},
    intent::UiIntentExecutionAdvanceOutcome,
};

fn presentation_deadline_cannot_drive_intent_execution(
    session: &mut WorthUiActiveApplicationSession,
    presentation_deadline: UiPresentationDeadline,
) -> UiIntentExecutionAdvanceOutcome {
    session.advance_intent_executions(presentation_deadline)
}

fn main() {}
