use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DerivedInvalidationOperatorCutoverErrorKind {
    PhaseSixSweepDoesNotMatchSelectedPlan,
    PhaseSixSweepIncomplete,
    ExecutionReceiptDoesNotMatchSelectedPlan,
    ExecutionReceiptDoesNotMatchTouchedClosure,
    OperatorTouchedBasisDoesNotMatchExecutionReceipt,
    MissingOperatorGraphObligationProof,
    ExecutionReceiptCarriesDeniedProducts,
    ExecutionReceiptCarriesWholeViewFallback,
    ExecutionReceiptCarriesCallerOwnedGraphWork,
    ProjectionReadStageScopeExpandedDirtyProducts,
    SourceFirewallViolation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DerivedInvalidationOperatorCutoverError {
    kind: DerivedInvalidationOperatorCutoverErrorKind,
    reason: String,
}

impl DerivedInvalidationOperatorCutoverError {
    pub(crate) fn new(
        kind: DerivedInvalidationOperatorCutoverErrorKind,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            reason: reason.into(),
        }
    }

    pub const fn kind(&self) -> DerivedInvalidationOperatorCutoverErrorKind {
        self.kind
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }
}

impl fmt::Display for DerivedInvalidationOperatorCutoverError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {}", self.kind, self.reason)
    }
}

impl std::error::Error for DerivedInvalidationOperatorCutoverError {}
