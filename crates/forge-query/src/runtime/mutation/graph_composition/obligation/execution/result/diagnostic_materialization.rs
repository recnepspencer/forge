#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryGraphObligationDiagnosticMaterialization {
    BoundedEvidenceOnly,
    RichCapabilityGapDiagnostics,
}

impl ForgeQueryGraphObligationDiagnosticMaterialization {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BoundedEvidenceOnly => "bounded-evidence-only",
            Self::RichCapabilityGapDiagnostics => "rich-capability-gap-diagnostics",
        }
    }
}
