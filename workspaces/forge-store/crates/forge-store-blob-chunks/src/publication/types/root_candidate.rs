use crate::{BlobChunkRootPublication, BlobGenerationObservation};

use super::super::{BlobPublicationDenial, BlobPublicationIntent};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobRootCandidateForPublication {
    intent: BlobPublicationIntent,
}

impl BlobRootCandidateForPublication {
    pub fn from_registry_observation(
        observation: BlobGenerationObservation<'_>,
        root_publication: BlobChunkRootPublication,
    ) -> Result<Self, BlobPublicationDenial> {
        let intent =
            BlobPublicationIntent::from_registry_observation(observation, &root_publication)?;
        Ok(Self { intent })
    }

    pub const fn intent(&self) -> &BlobPublicationIntent {
        &self.intent
    }

    pub(crate) fn into_intent(self) -> BlobPublicationIntent {
        self.intent
    }
}