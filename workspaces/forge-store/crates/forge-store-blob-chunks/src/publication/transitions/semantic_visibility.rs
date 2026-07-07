use super::super::receipt_construction::visibility;
use super::super::types::published::BlobGenerationPublished;
use super::super::types::published::BlobVisibleGeneration;
use super::super::types::semantic_visibility::{
    BlobSemanticVisibilityHandoff, BlobSemanticVisibilityOutcome,
};
use super::super::{BlobPublicationCounterSnapshot, BlobPublicationDenial};

pub(crate) fn observe_previous_or_published(
    previous: Option<BlobVisibleGeneration>,
    published: Option<&BlobGenerationPublished>,
) -> Result<BlobSemanticVisibilityHandoff, BlobPublicationDenial> {
    match published {
        Some(published) => {
            let visible = visibility::from_published(published);
            Ok(BlobSemanticVisibilityHandoff {
                counters: visible.counters(),
                outcome: BlobSemanticVisibilityOutcome::NewlyPublishedGeneration(visible),
            })
        }
        None => previous
            .map(|visible| BlobSemanticVisibilityHandoff {
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
