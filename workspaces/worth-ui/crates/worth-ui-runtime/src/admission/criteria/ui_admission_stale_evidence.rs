#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiAdmissionStaleEvidence {
    DeclarationArtifactMissing,
    QueryReceiptExpired,
}
