use crate::application::WorthQueryDeclarationFoundationalEvidenceClass;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryDeclarationEnvelopeClass {
    CoveredCrossing,
    DeferredCrossing,
    DeniedCrossing,
    FailedCrossing,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryDeclarationEnvelopeEvidenceOrigin {
    AdmittedProgression,
    ProgressionDeferred,
    ProgressionDenied,
    ProgressionStale,
    ProgressionRebindRequired,
    ProgressionFailed,
    LegalityEvidence,
    LegalityDenial,
}

impl WorthQueryDeclarationEnvelopeEvidenceOrigin {
    pub(crate) fn from_foundational_class(
        class: WorthQueryDeclarationFoundationalEvidenceClass,
    ) -> Self {
        match class {
            WorthQueryDeclarationFoundationalEvidenceClass::LegalityAdmitted => {
                Self::LegalityEvidence
            }
            WorthQueryDeclarationFoundationalEvidenceClass::LegalityDenied => Self::LegalityDenial,
            WorthQueryDeclarationFoundationalEvidenceClass::ProgressionAdmitted => {
                Self::AdmittedProgression
            }
            WorthQueryDeclarationFoundationalEvidenceClass::ProgressionDeferred => {
                Self::ProgressionDeferred
            }
            WorthQueryDeclarationFoundationalEvidenceClass::ProgressionDenied => {
                Self::ProgressionDenied
            }
            WorthQueryDeclarationFoundationalEvidenceClass::ProgressionStale => {
                Self::ProgressionStale
            }
            WorthQueryDeclarationFoundationalEvidenceClass::ProgressionRebindRequired => {
                Self::ProgressionRebindRequired
            }
            WorthQueryDeclarationFoundationalEvidenceClass::ProgressionFailed => {
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
