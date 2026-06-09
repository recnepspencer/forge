use crate::aspect_authority::HadwigerAspectAuthorityError;
use crate::domain_artifacts::HadwigerArtifactShapeError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MotifLanguageError {
    ArtifactShape(HadwigerArtifactShapeError),
    AspectAuthority(HadwigerAspectAuthorityError),
    DuplicateIdentityField { field: &'static str, value: String },
    MissingMotifVertex { vertex_label: String },
    MissingMotifTerminal { terminal_label: String },
    MotifSourceDeclarationMismatch,
    TerminalStudyDeclarationNotAdmitted,
    TerminalStudyMotifMismatch,
    TerminalStudyRelationGoalMismatch { expected: String, actual: String },
    TerminalStudyTerminalMismatch,
    TerminalRelationEvidenceNotAdmitted,
    TerminalRelationMotifMismatch,
    TerminalRelationColorCountMismatch { expected: u32, actual: u32 },
}

impl From<HadwigerArtifactShapeError> for MotifLanguageError {
    fn from(value: HadwigerArtifactShapeError) -> Self {
        Self::ArtifactShape(value)
    }
}

impl From<HadwigerAspectAuthorityError> for MotifLanguageError {
    fn from(value: HadwigerAspectAuthorityError) -> Self {
        Self::AspectAuthority(value)
    }
}
