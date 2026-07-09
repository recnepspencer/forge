use crate::application::{
    WorthQueryDeclarationEntryOrchestrationOutcome,
    WorthQueryDeclarationEntryOrchestrationRefusalClass,
    WorthQueryDeclarationEntryOrchestrationStage, WorthQueryDeclarationInput,
    WorthQueryDomainEntryMarker,
};
use crate::binding_pipeline::WorthQueryBindingLinkedArtifacts;
use crate::ordinary_outcome::{
    WorthQueryOrdinaryBindingCheckedTopologyKind, WorthQueryOrdinaryCheckedTopology,
    WorthQueryOrdinaryNextStep, WorthQueryOrdinaryOutcome, WorthQueryOrdinaryPosture,
    WorthQueryOrdinaryPostureKind,
};

use super::{
    WorthQueryGroupedMemberOrchestrationStop, WorthQueryGroupedOrchestration,
    WorthQueryGroupedOrchestrationAlignmentStop, WorthQueryGroupedOrchestrationChecked,
};

pub(crate) fn ordinary_outcome_from_grouped_orchestration_checked<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    checked: WorthQueryGroupedOrchestrationChecked<D, I>,
) -> WorthQueryOrdinaryOutcome<WorthQueryGroupedOrchestration<D, I>> {
    match checked {
        WorthQueryGroupedOrchestrationChecked::Bound(value) => {
            WorthQueryOrdinaryOutcome::Bound(value)
        }
        WorthQueryGroupedOrchestrationChecked::MemberStopped(stop) => {
            member_stop_to_ordinary_outcome(stop)
        }
        WorthQueryGroupedOrchestrationChecked::WrongWorld(stop) => {
            alignment_stop_to_ordinary_outcome(stop, true)
        }
        WorthQueryGroupedOrchestrationChecked::WrongHandle(stop) => {
            alignment_stop_to_ordinary_outcome(stop, false)
        }
    }
}

fn member_stop_to_ordinary_outcome<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    stop: WorthQueryGroupedMemberOrchestrationStop<D, I>,
) -> WorthQueryOrdinaryOutcome<WorthQueryGroupedOrchestration<D, I>> {
    let member_index = stop.member_index();
    let group_digest = stop.declaration().group_digest().to_string();
    match stop.member_outcome() {
        WorthQueryDeclarationEntryOrchestrationOutcome::Enveloped(_) => {
            panic!("member stop cannot hold an enveloped outcome")
        }
        WorthQueryDeclarationEntryOrchestrationOutcome::Deferred(value) => {
            WorthQueryOrdinaryOutcome::Deferred(posture(
                group_digest,
                member_index,
                value.reason(),
                WorthQueryOrdinaryPostureKind::Deferred,
                WorthQueryOrdinaryNextStep::RetryLater,
                value.stop_stage(),
                value.retained_digest(),
                None,
            ))
        }
        WorthQueryDeclarationEntryOrchestrationOutcome::Denied(value) => {
            WorthQueryOrdinaryOutcome::Denied(posture(
                group_digest,
                member_index,
                value.reason(),
                WorthQueryOrdinaryPostureKind::Denied,
                WorthQueryOrdinaryNextStep::InspectCheckedLane,
                value.stop_stage(),
                value.retained_digest(),
                None,
            ))
        }
        WorthQueryDeclarationEntryOrchestrationOutcome::Stale(value) => {
            WorthQueryOrdinaryOutcome::Stale(posture(
                group_digest,
                member_index,
                value.reason(),
                WorthQueryOrdinaryPostureKind::Stale,
                WorthQueryOrdinaryNextStep::RefreshBasis,
                value.stop_stage(),
                value.retained_digest(),
                None,
            ))
        }
        WorthQueryDeclarationEntryOrchestrationOutcome::RebindRequired(value) => {
            WorthQueryOrdinaryOutcome::RebindRequired(posture(
                group_digest,
                member_index,
                value.reason(),
                WorthQueryOrdinaryPostureKind::RebindRequired,
                WorthQueryOrdinaryNextStep::RebindContext,
                value.stop_stage(),
                value.retained_digest(),
                None,
            ))
        }
        WorthQueryDeclarationEntryOrchestrationOutcome::Failed(value) => {
            WorthQueryOrdinaryOutcome::Failed(posture(
                group_digest,
                member_index,
                value.reason(),
                WorthQueryOrdinaryPostureKind::Failed,
                WorthQueryOrdinaryNextStep::EscalateFailure,
                value.stop_stage(),
                value.retained_digest(),
                None,
            ))
        }
        WorthQueryDeclarationEntryOrchestrationOutcome::Refused(value) => {
            WorthQueryOrdinaryOutcome::Refused(posture(
                group_digest,
                member_index,
                value.reason(),
                WorthQueryOrdinaryPostureKind::Refused,
                refusal_next_step(value.refusal_class()),
                value.stop_stage(),
                value.retained_digest(),
                Some(value.refusal_class()),
            ))
        }
    }
}

fn posture(
    group_digest: String,
    member_index: usize,
    reason: &str,
    kind: WorthQueryOrdinaryPostureKind,
    next_step: WorthQueryOrdinaryNextStep,
    stop_stage: WorthQueryDeclarationEntryOrchestrationStage,
    retained_digest: Option<&str>,
    refusal_class: Option<WorthQueryDeclarationEntryOrchestrationRefusalClass>,
) -> WorthQueryOrdinaryPosture {
    WorthQueryOrdinaryPosture::new(
        format!("group {group_digest} member {member_index}: {reason}"),
        kind,
        next_step,
        WorthQueryOrdinaryCheckedTopology::orchestration(
            stop_stage,
            retained_digest.map(str::to_owned),
            refusal_class,
        ),
    )
}

fn refusal_next_step(
    refusal_class: WorthQueryDeclarationEntryOrchestrationRefusalClass,
) -> WorthQueryOrdinaryNextStep {
    use crate::application::WorthQueryDeclarationEntryOrchestrationRefusalClass as Refusal;

    match refusal_class {
        Refusal::UnsupportedAutomation => WorthQueryOrdinaryNextStep::CheckSupport,
        Refusal::ExplicitIntentRequired => WorthQueryOrdinaryNextStep::NarrowInput,
        Refusal::StrongerProofRequired => WorthQueryOrdinaryNextStep::InspectProofLane,
        Refusal::AuthorityTransitionRequired
        | Refusal::ExpensiveWorkNotAdmittedByDefault
        | Refusal::PreparedButNotExecutedContinuation => {
            WorthQueryOrdinaryNextStep::UseExplicitHandoff
        }
    }
}

fn alignment_posture(
    group_digest: &str,
    reason: &str,
    topology_kind: WorthQueryOrdinaryBindingCheckedTopologyKind,
    posture_kind: WorthQueryOrdinaryPostureKind,
    next_step: WorthQueryOrdinaryNextStep,
) -> WorthQueryOrdinaryPosture {
    WorthQueryOrdinaryPosture::new(
        format!("group {group_digest}: {reason}"),
        posture_kind,
        next_step,
        WorthQueryOrdinaryCheckedTopology::binding(
            topology_kind,
            WorthQueryBindingLinkedArtifacts::new()
                .with_orchestration_digest(group_digest.to_string()),
        ),
    )
}

fn alignment_stop_to_ordinary_outcome<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    stop: WorthQueryGroupedOrchestrationAlignmentStop<D, I>,
    wrong_world: bool,
) -> WorthQueryOrdinaryOutcome<WorthQueryGroupedOrchestration<D, I>> {
    let posture = alignment_posture(
        stop.declaration().group_digest(),
        stop.reason(),
        if wrong_world {
            WorthQueryOrdinaryBindingCheckedTopologyKind::WrongWorld
        } else {
            WorthQueryOrdinaryBindingCheckedTopologyKind::WrongHandle
        },
        if wrong_world {
            WorthQueryOrdinaryPostureKind::WrongWorld
        } else {
            WorthQueryOrdinaryPostureKind::WrongHandle
        },
        if wrong_world {
            WorthQueryOrdinaryNextStep::CorrectWorld
        } else {
            WorthQueryOrdinaryNextStep::CorrectHandle
        },
    );
    if wrong_world {
        WorthQueryOrdinaryOutcome::WrongWorld(posture)
    } else {
        WorthQueryOrdinaryOutcome::WrongHandle(posture)
    }
}
