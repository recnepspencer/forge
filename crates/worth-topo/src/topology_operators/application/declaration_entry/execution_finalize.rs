use std::collections::BTreeMap;

use crate::topology_operators::TopologyDeclaredMutationSequence;
use forge_query::facade::{
    ForgeQueryBatchWriteReceipt, ForgeQueryBatchWriteReceiptInspection, ForgeQueryInspection,
    ForgeQueryMutationBatchBuilder,
};
use schema::facade::platform::entities::TopologyEntityKind;

use super::super::{
    load_post_write_materialized_topology, TopologyDeclaredMutationArtifact,
    TopologyMutationApplicationError, TopologyMutationApplicationMode,
    TopologyMutationApplicationRunner, TopologyQueryBindingIndex,
};

pub(super) fn lower_mutation_sequence(
    runner: &TopologyMutationApplicationRunner<'_, '_>,
    sequence: &TopologyDeclaredMutationSequence,
    bindings: &TopologyQueryBindingIndex,
    created_entity_kinds: &BTreeMap<String, TopologyEntityKind>,
) -> Result<ForgeQueryMutationBatchBuilder, TopologyMutationApplicationError> {
    sequence
        .members()
        .try_fold(ForgeQueryMutationBatchBuilder::new(), |builder, member| {
            runner.lower_mutation_member(builder, bindings, created_entity_kinds, member)
        })
}

pub(super) fn finalize_lowered_mutations(
    runner: &mut TopologyMutationApplicationRunner<'_, '_>,
    lowered_mutations: ForgeQueryMutationBatchBuilder,
    semantic_family_key: &'static str,
    _mode: TopologyMutationApplicationMode,
    sequence: &TopologyDeclaredMutationSequence,
) -> Result<TopologyDeclaredMutationArtifact, TopologyMutationApplicationError> {
    let receipt = runner.workspace.batch(|_| lowered_mutations)?;
    let inspection = load_mutation_receipt_inspection(runner, &receipt)?;
    let materialized = load_post_write_materialized_topology(runner.workspace, runner.surfaces)?;
    Ok(TopologyDeclaredMutationArtifact::from_receipt(
        semantic_family_key,
        sequence,
        receipt,
        inspection,
        materialized,
    ))
}

fn load_mutation_receipt_inspection(
    runner: &mut TopologyMutationApplicationRunner<'_, '_>,
    receipt: &ForgeQueryBatchWriteReceipt,
) -> Result<ForgeQueryBatchWriteReceiptInspection, TopologyMutationApplicationError> {
    match runner.workspace.inspect(receipt)? {
        ForgeQueryInspection::BatchWriteReceipt(inspection) => Ok(inspection),
        _ => Err(TopologyMutationApplicationError::UnexpectedInspectionFamily),
    }
}
