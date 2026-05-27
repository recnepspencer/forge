use crate::application::{ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker};
use crate::ordinary_outcome::{
    ForgeQueryOrdinaryCheckedTopology, ForgeQueryOrdinaryContinuationCheckedTopologyKind,
    ForgeQueryOrdinaryNextStep, ForgeQueryOrdinaryOutcome, ForgeQueryOrdinaryPosture,
    ForgeQueryOrdinaryPostureKind,
};

use super::{ForgeQueryContinuationExecution, ForgeQueryPreparedContinuation};

pub enum ForgeQueryPreparedContinuationOutcome<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    Prepared(ForgeQueryPreparedContinuation<D, I>),
    Ambiguous(String),
    Unavailable(String),
    WrongWorld(String),
    WrongHandle(String),
    Stale(String),
    RebindRequired(String),
    AuthorityMismatch(String),
    BasisMismatch(String),
    Unsupported(String),
    Deferred(String),
    Denied(String),
    Failed(String),
}

pub struct ForgeQueryPreparedContinuationChecked<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    outcome: ForgeQueryPreparedContinuationOutcome<D, I>,
    prepared_digest: String,
    linked_artifacts: crate::binding_pipeline::ForgeQueryBindingLinkedArtifacts,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryPreparedContinuationChecked<D, I>
{
    pub(crate) fn new(
        outcome: ForgeQueryPreparedContinuationOutcome<D, I>,
        prepared_digest: String,
        linked_artifacts: crate::binding_pipeline::ForgeQueryBindingLinkedArtifacts,
    ) -> Self {
        Self {
            outcome,
            prepared_digest,
            linked_artifacts,
        }
    }

    pub fn outcome(&self) -> &ForgeQueryPreparedContinuationOutcome<D, I> {
        &self.outcome
    }

    pub fn prepared_digest(&self) -> &str {
        &self.prepared_digest
    }

    pub fn linked_artifacts(&self) -> &crate::binding_pipeline::ForgeQueryBindingLinkedArtifacts {
        &self.linked_artifacts
    }

    pub(crate) fn into_outcome(self) -> ForgeQueryPreparedContinuationOutcome<D, I> {
        self.outcome
    }
}

pub enum ForgeQueryContinuationExecutionOutcome<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    Executed(ForgeQueryContinuationExecution<D, I>),
    WrongWorld(String),
    WrongHandle(String),
    Stale(String),
    BasisMismatch(String),
    AuthorityMismatch(String),
    Unsupported(String),
    Failed(String),
}

pub struct ForgeQueryContinuationExecutionChecked<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    outcome: ForgeQueryContinuationExecutionOutcome<D, I>,
    execution_digest: String,
    linked_artifacts: crate::binding_pipeline::ForgeQueryBindingLinkedArtifacts,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryContinuationExecutionChecked<D, I>
{
    pub(crate) fn new(
        outcome: ForgeQueryContinuationExecutionOutcome<D, I>,
        execution_digest: String,
        linked_artifacts: crate::binding_pipeline::ForgeQueryBindingLinkedArtifacts,
    ) -> Self {
        Self {
            outcome,
            execution_digest,
            linked_artifacts,
        }
    }

    pub fn outcome(&self) -> &ForgeQueryContinuationExecutionOutcome<D, I> {
        &self.outcome
    }

    pub fn execution_digest(&self) -> &str {
        &self.execution_digest
    }

    pub fn linked_artifacts(&self) -> &crate::binding_pipeline::ForgeQueryBindingLinkedArtifacts {
        &self.linked_artifacts
    }

    pub(crate) fn into_outcome(self) -> ForgeQueryContinuationExecutionOutcome<D, I> {
        self.outcome
    }
}

pub fn ordinary_outcome_from_continuation_checked<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    checked: ForgeQueryPreparedContinuationChecked<D, I>,
) -> ForgeQueryOrdinaryOutcome<ForgeQueryPreparedContinuation<D, I>> {
    let topology = |kind| {
        ForgeQueryOrdinaryCheckedTopology::continuation(kind, checked.linked_artifacts.clone())
    };
    match checked.outcome {
        ForgeQueryPreparedContinuationOutcome::Prepared(value) => {
            ForgeQueryOrdinaryOutcome::Bound(value)
        }
        ForgeQueryPreparedContinuationOutcome::Ambiguous(reason) => {
            ForgeQueryOrdinaryOutcome::Ambiguous(ForgeQueryOrdinaryPosture::new(
                reason,
                ForgeQueryOrdinaryPostureKind::Ambiguous,
                ForgeQueryOrdinaryNextStep::NarrowInput,
                topology(ForgeQueryOrdinaryContinuationCheckedTopologyKind::Ambiguous),
            ))
        }
        ForgeQueryPreparedContinuationOutcome::Unavailable(reason) => {
            ForgeQueryOrdinaryOutcome::Unavailable(ForgeQueryOrdinaryPosture::new(
                reason,
                ForgeQueryOrdinaryPostureKind::Unavailable,
                ForgeQueryOrdinaryNextStep::GatherAvailability,
                topology(ForgeQueryOrdinaryContinuationCheckedTopologyKind::Unavailable),
            ))
        }
        ForgeQueryPreparedContinuationOutcome::WrongWorld(reason) => {
            ForgeQueryOrdinaryOutcome::WrongWorld(ForgeQueryOrdinaryPosture::new(
                reason,
                ForgeQueryOrdinaryPostureKind::WrongWorld,
                ForgeQueryOrdinaryNextStep::CorrectWorld,
                topology(ForgeQueryOrdinaryContinuationCheckedTopologyKind::WrongWorld),
            ))
        }
        ForgeQueryPreparedContinuationOutcome::WrongHandle(reason) => {
            ForgeQueryOrdinaryOutcome::WrongHandle(ForgeQueryOrdinaryPosture::new(
                reason,
                ForgeQueryOrdinaryPostureKind::WrongHandle,
                ForgeQueryOrdinaryNextStep::CorrectHandle,
                topology(ForgeQueryOrdinaryContinuationCheckedTopologyKind::WrongHandle),
            ))
        }
        ForgeQueryPreparedContinuationOutcome::Stale(reason) => {
            ForgeQueryOrdinaryOutcome::Stale(ForgeQueryOrdinaryPosture::new(
                reason,
                ForgeQueryOrdinaryPostureKind::Stale,
                ForgeQueryOrdinaryNextStep::RefreshBasis,
                topology(ForgeQueryOrdinaryContinuationCheckedTopologyKind::Stale),
            ))
        }
        ForgeQueryPreparedContinuationOutcome::RebindRequired(reason) => {
            ForgeQueryOrdinaryOutcome::RebindRequired(ForgeQueryOrdinaryPosture::new(
                reason,
                ForgeQueryOrdinaryPostureKind::RebindRequired,
                ForgeQueryOrdinaryNextStep::RebindContext,
                topology(ForgeQueryOrdinaryContinuationCheckedTopologyKind::RebindRequired),
            ))
        }
        ForgeQueryPreparedContinuationOutcome::AuthorityMismatch(reason) => {
            ForgeQueryOrdinaryOutcome::AuthorityMismatch(ForgeQueryOrdinaryPosture::new(
                reason,
                ForgeQueryOrdinaryPostureKind::AuthorityMismatch,
                ForgeQueryOrdinaryNextStep::InspectProofLane,
                topology(ForgeQueryOrdinaryContinuationCheckedTopologyKind::AuthorityMismatch),
            ))
        }
        ForgeQueryPreparedContinuationOutcome::BasisMismatch(reason) => {
            ForgeQueryOrdinaryOutcome::BasisMismatch(ForgeQueryOrdinaryPosture::new(
                reason,
                ForgeQueryOrdinaryPostureKind::BasisMismatch,
                ForgeQueryOrdinaryNextStep::RefreshBasis,
                topology(ForgeQueryOrdinaryContinuationCheckedTopologyKind::BasisMismatch),
            ))
        }
        ForgeQueryPreparedContinuationOutcome::Unsupported(reason) => {
            ForgeQueryOrdinaryOutcome::Unsupported(ForgeQueryOrdinaryPosture::new(
                reason,
                ForgeQueryOrdinaryPostureKind::Unsupported,
                ForgeQueryOrdinaryNextStep::CheckSupport,
                topology(ForgeQueryOrdinaryContinuationCheckedTopologyKind::Unsupported),
            ))
        }
        ForgeQueryPreparedContinuationOutcome::Deferred(reason) => {
            ForgeQueryOrdinaryOutcome::Deferred(ForgeQueryOrdinaryPosture::new(
                reason,
                ForgeQueryOrdinaryPostureKind::Deferred,
                ForgeQueryOrdinaryNextStep::RetryLater,
                topology(ForgeQueryOrdinaryContinuationCheckedTopologyKind::Deferred),
            ))
        }
        ForgeQueryPreparedContinuationOutcome::Denied(reason) => {
            ForgeQueryOrdinaryOutcome::Denied(ForgeQueryOrdinaryPosture::new(
                reason,
                ForgeQueryOrdinaryPostureKind::Denied,
                ForgeQueryOrdinaryNextStep::InspectCheckedLane,
                topology(ForgeQueryOrdinaryContinuationCheckedTopologyKind::Denied),
            ))
        }
        ForgeQueryPreparedContinuationOutcome::Failed(reason) => {
            ForgeQueryOrdinaryOutcome::Failed(ForgeQueryOrdinaryPosture::new(
                reason,
                ForgeQueryOrdinaryPostureKind::Failed,
                ForgeQueryOrdinaryNextStep::EscalateFailure,
                topology(ForgeQueryOrdinaryContinuationCheckedTopologyKind::Failed),
            ))
        }
    }
}

pub fn ordinary_outcome_from_execution_checked<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    checked: ForgeQueryContinuationExecutionChecked<D, I>,
) -> ForgeQueryOrdinaryOutcome<ForgeQueryContinuationExecution<D, I>> {
    let topology = |kind| {
        ForgeQueryOrdinaryCheckedTopology::continuation(kind, checked.linked_artifacts.clone())
    };
    match checked.outcome {
        ForgeQueryContinuationExecutionOutcome::Executed(value) => {
            ForgeQueryOrdinaryOutcome::Bound(value)
        }
        ForgeQueryContinuationExecutionOutcome::WrongWorld(reason) => {
            ForgeQueryOrdinaryOutcome::WrongWorld(ForgeQueryOrdinaryPosture::new(
                reason,
                ForgeQueryOrdinaryPostureKind::WrongWorld,
                ForgeQueryOrdinaryNextStep::CorrectWorld,
                topology(ForgeQueryOrdinaryContinuationCheckedTopologyKind::WrongWorld),
            ))
        }
        ForgeQueryContinuationExecutionOutcome::WrongHandle(reason) => {
            ForgeQueryOrdinaryOutcome::WrongHandle(ForgeQueryOrdinaryPosture::new(
                reason,
                ForgeQueryOrdinaryPostureKind::WrongHandle,
                ForgeQueryOrdinaryNextStep::CorrectHandle,
                topology(ForgeQueryOrdinaryContinuationCheckedTopologyKind::WrongHandle),
            ))
        }
        ForgeQueryContinuationExecutionOutcome::Stale(reason) => {
            ForgeQueryOrdinaryOutcome::Stale(ForgeQueryOrdinaryPosture::new(
                reason,
                ForgeQueryOrdinaryPostureKind::Stale,
                ForgeQueryOrdinaryNextStep::RefreshBasis,
                topology(ForgeQueryOrdinaryContinuationCheckedTopologyKind::Stale),
            ))
        }
        ForgeQueryContinuationExecutionOutcome::BasisMismatch(reason) => {
            ForgeQueryOrdinaryOutcome::BasisMismatch(ForgeQueryOrdinaryPosture::new(
                reason,
                ForgeQueryOrdinaryPostureKind::BasisMismatch,
                ForgeQueryOrdinaryNextStep::RefreshBasis,
                topology(ForgeQueryOrdinaryContinuationCheckedTopologyKind::BasisMismatch),
            ))
        }
        ForgeQueryContinuationExecutionOutcome::AuthorityMismatch(reason) => {
            ForgeQueryOrdinaryOutcome::AuthorityMismatch(ForgeQueryOrdinaryPosture::new(
                reason,
                ForgeQueryOrdinaryPostureKind::AuthorityMismatch,
                ForgeQueryOrdinaryNextStep::UseExplicitHandoff,
                topology(ForgeQueryOrdinaryContinuationCheckedTopologyKind::AuthorityMismatch),
            ))
        }
        ForgeQueryContinuationExecutionOutcome::Unsupported(reason) => {
            ForgeQueryOrdinaryOutcome::Unsupported(ForgeQueryOrdinaryPosture::new(
                reason,
                ForgeQueryOrdinaryPostureKind::Unsupported,
                ForgeQueryOrdinaryNextStep::CheckSupport,
                topology(ForgeQueryOrdinaryContinuationCheckedTopologyKind::Unsupported),
            ))
        }
        ForgeQueryContinuationExecutionOutcome::Failed(reason) => {
            ForgeQueryOrdinaryOutcome::Failed(ForgeQueryOrdinaryPosture::new(
                reason,
                ForgeQueryOrdinaryPostureKind::Failed,
                ForgeQueryOrdinaryNextStep::EscalateFailure,
                topology(ForgeQueryOrdinaryContinuationCheckedTopologyKind::Failed),
            ))
        }
    }
}
