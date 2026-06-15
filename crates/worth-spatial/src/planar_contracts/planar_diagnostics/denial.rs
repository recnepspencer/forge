#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarDiagnosticDenialKind {
    MissingDiagnosticSource,
    MissingTopologyDeclaredSurface,
    MissingCausalInspectionReference,
    MissingProjectionConsumptionReceipt,
    MissingRetainedTransformEvidence,
    MaterializedCausalArchiveNotSupported,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarDiagnosticDenial {
    kind: PlanarDiagnosticDenialKind,
    reason: String,
}

impl PlanarDiagnosticDenial {
    pub(crate) fn new(kind: PlanarDiagnosticDenialKind, reason: impl Into<String>) -> Self {
        Self {
            kind,
            reason: reason.into(),
        }
    }

    pub fn kind(&self) -> PlanarDiagnosticDenialKind {
        self.kind
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }
}
