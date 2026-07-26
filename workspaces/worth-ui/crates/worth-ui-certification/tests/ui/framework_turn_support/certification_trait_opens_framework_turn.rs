use worth_ui::facade::app::WorthUiActiveApplicationSession;
use worth_ui_test_support::WorthUiFrameworkTurnCertificationExt;

fn inspect_framework_turn(session: &mut WorthUiActiveApplicationSession) {
    if let Ok(completion) = session.execute_framework_turn(|_| {}) {
        let _ = completion.into_execution();
    }
}

fn main() {}
