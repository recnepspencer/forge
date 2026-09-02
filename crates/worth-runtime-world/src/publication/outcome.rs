use crate::recovery::ProductUnpublishedOwnerEffects;

use super::{NoEffectCompositePublication, PerformedCompositePublication};

/// The only public terminal vocabulary for coordinated publication.
#[must_use = "publication outcomes carry performed work or a retained terminal posture"]
#[derive(Debug)]
pub enum RuntimeWorldPublicationOutcome {
    Performed(PerformedCompositePublication),
    NoEffect(NoEffectCompositePublication),
    ProductUnpublished(ProductUnpublishedOwnerEffects),
}
