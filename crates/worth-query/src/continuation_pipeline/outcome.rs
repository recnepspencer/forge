use crate::application::{WorthQueryDeclarationInput, WorthQueryDomainEntryMarker};
use crate::domain_installation::WorthQueryInstalledDomainExecutionDrift;
use crate::ordinary_outcome::{
    WorthQueryOrdinaryCheckedTopology, WorthQueryOrdinaryContinuationCheckedTopologyKind,
    WorthQueryOrdinaryNextStep, WorthQueryOrdinaryOutcome, WorthQueryOrdinaryPosture,
    WorthQueryOrdinaryPostureKind,
};

use super::{
    WorthQueryContinuationExecution, WorthQueryContinuationExecutionReadmissionStop,
    WorthQueryPreparedContinuation,
};

mod execution_ordinary;

pub use execution_ordinary::ordinary_outcome_from_execution_checked;

pub enum WorthQueryPreparedContinuationOutcome<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    Prepared(WorthQueryPreparedContinuation<D, I>),
    Ambiguous(String),
    Unavailable(String),
    WrongWorld(String),
    WrongHandle(String),
    InstalledAuthorityDrift(WorthQueryInstalledDomainExecutionDrift),
    Stale(String),
    RebindRequired(String),
    AuthorityMismatch(String),
    BasisMismatch(String),
    Unsupported(String),
    Deferred(String),
    Denied(String),
    Failed(String),
}

pub struct WorthQueryPreparedContinuationChecked<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    outcome: WorthQueryPreparedContinuationOutcome<D, I>,
    prepared_digest: String,
    linked_artifacts: crate::binding_pipeline::WorthQueryBindingLinkedArtifacts,
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryPreparedContinuationChecked<D, I>
{
    pub(crate) fn new(
        outcome: WorthQueryPreparedContinuationOutcome<D, I>,
        prepared_digest: String,
        linked_artifacts: crate::binding_pipeline::WorthQueryBindingLinkedArtifacts,
    ) -> Self {
        Self {
            outcome,
            prepared_digest,
            linked_artifacts,
        }
    }

    pub fn outcome(&self) -> &WorthQueryPreparedContinuationOutcome<D, I> {
        &self.outcome
    }

    pub fn prepared_digest(&self) -> &str {
        &self.prepared_digest
    }

    pub fn linked_artifacts(&self) -> &crate::binding_pipeline::WorthQueryBindingLinkedArtifacts {
        &self.linked_artifacts
    }

    pub(crate) fn into_outcome(self) -> WorthQueryPreparedContinuationOutcome<D, I> {
        self.outcome
    }
}

pub enum WorthQueryContinuationExecutionOutcome<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    Executed(WorthQueryContinuationExecution<D, I>),
    WrongWorld(String),
    AsyncRequestDrift(WorthQueryContinuationExecutionReadmissionStop),
    ReplayDrift(WorthQueryContinuationExecutionReadmissionStop),
    RemaskDrift(WorthQueryContinuationExecutionReadmissionStop),
    PreviewCrossedResidue(WorthQueryContinuationExecutionReadmissionStop),
    InstalledAuthorityDrift(WorthQueryInstalledDomainExecutionDrift),
    Stale(WorthQueryContinuationExecutionReadmissionStop),
    StaleCompletion(WorthQueryContinuationExecutionReadmissionStop),
    BasisMismatch(WorthQueryContinuationExecutionReadmissionStop),
    LowerBindingMismatch(WorthQueryContinuationExecutionReadmissionStop),
    AuthorityMismatch(WorthQueryContinuationExecutionReadmissionStop),
    WrongHandle(String),
    Unsupported(String),
}

pub struct WorthQueryContinuationExecutionChecked<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    outcome: WorthQueryContinuationExecutionOutcome<D, I>,
    execution_digest: String,
    linked_artifacts: crate::binding_pipeline::WorthQueryBindingLinkedArtifacts,
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryContinuationExecutionChecked<D, I>
{
    pub(crate) fn new(
        outcome: WorthQueryContinuationExecutionOutcome<D, I>,
        execution_digest: String,
        linked_artifacts: crate::binding_pipeline::WorthQueryBindingLinkedArtifacts,
    ) -> Self {
        Self {
            outcome,
            execution_digest,
            linked_artifacts,
        }
    }

    pub fn outcome(&self) -> &WorthQueryContinuationExecutionOutcome<D, I> {
        &self.outcome
    }

    pub fn execution_digest(&self) -> &str {
        &self.execution_digest
    }

    pub fn linked_artifacts(&self) -> &crate::binding_pipeline::WorthQueryBindingLinkedArtifacts {
        &self.linked_artifacts
    }

    pub(crate) fn into_outcome(self) -> WorthQueryContinuationExecutionOutcome<D, I> {
        self.outcome
    }
}

pub fn ordinary_outcome_from_continuation_checked<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    checked: WorthQueryPreparedContinuationChecked<D, I>,
) -> WorthQueryOrdinaryOutcome<WorthQueryPreparedContinuation<D, I>> {
    let topology = |kind| {
        WorthQueryOrdinaryCheckedTopology::continuation(kind, checked.linked_artifacts.clone())
    };
    match checked.outcome {
        WorthQueryPreparedContinuationOutcome::Prepared(value) => {
            WorthQueryOrdinaryOutcome::Bound(value)
        }
        WorthQueryPreparedContinuationOutcome::Ambiguous(reason) => {
            WorthQueryOrdinaryOutcome::Ambiguous(WorthQueryOrdinaryPosture::new(
                reason,
                WorthQueryOrdinaryPostureKind::Ambiguous,
                WorthQueryOrdinaryNextStep::NarrowInput,
                topology(WorthQueryOrdinaryContinuationCheckedTopologyKind::Ambiguous),
            ))
        }
        WorthQueryPreparedContinuationOutcome::Unavailable(reason) => {
            WorthQueryOrdinaryOutcome::Unavailable(WorthQueryOrdinaryPosture::new(
                reason,
                WorthQueryOrdinaryPostureKind::Unavailable,
                WorthQueryOrdinaryNextStep::GatherAvailability,
                topology(WorthQueryOrdinaryContinuationCheckedTopologyKind::Unavailable),
            ))
        }
        WorthQueryPreparedContinuationOutcome::WrongWorld(reason) => {
            WorthQueryOrdinaryOutcome::WrongWorld(WorthQueryOrdinaryPosture::new(
                reason,
                WorthQueryOrdinaryPostureKind::WrongWorld,
                WorthQueryOrdinaryNextStep::CorrectWorld,
                topology(WorthQueryOrdinaryContinuationCheckedTopologyKind::WrongWorld),
            ))
        }
        WorthQueryPreparedContinuationOutcome::WrongHandle(reason) => {
            WorthQueryOrdinaryOutcome::WrongHandle(WorthQueryOrdinaryPosture::new(
                reason,
                WorthQueryOrdinaryPostureKind::WrongHandle,
                WorthQueryOrdinaryNextStep::CorrectHandle,
                topology(WorthQueryOrdinaryContinuationCheckedTopologyKind::WrongHandle),
            ))
        }
        WorthQueryPreparedContinuationOutcome::InstalledAuthorityDrift(drift) => {
            WorthQueryOrdinaryOutcome::RebindRequired(WorthQueryOrdinaryPosture::new(
                drift.to_string(),
                WorthQueryOrdinaryPostureKind::RebindRequired,
                WorthQueryOrdinaryNextStep::RebindContext,
                topology(
                    WorthQueryOrdinaryContinuationCheckedTopologyKind::InstalledAuthorityDrift,
                ),
            ))
        }
        WorthQueryPreparedContinuationOutcome::Stale(reason) => {
            WorthQueryOrdinaryOutcome::Stale(WorthQueryOrdinaryPosture::new(
                reason,
                WorthQueryOrdinaryPostureKind::Stale,
                WorthQueryOrdinaryNextStep::RefreshBasis,
                topology(WorthQueryOrdinaryContinuationCheckedTopologyKind::Stale),
            ))
        }
        WorthQueryPreparedContinuationOutcome::RebindRequired(reason) => {
            WorthQueryOrdinaryOutcome::RebindRequired(WorthQueryOrdinaryPosture::new(
                reason,
                WorthQueryOrdinaryPostureKind::RebindRequired,
                WorthQueryOrdinaryNextStep::RebindContext,
                topology(WorthQueryOrdinaryContinuationCheckedTopologyKind::RebindRequired),
            ))
        }
        WorthQueryPreparedContinuationOutcome::AuthorityMismatch(reason) => {
            WorthQueryOrdinaryOutcome::AuthorityMismatch(WorthQueryOrdinaryPosture::new(
                reason,
                WorthQueryOrdinaryPostureKind::AuthorityMismatch,
                WorthQueryOrdinaryNextStep::InspectProofLane,
                topology(WorthQueryOrdinaryContinuationCheckedTopologyKind::AuthorityMismatch),
            ))
        }
        WorthQueryPreparedContinuationOutcome::BasisMismatch(reason) => {
            WorthQueryOrdinaryOutcome::BasisMismatch(WorthQueryOrdinaryPosture::new(
                reason,
                WorthQueryOrdinaryPostureKind::BasisMismatch,
                WorthQueryOrdinaryNextStep::RefreshBasis,
                topology(WorthQueryOrdinaryContinuationCheckedTopologyKind::BasisMismatch),
            ))
        }
        WorthQueryPreparedContinuationOutcome::Unsupported(reason) => {
            WorthQueryOrdinaryOutcome::Unsupported(WorthQueryOrdinaryPosture::new(
                reason,
                WorthQueryOrdinaryPostureKind::Unsupported,
                WorthQueryOrdinaryNextStep::CheckSupport,
                topology(WorthQueryOrdinaryContinuationCheckedTopologyKind::Unsupported),
            ))
        }
        WorthQueryPreparedContinuationOutcome::Deferred(reason) => {
            WorthQueryOrdinaryOutcome::Deferred(WorthQueryOrdinaryPosture::new(
                reason,
                WorthQueryOrdinaryPostureKind::Deferred,
                WorthQueryOrdinaryNextStep::RetryLater,
                topology(WorthQueryOrdinaryContinuationCheckedTopologyKind::Deferred),
            ))
        }
        WorthQueryPreparedContinuationOutcome::Denied(reason) => {
            WorthQueryOrdinaryOutcome::Denied(WorthQueryOrdinaryPosture::new(
                reason,
                WorthQueryOrdinaryPostureKind::Denied,
                WorthQueryOrdinaryNextStep::InspectCheckedLane,
                topology(WorthQueryOrdinaryContinuationCheckedTopologyKind::Denied),
            ))
        }
        WorthQueryPreparedContinuationOutcome::Failed(reason) => {
            WorthQueryOrdinaryOutcome::Failed(WorthQueryOrdinaryPosture::new(
                reason,
                WorthQueryOrdinaryPostureKind::Failed,
                WorthQueryOrdinaryNextStep::EscalateFailure,
                topology(WorthQueryOrdinaryContinuationCheckedTopologyKind::Failed),
            ))
        }
    }
}
