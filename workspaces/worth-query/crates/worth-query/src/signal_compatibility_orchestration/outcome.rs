use crate::application::{WorthQueryDeclarationInput, WorthQueryDomainEntryMarker};
use crate::ordinary_outcome::{
    WorthQueryOrdinaryCheckedTopology, WorthQueryOrdinaryNextStep, WorthQueryOrdinaryOutcome,
    WorthQueryOrdinaryPosture, WorthQueryOrdinaryPostureKind,
    WorthQueryOrdinarySignalCompatibilityOrchestrationCheckedTopologyKind,
};

use super::WorthQuerySignalCompatibilityOrchestration;

pub enum WorthQuerySignalCompatibilityOrchestrationOutcome<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    Bound(WorthQuerySignalCompatibilityOrchestration<D, I>),
    Ambiguous(String),
    Unavailable(String),
    WrongWorld(String),
    WrongHandle(String),
    InstalledAuthorityDrift(crate::domain_installation::WorthQueryInstalledDomainExecutionDrift),
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

pub struct WorthQuerySignalCompatibilityOrchestrationChecked<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    outcome: WorthQuerySignalCompatibilityOrchestrationOutcome<D, I>,
    orchestration_digest: String,
    linked_artifacts: crate::binding_pipeline::WorthQueryBindingLinkedArtifacts,
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQuerySignalCompatibilityOrchestrationChecked<D, I>
{
    pub(crate) fn new(
        outcome: WorthQuerySignalCompatibilityOrchestrationOutcome<D, I>,
        orchestration_digest: String,
        linked_artifacts: crate::binding_pipeline::WorthQueryBindingLinkedArtifacts,
    ) -> Self {
        Self {
            outcome,
            orchestration_digest,
            linked_artifacts,
        }
    }

    pub fn outcome(&self) -> &WorthQuerySignalCompatibilityOrchestrationOutcome<D, I> {
        &self.outcome
    }

    pub fn orchestration_digest(&self) -> &str {
        &self.orchestration_digest
    }

    pub fn linked_artifacts(&self) -> &crate::binding_pipeline::WorthQueryBindingLinkedArtifacts {
        &self.linked_artifacts
    }

    pub(crate) fn into_outcome(self) -> WorthQuerySignalCompatibilityOrchestrationOutcome<D, I> {
        self.outcome
    }
}

pub fn ordinary_outcome_from_signal_compatibility_orchestration_checked<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    checked: WorthQuerySignalCompatibilityOrchestrationChecked<D, I>,
) -> WorthQueryOrdinaryOutcome<WorthQuerySignalCompatibilityOrchestration<D, I>> {
    let topology = |kind| {
        WorthQueryOrdinaryCheckedTopology::signal_compatibility_orchestration(
            kind,
            checked.linked_artifacts.clone(),
        )
    };
    match checked.outcome {
        WorthQuerySignalCompatibilityOrchestrationOutcome::Bound(value) => {
            WorthQueryOrdinaryOutcome::Bound(value)
        }
        WorthQuerySignalCompatibilityOrchestrationOutcome::Ambiguous(reason) => {
            WorthQueryOrdinaryOutcome::Ambiguous(WorthQueryOrdinaryPosture::new(
                reason,
                WorthQueryOrdinaryPostureKind::Ambiguous,
                WorthQueryOrdinaryNextStep::NarrowInput,
                topology(WorthQueryOrdinarySignalCompatibilityOrchestrationCheckedTopologyKind::Ambiguous),
            ))
        }
        WorthQuerySignalCompatibilityOrchestrationOutcome::Unavailable(reason) => {
            WorthQueryOrdinaryOutcome::Unavailable(WorthQueryOrdinaryPosture::new(
                reason,
                WorthQueryOrdinaryPostureKind::Unavailable,
                WorthQueryOrdinaryNextStep::GatherAvailability,
                topology(WorthQueryOrdinarySignalCompatibilityOrchestrationCheckedTopologyKind::Unavailable),
            ))
        }
        WorthQuerySignalCompatibilityOrchestrationOutcome::WrongWorld(reason) => {
            WorthQueryOrdinaryOutcome::WrongWorld(WorthQueryOrdinaryPosture::new(
                reason,
                WorthQueryOrdinaryPostureKind::WrongWorld,
                WorthQueryOrdinaryNextStep::CorrectWorld,
                topology(WorthQueryOrdinarySignalCompatibilityOrchestrationCheckedTopologyKind::WrongWorld),
            ))
        }
        WorthQuerySignalCompatibilityOrchestrationOutcome::WrongHandle(reason) => {
            WorthQueryOrdinaryOutcome::WrongHandle(WorthQueryOrdinaryPosture::new(
                reason,
                WorthQueryOrdinaryPostureKind::WrongHandle,
                WorthQueryOrdinaryNextStep::CorrectHandle,
                topology(WorthQueryOrdinarySignalCompatibilityOrchestrationCheckedTopologyKind::WrongHandle),
            ))
        }
        WorthQuerySignalCompatibilityOrchestrationOutcome::InstalledAuthorityDrift(drift) => {
            WorthQueryOrdinaryOutcome::RebindRequired(WorthQueryOrdinaryPosture::new(
                drift.to_string(),
                WorthQueryOrdinaryPostureKind::RebindRequired,
                WorthQueryOrdinaryNextStep::RebindContext,
                topology(WorthQueryOrdinarySignalCompatibilityOrchestrationCheckedTopologyKind::InstalledAuthorityDrift),
            ))
        }
        WorthQuerySignalCompatibilityOrchestrationOutcome::Stale(reason) => {
            WorthQueryOrdinaryOutcome::Stale(WorthQueryOrdinaryPosture::new(
                reason,
                WorthQueryOrdinaryPostureKind::Stale,
                WorthQueryOrdinaryNextStep::RefreshBasis,
                topology(WorthQueryOrdinarySignalCompatibilityOrchestrationCheckedTopologyKind::Stale),
            ))
        }
        WorthQuerySignalCompatibilityOrchestrationOutcome::RebindRequired(reason) => {
            WorthQueryOrdinaryOutcome::RebindRequired(WorthQueryOrdinaryPosture::new(
                reason,
                WorthQueryOrdinaryPostureKind::RebindRequired,
                WorthQueryOrdinaryNextStep::RebindContext,
                topology(WorthQueryOrdinarySignalCompatibilityOrchestrationCheckedTopologyKind::RebindRequired),
            ))
        }
        WorthQuerySignalCompatibilityOrchestrationOutcome::MissingRequiredAspect(reason) => {
            WorthQueryOrdinaryOutcome::MissingRequiredAspect(WorthQueryOrdinaryPosture::new(
                reason,
                WorthQueryOrdinaryPostureKind::MissingRequiredAspect,
                WorthQueryOrdinaryNextStep::InspectCheckedLane,
                topology(WorthQueryOrdinarySignalCompatibilityOrchestrationCheckedTopologyKind::MissingRequiredAspect),
            ))
        }
        WorthQuerySignalCompatibilityOrchestrationOutcome::AspectConflict(reason) => {
            WorthQueryOrdinaryOutcome::AspectConflict(WorthQueryOrdinaryPosture::new(
                reason,
                WorthQueryOrdinaryPostureKind::AspectConflict,
                WorthQueryOrdinaryNextStep::InspectCheckedLane,
                topology(WorthQueryOrdinarySignalCompatibilityOrchestrationCheckedTopologyKind::AspectConflict),
            ))
        }
        WorthQuerySignalCompatibilityOrchestrationOutcome::AuthorityMismatch(reason) => {
            WorthQueryOrdinaryOutcome::AuthorityMismatch(WorthQueryOrdinaryPosture::new(
                reason,
                WorthQueryOrdinaryPostureKind::AuthorityMismatch,
                WorthQueryOrdinaryNextStep::InspectProofLane,
                topology(WorthQueryOrdinarySignalCompatibilityOrchestrationCheckedTopologyKind::AuthorityMismatch),
            ))
        }
        WorthQuerySignalCompatibilityOrchestrationOutcome::BasisMismatch(reason) => {
            WorthQueryOrdinaryOutcome::BasisMismatch(WorthQueryOrdinaryPosture::new(
                reason,
                WorthQueryOrdinaryPostureKind::BasisMismatch,
                WorthQueryOrdinaryNextStep::RefreshBasis,
                topology(WorthQueryOrdinarySignalCompatibilityOrchestrationCheckedTopologyKind::BasisMismatch),
            ))
        }
        WorthQuerySignalCompatibilityOrchestrationOutcome::Deferred(reason) => {
            WorthQueryOrdinaryOutcome::Deferred(WorthQueryOrdinaryPosture::new(
                reason,
                WorthQueryOrdinaryPostureKind::Deferred,
                WorthQueryOrdinaryNextStep::RetryLater,
                topology(WorthQueryOrdinarySignalCompatibilityOrchestrationCheckedTopologyKind::Deferred),
            ))
        }
        WorthQuerySignalCompatibilityOrchestrationOutcome::Denied(reason) => {
            WorthQueryOrdinaryOutcome::Denied(WorthQueryOrdinaryPosture::new(
                reason,
                WorthQueryOrdinaryPostureKind::Denied,
                WorthQueryOrdinaryNextStep::InspectCheckedLane,
                topology(WorthQueryOrdinarySignalCompatibilityOrchestrationCheckedTopologyKind::Denied),
            ))
        }
        WorthQuerySignalCompatibilityOrchestrationOutcome::Unsupported(reason) => {
            WorthQueryOrdinaryOutcome::Unsupported(WorthQueryOrdinaryPosture::new(
                reason,
                WorthQueryOrdinaryPostureKind::Unsupported,
                WorthQueryOrdinaryNextStep::CheckSupport,
                topology(WorthQueryOrdinarySignalCompatibilityOrchestrationCheckedTopologyKind::Unsupported),
            ))
        }
        WorthQuerySignalCompatibilityOrchestrationOutcome::Failed(reason) => {
            WorthQueryOrdinaryOutcome::Failed(WorthQueryOrdinaryPosture::new(
                reason,
                WorthQueryOrdinaryPostureKind::Failed,
                WorthQueryOrdinaryNextStep::EscalateFailure,
                topology(WorthQueryOrdinarySignalCompatibilityOrchestrationCheckedTopologyKind::Failed),
            ))
        }
    }
}
