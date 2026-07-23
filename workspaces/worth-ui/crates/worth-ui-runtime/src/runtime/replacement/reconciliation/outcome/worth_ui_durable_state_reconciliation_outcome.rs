#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthUiDurableStateReconciliationOutcome {
    CarryForward,
    Replace,
    Drop,
    Recreate,
}
