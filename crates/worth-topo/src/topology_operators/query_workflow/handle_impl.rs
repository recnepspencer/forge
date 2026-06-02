use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationInput,
    ForgeQueryDomainOperatingContext, ForgeQueryGroupedSupportReport, ForgeQueryOrdinaryOutcome,
    ForgeQueryRecoveryBrief,
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
    TopologyOperatorWorkflowHandleExt,
};

impl<C> TopologyOperatorWorkflowHandleExt
    for ForgeQueryAdmittedConfiguredDomainHandle<TopologyQueryDomain, C>
where
    C: ForgeQueryDomainOperatingContext<TopologyQueryDomain>,
{
    fn declare_topology_operator<I>(
        &self,
        declaration: I,
    ) -> Result<TopologyOperatorCanonicalDeclaration<I>, TopologyOperatorDeclarationAdmissionError<I>>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
    {
        self.declare(declaration)
    }

    fn review_topology_operator<I>(
        &self,
        declaration: TopologyOperatorCanonicalDeclaration<I>,
    ) -> Result<
        TopologyOperatorDeclarationLegalityEvidence<I>,
        TopologyOperatorDeclarationLegalityDenial<I>,
    >
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
    {
        self.review_legality(declaration)
    }

    fn orchestrate_topology_operator_outcome<I>(
        &self,
        declaration: I,
    ) -> TopologyOperatorDeclarationOutcome<I>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
    {
        self.orchestrate_declaration_entry_outcome(declaration)
    }

    fn orchestrate_topology_operator_envelope<I>(
        &self,
        declaration: I,
    ) -> Result<TopologyOperatorEnvelope<I>, TopologyOperatorEnvelopeTerminalError<I>>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
    {
        self.orchestrate_declaration_entry(declaration)
    }

    fn orchestrate_topology_operator_envelope_checked<I>(
        &self,
        declaration: I,
    ) -> TopologyOperatorEnvelopeChecked<I>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
    {
        self.orchestrate_declaration_entry_checked(declaration)
    }

    fn orchestrate_topology_operator_envelope_proof<I>(
        &self,
        declaration: I,
    ) -> TopologyOperatorEnvelopeProof<I>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
    {
        self.orchestrate_declaration_entry_proof(declaration)
    }

    fn declare_review_and_progress_topology_operator<I>(
        &self,
        declaration: I,
    ) -> Result<TopologyOperatorProgressedDeclaration<I>, TopologyOperatorProgressionError<I>>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
    {
        self.declare_review_and_progress(declaration)
    }

    fn orchestrate_topology_operator_route<I>(
        &self,
        progressed: TopologyOperatorProgressedDeclaration<I>,
    ) -> Result<TopologyOperatorRoutePlan<I>, TopologyOperatorRoutePlanTerminalError<I>>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
    {
        self.orchestrate_routes_from_progressed(progressed)
    }

    fn orchestrate_topology_operator_route_checked<I>(
        &self,
        progressed: TopologyOperatorProgressedDeclaration<I>,
    ) -> TopologyOperatorRoutePlanChecked<I>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
    {
        self.orchestrate_routes_from_progressed_checked(progressed)
    }

    fn orchestrate_topology_operator_route_proof<I>(
        &self,
        progressed: TopologyOperatorProgressedDeclaration<I>,
    ) -> TopologyOperatorRoutePlanProof<I>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
    {
        self.orchestrate_routes_from_progressed_proof(progressed)
    }

    fn orchestrate_topology_operator_receipt_checked<I>(
        &self,
        progressed: TopologyOperatorProgressedDeclaration<I>,
    ) -> TopologyOperatorDeclarationReceiptChecked<I>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
    {
        self.orchestrate_receipt_from_progressed_checked(progressed)
    }

    fn orchestrate_topology_operator_receipt<I>(
        &self,
        progressed: TopologyOperatorProgressedDeclaration<I>,
    ) -> Result<
        TopologyOperatorDeclarationReceipt<I>,
        TopologyOperatorDeclarationReceiptTerminalError<I>,
    >
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
    {
        self.orchestrate_receipt_from_progressed(progressed)
    }

    fn orchestrate_topology_operator_receipt_proof<I>(
        &self,
        progressed: TopologyOperatorProgressedDeclaration<I>,
    ) -> TopologyOperatorDeclarationReceiptProof<I>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
    {
        self.orchestrate_receipt_from_progressed_proof(progressed)
    }

    fn orchestrate_topology_operator_envelope_from_progressed<I>(
        &self,
        progressed: TopologyOperatorProgressedDeclaration<I>,
    ) -> Result<TopologyOperatorEnvelope<I>, TopologyOperatorEnvelopeFromProgressedTerminalError<I>>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
    {
        self.orchestrate_envelope_from_progressed(progressed)
    }

    fn orchestrate_topology_operator_envelope_from_progressed_checked<I>(
        &self,
        progressed: TopologyOperatorProgressedDeclaration<I>,
    ) -> TopologyOperatorEnvelopeFromProgressedChecked<I>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
    {
        self.orchestrate_envelope_from_progressed_checked(progressed)
    }

    fn orchestrate_topology_operator_envelope_from_progressed_proof<I>(
        &self,
        progressed: TopologyOperatorProgressedDeclaration<I>,
    ) -> TopologyOperatorEnvelopeFromProgressedProof<I>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
    {
        self.orchestrate_envelope_from_progressed_proof(progressed)
    }

    fn recover_topology_operator_route_checked<I>(
        &self,
        checked: TopologyOperatorRoutePlanChecked<I>,
    ) -> Option<ForgeQueryRecoveryBrief>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
    {
        self.recover_from_declaration_route_plan_checked(checked)
    }

    fn orchestrate_topology_operator_signal_compatibility<I>(
        &self,
        input: TopologyOperatorSignalCompatibilityInput<I>,
    ) -> TopologyOperatorSignalCompatibilityOutcome<I>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
    {
        self.orchestrate_signal_compatibility(input)
    }

    fn orchestrate_topology_operator_signal_compatibility_outcome<I>(
        &self,
        input: TopologyOperatorSignalCompatibilityInput<I>,
    ) -> ForgeQueryOrdinaryOutcome<TopologyOperatorSignalCompatibilityArtifact<I>>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
    {
        self.orchestrate_signal_compatibility_outcome(input)
    }

    fn orchestrate_topology_operator_signal_compatibility_checked<I>(
        &self,
        input: TopologyOperatorSignalCompatibilityInput<I>,
    ) -> TopologyOperatorSignalCompatibilityChecked<I>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
    {
        self.orchestrate_signal_compatibility_checked(input)
    }

    fn orchestrate_topology_operator_signal_compatibility_proof<I>(
        &self,
        input: TopologyOperatorSignalCompatibilityInput<I>,
    ) -> TopologyOperatorSignalCompatibilityProof<I>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
    {
        self.orchestrate_signal_compatibility_proof(input)
    }

    fn recover_topology_operator_signal_compatibility_checked<I>(
        &self,
        checked: TopologyOperatorSignalCompatibilityChecked<I>,
    ) -> Option<ForgeQueryRecoveryBrief>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
    {
        self.recover_from_signal_compatibility_checked(checked)
    }

    fn recover_topology_operator_signal_compatibility_proof<I>(
        &self,
        proof: TopologyOperatorSignalCompatibilityProof<I>,
    ) -> Option<ForgeQueryRecoveryBrief>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
    {
        self.recover_from_signal_compatibility_proof(proof)
    }

    fn recover_topology_operator_envelope_checked<I>(
        &self,
        checked: TopologyOperatorEnvelopeChecked<I>,
    ) -> Option<ForgeQueryRecoveryBrief>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
    {
        self.recover_from_declaration_entry_checked(checked)
    }

    fn recover_topology_operator_envelope_proof<I>(
        &self,
        proof: &TopologyOperatorEnvelopeProof<I>,
    ) -> Option<ForgeQueryRecoveryBrief>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
    {
        self.recover_from_declaration_entry_proof(proof)
    }

    fn recover_topology_operator_receipt_checked<I>(
        &self,
        checked: TopologyOperatorDeclarationReceiptChecked<I>,
    ) -> Option<ForgeQueryRecoveryBrief>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
    {
        self.recover_from_declaration_receipt_checked(checked)
    }

    fn prepare_topology_operator_continuation<I>(
        &self,
        request: TopologyOperatorContinuationTarget<I>,
    ) -> TopologyOperatorPreparedContinuationOutcome<I>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
    {
        self.prepare_continuation_from_target(request)
    }

    fn prepare_topology_operator_continuation_outcome<I>(
        &self,
        request: TopologyOperatorContinuationTarget<I>,
    ) -> ForgeQueryOrdinaryOutcome<TopologyOperatorPreparedContinuation<I>>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
    {
        self.prepare_continuation_from_target_outcome(request)
    }

    fn prepare_topology_operator_continuation_checked<I>(
        &self,
        request: TopologyOperatorContinuationTarget<I>,
    ) -> TopologyOperatorPreparedContinuationChecked<I>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
    {
        self.prepare_continuation_from_target_checked(request)
    }

    fn prepare_topology_operator_continuation_proof<I>(
        &self,
        request: TopologyOperatorContinuationTarget<I>,
    ) -> TopologyOperatorPreparedContinuationProof<I>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
    {
        self.prepare_continuation_from_target_proof(request)
    }

    fn execute_topology_operator_prepared_continuation<I>(
        &self,
        prepared: TopologyOperatorPreparedContinuation<I>,
    ) -> TopologyOperatorContinuationExecutionOutcome<I>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
    {
        self.execute_prepared_continuation(prepared)
    }

    fn execute_topology_operator_prepared_continuation_outcome<I>(
        &self,
        prepared: TopologyOperatorPreparedContinuation<I>,
    ) -> ForgeQueryOrdinaryOutcome<TopologyOperatorContinuationExecution<I>>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
    {
        self.execute_prepared_continuation_outcome(prepared)
    }

    fn execute_topology_operator_prepared_continuation_checked<I>(
        &self,
        prepared: TopologyOperatorPreparedContinuation<I>,
    ) -> TopologyOperatorContinuationExecutionChecked<I>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
    {
        self.execute_prepared_continuation_checked(prepared)
    }

    fn execute_topology_operator_prepared_continuation_proof<I>(
        &self,
        prepared: TopologyOperatorPreparedContinuation<I>,
    ) -> TopologyOperatorContinuationExecutionProof<I>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
    {
        self.execute_prepared_continuation_proof(prepared)
    }

    fn recover_topology_operator_prepared_continuation_checked<I>(
        &self,
        checked: TopologyOperatorPreparedContinuationChecked<I>,
    ) -> Option<ForgeQueryRecoveryBrief>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
    {
        self.recover_from_prepared_continuation_checked(checked)
    }

    fn recover_topology_operator_prepared_continuation_proof<I>(
        &self,
        proof: TopologyOperatorPreparedContinuationProof<I>,
    ) -> Option<ForgeQueryRecoveryBrief>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
    {
        self.recover_from_prepared_continuation_proof(proof)
    }

    fn recover_topology_operator_continuation_execution_checked<I>(
        &self,
        checked: TopologyOperatorContinuationExecutionChecked<I>,
    ) -> Option<ForgeQueryRecoveryBrief>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
    {
        self.recover_from_continuation_execution_checked(checked)
    }

    fn recover_topology_operator_continuation_execution_proof<I>(
        &self,
        proof: TopologyOperatorContinuationExecutionProof<I>,
    ) -> Option<ForgeQueryRecoveryBrief>
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
    {
        self.recover_from_continuation_execution_proof(proof)
    }

    fn recover_topology_operator_outcome<T>(
        &self,
        outcome: &ForgeQueryOrdinaryOutcome<T>,
    ) -> Option<ForgeQueryRecoveryBrief> {
        self.recover_from_outcome(outcome)
    }

    grouped_and_contribution_workflow_methods!();
}
