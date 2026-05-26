use crate::application::ForgeQueryDeclarationFoundationalEvidenceClass;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationEnvelopeClass {
    CoveredCrossing,
    DeferredCrossing,
    DeniedCrossing,
    FailedCrossing,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationEnvelopeEvidenceOrigin {
    AdmittedProgression,
    ProgressionDeferred,
    ProgressionDenied,
    ProgressionStale,
    ProgressionRebindRequired,
    ProgressionFailed,
    LegalityEvidence,
    LegalityDenial,
}

impl ForgeQueryDeclarationEnvelopeEvidenceOrigin {
    pub(crate) fn from_foundational_class(
        class: ForgeQueryDeclarationFoundationalEvidenceClass,
    ) -> Self {
        match class {
            ForgeQueryDeclarationFoundationalEvidenceClass::LegalityAdmitted => {
                Self::LegalityEvidence
            }
            ForgeQueryDeclarationFoundationalEvidenceClass::LegalityDenied => Self::LegalityDenial,
            ForgeQueryDeclarationFoundationalEvidenceClass::ProgressionAdmitted => {
                Self::AdmittedProgression
            }
            ForgeQueryDeclarationFoundationalEvidenceClass::ProgressionDeferred => {
                Self::ProgressionDeferred
            }
            ForgeQueryDeclarationFoundationalEvidenceClass::ProgressionDenied => {
                Self::ProgressionDenied
            }
            ForgeQueryDeclarationFoundationalEvidenceClass::ProgressionStale => {
                Self::ProgressionStale
            }
            ForgeQueryDeclarationFoundationalEvidenceClass::ProgressionRebindRequired => {
                Self::ProgressionRebindRequired
            }
            ForgeQueryDeclarationFoundationalEvidenceClass::ProgressionFailed => {
                Self::ProgressionFailed
            }
        }
    }

    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::AdmittedProgression => "admitted_progression",
            Self::ProgressionDeferred => "progression_deferred",
            Self::ProgressionDenied => "progression_denied",
            Self::ProgressionStale => "progression_stale",
            Self::ProgressionRebindRequired => "progression_rebind_required",
            Self::ProgressionFailed => "progression_failed",
            Self::LegalityEvidence => "legality_evidence",
            Self::LegalityDenial => "legality_denial",
        }
    }
}
