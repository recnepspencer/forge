use std::collections::BTreeMap;

use crate::topology_operators::TopologyDeclaredMutationSequence;
use forge_query::facade::ForgeQueryDeclarationInput;
use forge_query::facade::ForgeQueryMutationBatchBuilder;
use schema::facade::platform::entities::TopologyEntityKind;

use super::super::{
    finalize_batch_write_closeout, TopologyDeclaredMutationArtifact,
    TopologyMutationApplicationError, TopologyMutationApplicationMode,
    TopologyMutationApplicationRunner, TopologyQueryBindingIndex,
    TopologyRetainedApplicationHandoff,
};
use crate::query_domain::TopologyQueryDomain;

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

pub(super) fn finalize_lowered_mutations<I>(
    runner: &mut TopologyMutationApplicationRunner<'_, '_>,
    retained_handoff: TopologyRetainedApplicationHandoff<I>,
    lowered_mutations: ForgeQueryMutationBatchBuilder,
    semantic_family_key: &'static str,
    _mode: TopologyMutationApplicationMode,
    sequence: &TopologyDeclaredMutationSequence,
) -> Result<TopologyDeclaredMutationArtifact, TopologyMutationApplicationError>
where
    I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
{
    finalize_batch_write_closeout(
        runner,
        retained_handoff,
        lowered_mutations,
        semantic_family_key,
        sequence,
    )
}
