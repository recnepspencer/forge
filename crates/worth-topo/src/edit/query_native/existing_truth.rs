use forge_query::facade::{
    ForgeQueryEntity, ForgeQueryExistingEntityTarget, ForgeQueryExistingRelationTarget,
    ForgeQueryMutationBatchBuilder,
};
use forge_relational::facade::identity::{EntityId, RelationId};
use worth_schema::facade::{WorthTopologyEntityKind, WorthTopologyRelationKind};

use super::bindings::{query_entity_binding, query_relation_binding};
use super::{
    WorthTopologyEditContract, WorthTopologyQueryEditExecutionError, WorthTopologyQueryEditRunner,
};

impl<'workspace, 'assembly> WorthTopologyQueryEditRunner<'workspace, 'assembly> {
    pub(super) fn lower_retire_topology_entity(
        &self,
        builder: ForgeQueryMutationBatchBuilder,
        entity_rows: &[ForgeQueryEntity],
        entity_id: EntityId,
        expected_kind: WorthTopologyEntityKind,
        contract: &WorthTopologyEditContract,
    ) -> Result<ForgeQueryMutationBatchBuilder, WorthTopologyQueryEditExecutionError> {
        let binding = query_entity_binding(entity_rows, entity_id)?
            .ok_or(WorthTopologyQueryEditExecutionError::MissingExistingEntityBinding(entity_id))?;
        if binding.kind != expected_kind {
            return Err(
                WorthTopologyQueryEditExecutionError::ExistingEntityKindMismatch {
                    entity_id,
                    expected: expected_kind,
                    actual: binding.kind,
                },
            );
        }
        let binding = self.workspace.bind_existing_entity(
            ForgeQueryExistingEntityTarget::new(format!("{entity_id:?}"), binding.query_identity)?
                .in_target_collection("WorthTopologyEntity")?,
        )?;
        Ok(builder.delete_existing_verified(
            binding,
            |verify| verify.aspect("topology.kind", expected_kind.kind_name()),
            |delete| {
                let mut delete = delete.target_collection("WorthTopologyEntity");
                for path in worth_schema::facade::worth_query_aspect_path_strings(
                    contract.touched_aspects.iter().copied(),
                ) {
                    delete = delete.touch(path);
                }
                delete
            },
        ))
    }

    pub(super) fn lower_delete_existing_relation(
        &self,
        builder: ForgeQueryMutationBatchBuilder,
        relation_rows: &[ForgeQueryEntity],
        relation_id: RelationId,
        expected_kind: WorthTopologyRelationKind,
        contract: &WorthTopologyEditContract,
    ) -> Result<ForgeQueryMutationBatchBuilder, WorthTopologyQueryEditExecutionError> {
        let binding = query_relation_binding(relation_rows, relation_id)?.ok_or(
            WorthTopologyQueryEditExecutionError::MissingExistingRelationBinding(relation_id),
        )?;
        if binding.kind != expected_kind {
            return Err(
                WorthTopologyQueryEditExecutionError::ExistingRelationKindMismatch {
                    relation_id,
                    expected: expected_kind,
                    actual: binding.kind,
                },
            );
        }
        let binding = self.workspace.bind_existing_relation(
            ForgeQueryExistingRelationTarget::new(
                format!("{relation_id:?}"),
                binding.query_identity,
            )?
            .in_target_collection("WorthTopologyRelation")?,
        )?;
        Ok(builder.delete_existing_verified(
            binding,
            |verify| verify.aspect("topology.kind", expected_kind.kind_name()),
            |delete| {
                let mut delete = delete.target_collection("WorthTopologyRelation");
                for path in worth_schema::facade::worth_query_aspect_path_strings(
                    contract.touched_aspects.iter().copied(),
                ) {
                    delete = delete.touch(path);
                }
                delete
            },
        ))
    }
}
