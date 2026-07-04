use super::UiEvidenceAuthorityKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct UiEvidenceAuthorityArtifactIdentity {
    kind: UiEvidenceAuthorityKind,
    digest: u64,
}

impl UiEvidenceAuthorityArtifactIdentity {
    pub const fn new(kind: UiEvidenceAuthorityKind, digest: u64) -> Self {
        Self { kind, digest }
    }

    pub const fn kind(self) -> UiEvidenceAuthorityKind {
        self.kind
    }

    pub const fn digest(self) -> u64 {
        self.digest
    }
}
