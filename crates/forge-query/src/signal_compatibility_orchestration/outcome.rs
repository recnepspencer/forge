use crate::application::{ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker};
use crate::ordinary_outcome::{
    ForgeQueryOrdinaryCheckedTopology, ForgeQueryOrdinaryNextStep, ForgeQueryOrdinaryOutcome,
    ForgeQueryOrdinaryPosture, ForgeQueryOrdinaryPostureKind,
    ForgeQueryOrdinarySignalCompatibilityOrchestrationCheckedTopologyKind,
};

use super::ForgeQuerySignalCompatibilityOrchestration;

pub enum ForgeQuerySignalCompatibilityOrchestrationOutcome<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    Bound(ForgeQuerySignalCompatibilityOrchestration<D, I>),
    Ambiguous(String),
    Unavailable(String),
    WrongWorld(String),
    WrongHandle(String),
    Stale(String),
    RebindRequired(String),
    MissingRequiredAspect(String),
    AspectConflict(String),
    AuthorityMismatch(String),
    BasisMismatch(String),
    Deferred(String),
    Denied(String),
    Unsupported(String),
    Failed(String),
}

pub struct ForgeQuerySignalCompatibilityOrchestrationChecked<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    outcome: ForgeQuerySignalCompatibilityOrchestrationOutcome<D, I>,
    orchestration_digest: String,
    linked_artifacts: crate::binding_pipeline::ForgeQueryBindingLinkedArtifacts,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQuerySignalCompatibilityOrchestrationChecked<D, I>
{
    pub(crate) fn new(
        outcome: ForgeQuerySignalCompatibilityOrchestrationOutcome<D, I>,
        orchestration_digest: String,
        linked_artifacts: crate::binding_pipeline::ForgeQueryBindingLinkedArtifacts,
    ) -> Self {
        Self {
            outcome,
            orchestration_digest,
            linked_artifacts,
        }
    }

    pub fn outcome(&self) -> &ForgeQuerySignalCompatibilityOrchestrationOutcome<D, I> {
        &self.outcome
    }

    pub fn orchestration_digest(&self) -> &str {
        &self.orchestration_digest
    }

    pub fn linked_artifacts(&self) -> &crate::binding_pipeline::ForgeQueryBindingLinkedArtifacts {
        &self.linked_artifacts
    }

    pub(crate) fn into_outcome(self) -> ForgeQuerySignalCompatibilityOrchestrationOutcome<D, I> {
        self.outcome
    }
}

pub fn ordinary_outcome_from_signal_compatibility_orchestration_checked<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    checked: ForgeQuerySignalCompatibilityOrchestrationChecked<D, I>,
) -> ForgeQueryOrdinaryOutcome<ForgeQuerySignalCompatibilityOrchestration<D, I>> {
    let topology = |kind| {
        ForgeQueryOrdinaryCheckedTopology::signal_compatibility_orchestration(
            kind,
            checked.linked_artifacts.clone(),
        )
    };
    match checked.outcome {
        ForgeQuerySignalCompatibilityOrchestrationOutcome::Bound(value) => {
            ForgeQueryOrdinaryOutcome::Bound(value)
        }
        ForgeQuerySignalCompatibilityOrchestrationOutcome::Ambiguous(reason) => {
            ForgeQueryOrdinaryOutcome::Ambiguous(ForgeQueryOrdinaryPosture::new(
                reason,
                ForgeQueryOrdinaryPostureKind::Ambiguous,
                ForgeQueryOrdinaryNextStep::NarrowInput,
                topology(ForgeQueryOrdinarySignalCompatibilityOrchestrationCheckedTopologyKind::Ambiguous),
            ))
        }
        ForgeQuerySignalCompatibilityOrchestrationOutcome::Unavailable(reason) => {
            ForgeQueryOrdinaryOutcome::Unavailable(ForgeQueryOrdinaryPosture::new(
                reason,
                ForgeQueryOrdinaryPostureKind::Unavailable,
                ForgeQueryOrdinaryNextStep::GatherAvailability,
                topology(ForgeQueryOrdinarySignalCompatibilityOrchestrationCheckedTopologyKind::Unavailable),
            ))
        }
        ForgeQuerySignalCompatibilityOrchestrationOutcome::WrongWorld(reason) => {
            ForgeQueryOrdinaryOutcome::WrongWorld(ForgeQueryOrdinaryPosture::new(
                reason,
                ForgeQueryOrdinaryPostureKind::WrongWorld,
                ForgeQueryOrdinaryNextStep::CorrectWorld,
                topology(ForgeQueryOrdinarySignalCompatibilityOrchestrationCheckedTopologyKind::WrongWorld),
            ))
        }
        ForgeQuerySignalCompatibilityOrchestrationOutcome::WrongHandle(reason) => {
            ForgeQueryOrdinaryOutcome::WrongHandle(ForgeQueryOrdinaryPosture::new(
                reason,
                ForgeQueryOrdinaryPostureKind::WrongHandle,
                ForgeQueryOrdinaryNextStep::CorrectHandle,
                topology(ForgeQueryOrdinarySignalCompatibilityOrchestrationCheckedTopologyKind::WrongHandle),
            ))
        }
        ForgeQuerySignalCompatibilityOrchestrationOutcome::Stale(reason) => {
            ForgeQueryOrdinaryOutcome::Stale(ForgeQueryOrdinaryPosture::new(
                reason,
                ForgeQueryOrdinaryPostureKind::Stale,
                ForgeQueryOrdinaryNextStep::RefreshBasis,
                topology(ForgeQueryOrdinarySignalCompatibilityOrchestrationCheckedTopologyKind::Stale),
            ))
        }
        ForgeQuerySignalCompatibilityOrchestrationOutcome::RebindRequired(reason) => {
            ForgeQueryOrdinaryOutcome::RebindRequired(ForgeQueryOrdinaryPosture::new(
                reason,
                ForgeQueryOrdinaryPostureKind::RebindRequired,
                ForgeQueryOrdinaryNextStep::RebindContext,
                topology(ForgeQueryOrdinarySignalCompatibilityOrchestrationCheckedTopologyKind::RebindRequired),
            ))
        }
        ForgeQuerySignalCompatibilityOrchestrationOutcome::MissingRequiredAspect(reason) => {
            ForgeQueryOrdinaryOutcome::MissingRequiredAspect(ForgeQueryOrdinaryPosture::new(
                reason,
                ForgeQueryOrdinaryPostureKind::MissingRequiredAspect,
                ForgeQueryOrdinaryNextStep::InspectCheckedLane,
                topology(ForgeQueryOrdinarySignalCompatibilityOrchestrationCheckedTopologyKind::MissingRequiredAspect),
            ))
        }
        ForgeQuerySignalCompatibilityOrchestrationOutcome::AspectConflict(reason) => {
            ForgeQueryOrdinaryOutcome::AspectConflict(ForgeQueryOrdinaryPosture::new(
                reason,
                ForgeQueryOrdinaryPostureKind::AspectConflict,
                ForgeQueryOrdinaryNextStep::InspectCheckedLane,
                topology(ForgeQueryOrdinarySignalCompatibilityOrchestrationCheckedTopologyKind::AspectConflict),
            ))
        }
        ForgeQuerySignalCompatibilityOrchestrationOutcome::AuthorityMismatch(reason) => {
            ForgeQueryOrdinaryOutcome::AuthorityMismatch(ForgeQueryOrdinaryPosture::new(
                reason,
                ForgeQueryOrdinaryPostureKind::AuthorityMismatch,
                ForgeQueryOrdinaryNextStep::InspectProofLane,
                topology(ForgeQueryOrdinarySignalCompatibilityOrchestrationCheckedTopologyKind::AuthorityMismatch),
            ))
        }
        ForgeQuerySignalCompatibilityOrchestrationOutcome::BasisMismatch(reason) => {
            ForgeQueryOrdinaryOutcome::BasisMismatch(ForgeQueryOrdinaryPosture::new(
                reason,
                ForgeQueryOrdinaryPostureKind::BasisMismatch,
                ForgeQueryOrdinaryNextStep::RefreshBasis,
                topology(ForgeQueryOrdinarySignalCompatibilityOrchestrationCheckedTopologyKind::BasisMismatch),
            ))
        }
        ForgeQuerySignalCompatibilityOrchestrationOutcome::Deferred(reason) => {
            ForgeQueryOrdinaryOutcome::Deferred(ForgeQueryOrdinaryPosture::new(
                reason,
                ForgeQueryOrdinaryPostureKind::Deferred,
                ForgeQueryOrdinaryNextStep::RetryLater,
                topology(ForgeQueryOrdinarySignalCompatibilityOrchestrationCheckedTopologyKind::Deferred),
            ))
        }
        ForgeQuerySignalCompatibilityOrchestrationOutcome::Denied(reason) => {
            ForgeQueryOrdinaryOutcome::Denied(ForgeQueryOrdinaryPosture::new(
                reason,
                ForgeQueryOrdinaryPostureKind::Denied,
                ForgeQueryOrdinaryNextStep::InspectCheckedLane,
                topology(ForgeQueryOrdinarySignalCompatibilityOrchestrationCheckedTopologyKind::Denied),
            ))
        }
        ForgeQuerySignalCompatibilityOrchestrationOutcome::Unsupported(reason) => {
            ForgeQueryOrdinaryOutcome::Unsupported(ForgeQueryOrdinaryPosture::new(
                reason,
                ForgeQueryOrdinaryPostureKind::Unsupported,
                ForgeQueryOrdinaryNextStep::CheckSupport,
                topology(ForgeQueryOrdinarySignalCompatibilityOrchestrationCheckedTopologyKind::Unsupported),
            ))
        }
        ForgeQuerySignalCompatibilityOrchestrationOutcome::Failed(reason) => {
            ForgeQueryOrdinaryOutcome::Failed(ForgeQueryOrdinaryPosture::new(
                reason,
                ForgeQueryOrdinaryPostureKind::Failed,
                ForgeQueryOrdinaryNextStep::EscalateFailure,
                topology(ForgeQueryOrdinarySignalCompatibilityOrchestrationCheckedTopologyKind::Failed),
            ))
        }
    }
}
