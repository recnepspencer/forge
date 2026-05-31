use crate::application::{ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker};
use crate::binding_pipeline::{
    ForgeQueryBindingLinkedArtifacts, ForgeQueryBindingNarrowingDecision,
    ForgeQueryBindingRequestDescriptor, ForgeQueryBindingWitnessCheck,
};

use super::{ForgeQueryContinuationExecutionOutcome, ForgeQueryPreparedContinuationOutcome};

pub struct ForgeQueryPreparedContinuationTranscript<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    request: ForgeQueryBindingRequestDescriptor,
    outcome: ForgeQueryPreparedContinuationOutcome<D, I>,
    witness_checks: Vec<ForgeQueryBindingWitnessCheck>,
    narrowing_decisions: Vec<ForgeQueryBindingNarrowingDecision>,
    prepared_digest: String,
    linked_artifacts: ForgeQueryBindingLinkedArtifacts,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryPreparedContinuationTranscript<D, I>
{
    pub(crate) fn new(
        request: ForgeQueryBindingRequestDescriptor,
        outcome: ForgeQueryPreparedContinuationOutcome<D, I>,
        witness_checks: Vec<ForgeQueryBindingWitnessCheck>,
        narrowing_decisions: Vec<ForgeQueryBindingNarrowingDecision>,
        prepared_digest: String,
        linked_artifacts: ForgeQueryBindingLinkedArtifacts,
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

    pub fn request(&self) -> &ForgeQueryBindingRequestDescriptor {
        &self.request
    }

    pub fn outcome(&self) -> &ForgeQueryPreparedContinuationOutcome<D, I> {
        &self.outcome
    }

    pub fn witness_checks(&self) -> &[ForgeQueryBindingWitnessCheck] {
        &self.witness_checks
    }

    pub fn narrowing_decisions(&self) -> &[ForgeQueryBindingNarrowingDecision] {
        &self.narrowing_decisions
    }

    pub fn prepared_digest(&self) -> &str {
        &self.prepared_digest
    }

    pub fn linked_artifacts(&self) -> &ForgeQueryBindingLinkedArtifacts {
        &self.linked_artifacts
    }

    pub fn into_checked(
        self,
    ) -> crate::continuation_pipeline::ForgeQueryPreparedContinuationChecked<D, I> {
        crate::continuation_pipeline::ForgeQueryPreparedContinuationChecked::new(
            self.outcome,
            self.prepared_digest,
            self.linked_artifacts,
        )
    }
}

pub struct ForgeQueryContinuationExecutionTranscript<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    request: ForgeQueryBindingRequestDescriptor,
    outcome: ForgeQueryContinuationExecutionOutcome<D, I>,
    witness_checks: Vec<ForgeQueryBindingWitnessCheck>,
    execution_digest: String,
    linked_artifacts: ForgeQueryBindingLinkedArtifacts,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryContinuationExecutionTranscript<D, I>
{
    pub(crate) fn new(
        request: ForgeQueryBindingRequestDescriptor,
        outcome: ForgeQueryContinuationExecutionOutcome<D, I>,
        witness_checks: Vec<ForgeQueryBindingWitnessCheck>,
        execution_digest: String,
        linked_artifacts: ForgeQueryBindingLinkedArtifacts,
    ) -> Self {
        Self {
            request,
            outcome,
            witness_checks,
            execution_digest,
            linked_artifacts,
        }
    }

    pub fn request(&self) -> &ForgeQueryBindingRequestDescriptor {
        &self.request
    }

    pub fn outcome(&self) -> &ForgeQueryContinuationExecutionOutcome<D, I> {
        &self.outcome
    }

    pub fn witness_checks(&self) -> &[ForgeQueryBindingWitnessCheck] {
        &self.witness_checks
    }

    pub fn execution_digest(&self) -> &str {
        &self.execution_digest
    }

    pub fn linked_artifacts(&self) -> &ForgeQueryBindingLinkedArtifacts {
        &self.linked_artifacts
    }

    pub fn into_checked(
        self,
    ) -> crate::continuation_pipeline::ForgeQueryContinuationExecutionChecked<D, I> {
        crate::continuation_pipeline::ForgeQueryContinuationExecutionChecked::new(
            self.outcome,
            self.execution_digest,
            self.linked_artifacts,
        )
    }
}
