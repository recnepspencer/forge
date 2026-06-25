#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthGraphReadAccessOutOfScopeReason {
    DocumentationOnly,
    NonGraphReadCloseout,
    NonExecutionCertificationBoundary,
}
