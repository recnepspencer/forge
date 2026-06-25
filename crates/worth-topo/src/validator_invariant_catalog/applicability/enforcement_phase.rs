#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WorthTopologyEnforcementPhase {
    Preflight,
    SelectedObligationExecution,
    CommitBackstop,
    DiagnosticOnly,
}

impl WorthTopologyEnforcementPhase {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Preflight => "preflight",
            Self::SelectedObligationExecution => "selected-obligation-execution",
            Self::CommitBackstop => "commit-backstop",
            Self::DiagnosticOnly => "diagnostic-only",
        }
    }
}
