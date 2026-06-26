use forge_query::facade::{
    ForgeQueryExistingEntityTarget, ForgeQueryExistingRelationTarget,
    ForgeQueryExistingTruthTargetBinding, ForgeQueryMutationBatchBuilder,
};
use forge_relational::facade::identity::{EntityId, RelationId};
use schema::facade::platform::entities::TopologyEntityKind;
use schema::facade::platform::relations::TopologyRelationKind;

use crate::projection::runtime_boundary::query_runtime::TopologyQueryBindingIndex;
use crate::query_native_runtime_boundary::TopologyNativeQueryRowField;
use crate::topology_operators::authority_identity::{
    existing_entity_authority, existing_relation_authority,
};
use crate::topology_operators::touched_graph_basis::{
    query_aspect_touch, topology_touched_aspect_from_schema_aspect,
};
use crate::topology_operators::TopologyDeclaredMutationMember;

use super::bindings::{query_entity_binding, query_relation_binding};
use super::{TopologyMutationApplicationError, TopologyMutationApplicationRunner};

impl<'workspace, 'surfaces> TopologyMutationApplicationRunner<'workspace, 'surfaces> {
    pub(crate) fn lower_retire_topology_entity(
        &self,
        builder: ForgeQueryMutationBatchBuilder,
        bindings: &TopologyQueryBindingIndex,
        entity_id: EntityId,
        expected_kind: TopologyEntityKind,
        contract: TopologyDeclaredMutationMember<'_>,
    ) -> Result<ForgeQueryMutationBatchBuilder, TopologyMutationApplicationError> {
        let binding = query_entity_binding(bindings, entity_id)?
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
        let binding = ForgeQueryExistingTruthTargetBinding::from_entity_target(
            ForgeQueryExistingEntityTarget::new(
                existing_entity_authority(entity_id)?,
                binding.query_identity,
            )?
            .in_target_collection("TopologyEntity")?,
        )?;
        Ok(builder.delete_existing_verified(
            binding,
            |verify| {
                verify.set_aspect(
                    TopologyNativeQueryRowField::TopologyKind.touch(),
                    TopologyNativeQueryRowField::TopologyKind
                        .authored_string(expected_kind.kind_name()),
                )
            },
            |delete| {
                let delete = delete.target_collection("TopologyEntity");
                delete.touches(contract.touched_aspects().iter().copied().map(|aspect| {
                    query_aspect_touch(topology_touched_aspect_from_schema_aspect(aspect))
                }))
            },
        ))
    }

    pub(super) fn lower_delete_existing_relation(
        &self,
        builder: ForgeQueryMutationBatchBuilder,
        bindings: &TopologyQueryBindingIndex,
        relation_id: RelationId,
        expected_kind: TopologyRelationKind,
        contract: TopologyDeclaredMutationMember<'_>,
    ) -> Result<ForgeQueryMutationBatchBuilder, TopologyMutationApplicationError> {
        let binding = query_relation_binding(bindings, relation_id)?
            .ok_or(TopologyMutationApplicationError::MissingExistingRelationBinding(relation_id))?;
        if binding.kind != expected_kind {
            return Err(
                TopologyMutationApplicationError::ExistingRelationKindMismatch {
                    relation_id,
                    expected: expected_kind,
                    actual: binding.kind,
                },
            );
        }
        let binding = ForgeQueryExistingTruthTargetBinding::from_relation_target(
            ForgeQueryExistingRelationTarget::new(
                existing_relation_authority(relation_id)?,
                binding.query_identity,
            )?
            .in_target_collection("TopologyRelation")?,
        )?;
        Ok(builder.delete_existing_verified(
            binding,
            |verify| {
                verify.set_aspect(
                    TopologyNativeQueryRowField::TopologyKind.touch(),
                    TopologyNativeQueryRowField::TopologyKind
                        .authored_string(expected_kind.kind_name()),
                )
            },
            |delete| {
                let delete = delete.target_collection("TopologyRelation");
                delete.touches(contract.touched_aspects().iter().copied().map(|aspect| {
                    query_aspect_touch(topology_touched_aspect_from_schema_aspect(aspect))
                }))
            },
        ))
    }
}
