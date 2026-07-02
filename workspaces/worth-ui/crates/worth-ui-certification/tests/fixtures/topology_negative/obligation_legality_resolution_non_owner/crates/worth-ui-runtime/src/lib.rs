use worth_ui_runtime::admission::{UiLegalityDecision, UiLegalityPosture, UiLegalityReason};

pub fn forbidden_legality_resolution(decision: &UiLegalityDecision) -> Option<UiLegalityReason> {
    match decision.posture() {
        UiLegalityPosture::Denied(reason) | UiLegalityPosture::AdmittedWithAdvisory(reason) => {
            match reason {
                UiLegalityReason::WrongQueryBasis { .. } => Some(reason),
                _ => None,
            }
        }
        UiLegalityPosture::Admitted => None,
    }
}
