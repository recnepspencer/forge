use crate::domain_artifacts::{
    HadwigerCanonicalArtifact, HadwigerDeclaredFamilyCheckedExt, HadwigerQueryDeclarationReference,
};
use crate::domain_declarations::{
    declare_research_request_checked, LowerBoundTilingIterationDeclaration,
    UpperBoundTilingIterationDeclaration,
};
use crate::query_entry::HadwigerResearchHandle;
use crate::research_graph_invariants::ResearchGraphInvariantFamily;

use super::packet_artifacts::{
    TilingIterationAction, TilingIterationActionKind, TilingIterationPacket,
    TilingIterationPacketKind,
};
use super::packet_blockers::TilingIterationBlocker;
use super::packet_counters::{TilingIterationCounterInput, TilingIterationCounters};
use super::packet_eligibility::TilingIterationActionEligibility;
use super::packet_errors::TilingIterationError;
use super::packet_replay::TilingIterationReplayReport;
use super::packet_requests::TilingIterationPacketRequest;

pub fn derive_tiling_iteration_packet_checked(
    handle: &HadwigerResearchHandle,
    request: TilingIterationPacketRequest,
) -> Result<TilingIterationPacket, TilingIterationError> {
    let session = request
        .cockpit_session()
        .ok_or(TilingIterationError::MissingCockpitSession)?;
    if request.required_checker_lanes().is_empty() {
        return Err(TilingIterationError::MissingRequiredCheckerLane);
    }
    if request.evidence_basis().is_empty() {
        return Err(TilingIterationError::MissingEvidenceBasis);
    }
    if request.expected_information_gain().is_none() {
        return Err(TilingIterationError::MissingExpectedInformationGain);
    }
    if request.reactivation_obligations().is_empty() {
        return Err(TilingIterationError::MissingReactivationObligation);
    }
    let query_reference = declare_iteration_intent(handle, &request)?;
    let readiness_rows = iteration_readiness_rows(handle, request.packet_kind());
    let global_blocker = global_packet_blocker(session, readiness_rows);
    let actions = iteration_actions(&request, global_blocker);
    let counters = iteration_counters(&request, &actions, readiness_rows);
    TilingIterationPacket::checked(&request, query_reference, actions, counters)
        .map_err(TilingIterationError::from)
}

pub fn replay_tiling_iteration_packet_checked(
    _handle: &HadwigerResearchHandle,
    packet: &TilingIterationPacket,
) -> Result<TilingIterationReplayReport, TilingIterationError> {
    TilingIterationReplayReport::checked(packet).map_err(TilingIterationError::from)
}

fn declare_iteration_intent(
    handle: &HadwigerResearchHandle,
    request: &TilingIterationPacketRequest,
) -> Result<HadwigerQueryDeclarationReference, TilingIterationError> {
    match request.packet_kind() {
        TilingIterationPacketKind::LowerBoundObstruction => {
            let checked = declare_research_request_checked(
                handle,
                LowerBoundTilingIterationDeclaration::try_new(
                    request.packet_id(),
                    request
                        .cockpit_session()
                        .ok_or(TilingIterationError::MissingCockpitSession)?
                        .session_digest(),
                    request.evidence_basis().to_vec(),
                    request.required_checker_lanes().to_vec(),
                    request.reactivation_obligations().to_vec(),
                )?,
            );
            checked.admitted().map(Into::into).ok_or(
                TilingIterationError::QueryDeclarationNotAdmitted {
                    declaration: "lower_bound_tiling_iteration",
                },
            )
        }
        TilingIterationPacketKind::UpperBoundPeriodicQuotient => {
            let checked = declare_research_request_checked(
                handle,
                UpperBoundTilingIterationDeclaration::try_new(
                    request.packet_id(),
                    request
                        .cockpit_session()
                        .ok_or(TilingIterationError::MissingCockpitSession)?
                        .session_digest(),
                    request.evidence_basis().to_vec(),
                    request.required_checker_lanes().to_vec(),
                    request.reactivation_obligations().to_vec(),
                )?,
            );
            checked.admitted().map(Into::into).ok_or(
                TilingIterationError::QueryDeclarationNotAdmitted {
                    declaration: "upper_bound_tiling_iteration",
                },
            )
        }
    }
}

fn iteration_readiness_rows(
    handle: &HadwigerResearchHandle,
    packet_kind: TilingIterationPacketKind,
) -> usize {
    match packet_kind {
        TilingIterationPacketKind::LowerBoundObstruction => handle
            .declaration_entry_readiness::<LowerBoundTilingIterationDeclaration>()
            .rows()
            .len(),
        TilingIterationPacketKind::UpperBoundPeriodicQuotient => handle
            .declaration_entry_readiness::<UpperBoundTilingIterationDeclaration>()
            .rows()
            .len(),
    }
}

fn global_packet_blocker(
    session: &crate::research_cockpit::ResearchCockpitSession,
    readiness_rows: usize,
) -> Option<TilingIterationBlocker> {
    if readiness_rows == 0 {
        return Some(TilingIterationBlocker::MissingQueryReadiness);
    }
    if let Some(derived) = session.derived_frontier_state() {
        if derived.source_corpus_digest() != session.corpus().corpus_digest() {
            return Some(TilingIterationBlocker::StaleDerivedFrontier);
        }
    }
    if !ResearchGraphInvariantFamily::all()
        .into_iter()
        .all(|family| session.invariant_catalog().has_rule_family(family))
    {
        return Some(TilingIterationBlocker::ResearchGraphInvariantLegality);
    }
    None
}

fn iteration_actions(
    request: &TilingIterationPacketRequest,
    global_blocker: Option<TilingIterationBlocker>,
) -> Vec<TilingIterationAction> {
    let mut actions = frontier_plan_actions(request, global_blocker);
    actions.extend(agent_advisory_actions(request));
    if actions.is_empty() {
        actions.push(TilingIterationAction::new(
            "unsupported:no_iteration_work",
            TilingIterationActionKind::UnsupportedWork,
            TilingIterationActionEligibility::Unsupported,
            Some(TilingIterationBlocker::UnsupportedTypedEvidence),
            "unsupported:no_iteration_work",
        ));
    }
    actions
}

fn frontier_plan_actions(
    request: &TilingIterationPacketRequest,
    global_blocker: Option<TilingIterationBlocker>,
) -> Vec<TilingIterationAction> {
    let session = request
        .cockpit_session()
        .expect("packet request session already validated");
    session
        .frontier()
        .experiment_plans()
        .iter()
        .enumerate()
        .map(|(index, plan)| {
            let blocker = global_blocker.or_else(|| {
                if plan.is_suppressed() {
                    Some(TilingIterationBlocker::SuppressedDeadEndEquivalence)
                } else {
                    None
                }
            });
            let eligibility = if blocker.is_some() {
                TilingIterationActionEligibility::Blocked
            } else {
                TilingIterationActionEligibility::EligibleNextCheckerInput
            };
            let basis = plan
                .suppression_proof()
                .map(|proof| proof.dead_end_signature().stable_token().to_string())
                .unwrap_or_else(|| plan.hypothesis_reference().stable_token());
            TilingIterationAction::new(
                format!(
                    "{}:frontier_plan:{index:04}",
                    request.packet_kind().as_str()
                ),
                TilingIterationActionKind::CheckerInputPreparation,
                eligibility,
                blocker,
                basis,
            )
        })
        .collect()
}

fn agent_advisory_actions(request: &TilingIterationPacketRequest) -> Vec<TilingIterationAction> {
    let Some(session) = request.cockpit_session() else {
        return Vec::new();
    };
    let Some(agent) = session.agent_admission() else {
        return Vec::new();
    };
    agent
        .advisory_artifacts()
        .iter()
        .map(|artifact| {
            TilingIterationAction::new(
                format!("agent_advisory:{}", artifact.advisory_id()),
                TilingIterationActionKind::AgentAdvisoryPreview,
                TilingIterationActionEligibility::AdvisoryOnly,
                Some(TilingIterationBlocker::AdvisoryOnlyAgentProposal),
                artifact.reference().stable_token(),
            )
        })
        .collect()
}

fn iteration_counters(
    request: &TilingIterationPacketRequest,
    actions: &[TilingIterationAction],
    readiness_rows: usize,
) -> TilingIterationCounters {
    TilingIterationCounters::new(TilingIterationCounterInput {
        query_declarations_checked: 1,
        query_readiness_rows: readiness_rows,
        required_checker_lanes: request.required_checker_lanes().len(),
        eligible_actions: actions
            .iter()
            .filter(|action| {
                action.eligibility() == TilingIterationActionEligibility::EligibleNextCheckerInput
            })
            .count(),
        blocked_actions: actions
            .iter()
            .filter(|action| action.eligibility() == TilingIterationActionEligibility::Blocked)
            .count(),
        stale_frontier_blocks: blocker_count(actions, TilingIterationBlocker::StaleDerivedFrontier),
        suppression_blocks: blocker_count(
            actions,
            TilingIterationBlocker::SuppressedDeadEndEquivalence,
        ),
        invariant_legality_blocks: blocker_count(
            actions,
            TilingIterationBlocker::ResearchGraphInvariantLegality,
        ),
        advisory_only_rows: actions
            .iter()
            .filter(|action| action.eligibility() == TilingIterationActionEligibility::AdvisoryOnly)
            .count(),
        unsupported_rows: actions
            .iter()
            .filter(|action| action.eligibility() == TilingIterationActionEligibility::Unsupported)
            .count(),
        equivalence_basis_rows: actions
            .iter()
            .map(TilingIterationAction::equivalence_basis)
            .collect::<std::collections::BTreeSet<_>>()
            .len(),
    })
}

fn blocker_count(actions: &[TilingIterationAction], blocker: TilingIterationBlocker) -> usize {
    actions
        .iter()
        .filter(|action| action.blocker() == Some(blocker))
        .count()
}
