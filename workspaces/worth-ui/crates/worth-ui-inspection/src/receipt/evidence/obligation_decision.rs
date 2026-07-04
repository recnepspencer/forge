#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiInspectionObligationDecision {
    Selected,
    NotSelected,
    Dispatch,
    Verdict,
    Admission,
}
