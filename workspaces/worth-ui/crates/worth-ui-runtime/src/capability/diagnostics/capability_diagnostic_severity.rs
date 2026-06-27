/// Severity assigned to a capability registration diagnostic.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CapabilityDiagnosticSeverity {
    Info,
    Warning,
    Error,
}

impl CapabilityDiagnosticSeverity {
    pub fn is_error(self) -> bool {
        self == Self::Error
    }
}
