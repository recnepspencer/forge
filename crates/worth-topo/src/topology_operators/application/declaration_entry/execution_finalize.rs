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
use super::orchestration_boundary::operating_world_for_application_mode;
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
    mode: TopologyMutationApplicationMode,
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
        mode,
        sequence,
    )
}

pub(crate) fn ensure_declared_touched_basis_covers_sequence<I>(
    retained_handoff: &TopologyRetainedApplicationHandoff<I>,
    sequence: &TopologyDeclaredMutationSequence,
    mode: TopologyMutationApplicationMode,
) -> Result<(), TopologyMutationApplicationError>
where
    I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
{
    let proof = retained_handoff.declared_touched_basis_proof();
    if proof.covers_sequence(sequence)
        && proof.basis().operating_world() == &operating_world_for_application_mode(mode)
    {
        return Ok(());
    }
    Err(TopologyMutationApplicationError::DeclarationEntry {
        family: sequence
            .families()
            .first()
            .copied()
            .unwrap_or(crate::topology_operators::TopologyMutationFamily::CreateTopologyEntity),
        stop_class: super::super::TopologyDeclarationEntryStopClass::BasisMismatch,
        stop_stage: None,
        refusal_class: Some(
            super::super::error::TopologyDeclarationEntryRefusalClass::AuthorityTransitionRequired,
        ),
        recovery: None,
        graph_obligation_envelope_digest: None,
        reason: format!(
            "declared touched graph basis `{}` does not cover mutation sequence `{}` before commit",
            proof.basis_digest(),
            sequence.topology_mutation_digest().digest.digest_hex
        ),
    })
}
