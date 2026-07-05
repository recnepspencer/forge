use forge_store_budgets::CounterEvidenceStrength;

use super::InterferenceCounterName;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InterferenceCounterDenial {
    MissingCounter(InterferenceCounterName),
    InsufficientCounterStrength {
        counter: InterferenceCounterName,
        required: CounterEvidenceStrength,
        actual: CounterEvidenceStrength,
    },
    MissingCausalAttribution(InterferenceCounterName),
    LaneMismatch,
    ProfileScopeMismatch,
    MissingPostAdmissionViolationAttribution,
}
