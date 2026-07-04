#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum UiEvidenceAuthorityKind {
    DeclarationArtifact,
    AdmissionReport,
    GraphSnapshot,
    AspectAuthority,
    ObligationAuthority,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct UiEvidenceProvenance {
    authority_kind: UiEvidenceAuthorityKind,
    authority_digest: u64,
}

impl UiEvidenceProvenance {
    pub(crate) fn new(authority_kind: UiEvidenceAuthorityKind, authority_digest: u64) -> Self {
        Self {
            authority_kind,
            authority_digest,
        }
    }

    pub fn authority_kind(&self) -> UiEvidenceAuthorityKind {
        self.authority_kind
    }

    pub fn authority_digest(&self) -> u64 {
        self.authority_digest
    }
}
