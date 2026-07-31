use std::marker::PhantomData;

use worth_foundational::facade::{AspectFieldLocator, AspectValue};
use worth_query_installation::facade::ApplicationSchemaBindingIdentity;
use worth_relational::facade::identity::{EntityId, KindId};
use worth_relational::facade::indexes::{DerivedIndexGenerationId, DerivedIndexId};

use crate::domain_computation::execution_runtime::WorthQueryRuntimeAuthorityIdentity;

use super::WorthQueryPrincipalResolutionMode;

/// Opaque, typed identity of one application entity resolved through an exact
/// equality index generation.
///
/// This is descriptive scope identity only. It grants no ability or operation
/// authority.
pub struct WorthQueryApplicationEntityIdentity<Schema, Entity> {
    entity_id: EntityId,
    entity_kind: KindId,
    entity_name: String,
    binding_identity: ApplicationSchemaBindingIdentity,
    runtime_authority: WorthQueryRuntimeAuthorityIdentity,
    identity_index_id: DerivedIndexId,
    identity_index_generation: DerivedIndexGenerationId,
    identity_locator: AspectFieldLocator,
    identity_value: AspectValue,
    examined_candidate_count: usize,
    resolution_mode: WorthQueryPrincipalResolutionMode,
    _marker: PhantomData<fn() -> (Schema, Entity)>,
}

pub(in crate::domain_computation) struct WorthQueryResolvedEntityEvidence {
    pub(in crate::domain_computation) entity_id: EntityId,
    pub(in crate::domain_computation) entity_kind: KindId,
    pub(in crate::domain_computation) entity_name: String,
    pub(in crate::domain_computation) binding_identity: ApplicationSchemaBindingIdentity,
    pub(in crate::domain_computation) runtime_authority: WorthQueryRuntimeAuthorityIdentity,
    pub(in crate::domain_computation) identity_index_id: DerivedIndexId,
    pub(in crate::domain_computation) identity_index_generation: DerivedIndexGenerationId,
    pub(in crate::domain_computation) identity_locator: AspectFieldLocator,
    pub(in crate::domain_computation) identity_value: AspectValue,
    pub(in crate::domain_computation) examined_candidate_count: usize,
    pub(in crate::domain_computation) resolution_mode: WorthQueryPrincipalResolutionMode,
}

impl<Schema, Entity> WorthQueryApplicationEntityIdentity<Schema, Entity> {
    pub(in crate::domain_computation) fn mint(evidence: WorthQueryResolvedEntityEvidence) -> Self {
        Self {
            entity_id: evidence.entity_id,
            entity_kind: evidence.entity_kind,
            entity_name: evidence.entity_name,
            binding_identity: evidence.binding_identity,
            runtime_authority: evidence.runtime_authority,
            identity_index_id: evidence.identity_index_id,
            identity_index_generation: evidence.identity_index_generation,
            identity_locator: evidence.identity_locator,
            identity_value: evidence.identity_value,
            examined_candidate_count: evidence.examined_candidate_count,
            resolution_mode: evidence.resolution_mode,
            _marker: PhantomData,
        }
    }

    pub fn binding_identity(&self) -> &ApplicationSchemaBindingIdentity {
        &self.binding_identity
    }

    pub const fn examined_candidate_count(&self) -> usize {
        self.examined_candidate_count
    }

    pub(crate) const fn entity_id(&self) -> EntityId {
        self.entity_id
    }

    pub(crate) const fn entity_kind(&self) -> KindId {
        self.entity_kind
    }

    pub(crate) fn entity_name(&self) -> &str {
        &self.entity_name
    }

    pub(crate) const fn runtime_authority(&self) -> WorthQueryRuntimeAuthorityIdentity {
        self.runtime_authority
    }

    pub(crate) const fn identity_index_id(&self) -> DerivedIndexId {
        self.identity_index_id
    }

    pub(crate) fn identity_locator(&self) -> &AspectFieldLocator {
        &self.identity_locator
    }

    pub(crate) fn identity_value(&self) -> &AspectValue {
        &self.identity_value
    }

    pub(crate) const fn resolution_mode(&self) -> WorthQueryPrincipalResolutionMode {
        self.resolution_mode
    }
}

impl<Schema, Entity> std::fmt::Debug for WorthQueryApplicationEntityIdentity<Schema, Entity> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthQueryApplicationEntityIdentity")
            .field("binding_identity", &self.binding_identity)
            .field("identity_index_id", &self.identity_index_id)
            .field("identity_index_generation", &self.identity_index_generation)
            .field("examined_candidate_count", &self.examined_candidate_count)
            .finish_non_exhaustive()
    }
}
