use super::{
    WorthQueryCollectionCapabilityCounters, WorthQueryCollectionCapabilityDenialKind,
    WorthQueryCollectionRowHandle,
};
use crate::domain_installation::WorthQueryBoundCapabilityGeneration;

pub(super) fn collection_rows(
    authority: &crate::projection_consumption::WorthQueryConsumedProjectionAuthority,
    capability_identity: u64,
    capability_generation: WorthQueryBoundCapabilityGeneration,
    counters: &mut WorthQueryCollectionCapabilityCounters,
) -> Result<Vec<WorthQueryCollectionRowHandle>, WorthQueryCollectionCapabilityDenialKind> {
    let entity_facts = authority.facts().entity_identities();
    let view_facts = authority.facts().view_local_identities();
    if entity_facts.is_empty() && !view_facts.is_empty() {
        return Err(WorthQueryCollectionCapabilityDenialKind::MissingEntityIdentityFacts);
    }
    if view_facts.is_empty() && !entity_facts.is_empty() {
        return Err(WorthQueryCollectionCapabilityDenialKind::MissingViewLocalIdentityFacts);
    }
    if entity_facts.len() != view_facts.len() {
        return Err(WorthQueryCollectionCapabilityDenialKind::IdentityFactCardinalityMismatch);
    }
    entity_facts
        .iter()
        .zip(view_facts)
        .enumerate()
        .map(|(row_ordinal, (entity, view))| {
            counters.identity_relationship_checks += 1;
            if entity.source_row_identity() != view.source_row_identity() {
                return Err(
                    WorthQueryCollectionCapabilityDenialKind::IdentityFactRelationshipMismatch,
                );
            }
            counters.identity_rows_indexed += 1;
            Ok(WorthQueryCollectionRowHandle::new(
                super::WorthQueryCollectionRowParts {
                    entity_identity: entity.entity_identity().clone(),
                    view_local_identity: view.view_local_identity().to_string(),
                    source_row_identity: entity.source_row_identity().to_string(),
                    row_ordinal,
                    capability_identity,
                    capability_generation,
                },
            ))
        })
        .collect()
}
