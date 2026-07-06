use forge_store_physical_isolation::SemanticVisibilityReference;

use super::{
    BlobGenerationPublished, BlobPublicationCounterSnapshot, BlobPublicationDenial,
    BlobVisibleGeneration,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobSemanticVisibilityHandoff {
    outcome: BlobSemanticVisibilityOutcome,
    counters: BlobPublicationCounterSnapshot,
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
        match published {
            Some(published) => {
                let visible = BlobVisibleGeneration::from_published(published);
                Ok(Self {
                    counters: visible.counters(),
                    outcome: BlobSemanticVisibilityOutcome::NewlyPublishedGeneration(visible),
                })
            }
            None => previous
                .map(|visible| Self {
                    counters: visible.counters(),
                    outcome: BlobSemanticVisibilityOutcome::PreviousGeneration(visible),
                })
                .ok_or(
                    BlobPublicationDenial::VisibilityRequiresPublishedGeneration {
                        counters: BlobPublicationCounterSnapshot::start().with_denied_promotion(),
                    },
                ),
        }
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
