use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
};
use crate::binding_pipeline::{ForgeQueryBindingRequestDescriptor, ForgeQueryBindingWitnessCheck};
use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};

use super::readmission::{
    current_readmission_evidence_from_handle, validate_execution_readmission,
    ForgeQueryPreparedContinuationExecutionReadmissionDenial,
};
use super::support::{linked_artifacts_identity, linked_from_prepared, transcript_digest};
use super::ForgeQueryContinuationExecution;
use crate::continuation_pipeline::request::ForgeQueryExecutePreparedContinuationRequest;
use crate::continuation_pipeline::transcript::ForgeQueryContinuationExecutionTranscript;
use crate::continuation_pipeline::ForgeQueryContinuationExecutionOutcome;

fn missing_required_capability<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    prepared: &super::ForgeQueryPreparedContinuation<D, I>,
) -> Option<crate::application::ForgeQueryCapabilityFamily> {
    prepared
        .required_capability_families()
        .iter()
        .copied()
        .find(|family| {
            handle.support_snapshot().capability_status(*family)
                != Some(crate::application::ForgeQueryCapabilityStatus::Admitted)
        })
}

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

    if let Some(family) = missing_required_capability(handle, &prepared) {
        return ForgeQueryContinuationExecutionTranscript::new(
            request_descriptor,
            ForgeQueryContinuationExecutionOutcome::Unsupported(format!(
                "the current admitted handle no longer admits required continuation capability {}",
                family.as_str()
            )),
            vec![ForgeQueryBindingWitnessCheck::failed(
                "execution_support",
                "required continuation capability is not currently admitted",
            )],
            transcript_digest(
                "execute_prepared_continuation",
                I::Family::semantic_family_key(),
                &linked,
                "unsupported",
            ),
            linked,
        );
    }

    let current_evidence = current_readmission_evidence_from_handle(handle, &prepared);
    if let Err(denial) = validate_execution_readmission(&prepared, &current_evidence) {
        return transcript_from_readmission_denial(request_descriptor, linked, denial);
    }

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

    let execution_digest =
        forge_query_evidence_identity(ForgeQueryEvidenceScope::ContinuationExecutionDigest)
            .field_identity(
                ForgeQueryEvidenceTag::new("prepared"),
                prepared.prepared_digest(),
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("family"),
                prepared.family().as_str(),
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("truth_context"),
                prepared.truth_context().as_str(),
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("basis_posture"),
                prepared.basis_posture().as_str(),
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("workspace_contract"),
                prepared.workspace_contract().as_str(),
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("runtime_contract"),
                prepared.runtime_contract().as_str(),
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("execution_mode"),
                prepared.execution_mode().as_str(),
            )
            .field_value_sequence(
                ForgeQueryEvidenceTag::new("required_basis_families"),
                prepared
                    .required_basis_families()
                    .iter()
                    .map(|family| family.as_str()),
            )
            .field_value_sequence(
                ForgeQueryEvidenceTag::new("required_capability_families"),
                prepared
                    .required_capability_families()
                    .iter()
                    .map(|family| family.as_str()),
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("signal_posture"),
                prepared.signal_posture().as_str(),
            )
            .optional_shape(
                ForgeQueryEvidenceTag::new("signal_execution_family"),
                prepared
                    .signal_execution_family()
                    .map(|family| family.as_str()),
            )
            .optional_value(
                ForgeQueryEvidenceTag::new("signal_compatibility"),
                prepared.signal_compatibility_digest(),
            )
            .field_identity(
                ForgeQueryEvidenceTag::new("linked_artifacts"),
                linked_artifacts_identity(&linked),
            )
            .seal()
            .as_str()
            .to_string();
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
            ForgeQueryBindingWitnessCheck::passed("execution_support"),
            ForgeQueryBindingWitnessCheck::passed("handle_alignment"),
            ForgeQueryBindingWitnessCheck::passed("prepared_continuation"),
        ],
        execution_digest,
        linked,
    )
}

fn transcript_from_readmission_denial<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    request_descriptor: ForgeQueryBindingRequestDescriptor,
    linked: crate::binding_pipeline::ForgeQueryBindingLinkedArtifacts,
    denial: ForgeQueryPreparedContinuationExecutionReadmissionDenial,
) -> ForgeQueryContinuationExecutionTranscript<D, I> {
    match denial {
        ForgeQueryPreparedContinuationExecutionReadmissionDenial::Stale(reason) => {
            ForgeQueryContinuationExecutionTranscript::new(
                request_descriptor,
                ForgeQueryContinuationExecutionOutcome::Stale(reason),
                vec![
                    ForgeQueryBindingWitnessCheck::passed("world_alignment"),
                    ForgeQueryBindingWitnessCheck::passed("execution_support"),
                    ForgeQueryBindingWitnessCheck::failed(
                        "basis_freshness",
                        "retained continuation basis evidence is stale",
                    ),
                ],
                transcript_digest(
                    "execute_prepared_continuation",
                    I::Family::semantic_family_key(),
                    &linked,
                    "stale",
                ),
                linked,
            )
        }
        ForgeQueryPreparedContinuationExecutionReadmissionDenial::AsyncRequestDrift(reason) => {
            ForgeQueryContinuationExecutionTranscript::new(
                request_descriptor,
                ForgeQueryContinuationExecutionOutcome::AsyncRequestDrift(reason),
                vec![
                    ForgeQueryBindingWitnessCheck::passed("world_alignment"),
                    ForgeQueryBindingWitnessCheck::passed("execution_support"),
                    ForgeQueryBindingWitnessCheck::passed("basis_freshness"),
                    ForgeQueryBindingWitnessCheck::failed(
                        "async_request_alignment",
                        "current continuation async request identity drifted from the retained request",
                    ),
                ],
                transcript_digest(
                    "execute_prepared_continuation",
                    I::Family::semantic_family_key(),
                    &linked,
                    "async_request_drift",
                ),
                linked,
            )
        }
        ForgeQueryPreparedContinuationExecutionReadmissionDenial::ReplayDrift(reason) => {
            ForgeQueryContinuationExecutionTranscript::new(
                request_descriptor,
                ForgeQueryContinuationExecutionOutcome::ReplayDrift(reason),
                vec![
                    ForgeQueryBindingWitnessCheck::passed("world_alignment"),
                    ForgeQueryBindingWitnessCheck::passed("execution_support"),
                    ForgeQueryBindingWitnessCheck::passed("basis_freshness"),
                    ForgeQueryBindingWitnessCheck::failed(
                        "replay_alignment",
                        "current continuation replay identity drifted from the retained replay witness",
                    ),
                ],
                transcript_digest(
                    "execute_prepared_continuation",
                    I::Family::semantic_family_key(),
                    &linked,
                    "replay_drift",
                ),
                linked,
            )
        }
        ForgeQueryPreparedContinuationExecutionReadmissionDenial::RemaskDrift(reason) => {
            ForgeQueryContinuationExecutionTranscript::new(
                request_descriptor,
                ForgeQueryContinuationExecutionOutcome::RemaskDrift(reason),
                vec![
                    ForgeQueryBindingWitnessCheck::passed("world_alignment"),
                    ForgeQueryBindingWitnessCheck::passed("execution_support"),
                    ForgeQueryBindingWitnessCheck::passed("basis_freshness"),
                    ForgeQueryBindingWitnessCheck::failed(
                        "remask_alignment",
                        "current continuation meaning was remasked before execution",
                    ),
                ],
                transcript_digest(
                    "execute_prepared_continuation",
                    I::Family::semantic_family_key(),
                    &linked,
                    "remask_drift",
                ),
                linked,
            )
        }
        ForgeQueryPreparedContinuationExecutionReadmissionDenial::PreviewCrossedResidue(
            reason,
        ) => ForgeQueryContinuationExecutionTranscript::new(
            request_descriptor,
            ForgeQueryContinuationExecutionOutcome::PreviewCrossedResidue(reason),
            vec![
                ForgeQueryBindingWitnessCheck::passed("world_alignment"),
                ForgeQueryBindingWitnessCheck::passed("execution_support"),
                ForgeQueryBindingWitnessCheck::passed("basis_freshness"),
                ForgeQueryBindingWitnessCheck::failed(
                    "preview_residue_alignment",
                    "current continuation crossed preview residue before execution",
                ),
            ],
            transcript_digest(
                "execute_prepared_continuation",
                I::Family::semantic_family_key(),
                &linked,
                "preview_crossed_residue",
            ),
            linked,
        ),
        ForgeQueryPreparedContinuationExecutionReadmissionDenial::StaleCompletion(reason) => {
            ForgeQueryContinuationExecutionTranscript::new(
                request_descriptor,
                ForgeQueryContinuationExecutionOutcome::StaleCompletion(reason),
                vec![
                    ForgeQueryBindingWitnessCheck::passed("world_alignment"),
                    ForgeQueryBindingWitnessCheck::passed("execution_support"),
                    ForgeQueryBindingWitnessCheck::passed("basis_freshness"),
                    ForgeQueryBindingWitnessCheck::failed(
                        "completion_freshness",
                        "current continuation completion posture is stale at execution time",
                    ),
                ],
                transcript_digest(
                    "execute_prepared_continuation",
                    I::Family::semantic_family_key(),
                    &linked,
                    "stale_completion",
                ),
                linked,
            )
        }
        ForgeQueryPreparedContinuationExecutionReadmissionDenial::BasisMismatch(reason) => {
            ForgeQueryContinuationExecutionTranscript::new(
                request_descriptor,
                ForgeQueryContinuationExecutionOutcome::BasisMismatch(reason),
                vec![
                    ForgeQueryBindingWitnessCheck::passed("world_alignment"),
                    ForgeQueryBindingWitnessCheck::passed("execution_support"),
                    ForgeQueryBindingWitnessCheck::passed("basis_freshness"),
                    ForgeQueryBindingWitnessCheck::failed(
                        "basis_alignment",
                        "current lower-runtime basis evidence drifted from the retained continuation basis",
                    ),
                ],
                transcript_digest(
                    "execute_prepared_continuation",
                    I::Family::semantic_family_key(),
                    &linked,
                    "basis_mismatch",
                ),
                linked,
            )
        }
        ForgeQueryPreparedContinuationExecutionReadmissionDenial::AuthorityMismatch(reason) => {
            ForgeQueryContinuationExecutionTranscript::new(
                request_descriptor,
                ForgeQueryContinuationExecutionOutcome::AuthorityMismatch(reason),
                vec![
                    ForgeQueryBindingWitnessCheck::passed("world_alignment"),
                    ForgeQueryBindingWitnessCheck::passed("execution_support"),
                    ForgeQueryBindingWitnessCheck::passed("basis_freshness"),
                    ForgeQueryBindingWitnessCheck::passed("basis_alignment"),
                    ForgeQueryBindingWitnessCheck::failed(
                        "authority_alignment",
                        "current lower-runtime authority no longer matches retained continuation authority",
                    ),
                ],
                transcript_digest(
                    "execute_prepared_continuation",
                    I::Family::semantic_family_key(),
                    &linked,
                    "authority_mismatch",
                ),
                linked,
            )
        }
    }
}
