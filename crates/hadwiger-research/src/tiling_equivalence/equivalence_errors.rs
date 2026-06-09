use crate::domain_artifacts::HadwigerArtifactShapeError;
use crate::domain_declarations::HadwigerResearchDeclarationShapeError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TilingEquivalenceError {
    EmptyField { field: &'static str },
    ScopeInputMismatch { scope: &'static str },
    UnsupportedEquivalenceScope { scope: &'static str },
    EquivalenceCertificateRejected { reason: &'static str },
    MissingDeadEndEvidence,
    MissingSuppressionProof,
    ReactivationEvidenceNotNew,
    QueryDeclarationNotAdmitted { declaration: &'static str },
    Artifact(HadwigerArtifactShapeError),
    Declaration(HadwigerResearchDeclarationShapeError),
}

impl From<HadwigerArtifactShapeError> for TilingEquivalenceError {
    fn from(error: HadwigerArtifactShapeError) -> Self {
        Self::Artifact(error)
    }
}

impl From<HadwigerResearchDeclarationShapeError> for TilingEquivalenceError {
    fn from(error: HadwigerResearchDeclarationShapeError) -> Self {
        Self::Declaration(error)
    }
}

pub(crate) fn require_equivalence_non_empty(
    value: impl Into<String>,
    field: &'static str,
) -> Result<String, TilingEquivalenceError> {
    let value = value.into();
    if value.trim().is_empty() {
        Err(TilingEquivalenceError::EmptyField { field })
    } else {
        Ok(value)
    }
}
