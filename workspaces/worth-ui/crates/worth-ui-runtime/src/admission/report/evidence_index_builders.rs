use crate::admission::UiAdmissionAggregation;
use crate::obligations::selection::UiSelectedObligationSet;
use crate::obligations::verdict::{UiObligationVerdict, UiObligationVerdictClass};

pub(crate) fn aggregation_from_selected(
    selected: &UiSelectedObligationSet,
    verdicts: &[UiObligationVerdict],
) -> UiAdmissionAggregation {
    match selected.support_snapshot().posture() {
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
