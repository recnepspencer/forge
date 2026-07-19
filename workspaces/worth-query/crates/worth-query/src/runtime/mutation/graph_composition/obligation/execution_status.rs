#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryGraphObligationExecutionStatus {
    Selected,
    Executed,
    NotSelected,
    NotApplicableAfterStateLoad,
    DiagnosticOnly,
    DeferredToBackstop,
    Unsupported,
    SuppressedByPolicy,
    BlockedByPrerequisite,
    BudgetExceeded,
    ExecutorError,
}

impl WorthQueryGraphObligationExecutionStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Selected => "selected",
            Self::Executed => "executed",
            Self::NotSelected => "not-selected",
            Self::NotApplicableAfterStateLoad => "not-applicable-after-state-load",
            Self::DiagnosticOnly => "diagnostic-only",
            Self::DeferredToBackstop => "deferred-to-backstop",
            Self::Unsupported => "unsupported",
            Self::SuppressedByPolicy => "suppressed-by-policy",
            Self::BlockedByPrerequisite => "blocked-by-prerequisite",
            Self::BudgetExceeded => "budget-exceeded",
            Self::ExecutorError => "executor-error",
        }
    }

    pub fn is_execution_failure(self) -> bool {
        matches!(self, Self::ExecutorError)
    }

    pub fn is_budget_denial(self) -> bool {
        matches!(self, Self::BudgetExceeded)
    }
}
