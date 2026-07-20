use crate::application::{WorthQueryDeclarationInput, WorthQueryDomainEntryMarker};
use crate::ordinary_outcome::{
    WorthQueryOrdinaryCheckedTopology, WorthQueryOrdinaryContinuationCheckedTopologyKind,
    WorthQueryOrdinaryNextStep, WorthQueryOrdinaryOutcome, WorthQueryOrdinaryPosture,
    WorthQueryOrdinaryPostureKind,
};

use super::{WorthQueryContinuationExecutionChecked, WorthQueryContinuationExecutionOutcome};
use crate::continuation_pipeline::{
    WorthQueryContinuationExecution, WorthQueryContinuationExecutionReadmissionNextAction,
};

pub fn ordinary_outcome_from_execution_checked<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    checked: WorthQueryContinuationExecutionChecked<D, I>,
) -> WorthQueryOrdinaryOutcome<WorthQueryContinuationExecution<D, I>> {
    let topology = |kind| {
        WorthQueryOrdinaryCheckedTopology::continuation(kind, checked.linked_artifacts.clone())
    };
    match checked.outcome {
        WorthQueryContinuationExecutionOutcome::Executed(value) => {
            WorthQueryOrdinaryOutcome::Bound(value)
        }
        WorthQueryContinuationExecutionOutcome::WrongWorld(reason) => {
            WorthQueryOrdinaryOutcome::WrongWorld(WorthQueryOrdinaryPosture::new(
                reason,
                WorthQueryOrdinaryPostureKind::WrongWorld,
                WorthQueryOrdinaryNextStep::CorrectWorld,
                topology(WorthQueryOrdinaryContinuationCheckedTopologyKind::WrongWorld),
            ))
        }
        WorthQueryContinuationExecutionOutcome::AsyncRequestDrift(stop) => {
            WorthQueryOrdinaryOutcome::RebindRequired(WorthQueryOrdinaryPosture::new(
                stop.reason().to_string(),
                WorthQueryOrdinaryPostureKind::RebindRequired,
                ordinary_next_step_for_readmission(stop.next_action()),
                topology(WorthQueryOrdinaryContinuationCheckedTopologyKind::AsyncRequestDrift),
            ))
        }
        WorthQueryContinuationExecutionOutcome::ReplayDrift(stop) => {
            WorthQueryOrdinaryOutcome::BasisMismatch(WorthQueryOrdinaryPosture::new(
                stop.reason().to_string(),
                WorthQueryOrdinaryPostureKind::BasisMismatch,
                ordinary_next_step_for_readmission(stop.next_action()),
                topology(WorthQueryOrdinaryContinuationCheckedTopologyKind::ReplayDrift),
            ))
        }
        WorthQueryContinuationExecutionOutcome::RemaskDrift(stop) => {
            WorthQueryOrdinaryOutcome::Unsupported(WorthQueryOrdinaryPosture::new(
                stop.reason().to_string(),
                WorthQueryOrdinaryPostureKind::Unsupported,
                ordinary_next_step_for_readmission(stop.next_action()),
                topology(WorthQueryOrdinaryContinuationCheckedTopologyKind::RemaskDrift),
            ))
        }
        WorthQueryContinuationExecutionOutcome::PreviewCrossedResidue(stop) => {
            WorthQueryOrdinaryOutcome::RebindRequired(WorthQueryOrdinaryPosture::new(
                stop.reason().to_string(),
                WorthQueryOrdinaryPostureKind::RebindRequired,
                ordinary_next_step_for_readmission(stop.next_action()),
                topology(WorthQueryOrdinaryContinuationCheckedTopologyKind::PreviewCrossedResidue),
            ))
        }
        WorthQueryContinuationExecutionOutcome::InstalledAuthorityDrift(drift) => {
            WorthQueryOrdinaryOutcome::RebindRequired(WorthQueryOrdinaryPosture::new(
                drift.to_string(),
                WorthQueryOrdinaryPostureKind::RebindRequired,
                WorthQueryOrdinaryNextStep::RebindContext,
                topology(
                    WorthQueryOrdinaryContinuationCheckedTopologyKind::InstalledAuthorityDrift,
                ),
            ))
        }
        WorthQueryContinuationExecutionOutcome::Stale(stop) => {
            WorthQueryOrdinaryOutcome::Stale(WorthQueryOrdinaryPosture::new(
                stop.reason().to_string(),
                WorthQueryOrdinaryPostureKind::Stale,
                ordinary_next_step_for_readmission(stop.next_action()),
                topology(WorthQueryOrdinaryContinuationCheckedTopologyKind::Stale),
            ))
        }
        WorthQueryContinuationExecutionOutcome::StaleCompletion(stop) => {
            WorthQueryOrdinaryOutcome::Stale(WorthQueryOrdinaryPosture::new(
                stop.reason().to_string(),
                WorthQueryOrdinaryPostureKind::Stale,
                ordinary_next_step_for_readmission(stop.next_action()),
                topology(WorthQueryOrdinaryContinuationCheckedTopologyKind::StaleCompletion),
            ))
        }
        WorthQueryContinuationExecutionOutcome::BasisMismatch(stop) => {
            WorthQueryOrdinaryOutcome::BasisMismatch(WorthQueryOrdinaryPosture::new(
                stop.reason().to_string(),
                WorthQueryOrdinaryPostureKind::BasisMismatch,
                ordinary_next_step_for_readmission(stop.next_action()),
                topology(WorthQueryOrdinaryContinuationCheckedTopologyKind::BasisMismatch),
            ))
        }
        WorthQueryContinuationExecutionOutcome::LowerBindingMismatch(stop) => {
            WorthQueryOrdinaryOutcome::AuthorityMismatch(WorthQueryOrdinaryPosture::new(
                stop.reason().to_string(),
                WorthQueryOrdinaryPostureKind::AuthorityMismatch,
                ordinary_next_step_for_readmission(stop.next_action()),
                topology(WorthQueryOrdinaryContinuationCheckedTopologyKind::LowerBindingMismatch),
            ))
        }
        WorthQueryContinuationExecutionOutcome::AuthorityMismatch(stop) => {
            WorthQueryOrdinaryOutcome::AuthorityMismatch(WorthQueryOrdinaryPosture::new(
                stop.reason().to_string(),
                WorthQueryOrdinaryPostureKind::AuthorityMismatch,
                ordinary_next_step_for_readmission(stop.next_action()),
                topology(WorthQueryOrdinaryContinuationCheckedTopologyKind::AuthorityMismatch),
            ))
        }
        WorthQueryContinuationExecutionOutcome::WrongHandle(reason) => {
            WorthQueryOrdinaryOutcome::WrongHandle(WorthQueryOrdinaryPosture::new(
                reason,
                WorthQueryOrdinaryPostureKind::WrongHandle,
                WorthQueryOrdinaryNextStep::CorrectHandle,
                topology(WorthQueryOrdinaryContinuationCheckedTopologyKind::WrongHandle),
            ))
        }
        WorthQueryContinuationExecutionOutcome::Unsupported(reason) => {
            WorthQueryOrdinaryOutcome::Unsupported(WorthQueryOrdinaryPosture::new(
                reason,
                WorthQueryOrdinaryPostureKind::Unsupported,
                WorthQueryOrdinaryNextStep::CheckSupport,
                topology(WorthQueryOrdinaryContinuationCheckedTopologyKind::Unsupported),
            ))
        }
    }
}

fn ordinary_next_step_for_readmission(
    action: WorthQueryContinuationExecutionReadmissionNextAction,
) -> WorthQueryOrdinaryNextStep {
    match action {
        WorthQueryContinuationExecutionReadmissionNextAction::RefreshBasis => {
            WorthQueryOrdinaryNextStep::RefreshBasis
        }
        WorthQueryContinuationExecutionReadmissionNextAction::RebindContext => {
            WorthQueryOrdinaryNextStep::RebindContext
        }
        WorthQueryContinuationExecutionReadmissionNextAction::CheckPolicySupport => {
            WorthQueryOrdinaryNextStep::CheckSupport
        }
        WorthQueryContinuationExecutionReadmissionNextAction::UseExplicitHandoff => {
            WorthQueryOrdinaryNextStep::UseExplicitHandoff
        }
        WorthQueryContinuationExecutionReadmissionNextAction::InspectProofLane => {
            WorthQueryOrdinaryNextStep::InspectProofLane
        }
    }
}
