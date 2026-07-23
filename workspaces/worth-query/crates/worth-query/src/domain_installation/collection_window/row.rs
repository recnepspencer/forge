use crate::domain_installation::WorthQueryBoundCapabilityGeneration;
use crate::memory_workspace::WorthQueryEntityIdentity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryCollectionRowHandle {
    entity_identity: WorthQueryEntityIdentity,
    view_local_identity: String,
    pub(crate) source_row_identity: String,
    pub(super) row_ordinal: usize,
    pub(super) capability_identity: u64,
    pub(super) capability_generation: WorthQueryBoundCapabilityGeneration,
}

pub(crate) struct WorthQueryCollectionRowParts {
    pub entity_identity: WorthQueryEntityIdentity,
    pub view_local_identity: String,
    pub source_row_identity: String,
    pub row_ordinal: usize,
    pub capability_identity: u64,
    pub capability_generation: WorthQueryBoundCapabilityGeneration,
}

impl WorthQueryCollectionRowHandle {
    pub(crate) fn new(parts: WorthQueryCollectionRowParts) -> Self {
        Self {
            entity_identity: parts.entity_identity,
            view_local_identity: parts.view_local_identity,
            source_row_identity: parts.source_row_identity,
            row_ordinal: parts.row_ordinal,
            capability_identity: parts.capability_identity,
            capability_generation: parts.capability_generation,
        }
    }

    pub fn entity_identity(&self) -> &WorthQueryEntityIdentity {
        &self.entity_identity
    }

    pub fn view_local_identity(&self) -> &str {
        &self.view_local_identity
    }

    pub(super) fn rebind(
        &self,
        capability_identity: u64,
        capability_generation: WorthQueryBoundCapabilityGeneration,
    ) -> Self {
        Self {
            entity_identity: self.entity_identity.clone(),
            view_local_identity: self.view_local_identity.clone(),
            source_row_identity: self.source_row_identity.clone(),
            row_ordinal: self.row_ordinal,
            capability_identity,
            capability_generation,
        }
    }
}
