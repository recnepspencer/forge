use crate::application::{WorthQueryDeclarationInput, WorthQueryDomainEntryMarker};
use crate::binding_pipeline::{
    WorthQueryBindingLinkedArtifacts, WorthQueryBindingNarrowingDecision,
    WorthQueryBindingRequestDescriptor, WorthQueryBindingWitnessCheck,
};

use super::{WorthQueryContinuationExecutionOutcome, WorthQueryPreparedContinuationOutcome};

pub struct WorthQueryPreparedContinuationTranscript<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    request: WorthQueryBindingRequestDescriptor,
    outcome: WorthQueryPreparedContinuationOutcome<D, I>,
    witness_checks: Vec<WorthQueryBindingWitnessCheck>,
    narrowing_decisions: Vec<WorthQueryBindingNarrowingDecision>,
    prepared_digest: String,
    linked_artifacts: WorthQueryBindingLinkedArtifacts,
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryPreparedContinuationTranscript<D, I>
{
    pub(crate) fn new(
        request: WorthQueryBindingRequestDescriptor,
        outcome: WorthQueryPreparedContinuationOutcome<D, I>,
        witness_checks: Vec<WorthQueryBindingWitnessCheck>,
        narrowing_decisions: Vec<WorthQueryBindingNarrowingDecision>,
        prepared_digest: String,
        linked_artifacts: WorthQueryBindingLinkedArtifacts,
    ) -> Self {
        Self {
            request,
            outcome,
            witness_checks,
            narrowing_decisions,
            prepared_digest,
            linked_artifacts,
        }
    }

    pub fn request(&self) -> &WorthQueryBindingRequestDescriptor {
        &self.request
    }

    pub fn outcome(&self) -> &WorthQueryPreparedContinuationOutcome<D, I> {
        &self.outcome
    }

    pub fn witness_checks(&self) -> &[WorthQueryBindingWitnessCheck] {
        &self.witness_checks
    }

    pub fn narrowing_decisions(&self) -> &[WorthQueryBindingNarrowingDecision] {
        &self.narrowing_decisions
    }

    pub fn prepared_digest(&self) -> &str {
        &self.prepared_digest
    }

    pub fn linked_artifacts(&self) -> &WorthQueryBindingLinkedArtifacts {
        &self.linked_artifacts
    }

    pub fn into_checked(
        self,
    ) -> crate::continuation_pipeline::WorthQueryPreparedContinuationChecked<D, I> {
        crate::continuation_pipeline::WorthQueryPreparedContinuationChecked::new(
            self.outcome,
            self.prepared_digest,
            self.linked_artifacts,
        )
    }
}

pub struct WorthQueryContinuationExecutionTranscript<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    request: WorthQueryBindingRequestDescriptor,
    outcome: WorthQueryContinuationExecutionOutcome<D, I>,
    witness_checks: Vec<WorthQueryBindingWitnessCheck>,
    execution_digest: String,
    linked_artifacts: WorthQueryBindingLinkedArtifacts,
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryContinuationExecutionTranscript<D, I>
{
    pub(crate) fn new(
        request: WorthQueryBindingRequestDescriptor,
        outcome: WorthQueryContinuationExecutionOutcome<D, I>,
        witness_checks: Vec<WorthQueryBindingWitnessCheck>,
        execution_digest: String,
        linked_artifacts: WorthQueryBindingLinkedArtifacts,
    ) -> Self {
        Self {
            request,
            outcome,
            witness_checks,
            execution_digest,
            linked_artifacts,
        }
    }

    pub fn request(&self) -> &WorthQueryBindingRequestDescriptor {
        &self.request
    }

    pub fn outcome(&self) -> &WorthQueryContinuationExecutionOutcome<D, I> {
        &self.outcome
    }

    pub fn witness_checks(&self) -> &[WorthQueryBindingWitnessCheck] {
        &self.witness_checks
    }

    pub fn execution_digest(&self) -> &str {
        &self.execution_digest
    }

    pub fn linked_artifacts(&self) -> &WorthQueryBindingLinkedArtifacts {
        &self.linked_artifacts
    }

    pub fn into_checked(
        self,
    ) -> crate::continuation_pipeline::WorthQueryContinuationExecutionChecked<D, I> {
        crate::continuation_pipeline::WorthQueryContinuationExecutionChecked::new(
            self.outcome,
            self.execution_digest,
            self.linked_artifacts,
        )
    }
}
