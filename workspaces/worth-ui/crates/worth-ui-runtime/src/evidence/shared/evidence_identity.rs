use super::evidence_family::UiEvidenceFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct UiEvidenceIdentity {
    family: UiEvidenceFamily,
    digest: u64,
}

impl UiEvidenceIdentity {
    pub(crate) fn new(family: UiEvidenceFamily, digest: u64) -> Self {
        Self { family, digest }
    }

    pub fn family(&self) -> UiEvidenceFamily {
        self.family
    }

    pub fn digest(&self) -> u64 {
        self.digest
    }
}
