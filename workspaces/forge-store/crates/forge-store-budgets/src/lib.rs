#![forbid(unsafe_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BudgetAdmissionDecision {
    Admit,
    Defer,
    Deny,
    AdmitDegraded,
}
