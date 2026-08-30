#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiHostSolicitedEffectCancellationOutcome {
    CancelledBeforeEffect,
    EffectAlreadyIssued,
    Indeterminate,
}
