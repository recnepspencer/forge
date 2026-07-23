use super::super::compiled::WorthQuerySemanticAspectDependencyCompilationCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQuerySemanticAspectDependencyCompilationDenialKind {
    InvalidInstalledConditionalLocation,
    EmptyRequiredClosure,
    NonCanonicalClosure,
    DuplicateDependencyLocus,
    DirectExecutionCannotRealizeSemanticContract,
    DirectExecutionReceiptMismatch,
    RealizedGraphReceiptMismatch,
    RealizedConditionalAuthorityMismatch,
    RealizedConditionalObservationMismatch,
    RealizedConditionalDeclarationMismatch,
    WorkflowTraceMismatch,
    IncompleteDependencyClosure,
    AmbiguousDependencyGraph,
    CyclicDependencyGraph,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQuerySemanticAspectDependencyCompilationDenial {
    kind: WorthQuerySemanticAspectDependencyCompilationDenialKind,
    counters: WorthQuerySemanticAspectDependencyCompilationCounters,
}

impl WorthQuerySemanticAspectDependencyCompilationDenial {
    pub(super) const fn new(
        kind: WorthQuerySemanticAspectDependencyCompilationDenialKind,
        counters: WorthQuerySemanticAspectDependencyCompilationCounters,
    ) -> Self {
        Self { kind, counters }
    }

    pub const fn kind(self) -> WorthQuerySemanticAspectDependencyCompilationDenialKind {
        self.kind
    }

    pub const fn counters(self) -> WorthQuerySemanticAspectDependencyCompilationCounters {
        self.counters
    }
}
