#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum UiEvidenceAuthorityKind {
    DeclarationArtifact,
    AdmissionReport,
    GraphSnapshot,
    AspectAuthority,
    ObligationAuthority,
}
