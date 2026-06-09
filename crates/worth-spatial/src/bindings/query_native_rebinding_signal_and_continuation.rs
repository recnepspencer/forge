use forge_query::facade::{
    ForgeQueryContinuationExecution, ForgeQueryContinuationExecutionChecked,
    ForgeQueryContinuationExecutionOutcome, ForgeQueryContinuationExecutionTranscript,
    ForgeQueryDeclarationEnvelope, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationSignalCompatibilityInput, ForgeQueryPreparedContinuation,
    ForgeQueryPreparedContinuationChecked, ForgeQueryPreparedContinuationOutcome,
    ForgeQueryPreparedContinuationTranscript, ForgeQueryResolveContinuationFromTargetRequest,
    ForgeQuerySignalCompatibilityOrchestration, ForgeQuerySignalCompatibilityOrchestrationChecked,
    ForgeQuerySignalCompatibilityOrchestrationInput,
    ForgeQuerySignalCompatibilityOrchestrationOutcome,
    ForgeQuerySignalCompatibilityOrchestrationTranscript,
};

use crate::bindings::query_native_rebinding::{
    PrimitiveRebindingDeclarationFamily, PrimitiveRebindingQueryDomain,
};
use crate::bindings::query_native_rebinding_authoring::PrimitiveRebindingDeclarationEntry;

pub type PrimitiveRebindingSignalCompatibilitySubject =
    ForgeQueryDeclarationSignalCompatibilityInput<
        PrimitiveRebindingQueryDomain,
        PrimitiveRebindingDeclarationEntry,
    >;
pub type PrimitiveRebindingSignalCompatibilityInput =
    ForgeQuerySignalCompatibilityOrchestrationInput<
        PrimitiveRebindingQueryDomain,
        PrimitiveRebindingDeclarationEntry,
    >;
pub type PrimitiveRebindingSignalCompatibilityArtifact = ForgeQuerySignalCompatibilityOrchestration<
    PrimitiveRebindingQueryDomain,
    PrimitiveRebindingDeclarationEntry,
>;
pub type PrimitiveRebindingSignalCompatibilityChecked =
    ForgeQuerySignalCompatibilityOrchestrationChecked<
        PrimitiveRebindingQueryDomain,
        PrimitiveRebindingDeclarationEntry,
    >;
pub type PrimitiveRebindingSignalCompatibilityOutcome =
    ForgeQuerySignalCompatibilityOrchestrationOutcome<
        PrimitiveRebindingQueryDomain,
        PrimitiveRebindingDeclarationEntry,
    >;
pub type PrimitiveRebindingSignalCompatibilityProof =
    ForgeQuerySignalCompatibilityOrchestrationTranscript<
        PrimitiveRebindingQueryDomain,
        PrimitiveRebindingDeclarationEntry,
    >;

pub type PrimitiveRebindingContinuationTarget = ForgeQueryResolveContinuationFromTargetRequest<
    PrimitiveRebindingQueryDomain,
    PrimitiveRebindingDeclarationEntry,
>;
pub type PrimitiveRebindingPreparedContinuation = ForgeQueryPreparedContinuation<
    PrimitiveRebindingQueryDomain,
    PrimitiveRebindingDeclarationEntry,
>;
pub type PrimitiveRebindingPreparedContinuationChecked = ForgeQueryPreparedContinuationChecked<
    PrimitiveRebindingQueryDomain,
    PrimitiveRebindingDeclarationEntry,
>;
pub type PrimitiveRebindingPreparedContinuationOutcome = ForgeQueryPreparedContinuationOutcome<
    PrimitiveRebindingQueryDomain,
    PrimitiveRebindingDeclarationEntry,
>;
pub type PrimitiveRebindingPreparedContinuationProof = ForgeQueryPreparedContinuationTranscript<
    PrimitiveRebindingQueryDomain,
    PrimitiveRebindingDeclarationEntry,
>;
pub type PrimitiveRebindingContinuationExecution = ForgeQueryContinuationExecution<
    PrimitiveRebindingQueryDomain,
    PrimitiveRebindingDeclarationEntry,
>;
pub type PrimitiveRebindingContinuationExecutionChecked = ForgeQueryContinuationExecutionChecked<
    PrimitiveRebindingQueryDomain,
    PrimitiveRebindingDeclarationEntry,
>;
pub type PrimitiveRebindingContinuationExecutionOutcome = ForgeQueryContinuationExecutionOutcome<
    PrimitiveRebindingQueryDomain,
    PrimitiveRebindingDeclarationEntry,
>;
pub type PrimitiveRebindingContinuationExecutionProof = ForgeQueryContinuationExecutionTranscript<
    PrimitiveRebindingQueryDomain,
    PrimitiveRebindingDeclarationEntry,
>;

pub fn primitive_rebinding_signal_workflow(
    envelope: ForgeQueryDeclarationEnvelope<
        PrimitiveRebindingQueryDomain,
        PrimitiveRebindingDeclarationEntry,
    >,
) -> PrimitiveRebindingSignalCompatibilityInput {
    ForgeQuerySignalCompatibilityOrchestrationInput::new(
        ForgeQueryDeclarationSignalCompatibilityInput::enveloped(envelope),
    )
}

pub fn primitive_rebinding_continuation_target(
    envelope: ForgeQueryDeclarationEnvelope<
        PrimitiveRebindingQueryDomain,
        PrimitiveRebindingDeclarationEntry,
    >,
) -> PrimitiveRebindingContinuationTarget {
    ForgeQueryResolveContinuationFromTargetRequest::new(
        envelope,
        PrimitiveRebindingDeclarationFamily::aspect_contract(),
    )
}
