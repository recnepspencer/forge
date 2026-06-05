use crate::domain_artifacts::HadwigerArtifactKind;
use crate::domain_artifacts::HadwigerCanonicalArtifact;
use crate::domain_declarations::CandidateGraphDeclaration;
use crate::query_entry::HadwigerResearchHandle;
use crate::research_graph_invariants::ResearchGraphInvariantFamily;

use super::actions::{
    action_equivalence_class, ResearchCockpitAction, ResearchCockpitActionBlocker,
    ResearchCockpitActionEligibility, ResearchCockpitActionKind, ResearchCockpitActionPacket,
    ResearchCockpitEquivalenceClass, ResearchCockpitEquivalenceScope,
};
use super::certification::HadwigerCertificationBundle;
use super::report::ResearchCockpitCounters;
use super::session::{ResearchCockpitSession, ResearchCockpitSessionBuilder};
use super::tile_equivalence::{TileEquivalenceWitness, TileEquivalenceWitnessChecked};
use super::ResearchCockpitError;

pub fn assemble_research_cockpit_session_checked(
    handle: &HadwigerResearchHandle,
    builder: ResearchCockpitSessionBuilder,
) -> Result<ResearchCockpitSession, ResearchCockpitError> {
    ResearchCockpitSession::from_builder(builder, handle.handle_identity_digest())
}

pub fn derive_research_cockpit_action_packet_checked(
    handle: &HadwigerResearchHandle,
    session: &ResearchCockpitSession,
) -> Result<ResearchCockpitActionPacket, ResearchCockpitError> {
    let readiness_rows = handle
        .declaration_entry_readiness::<CandidateGraphDeclaration>()
        .rows()
        .len();
    let mut actions = cockpit_actions(session);
    add_stale_frontier_action(session, &mut actions);
    add_invariant_legality_action(session, &mut actions);
    add_missing_checker_evidence_actions(session, &mut actions);
    add_tile_equivalence_actions(session, &mut actions);
    if actions.is_empty() {
        actions.push(ResearchCockpitAction::new(
            "unsupported:no_plannable_work",
            ResearchCockpitActionKind::UnsupportedWork,
            ResearchCockpitActionEligibility::Unsupported,
            Some(ResearchCockpitActionBlocker::UnsupportedTypedEvidence),
            "unsupported:no_plannable_work",
        ));
    }
    let counters = cockpit_counters(session, &actions, readiness_rows);
    let mut equivalence_classes = cockpit_equivalence_classes(session, &actions)?;
    equivalence_classes.sort_by_key(|class| class.reference().stable_token());
    ResearchCockpitActionPacket::new(session, actions, equivalence_classes, counters)
        .map_err(ResearchCockpitError::from)
}

pub fn replay_research_cockpit_session_checked(
    handle: &HadwigerResearchHandle,
    session: &ResearchCockpitSession,
) -> Result<ResearchCockpitActionPacket, ResearchCockpitError> {
    derive_research_cockpit_action_packet_checked(handle, session)
}

pub fn certify_hadwiger_milestone_one_bundle_checked(
    handle: &HadwigerResearchHandle,
    session: &ResearchCockpitSession,
) -> Result<HadwigerCertificationBundle, ResearchCockpitError> {
    let packet = derive_research_cockpit_action_packet_checked(handle, session)?;
    HadwigerCertificationBundle::new(session, &packet).map_err(ResearchCockpitError::from)
}

pub fn declare_tile_equivalence_witness_checked(
    handle: &HadwigerResearchHandle,
    witness: TileEquivalenceWitness,
) -> Result<TileEquivalenceWitnessChecked, ResearchCockpitError> {
    let readiness_rows = handle
        .declaration_entry_readiness::<CandidateGraphDeclaration>()
        .rows()
        .len();
    Ok(TileEquivalenceWitnessChecked::new(witness, readiness_rows))
}

fn cockpit_actions(session: &ResearchCockpitSession) -> Vec<ResearchCockpitAction> {
    let mut actions = Vec::new();
    for (index, plan) in session.frontier().experiment_plans().iter().enumerate() {
        if plan.is_suppressed() {
            let token = plan
                .suppression_proof()
                .map(|proof| proof.dead_end_signature().stable_token().to_string())
                .unwrap_or_else(|| plan.reference().stable_token());
            actions.push(ResearchCockpitAction::new(
                format!("suppressed:{index:04}"),
                ResearchCockpitActionKind::CheckerWork,
                ResearchCockpitActionEligibility::Blocked,
                Some(ResearchCockpitActionBlocker::SuppressedDeadEndEquivalence),
                token,
            ));
        } else {
            actions.push(ResearchCockpitAction::new(
                format!("checker:{index:04}"),
                ResearchCockpitActionKind::CheckerWork,
                ResearchCockpitActionEligibility::Eligible,
                None,
                plan.hypothesis_reference().stable_token(),
            ));
        }
    }
    if let Some(agent) = session.agent_admission() {
        for artifact in agent.advisory_artifacts() {
            actions.push(ResearchCockpitAction::new(
                format!("agent:{}", artifact.advisory_id()),
                ResearchCockpitActionKind::AgentAdvisory,
                ResearchCockpitActionEligibility::AdvisoryOnly,
                Some(ResearchCockpitActionBlocker::AdvisoryOnlyAgentProposal),
                artifact.reference().stable_token(),
            ));
        }
    }
    actions
}

fn cockpit_equivalence_classes(
    session: &ResearchCockpitSession,
    actions: &[ResearchCockpitAction],
) -> Result<Vec<ResearchCockpitEquivalenceClass>, ResearchCockpitError> {
    let mut scoped_tokens = actions
        .iter()
        .filter_map(blocked_action_equivalence_scope_and_token)
        .collect::<Vec<_>>();
    scoped_tokens.sort();
    scoped_tokens.dedup();
    let mut classes = Vec::new();
    for (scope, token) in scoped_tokens {
        if let Some(class) = action_equivalence_class(session, actions, scope, &token)? {
            classes.push(class);
        }
    }
    Ok(classes)
}

fn add_stale_frontier_action(
    session: &ResearchCockpitSession,
    actions: &mut Vec<ResearchCockpitAction>,
) {
    if let Some(derived) = session.derived_frontier_state() {
        if derived.source_corpus_digest() != session.corpus().corpus_digest() {
            actions.push(ResearchCockpitAction::new(
                "blocked:stale_derived_frontier",
                ResearchCockpitActionKind::CheckerWork,
                ResearchCockpitActionEligibility::Blocked,
                Some(ResearchCockpitActionBlocker::StaleDerivedFrontier),
                "stale_derived_frontier",
            ));
        }
    }
}

fn add_invariant_legality_action(
    session: &ResearchCockpitSession,
    actions: &mut Vec<ResearchCockpitAction>,
) {
    if !ResearchGraphInvariantFamily::all()
        .into_iter()
        .all(|family| session.invariant_catalog().has_rule_family(family))
    {
        actions.push(ResearchCockpitAction::new(
            "blocked:invariant_legality",
            ResearchCockpitActionKind::InvariantDenial,
            ResearchCockpitActionEligibility::Blocked,
            Some(ResearchCockpitActionBlocker::ResearchGraphInvariantLegality),
            "research_graph_invariant_legality",
        ));
    }
}

fn add_missing_checker_evidence_actions(
    session: &ResearchCockpitSession,
    actions: &mut Vec<ResearchCockpitAction>,
) {
    let mut partial_tokens =
        evidence_tokens_for_kind(session, HadwigerArtifactKind::PartialAdmissionExplanation);
    partial_tokens.sort();
    partial_tokens.dedup();
    for (index, token) in partial_tokens.into_iter().enumerate() {
        actions.push(ResearchCockpitAction::new(
            format!("blocked:missing_checker_evidence:{index:04}"),
            ResearchCockpitActionKind::ProofAdmission,
            ResearchCockpitActionEligibility::Blocked,
            Some(ResearchCockpitActionBlocker::MissingCheckerEvidence),
            token,
        ));
    }
}

fn add_tile_equivalence_actions(
    session: &ResearchCockpitSession,
    actions: &mut Vec<ResearchCockpitAction>,
) {
    let mut tile_tokens =
        evidence_tokens_for_kind(session, HadwigerArtifactKind::TileEquivalenceWitness);
    tile_tokens.sort();
    tile_tokens.dedup();
    for (index, token) in tile_tokens.into_iter().enumerate() {
        actions.push(ResearchCockpitAction::new(
            format!("blocked:tile_equivalence:{index:04}"),
            ResearchCockpitActionKind::CheckerWork,
            ResearchCockpitActionEligibility::Blocked,
            Some(ResearchCockpitActionBlocker::TileEquivalenceDuplicateCheckerWork),
            token,
        ));
    }
}

fn evidence_tokens_for_kind(
    session: &ResearchCockpitSession,
    kind: HadwigerArtifactKind,
) -> Vec<String> {
    session
        .corpus()
        .evidence_references()
        .iter()
        .map(|reference| reference.stable_token())
        .filter(|token| token.contains(kind.as_str()))
        .collect()
}

fn blocked_action_equivalence_scope_and_token(
    action: &ResearchCockpitAction,
) -> Option<(ResearchCockpitEquivalenceScope, String)> {
    let scope = match action.blocker()? {
        ResearchCockpitActionBlocker::SuppressedDeadEndEquivalence => {
            ResearchCockpitEquivalenceScope::ActionSuppression
        }
        ResearchCockpitActionBlocker::MissingCheckerEvidence => {
            ResearchCockpitEquivalenceScope::ProofAdmission
        }
        ResearchCockpitActionBlocker::TileEquivalenceDuplicateCheckerWork => {
            ResearchCockpitEquivalenceScope::TileContact
        }
        ResearchCockpitActionBlocker::StaleDerivedFrontier
        | ResearchCockpitActionBlocker::ResearchGraphInvariantLegality => {
            ResearchCockpitEquivalenceScope::ActionSuppression
        }
        ResearchCockpitActionBlocker::AdvisoryOnlyAgentProposal
        | ResearchCockpitActionBlocker::UnsupportedTypedEvidence => return None,
    };
    Some((scope, action.equivalence_token().to_string()))
}

fn cockpit_counters(
    session: &ResearchCockpitSession,
    actions: &[ResearchCockpitAction],
    readiness_rows: usize,
) -> ResearchCockpitCounters {
    ResearchCockpitCounters::new(
        session.corpus().evidence_references().len(),
        session.frontier().experiment_plans().len(),
        actions.len(),
        actions
            .iter()
            .filter(|action| {
                action.blocker() == Some(ResearchCockpitActionBlocker::SuppressedDeadEndEquivalence)
            })
            .count(),
        actions
            .iter()
            .filter(|action| {
                action.blocker()
                    == Some(ResearchCockpitActionBlocker::TileEquivalenceDuplicateCheckerWork)
            })
            .count(),
        readiness_rows,
        actions
            .iter()
            .filter(|action| action.eligibility() == ResearchCockpitActionEligibility::Blocked)
            .count(),
    )
}
