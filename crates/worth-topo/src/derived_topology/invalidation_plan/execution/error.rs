use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DerivedInvalidationExecutionErrorKind {
    DeniedPlanCarriedExecutableRows,
    CallerOwnedGraphWorkNotAdmitted,
    ExecutionReportSourceRowMismatch,
    OrdinaryWholeViewFallbackNotAdmitted,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DerivedInvalidationExecutionError {
    kind: DerivedInvalidationExecutionErrorKind,
}

impl DerivedInvalidationExecutionError {
    pub(super) const fn new(kind: DerivedInvalidationExecutionErrorKind) -> Self {
        Self { kind }
    }

    pub const fn kind(&self) -> DerivedInvalidationExecutionErrorKind {
        self.kind
    }
}

impl fmt::Display for DerivedInvalidationExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            DerivedInvalidationExecutionErrorKind::DeniedPlanCarriedExecutableRows => {
                write!(
                    f,
                    "derived invalidation execution denied selected executable rows"
                )
            }
            DerivedInvalidationExecutionErrorKind::CallerOwnedGraphWorkNotAdmitted => {
                write!(
                    f,
                    "derived invalidation execution cannot admit caller-owned graph work"
                )
            }
            DerivedInvalidationExecutionErrorKind::ExecutionReportSourceRowMismatch => {
                write!(
                    f,
                    "derived invalidation execution report did not match selected source row"
                )
            }
            DerivedInvalidationExecutionErrorKind::OrdinaryWholeViewFallbackNotAdmitted => {
                write!(
                    f,
                    "ordinary derived invalidation execution cannot admit whole-view fallback"
                )
            }
        }
    }
}

impl std::error::Error for DerivedInvalidationExecutionError {}
