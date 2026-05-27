use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
};
use crate::binding_pipeline::{ForgeQueryBindingRequestDescriptor, ForgeQueryBindingWitnessCheck};
use crate::identity::hash_parts;

use super::support::{linked_from_prepared, transcript_digest};
use super::ForgeQueryContinuationExecution;
use crate::continuation_pipeline::request::ForgeQueryExecutePreparedContinuationRequest;
use crate::continuation_pipeline::transcript::ForgeQueryContinuationExecutionTranscript;
use crate::continuation_pipeline::ForgeQueryContinuationExecutionOutcome;

pub(crate) fn execute_prepared_continuation_on_handle<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    request: ForgeQueryExecutePreparedContinuationRequest<D, I>,
) -> ForgeQueryContinuationExecutionTranscript<D, I> {
    let prepared = request.into_prepared();
    let request_descriptor = ForgeQueryBindingRequestDescriptor::new(
        I::Family::semantic_family_key(),
        "execute_prepared_continuation",
        prepared.bridge_routing().aspect_contract().clone(),
    );
    let linked = linked_from_prepared(&prepared);

    if prepared.handle_identity_digest() != handle.handle_identity_digest() {
        return ForgeQueryContinuationExecutionTranscript::new(
            request_descriptor,
            ForgeQueryContinuationExecutionOutcome::WrongHandle(
                "the prepared continuation belongs to a different admitted handle".to_string(),
            ),
            vec![ForgeQueryBindingWitnessCheck::failed(
                "handle_alignment",
                "prepared continuation handle digest did not match",
            )],
            transcript_digest(
                "execute_prepared_continuation",
                I::Family::semantic_family_key(),
                &linked,
                "wrong_handle",
            ),
            linked,
        );
    }

    if prepared.operating_context_identity_digest() != handle.operating_context_identity_digest() {
        return ForgeQueryContinuationExecutionTranscript::new(
            request_descriptor,
            ForgeQueryContinuationExecutionOutcome::WrongWorld(
                "the prepared continuation belongs to a different admitted world".to_string(),
            ),
            vec![ForgeQueryBindingWitnessCheck::failed(
                "world_alignment",
                "prepared continuation operating context digest did not match",
            )],
            transcript_digest(
                "execute_prepared_continuation",
                I::Family::semantic_family_key(),
                &linked,
                "wrong_world",
            ),
            linked,
        );
    }

    let execution_digest = hash_parts(&[
        "forge_query_continuation_execution_v1".to_string(),
        prepared.prepared_digest().to_string(),
        format!("family:{:?}", prepared.family()),
        format!("workspace:{:?}", prepared.workspace_contract()),
        format!("runtime:{:?}", prepared.runtime_contract()),
        format!("signal:{:?}", prepared.signal_execution_family()),
    ]);
    let binding_surface = prepared.bridge_routing().binding().surface().to_string();
    let signal_execution_family = prepared.signal_execution_family();
    let execution = ForgeQueryContinuationExecution::new(
        prepared,
        signal_execution_family,
        binding_surface,
        execution_digest.clone(),
    );

    ForgeQueryContinuationExecutionTranscript::new(
        request_descriptor,
        ForgeQueryContinuationExecutionOutcome::Executed(execution),
        vec![
            ForgeQueryBindingWitnessCheck::passed("world_alignment"),
            ForgeQueryBindingWitnessCheck::passed("handle_alignment"),
            ForgeQueryBindingWitnessCheck::passed("prepared_continuation"),
        ],
        execution_digest,
        linked,
    )
}
