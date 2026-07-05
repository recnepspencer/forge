use crate::admission::UiAdmissionAggregation;
use crate::admission::UiSupportSnapshot;
use crate::obligations::verdict::{UiObligationVerdict, UiObligationVerdictClass};

pub(crate) fn aggregation_from_selected(
    support_snapshot: &UiSupportSnapshot,
    verdicts: &[UiObligationVerdict],
) -> UiAdmissionAggregation {
    match support_snapshot.posture() {
        crate::admission::UiSupportPosture::Unsupported { .. } => {
            UiAdmissionAggregation::Unsupported
        }
        crate::admission::UiSupportPosture::WrongWorld { .. } => UiAdmissionAggregation::WrongWorld,
        crate::admission::UiSupportPosture::Deferred { .. } => UiAdmissionAggregation::Deferred,
        crate::admission::UiSupportPosture::DiagnosticOnly { .. } => {
            UiAdmissionAggregation::DiagnosticOnly
        }
        crate::admission::UiSupportPosture::Supported { .. } => {
            if verdicts
                .iter()
                .any(|verdict| verdict.class() == UiObligationVerdictClass::Violation)
            {
                UiAdmissionAggregation::Denied
            } else if verdicts
                .iter()
                .any(|verdict| verdict.class() == UiObligationVerdictClass::Advisory)
            {
                UiAdmissionAggregation::AdmittedWithAdvisory
            } else {
                UiAdmissionAggregation::Admitted
            }
        }
    }
}
