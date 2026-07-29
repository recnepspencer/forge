use worth_ui::facade::app::WorthUiActiveApplicationSession;
use worth_ui::facade::observation::UiChangeClassificationOutcome;
use worth_ui::facade::rebind::{UiRebindExecutionPolicy, UiRebindPlanningDenial};

fn compile_owner_issued_change(
    session: &WorthUiActiveApplicationSession,
    outcome: UiChangeClassificationOutcome,
) -> Result<(), UiRebindPlanningDenial> {
    match outcome {
        UiChangeClassificationOutcome::Changed(change) => {
            let scope = session
                .resolve_affected_scope(change)
                .expect("owner-issued change resolves");
            let lifecycle = scope
                .resolve_identity_lifecycle()
                .expect("resolved scope advances one phase");
            session
                .compile_rebind_plan(lifecycle, UiRebindExecutionPolicy::ordinary())
                .map(|_| ())
        }
        UiChangeClassificationOutcome::EvidenceOnly(evidence) => session
            .compile_preservation_rebind(evidence, UiRebindExecutionPolicy::ordinary())
            .map(|_| ()),
        UiChangeClassificationOutcome::ObservedNoChange(_) => Ok(()),
    }
}

fn main() {
    let _ = compile_owner_issued_change;
}
