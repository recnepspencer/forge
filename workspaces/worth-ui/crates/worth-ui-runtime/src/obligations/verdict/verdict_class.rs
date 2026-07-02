#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiObligationVerdictClass {
    Success,
    Advisory,
    Violation,
}
