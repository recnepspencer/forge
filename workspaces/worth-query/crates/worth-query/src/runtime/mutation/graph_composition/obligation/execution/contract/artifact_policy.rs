use crate::runtime::WorthQueryGraphObligationDiagnosticMaterialization;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub enum WorthQueryGraphObligationArtifactPolicy {
    #[default]
    BoundedEvidenceOnly,
    RichCapabilityGapDiagnostics,
}

impl WorthQueryGraphObligationArtifactPolicy {
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

    pub fn diagnostic_materialization(self) -> WorthQueryGraphObligationDiagnosticMaterialization {
        match self {
            Self::BoundedEvidenceOnly => {
                WorthQueryGraphObligationDiagnosticMaterialization::BoundedEvidenceOnly
            }
            Self::RichCapabilityGapDiagnostics => {
                WorthQueryGraphObligationDiagnosticMaterialization::RichCapabilityGapDiagnostics
            }
        }
    }
}

impl From<WorthQueryGraphObligationDiagnosticMaterialization>
    for WorthQueryGraphObligationArtifactPolicy
{
    fn from(
        diagnostic_materialization: WorthQueryGraphObligationDiagnosticMaterialization,
    ) -> Self {
        match diagnostic_materialization {
            WorthQueryGraphObligationDiagnosticMaterialization::BoundedEvidenceOnly => {
                Self::BoundedEvidenceOnly
            }
            WorthQueryGraphObligationDiagnosticMaterialization::RichCapabilityGapDiagnostics => {
                Self::RichCapabilityGapDiagnostics
            }
        }
    }
}
