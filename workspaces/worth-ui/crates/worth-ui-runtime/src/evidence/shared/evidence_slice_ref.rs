use worth_ui_inspection::UiEvidenceAuthorityGeneration;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct UiEvidenceSliceRef {
    digest: u64,
    authority_generation: UiEvidenceAuthorityGeneration,
}

impl UiEvidenceSliceRef {
    pub(crate) fn new(digest: u64, authority_generation: UiEvidenceAuthorityGeneration) -> Self {
        Self {
            digest,
            authority_generation,
        }
    }

    pub fn digest(self) -> u64 {
        self.digest
    }

    pub fn authority_generation(self) -> UiEvidenceAuthorityGeneration {
        self.authority_generation
    }
}
