use forge_query::facade::{
    ForgeQueryEntityIdentity, ForgeQueryExistingEntityTarget, ForgeQueryExistingRelationTarget,
    ForgeQueryExistingTruthTargetBinding, ForgeQueryGraphCompositionBuilder,
    ForgeQueryRuntimeError,
};
use forge_relational::facade::identity::{EntityId, RelationId};
use schema::facade::platform::entities::TopologyEntityKind;

use crate::projection::runtime_boundary::query_runtime::TopologyQueryBindingIndex;
use crate::query_native_runtime_boundary::TopologyNativeQueryRowField;
use crate::topology_operators::application::{
    TopologyMutationApplicationError, TopologyMutationApplicationRunner,
};
use crate::topology_operators::authority_identity::{
    existing_entity_authority, existing_relation_authority,
};
use crate::topology_operators::mutation_sequence::TopologyDeclaredMutationMember;
use crate::topology_operators::touched_graph_basis::{
    query_aspect_touch, topology_touched_aspect_from_schema_aspect,
};

pub(super) fn bind_existing_relation_handle(
    _runner: &TopologyMutationApplicationRunner<'_, '_>,
    relation_id: RelationId,
    query_identity: ForgeQueryEntityIdentity,
) -> Result<ForgeQueryExistingTruthTargetBinding, TopologyMutationApplicationError> {
    Ok(ForgeQueryExistingTruthTargetBinding::from_relation_target(
        ForgeQueryExistingRelationTarget::new(
            existing_relation_authority(relation_id)?,
            query_identity,
        )?
        .in_target_collection("TopologyRelation")?,
    )?)
}

pub(super) fn bind_existing_entity_handle(
    _runner: &TopologyMutationApplicationRunner<'_, '_>,
    bindings: &TopologyQueryBindingIndex,
    entity_id: EntityId,
    expected_kind: TopologyEntityKind,
) -> Result<ForgeQueryExistingTruthTargetBinding, TopologyMutationApplicationError> {
    let binding = crate::topology_operators::application::bindings::query_entity_binding(
        bindings, entity_id,
    )?
    .ok_or(TopologyMutationApplicationError::MissingExistingEntityBinding(entity_id))?;
    if binding.kind != expected_kind {
        return Err(
            TopologyMutationApplicationError::ExistingEntityKindMismatch {
                entity_id,
                expected: expected_kind,
                actual: binding.kind,
            },
        );
    }
    Ok(ForgeQueryExistingTruthTargetBinding::from_entity_target(
        ForgeQueryExistingEntityTarget::new(
            existing_entity_authority(entity_id)?,
            binding.query_identity,
        )?
        .in_target_collection("TopologyEntity")?,
    )?)
}

pub(super) fn delete_existing_entity_from_graph(
    graph: &mut ForgeQueryGraphCompositionBuilder,
    binding: ForgeQueryExistingTruthTargetBinding,
    target_collection: &str,
    expected_kind_name: &str,
    contract: TopologyDeclaredMutationMember<'_>,
) -> Result<(), ForgeQueryRuntimeError> {
    graph.delete_existing_verified(
        binding,
        |verify| TopologyNativeQueryRowField::TopologyKind.set_on(verify, expected_kind_name),
        |delete| {
            delete.target_collection(target_collection).touches(
                contract.touched_aspects().iter().copied().map(|aspect| {
                    query_aspect_touch(topology_touched_aspect_from_schema_aspect(aspect))
                }),
            )
        },
    )?;
    Ok(())
}
