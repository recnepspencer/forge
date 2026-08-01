#[must_use]
pub enum UiIntentAdmissionDecision<I: crate::capability::UiIntent> {
    Admitted(super::UiAdmittedIntent<I>),
    ConfirmationRequired(super::super::UiPendingIntentConfirmation),
    Stopped(super::UiIntentAdmissionStop),
}
