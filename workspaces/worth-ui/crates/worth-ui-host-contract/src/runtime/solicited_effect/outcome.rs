#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiHostSolicitedEffectOutcome<Rejection> {
    Applied,
    RejectedBeforeEffect(Rejection),
    Indeterminate,
}
