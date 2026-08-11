#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationReconciliationDenial {
    InvalidLease,
    DuplicateIdentity,
    BindingLimit,
    FreshnessMismatch,
}
