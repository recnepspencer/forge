use crate::runtime::ForgeQueryGraphObligationDiagnosticMaterialization;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryGraphObligationArtifactPolicy {
    BoundedEvidenceOnly,
    RichCapabilityGapDiagnostics,
}

impl ForgeQueryGraphObligationArtifactPolicy {
    pub fn bounded_evidence_only() -> Self {
        Self::BoundedEvidenceOnly
    }

    pub fn rich_capability_gap_diagnostics() -> Self {
        Self::RichCapabilityGapDiagnostics
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::BoundedEvidenceOnly => "bounded-evidence-only",
            Self::RichCapabilityGapDiagnostics => "rich-capability-gap-diagnostics",
        }
    }

    pub fn diagnostic_materialization(self) -> ForgeQueryGraphObligationDiagnosticMaterialization {
        match self {
            Self::BoundedEvidenceOnly => {
                ForgeQueryGraphObligationDiagnosticMaterialization::BoundedEvidenceOnly
            }
            Self::RichCapabilityGapDiagnostics => {
                ForgeQueryGraphObligationDiagnosticMaterialization::RichCapabilityGapDiagnostics
            }
        }
    }
}

impl Default for ForgeQueryGraphObligationArtifactPolicy {
    fn default() -> Self {
        Self::BoundedEvidenceOnly
    }
}

impl From<ForgeQueryGraphObligationDiagnosticMaterialization>
    for ForgeQueryGraphObligationArtifactPolicy
{
    fn from(
        diagnostic_materialization: ForgeQueryGraphObligationDiagnosticMaterialization,
    ) -> Self {
        match diagnostic_materialization {
            ForgeQueryGraphObligationDiagnosticMaterialization::BoundedEvidenceOnly => {
                Self::BoundedEvidenceOnly
            }
            ForgeQueryGraphObligationDiagnosticMaterialization::RichCapabilityGapDiagnostics => {
                Self::RichCapabilityGapDiagnostics
            }
        }
    }
}
