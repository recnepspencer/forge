use crate::application::{
    ForgeQueryDeclarationEntryOrchestrationOutcome,
    ForgeQueryDeclarationEntryOrchestrationRefusalClass,
    ForgeQueryDeclarationEntryOrchestrationStage, ForgeQueryDeclarationInput,
    ForgeQueryDomainEntryMarker,
};
use crate::binding_pipeline::ForgeQueryBindingLinkedArtifacts;
use crate::ordinary_outcome::{
    ForgeQueryOrdinaryBindingCheckedTopologyKind, ForgeQueryOrdinaryCheckedTopology,
    ForgeQueryOrdinaryNextStep, ForgeQueryOrdinaryOutcome, ForgeQueryOrdinaryPosture,
    ForgeQueryOrdinaryPostureKind,
};

use super::{
    ForgeQueryGroupedMemberOrchestrationStop, ForgeQueryGroupedOrchestration,
    ForgeQueryGroupedOrchestrationAlignmentStop, ForgeQueryGroupedOrchestrationChecked,
};

pub(crate) fn ordinary_outcome_from_grouped_orchestration_checked<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    checked: ForgeQueryGroupedOrchestrationChecked<D, I>,
) -> ForgeQueryOrdinaryOutcome<ForgeQueryGroupedOrchestration<D, I>> {
    match checked {
        ForgeQueryGroupedOrchestrationChecked::Bound(value) => {
            ForgeQueryOrdinaryOutcome::Bound(value)
        }
        ForgeQueryGroupedOrchestrationChecked::MemberStopped(stop) => {
            member_stop_to_ordinary_outcome(stop)
        }
        ForgeQueryGroupedOrchestrationChecked::WrongWorld(stop) => {
            alignment_stop_to_ordinary_outcome(stop, true)
        }
        ForgeQueryGroupedOrchestrationChecked::WrongHandle(stop) => {
            alignment_stop_to_ordinary_outcome(stop, false)
        }
    }
}

fn member_stop_to_ordinary_outcome<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    stop: ForgeQueryGroupedMemberOrchestrationStop<D, I>,
) -> ForgeQueryOrdinaryOutcome<ForgeQueryGroupedOrchestration<D, I>> {
    let member_index = stop.member_index();
    let group_digest = stop.declaration().group_digest().to_string();
    match stop.member_outcome() {
        ForgeQueryDeclarationEntryOrchestrationOutcome::Enveloped(_) => {
            panic!("member stop cannot hold an enveloped outcome")
        }
        ForgeQueryDeclarationEntryOrchestrationOutcome::Deferred(value) => {
            ForgeQueryOrdinaryOutcome::Deferred(posture(
                group_digest,
                member_index,
                value.reason(),
                ForgeQueryOrdinaryPostureKind::Deferred,
                ForgeQueryOrdinaryNextStep::RetryLater,
                value.stop_stage(),
                value.retained_digest(),
                None,
            ))
        }
        ForgeQueryDeclarationEntryOrchestrationOutcome::Denied(value) => {
            ForgeQueryOrdinaryOutcome::Denied(posture(
                group_digest,
                member_index,
                value.reason(),
                ForgeQueryOrdinaryPostureKind::Denied,
                ForgeQueryOrdinaryNextStep::InspectCheckedLane,
                value.stop_stage(),
                value.retained_digest(),
                None,
            ))
        }
        ForgeQueryDeclarationEntryOrchestrationOutcome::Stale(value) => {
            ForgeQueryOrdinaryOutcome::Stale(posture(
                group_digest,
                member_index,
                value.reason(),
                ForgeQueryOrdinaryPostureKind::Stale,
                ForgeQueryOrdinaryNextStep::RefreshBasis,
                value.stop_stage(),
                value.retained_digest(),
                None,
            ))
        }
        ForgeQueryDeclarationEntryOrchestrationOutcome::RebindRequired(value) => {
            ForgeQueryOrdinaryOutcome::RebindRequired(posture(
                group_digest,
                member_index,
                value.reason(),
                ForgeQueryOrdinaryPostureKind::RebindRequired,
                ForgeQueryOrdinaryNextStep::RebindContext,
                value.stop_stage(),
                value.retained_digest(),
                None,
            ))
        }
        ForgeQueryDeclarationEntryOrchestrationOutcome::Failed(value) => {
            ForgeQueryOrdinaryOutcome::Failed(posture(
                group_digest,
                member_index,
                value.reason(),
                ForgeQueryOrdinaryPostureKind::Failed,
                ForgeQueryOrdinaryNextStep::EscalateFailure,
                value.stop_stage(),
                value.retained_digest(),
                None,
            ))
        }
        ForgeQueryDeclarationEntryOrchestrationOutcome::Refused(value) => {
            ForgeQueryOrdinaryOutcome::Refused(posture(
                group_digest,
                member_index,
                value.reason(),
                ForgeQueryOrdinaryPostureKind::Refused,
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
    kind: ForgeQueryOrdinaryPostureKind,
    next_step: ForgeQueryOrdinaryNextStep,
    stop_stage: ForgeQueryDeclarationEntryOrchestrationStage,
    retained_digest: Option<&str>,
    refusal_class: Option<ForgeQueryDeclarationEntryOrchestrationRefusalClass>,
) -> ForgeQueryOrdinaryPosture {
    ForgeQueryOrdinaryPosture::new(
        format!("group {group_digest} member {member_index}: {reason}"),
        kind,
        next_step,
        ForgeQueryOrdinaryCheckedTopology::orchestration(
            stop_stage,
            retained_digest.map(str::to_owned),
            refusal_class,
        ),
    )
}

fn refusal_next_step(
    refusal_class: ForgeQueryDeclarationEntryOrchestrationRefusalClass,
) -> ForgeQueryOrdinaryNextStep {
    use crate::application::ForgeQueryDeclarationEntryOrchestrationRefusalClass as Refusal;

    match refusal_class {
        Refusal::UnsupportedAutomation => ForgeQueryOrdinaryNextStep::CheckSupport,
        Refusal::ExplicitIntentRequired => ForgeQueryOrdinaryNextStep::NarrowInput,
        Refusal::StrongerProofRequired => ForgeQueryOrdinaryNextStep::InspectProofLane,
        Refusal::AuthorityTransitionRequired
        | Refusal::ExpensiveWorkNotAdmittedByDefault
        | Refusal::PreparedButNotExecutedContinuation => {
            ForgeQueryOrdinaryNextStep::UseExplicitHandoff
        }
    }
}

fn alignment_posture(
    group_digest: &str,
    reason: &str,
    topology_kind: ForgeQueryOrdinaryBindingCheckedTopologyKind,
    posture_kind: ForgeQueryOrdinaryPostureKind,
    next_step: ForgeQueryOrdinaryNextStep,
) -> ForgeQueryOrdinaryPosture {
    ForgeQueryOrdinaryPosture::new(
        format!("group {group_digest}: {reason}"),
        posture_kind,
        next_step,
        ForgeQueryOrdinaryCheckedTopology::binding(
            topology_kind,
            ForgeQueryBindingLinkedArtifacts::new()
                .with_orchestration_digest(group_digest.to_string()),
        ),
    )
}

fn alignment_stop_to_ordinary_outcome<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    stop: ForgeQueryGroupedOrchestrationAlignmentStop<D, I>,
    wrong_world: bool,
) -> ForgeQueryOrdinaryOutcome<ForgeQueryGroupedOrchestration<D, I>> {
    let posture = alignment_posture(
        stop.declaration().group_digest(),
        stop.reason(),
        if wrong_world {
            ForgeQueryOrdinaryBindingCheckedTopologyKind::WrongWorld
        } else {
            ForgeQueryOrdinaryBindingCheckedTopologyKind::WrongHandle
        },
        if wrong_world {
            ForgeQueryOrdinaryPostureKind::WrongWorld
        } else {
            ForgeQueryOrdinaryPostureKind::WrongHandle
        },
        if wrong_world {
            ForgeQueryOrdinaryNextStep::CorrectWorld
        } else {
            ForgeQueryOrdinaryNextStep::CorrectHandle
        },
    );
    if wrong_world {
        ForgeQueryOrdinaryOutcome::WrongWorld(posture)
    } else {
        ForgeQueryOrdinaryOutcome::WrongHandle(posture)
    }
}
