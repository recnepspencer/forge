#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiViewportResizeDenial {
    ReceiptBudgetExceeded { selected: u16, maximum: u16 },
    TransactionCommitDenied,
}
