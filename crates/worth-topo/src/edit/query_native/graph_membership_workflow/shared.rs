use forge_query::facade::{
    ForgeQueryEntity, ForgeQueryExistingEntityTarget, ForgeQueryExistingRelationTarget,
    ForgeQueryExistingTruthTargetBinding, ForgeQueryGraphCompositionBuilder,
    ForgeQueryRuntimeError,
};
use forge_relational::facade::identity::{EntityId, RelationId};
use schema::facade::TopologyEntityKind;

use crate::edit::TopologyEditContract;
use crate::edit::{TopologyQueryEditExecutionError, TopologyQueryEditRunner};

pub(super) fn bind_existing_relation_handle(
    runner: &TopologyQueryEditRunner<'_, '_>,
    relation_id: RelationId,
    query_identity: &str,
) -> Result<ForgeQueryExistingTruthTargetBinding, ForgeQueryRuntimeError> {
    runner.workspace.bind_existing_relation(
        ForgeQueryExistingRelationTarget::new(format!("{relation_id:?}"), query_identity)?
            .in_target_collection("TopologyRelation")?,
    )
}

pub(super) fn bind_existing_entity_handle(
    runner: &TopologyQueryEditRunner<'_, '_>,
    entity_rows: &[ForgeQueryEntity],
    entity_id: EntityId,
    expected_kind: TopologyEntityKind,
) -> Result<ForgeQueryExistingTruthTargetBinding, TopologyQueryEditExecutionError> {
    let binding = super::super::bindings::query_entity_binding(entity_rows, entity_id)?
        .ok_or(TopologyQueryEditExecutionError::MissingExistingEntityBinding(entity_id))?;
    if binding.kind != expected_kind {
        return Err(
            TopologyQueryEditExecutionError::ExistingEntityKindMismatch {
                entity_id,
                expected: expected_kind,
                actual: binding.kind,
            },
        );
    }
    Ok(runner.workspace.bind_existing_entity(
        ForgeQueryExistingEntityTarget::new(format!("{entity_id:?}"), binding.query_identity)?
            .in_target_collection("TopologyEntity")?,
    )?)
}

pub(super) fn delete_existing_entity_from_graph(
    graph: &mut ForgeQueryGraphCompositionBuilder,
    binding: ForgeQueryExistingTruthTargetBinding,
    target_collection: &str,
    expected_kind_name: &str,
    contract: &TopologyEditContract,
) -> Result<(), ForgeQueryRuntimeError> {
    graph.delete_existing_verified(
        binding,
        |verify| verify.aspect("topology.kind", expected_kind_name),
        |delete| {
            let mut delete = delete.target_collection(target_collection);
            for path in
                schema::facade::query_aspect_path_strings(contract.touched_aspects.iter().copied())
            {
                delete = delete.touch(path);
            }
            delete
        },
    )?;
    Ok(())
}
