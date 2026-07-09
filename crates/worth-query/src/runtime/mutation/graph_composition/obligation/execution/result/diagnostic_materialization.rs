#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryGraphObligationDiagnosticMaterialization {
    BoundedEvidenceOnly,
    RichCapabilityGapDiagnostics,
}

impl WorthQueryGraphObligationDiagnosticMaterialization {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BoundedEvidenceOnly => "bounded-evidence-only",
            Self::RichCapabilityGapDiagnostics => "rich-capability-gap-diagnostics",
        }
    }
}
