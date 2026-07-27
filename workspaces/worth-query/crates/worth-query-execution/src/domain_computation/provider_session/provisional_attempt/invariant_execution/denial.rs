use std::sync::Arc;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryInvariantExecutionDenialKind {
    InvariantNotInstalled,
    ExecutorRoleMismatch,
    EmptyStateLoadPlan,
    UndeclaredStateLoadFamily,
    StateLoadBudgetExceeded,
    ExecutionBudgetExceeded,
    ProviderUnsupported,
    ProviderRejected,
    ProviderPanicked,
    EvidenceSubstitution,
    EmptyStateLoad,
    StateLoadClosureMismatch,
    VerdictPostureMismatch,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryInvariantExecutionFailurePosture {
    Denied,
    Exhausted,
    Indeterminate,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryInvariantExecutionFailure {
    kind: WorthQueryInvariantExecutionDenialKind,
    posture: WorthQueryInvariantExecutionFailurePosture,
    detail: Arc<str>,
}

impl WorthQueryInvariantExecutionFailure {
    pub fn new(kind: WorthQueryInvariantExecutionDenialKind, detail: impl Into<Arc<str>>) -> Self {
        Self {
            kind,
            posture: WorthQueryInvariantExecutionFailurePosture::Denied,
            detail: detail.into(),
        }
    }

    pub(crate) fn exhausted(
        kind: WorthQueryInvariantExecutionDenialKind,
        detail: impl Into<Arc<str>>,
    ) -> Self {
        Self {
            kind,
            posture: WorthQueryInvariantExecutionFailurePosture::Exhausted,
            detail: detail.into(),
        }
    }

    pub fn kind(&self) -> WorthQueryInvariantExecutionDenialKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn posture(&self) -> WorthQueryInvariantExecutionFailurePosture {
        self.posture
    }
}
