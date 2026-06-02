use forge_query::facade::{
    ForgeQueryAdmittedDeclarationProgression, ForgeQueryCanonicalDeclarationArtifact,
    ForgeQueryContinuationExecution, ForgeQueryContinuationExecutionChecked,
    ForgeQueryContinuationExecutionOutcome, ForgeQueryContinuationExecutionTranscript,
    ForgeQueryContributionComposedOrchestration,
    ForgeQueryContributionComposedOrchestrationChecked,
    ForgeQueryContributionComposedOrchestrationInput,
    ForgeQueryContributionComposedOrchestrationOutcome,
    ForgeQueryContributionComposedOrchestrationTranscript, ForgeQueryContributionIntent,
    ForgeQueryDeclarationAdmissionError, ForgeQueryDeclarationEntryContributionComposition,
    ForgeQueryDeclarationEntryOrchestrationChecked, ForgeQueryDeclarationEntryOrchestrationProof,
    ForgeQueryDeclarationEntryOrchestrationTerminalError,
    ForgeQueryDeclarationEntryProgressionError, ForgeQueryDeclarationEnvelope,
    ForgeQueryDeclarationEnvelopeChecked, ForgeQueryDeclarationEnvelopeOrchestrationTranscript,
    ForgeQueryDeclarationEnvelopeTerminalError, ForgeQueryDeclarationLegalityDenial,
    ForgeQueryDeclarationLegalityEvidence, ForgeQueryDeclarationReceipt,
    ForgeQueryDeclarationReceiptChecked, ForgeQueryDeclarationReceiptOrchestrationTranscript,
    ForgeQueryDeclarationReceiptTerminalError, ForgeQueryDeclarationRouteOrchestrationTranscript,
    ForgeQueryDeclarationRoutePlan, ForgeQueryDeclarationRoutePlanChecked,
    ForgeQueryDeclarationRoutePlanTerminalError, ForgeQueryDeclarationSignalCompatibilityInput,
    ForgeQueryGroupedContributionComposition, ForgeQueryGroupedContributionInput,
    ForgeQueryGroupedContributionMemberContext, ForgeQueryGroupedContributionStop,
    ForgeQueryGroupedDeclarationArtifact, ForgeQueryGroupedDeclarationInput,
    ForgeQueryGroupedDeclarationStop, ForgeQueryGroupedOrchestration, ForgeQueryOrdinaryOutcome,
    ForgeQueryPreparedContinuation, ForgeQueryPreparedContinuationChecked,
    ForgeQueryPreparedContinuationOutcome, ForgeQueryPreparedContinuationTranscript,
    ForgeQueryResolveContinuationFromTargetRequest, ForgeQuerySignalCompatibilityOrchestration,
    ForgeQuerySignalCompatibilityOrchestrationChecked,
    ForgeQuerySignalCompatibilityOrchestrationInput,
    ForgeQuerySignalCompatibilityOrchestrationOutcome,
    ForgeQuerySignalCompatibilityOrchestrationTranscript,
};

use crate::query_domain::TopologyQueryDomain;

pub(crate) mod contribution_declaration_private {
    pub trait Sealed {}
}

pub trait TopologyOperatorContributionDeclaration:
    forge_query::facade::ForgeQueryDeclarationInput<TopologyQueryDomain>
    + Clone
    + contribution_declaration_private::Sealed
{
    #[doc(hidden)]
    fn topology_semantic_contributions(&self) -> Vec<ForgeQueryContributionIntent>;
}

pub type TopologyOperatorCanonicalDeclaration<I> =
    ForgeQueryCanonicalDeclarationArtifact<TopologyQueryDomain, I>;
pub type TopologyOperatorDeclarationAdmissionError<I> =
    ForgeQueryDeclarationAdmissionError<TopologyQueryDomain, I>;
pub type TopologyOperatorDeclarationLegalityEvidence<I> =
    ForgeQueryDeclarationLegalityEvidence<TopologyQueryDomain, I>;
pub type TopologyOperatorDeclarationLegalityDenial<I> =
    ForgeQueryDeclarationLegalityDenial<TopologyQueryDomain, I>;
pub type TopologyOperatorDeclarationOutcome<I> =
    ForgeQueryOrdinaryOutcome<ForgeQueryDeclarationEnvelope<TopologyQueryDomain, I>>;
pub type TopologyOperatorEnvelope<I> = ForgeQueryDeclarationEnvelope<TopologyQueryDomain, I>;
pub type TopologyOperatorEnvelopeChecked<I> =
    ForgeQueryDeclarationEntryOrchestrationChecked<TopologyQueryDomain, I>;
pub type TopologyOperatorEnvelopeProof<I> =
    ForgeQueryDeclarationEntryOrchestrationProof<TopologyQueryDomain, I>;
pub type TopologyOperatorEnvelopeTerminalError<I> =
    ForgeQueryDeclarationEntryOrchestrationTerminalError<TopologyQueryDomain, I>;
pub type TopologyOperatorProgressedDeclaration<I> =
    ForgeQueryAdmittedDeclarationProgression<TopologyQueryDomain, I>;
pub type TopologyOperatorProgressionError<I> =
    ForgeQueryDeclarationEntryProgressionError<TopologyQueryDomain, I>;
pub type TopologyOperatorRoutePlan<I> = ForgeQueryDeclarationRoutePlan<TopologyQueryDomain, I>;
pub type TopologyOperatorRoutePlanChecked<I> =
    ForgeQueryDeclarationRoutePlanChecked<TopologyQueryDomain, I>;
pub type TopologyOperatorRoutePlanProof<I> =
    ForgeQueryDeclarationRouteOrchestrationTranscript<TopologyQueryDomain, I>;
pub type TopologyOperatorRoutePlanTerminalError<I> =
    ForgeQueryDeclarationRoutePlanTerminalError<TopologyQueryDomain, I>;
pub type TopologyOperatorSignalCompatibilitySubject<I> =
    ForgeQueryDeclarationSignalCompatibilityInput<TopologyQueryDomain, I>;
pub type TopologyOperatorSignalCompatibilityInput<I> =
    ForgeQuerySignalCompatibilityOrchestrationInput<TopologyQueryDomain, I>;
pub type TopologyOperatorSignalCompatibilityArtifact<I> =
    ForgeQuerySignalCompatibilityOrchestration<TopologyQueryDomain, I>;
pub type TopologyOperatorSignalCompatibilityChecked<I> =
    ForgeQuerySignalCompatibilityOrchestrationChecked<TopologyQueryDomain, I>;
pub type TopologyOperatorSignalCompatibilityOutcome<I> =
    ForgeQuerySignalCompatibilityOrchestrationOutcome<TopologyQueryDomain, I>;
pub type TopologyOperatorSignalCompatibilityProof<I> =
    ForgeQuerySignalCompatibilityOrchestrationTranscript<TopologyQueryDomain, I>;
pub type TopologyOperatorDeclarationReceipt<I> =
    ForgeQueryDeclarationReceipt<TopologyQueryDomain, I>;
pub type TopologyOperatorDeclarationReceiptChecked<I> =
    ForgeQueryDeclarationReceiptChecked<TopologyQueryDomain, I>;
pub type TopologyOperatorDeclarationReceiptProof<I> =
    ForgeQueryDeclarationReceiptOrchestrationTranscript<TopologyQueryDomain, I>;
pub type TopologyOperatorDeclarationReceiptTerminalError<I> =
    ForgeQueryDeclarationReceiptTerminalError<TopologyQueryDomain, I>;
pub type TopologyOperatorEnvelopeFromProgressedChecked<I> =
    ForgeQueryDeclarationEnvelopeChecked<TopologyQueryDomain, I>;
pub type TopologyOperatorEnvelopeFromProgressedTerminalError<I> =
    ForgeQueryDeclarationEnvelopeTerminalError<TopologyQueryDomain, I>;
pub type TopologyOperatorContinuationTarget<I> =
    ForgeQueryResolveContinuationFromTargetRequest<TopologyQueryDomain, I>;
pub type TopologyOperatorPreparedContinuation<I> =
    ForgeQueryPreparedContinuation<TopologyQueryDomain, I>;
pub type TopologyOperatorPreparedContinuationChecked<I> =
    ForgeQueryPreparedContinuationChecked<TopologyQueryDomain, I>;
pub type TopologyOperatorPreparedContinuationOutcome<I> =
    ForgeQueryPreparedContinuationOutcome<TopologyQueryDomain, I>;
pub type TopologyOperatorPreparedContinuationProof<I> =
    ForgeQueryPreparedContinuationTranscript<TopologyQueryDomain, I>;
pub type TopologyOperatorContinuationExecution<I> =
    ForgeQueryContinuationExecution<TopologyQueryDomain, I>;
pub type TopologyOperatorContinuationExecutionChecked<I> =
    ForgeQueryContinuationExecutionChecked<TopologyQueryDomain, I>;
pub type TopologyOperatorContinuationExecutionOutcome<I> =
    ForgeQueryContinuationExecutionOutcome<TopologyQueryDomain, I>;
pub type TopologyOperatorContinuationExecutionProof<I> =
    ForgeQueryContinuationExecutionTranscript<TopologyQueryDomain, I>;
pub type TopologyOperatorEnvelopeFromProgressedProof<I> =
    ForgeQueryDeclarationEnvelopeOrchestrationTranscript<TopologyQueryDomain, I>;
pub type TopologyOperatorGroupedInput<I> =
    ForgeQueryGroupedDeclarationInput<TopologyQueryDomain, I>;
pub type TopologyOperatorGroupedDeclaration<I> =
    ForgeQueryGroupedDeclarationArtifact<TopologyQueryDomain, I>;
pub type TopologyOperatorGroupedDeclarationStop = ForgeQueryGroupedDeclarationStop;
pub type TopologyOperatorGroupedOutcome<I> =
    ForgeQueryOrdinaryOutcome<ForgeQueryGroupedOrchestration<TopologyQueryDomain, I>>;
pub type TopologyOperatorGroupedContributionInput<I> =
    ForgeQueryGroupedContributionInput<TopologyQueryDomain, I>;
pub type TopologyOperatorGroupedContributionComposition<I> =
    ForgeQueryGroupedContributionComposition<TopologyQueryDomain, I>;
pub type TopologyOperatorGroupedContributionStop<I> =
    ForgeQueryGroupedContributionStop<TopologyQueryDomain, I>;
pub type TopologyOperatorGroupedContributionMemberContext =
    ForgeQueryGroupedContributionMemberContext;
pub type TopologyOperatorContributionIntent = ForgeQueryContributionIntent;
pub type TopologyOperatorContributionInput<I> =
    ForgeQueryContributionComposedOrchestrationInput<TopologyQueryDomain, I>;
pub type TopologyOperatorContributionArtifact<I> =
    ForgeQueryContributionComposedOrchestration<TopologyQueryDomain, I>;
pub type TopologyOperatorContributionChecked<I> =
    ForgeQueryContributionComposedOrchestrationChecked<TopologyQueryDomain, I>;
pub type TopologyOperatorContributionProof<I> =
    ForgeQueryContributionComposedOrchestrationTranscript<TopologyQueryDomain, I>;
pub type TopologyOperatorContributionCheckedOutcome<I> =
    ForgeQueryContributionComposedOrchestrationOutcome<TopologyQueryDomain, I>;
pub type TopologyOperatorContributionOutcome<I> =
    ForgeQueryOrdinaryOutcome<ForgeQueryContributionComposedOrchestration<TopologyQueryDomain, I>>;

pub(crate) type TopologyOperatorRetainedContributionComposition =
    ForgeQueryDeclarationEntryContributionComposition;
