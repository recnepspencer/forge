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
    declaration_and_route_workflow_methods!();
    signal_compatibility_workflow_methods!();
    continuation_workflow_methods!();
    grouped_and_contribution_workflow_methods!();
}
