use forge_store_physical_isolation::SemanticVisibilityReference;

use super::super::{
    BlobGenerationPublished, BlobPublicationCounterSnapshot, BlobPublicationDenial,
    BlobVisibleGeneration,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobSemanticVisibilityHandoff {
    pub(crate) outcome: BlobSemanticVisibilityOutcome,
    pub(crate) counters: BlobPublicationCounterSnapshot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlobSemanticVisibilityOutcome {
    PreviousGeneration(BlobVisibleGeneration),
    NewlyPublishedGeneration(BlobVisibleGeneration),
}

impl BlobSemanticVisibilityHandoff {
    pub fn observe_previous_or_published(
        previous: Option<BlobVisibleGeneration>,
        published: Option<&BlobGenerationPublished>,
    ) -> Result<Self, BlobPublicationDenial> {
        super::super::transitions::semantic_visibility::observe_previous_or_published(
            previous, published,
        )
    }

    pub const fn reject_semantic_reference(
        reference: &SemanticVisibilityReference,
    ) -> BlobPublicationDenial {
        let _ = reference;
        BlobPublicationDenial::SemanticReferenceRejected {
            counters: BlobPublicationCounterSnapshot::start().with_denied_promotion(),
        }
    }

    pub const fn outcome(&self) -> &BlobSemanticVisibilityOutcome {
        &self.outcome
    }

    pub const fn counters(&self) -> BlobPublicationCounterSnapshot {
        self.counters
    }
}