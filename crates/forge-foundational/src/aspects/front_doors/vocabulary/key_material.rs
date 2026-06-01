use crate::{AspectContractRevision, AspectIdentity, AspectKey};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AspectVocabularyFrontDoor;

impl AspectVocabularyFrontDoor {
    pub fn key(
        self,
        raw: impl Into<String>,
    ) -> Result<AspectKey, super::AspectFrontDoorConstructionDenial> {
        let raw = raw.into();
        AspectKey::new(raw.clone()).ok_or(
            super::AspectFrontDoorConstructionDenial::InvalidAspectKey(raw),
        )
    }

    pub const fn identity(self, raw: u64) -> AspectIdentity {
        AspectIdentity(raw)
    }

    pub const fn revision(self, raw: u64) -> AspectContractRevision {
        AspectContractRevision(raw)
    }
}
