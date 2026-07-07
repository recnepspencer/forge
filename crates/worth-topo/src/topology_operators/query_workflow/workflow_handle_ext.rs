use forge_query::facade::{
    ForgeQueryDeclarationInput, ForgeQueryOrdinaryOutcome, ForgeQueryRecoveryBrief,
};

use crate::query_domain::TopologyQueryDomain;

use super::{
    TopologyOperatorCanonicalDeclaration, TopologyOperatorContinuationExecution,
    TopologyOperatorContinuationExecutionChecked, TopologyOperatorContinuationExecutionOutcome,
    TopologyOperatorContinuationExecutionProof, TopologyOperatorContinuationTarget,
    TopologyOperatorContributionArtifact, TopologyOperatorContributionChecked,
    TopologyOperatorContributionCheckedOutcome, TopologyOperatorContributionInput,
    TopologyOperatorContributionOutcome, TopologyOperatorContributionProof,
    TopologyOperatorDeclarationAdmissionError, TopologyOperatorDeclarationLegalityDenial,
    TopologyOperatorDeclarationLegalityEvidence, TopologyOperatorDeclarationOutcome,
    TopologyOperatorDeclarationReceipt, TopologyOperatorDeclarationReceiptChecked,
    TopologyOperatorDeclarationReceiptProof, TopologyOperatorDeclarationReceiptTerminalError,
    TopologyOperatorEnvelope, TopologyOperatorEnvelopeChecked,
    TopologyOperatorEnvelopeFromProgressedChecked, TopologyOperatorEnvelopeFromProgressedProof,
    TopologyOperatorEnvelopeFromProgressedTerminalError, TopologyOperatorEnvelopeProof,
    TopologyOperatorEnvelopeTerminalError, TopologyOperatorGroupedContributionComposition,
    TopologyOperatorGroupedContributionInput, TopologyOperatorGroupedContributionStop,
    TopologyOperatorGroupedDeclaration, TopologyOperatorGroupedDeclarationStop,
    TopologyOperatorGroupedInput, TopologyOperatorGroupedOutcome,
    TopologyOperatorPreparedContinuation, TopologyOperatorPreparedContinuationChecked,
    TopologyOperatorPreparedContinuationOutcome, TopologyOperatorPreparedContinuationProof,
    TopologyOperatorProgressedDeclaration, TopologyOperatorProgressionError,
    TopologyOperatorRoutePlan, TopologyOperatorRoutePlanChecked, TopologyOperatorRoutePlanProof,
    TopologyOperatorRoutePlanTerminalError, TopologyOperatorSignalCompatibilityArtifact,
    TopologyOperatorSignalCompatibilityChecked, TopologyOperatorSignalCompatibilityInput,
    TopologyOperatorSignalCompatibilityOutcome, TopologyOperatorSignalCompatibilityProof,
};

pub trait TopologyOperatorWorkflowHandleExt {
    fn declare_topology_operator<I>(
        &self,
        declaration: I,
    ) -> Result<TopologyOperatorCanonicalDeclaration<I>, TopologyOperatorDeclarationAdmissionError<I>>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>;

    fn review_topology_operator<I>(
        &self,
        declaration: TopologyOperatorCanonicalDeclaration<I>,
    ) -> Result<
        TopologyOperatorDeclarationLegalityEvidence<I>,
        TopologyOperatorDeclarationLegalityDenial<I>,
    >
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>;

    fn orchestrate_topology_operator_outcome<I>(
        &self,
        declaration: I,
    ) -> TopologyOperatorDeclarationOutcome<I>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>;

    fn orchestrate_topology_operator_envelope<I>(
        &self,
        declaration: I,
    ) -> Result<TopologyOperatorEnvelope<I>, TopologyOperatorEnvelopeTerminalError<I>>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>;

    fn orchestrate_topology_operator_envelope_checked<I>(
        &self,
        declaration: I,
    ) -> TopologyOperatorEnvelopeChecked<I>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>;

    fn orchestrate_topology_operator_envelope_proof<I>(
        &self,
        declaration: I,
    ) -> TopologyOperatorEnvelopeProof<I>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>;

    fn declare_review_and_progress_topology_operator<I>(
        &self,
        declaration: I,
    ) -> Result<TopologyOperatorProgressedDeclaration<I>, TopologyOperatorProgressionError<I>>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>;

    fn orchestrate_topology_operator_route<I>(
        &self,
        progressed: TopologyOperatorProgressedDeclaration<I>,
    ) -> Result<TopologyOperatorRoutePlan<I>, TopologyOperatorRoutePlanTerminalError<I>>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>;

    fn orchestrate_topology_operator_route_checked<I>(
        &self,
        progressed: TopologyOperatorProgressedDeclaration<I>,
    ) -> TopologyOperatorRoutePlanChecked<I>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>;

    fn orchestrate_topology_operator_route_proof<I>(
        &self,
        progressed: TopologyOperatorProgressedDeclaration<I>,
    ) -> TopologyOperatorRoutePlanProof<I>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>;

    fn orchestrate_topology_operator_receipt_checked<I>(
        &self,
        progressed: TopologyOperatorProgressedDeclaration<I>,
    ) -> TopologyOperatorDeclarationReceiptChecked<I>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>;

    fn orchestrate_topology_operator_receipt<I>(
        &self,
        progressed: TopologyOperatorProgressedDeclaration<I>,
    ) -> Result<
        TopologyOperatorDeclarationReceipt<I>,
        TopologyOperatorDeclarationReceiptTerminalError<I>,
    >
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>;

    fn orchestrate_topology_operator_receipt_proof<I>(
        &self,
        progressed: TopologyOperatorProgressedDeclaration<I>,
    ) -> TopologyOperatorDeclarationReceiptProof<I>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>;

    fn orchestrate_topology_operator_envelope_from_progressed<I>(
        &self,
        progressed: TopologyOperatorProgressedDeclaration<I>,
    ) -> Result<TopologyOperatorEnvelope<I>, TopologyOperatorEnvelopeFromProgressedTerminalError<I>>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>;

    fn orchestrate_topology_operator_envelope_from_progressed_checked<I>(
        &self,
        progressed: TopologyOperatorProgressedDeclaration<I>,
    ) -> TopologyOperatorEnvelopeFromProgressedChecked<I>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>;

    fn orchestrate_topology_operator_envelope_from_progressed_proof<I>(
        &self,
        progressed: TopologyOperatorProgressedDeclaration<I>,
    ) -> TopologyOperatorEnvelopeFromProgressedProof<I>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>;

    fn recover_topology_operator_route_checked<I>(
        &self,
        checked: TopologyOperatorRoutePlanChecked<I>,
    ) -> Option<ForgeQueryRecoveryBrief>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>;

    fn orchestrate_topology_operator_signal_compatibility<I>(
        &self,
        input: TopologyOperatorSignalCompatibilityInput<I>,
    ) -> TopologyOperatorSignalCompatibilityOutcome<I>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>;

    fn orchestrate_topology_operator_signal_compatibility_outcome<I>(
        &self,
        input: TopologyOperatorSignalCompatibilityInput<I>,
    ) -> ForgeQueryOrdinaryOutcome<TopologyOperatorSignalCompatibilityArtifact<I>>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>;

    fn orchestrate_topology_operator_signal_compatibility_checked<I>(
        &self,
        input: TopologyOperatorSignalCompatibilityInput<I>,
    ) -> TopologyOperatorSignalCompatibilityChecked<I>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>;

    fn orchestrate_topology_operator_signal_compatibility_proof<I>(
        &self,
        input: TopologyOperatorSignalCompatibilityInput<I>,
    ) -> TopologyOperatorSignalCompatibilityProof<I>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>;

    fn recover_topology_operator_signal_compatibility_checked<I>(
        &self,
        checked: TopologyOperatorSignalCompatibilityChecked<I>,
    ) -> Option<ForgeQueryRecoveryBrief>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>;

    fn recover_topology_operator_signal_compatibility_proof<I>(
        &self,
        proof: TopologyOperatorSignalCompatibilityProof<I>,
    ) -> Option<ForgeQueryRecoveryBrief>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>;

    fn recover_topology_operator_envelope_checked<I>(
        &self,
        checked: TopologyOperatorEnvelopeChecked<I>,
    ) -> Option<ForgeQueryRecoveryBrief>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>;

    fn recover_topology_operator_envelope_proof<I>(
        &self,
        proof: &TopologyOperatorEnvelopeProof<I>,
    ) -> Option<ForgeQueryRecoveryBrief>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>;

    fn recover_topology_operator_receipt_checked<I>(
        &self,
        checked: TopologyOperatorDeclarationReceiptChecked<I>,
    ) -> Option<ForgeQueryRecoveryBrief>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>;

    fn prepare_topology_operator_continuation<I>(
        &self,
        request: TopologyOperatorContinuationTarget<I>,
    ) -> TopologyOperatorPreparedContinuationOutcome<I>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>;

    fn prepare_topology_operator_continuation_outcome<I>(
        &self,
        request: TopologyOperatorContinuationTarget<I>,
    ) -> ForgeQueryOrdinaryOutcome<TopologyOperatorPreparedContinuation<I>>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>;

    fn prepare_topology_operator_continuation_checked<I>(
        &self,
        request: TopologyOperatorContinuationTarget<I>,
    ) -> TopologyOperatorPreparedContinuationChecked<I>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>;

    fn prepare_topology_operator_continuation_proof<I>(
        &self,
        request: TopologyOperatorContinuationTarget<I>,
    ) -> TopologyOperatorPreparedContinuationProof<I>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>;

    fn execute_topology_operator_prepared_continuation<I>(
        &self,
        prepared: TopologyOperatorPreparedContinuation<I>,
    ) -> TopologyOperatorContinuationExecutionOutcome<I>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>;

    fn execute_topology_operator_prepared_continuation_outcome<I>(
        &self,
        prepared: TopologyOperatorPreparedContinuation<I>,
    ) -> ForgeQueryOrdinaryOutcome<TopologyOperatorContinuationExecution<I>>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>;

    fn execute_topology_operator_prepared_continuation_checked<I>(
        &self,
        prepared: TopologyOperatorPreparedContinuation<I>,
    ) -> TopologyOperatorContinuationExecutionChecked<I>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>;

    fn execute_topology_operator_prepared_continuation_proof<I>(
        &self,
        prepared: TopologyOperatorPreparedContinuation<I>,
    ) -> TopologyOperatorContinuationExecutionProof<I>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>;

    fn recover_topology_operator_prepared_continuation_checked<I>(
        &self,
        checked: TopologyOperatorPreparedContinuationChecked<I>,
    ) -> Option<ForgeQueryRecoveryBrief>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>;

    fn recover_topology_operator_prepared_continuation_proof<I>(
        &self,
        proof: TopologyOperatorPreparedContinuationProof<I>,
    ) -> Option<ForgeQueryRecoveryBrief>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>;

    fn recover_topology_operator_continuation_execution_checked<I>(
        &self,
        checked: TopologyOperatorContinuationExecutionChecked<I>,
    ) -> Option<ForgeQueryRecoveryBrief>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>;

    fn recover_topology_operator_continuation_execution_proof<I>(
        &self,
        proof: TopologyOperatorContinuationExecutionProof<I>,
    ) -> Option<ForgeQueryRecoveryBrief>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>;

    fn recover_topology_operator_outcome<T>(
        &self,
        outcome: &ForgeQueryOrdinaryOutcome<T>,
    ) -> Option<ForgeQueryRecoveryBrief>;

    fn declare_topology_grouped_operator<I>(
        &self,
        input: TopologyOperatorGroupedInput<I>,
    ) -> Result<TopologyOperatorGroupedDeclaration<I>, TopologyOperatorGroupedDeclarationStop>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain> + Clone;

    fn orchestrate_topology_grouped_operator_outcome<I>(
        &self,
        declaration: TopologyOperatorGroupedDeclaration<I>,
    ) -> TopologyOperatorGroupedOutcome<I>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain> + Clone;

    fn topology_grouped_operator_support<I>(
        &self,
        declaration: &TopologyOperatorGroupedDeclaration<I>,
    ) -> forge_query::facade::ForgeQueryGroupedSupportReport
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>;

    fn grouped_topology_operator_contributions_checked<I>(
        &self,
        input: TopologyOperatorGroupedContributionInput<I>,
    ) -> Result<
        TopologyOperatorGroupedContributionComposition<I>,
        TopologyOperatorGroupedContributionStop<I>,
    >
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain> + Clone;

    fn orchestrate_topology_operator_with_contributions<I>(
        &self,
        input: TopologyOperatorContributionInput<I>,
    ) -> Result<
        TopologyOperatorContributionArtifact<I>,
        TopologyOperatorContributionCheckedOutcome<I>,
    >
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>;

    fn orchestrate_topology_operator_with_contributions_outcome<I>(
        &self,
        input: TopologyOperatorContributionInput<I>,
    ) -> TopologyOperatorContributionOutcome<I>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>;

    fn orchestrate_topology_operator_with_contributions_checked<I>(
        &self,
        input: TopologyOperatorContributionInput<I>,
    ) -> TopologyOperatorContributionChecked<I>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>;

    fn orchestrate_topology_operator_with_contributions_proof<I>(
        &self,
        input: TopologyOperatorContributionInput<I>,
    ) -> TopologyOperatorContributionProof<I>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>;

    fn recover_topology_operator_contribution_checked<I>(
        &self,
        checked: TopologyOperatorContributionChecked<I>,
    ) -> Option<ForgeQueryRecoveryBrief>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>;

    fn recover_topology_operator_contribution_proof<I>(
        &self,
        proof: TopologyOperatorContributionProof<I>,
    ) -> Option<ForgeQueryRecoveryBrief>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>;
}
