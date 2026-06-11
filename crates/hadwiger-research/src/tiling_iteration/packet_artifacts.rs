use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactReference, HadwigerArtifactShapeError,
    HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::{HadwigerCanonicalArtifact, HadwigerQueryDeclarationReference};

use super::packet_blockers::TilingIterationBlocker;
use super::packet_counters::TilingIterationCounters;
use super::packet_eligibility::TilingIterationActionEligibility;
use super::packet_requests::TilingIterationPacketRequest;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum TilingIterationPacketKind {
    LowerBoundObstruction,
    UpperBoundPeriodicQuotient,
}

impl TilingIterationPacketKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::LowerBoundObstruction => "lower_bound_obstruction",
            Self::UpperBoundPeriodicQuotient => "upper_bound_periodic_quotient",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum TilingIterationActionKind {
    CheckerInputPreparation,
    ProofGapReview,
    InvariantLegalityReview,
    AgentAdvisoryPreview,
    UnsupportedWork,
}

impl TilingIterationActionKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CheckerInputPreparation => "checker_input_preparation",
            Self::ProofGapReview => "proof_gap_review",
            Self::InvariantLegalityReview => "invariant_legality_review",
            Self::AgentAdvisoryPreview => "agent_advisory_preview",
            Self::UnsupportedWork => "unsupported_work",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TilingIterationAction {
    action_id: String,
    action_kind: TilingIterationActionKind,
    eligibility: TilingIterationActionEligibility,
    blocker: Option<TilingIterationBlocker>,
    equivalence_basis: String,
}

impl TilingIterationAction {
    pub(crate) fn new(
        action_id: impl Into<String>,
        action_kind: TilingIterationActionKind,
        eligibility: TilingIterationActionEligibility,
        blocker: Option<TilingIterationBlocker>,
        equivalence_basis: impl Into<String>,
    ) -> Self {
        Self {
            action_id: action_id.into(),
            action_kind,
            eligibility,
            blocker,
            equivalence_basis: equivalence_basis.into(),
        }
    }

    pub fn action_id(&self) -> &str {
        &self.action_id
    }

    pub fn action_kind(&self) -> TilingIterationActionKind {
        self.action_kind
    }

    pub fn eligibility(&self) -> TilingIterationActionEligibility {
        self.eligibility
    }

    pub fn blocker(&self) -> Option<TilingIterationBlocker> {
        self.blocker
    }

    pub fn equivalence_basis(&self) -> &str {
        &self.equivalence_basis
    }

    pub fn executes(&self) -> bool {
        false
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TilingIterationPacket {
    core: HadwigerArtifactCore,
    packet_id: String,
    packet_kind: TilingIterationPacketKind,
    query_declaration_reference: HadwigerQueryDeclarationReference,
    source_session_digest: String,
    evidence_basis: Vec<String>,
    required_checker_lanes: Vec<String>,
    expected_information_gain: String,
    reactivation_obligations: Vec<String>,
    actions: Vec<TilingIterationAction>,
    counters: TilingIterationCounters,
}

impl TilingIterationPacket {
    pub(crate) fn checked(
        request: &TilingIterationPacketRequest,
        query_declaration_reference: HadwigerQueryDeclarationReference,
        mut actions: Vec<TilingIterationAction>,
        counters: TilingIterationCounters,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let session = request
            .cockpit_session()
            .expect("packet request session already validated");
        let mut evidence_basis = request.evidence_basis().to_vec();
        evidence_basis.sort();
        evidence_basis.dedup();
        let mut required_checker_lanes = request.required_checker_lanes().to_vec();
        required_checker_lanes.sort();
        required_checker_lanes.dedup();
        let mut reactivation_obligations = request.reactivation_obligations().to_vec();
        reactivation_obligations.sort();
        reactivation_obligations.dedup();
        actions.sort_by(|left, right| left.action_id.cmp(&right.action_id));
        let source_session_digest = session.session_digest().to_string();
        let expected_information_gain = request
            .expected_information_gain()
            .expect("expected information gain already validated")
            .to_string();
        let mut parents = vec![session.reference()];
        parents.extend(action_parent_references(session));
        let core = artifact_core(
            HadwigerArtifactKind::TilingIterationPacket,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::QueryDeclaration(query_declaration_reference.clone()),
            parents,
            packet_payload(
                request,
                &source_session_digest,
                &evidence_basis,
                &required_checker_lanes,
                &expected_information_gain,
                &reactivation_obligations,
                &actions,
                &counters,
            ),
        )?;
        Ok(Self {
            core,
            packet_id: request.packet_id().to_string(),
            packet_kind: request.packet_kind(),
            query_declaration_reference,
            source_session_digest,
            evidence_basis,
            required_checker_lanes,
            expected_information_gain,
            reactivation_obligations,
            actions,
            counters,
        })
    }

    pub fn packet_id(&self) -> &str {
        &self.packet_id
    }

    pub fn packet_kind(&self) -> TilingIterationPacketKind {
        self.packet_kind
    }

    pub fn packet_digest(&self) -> &str {
        self.artifact_digest().stable_token()
    }

    pub fn query_declaration_reference(&self) -> &HadwigerQueryDeclarationReference {
        &self.query_declaration_reference
    }

    pub fn source_session_digest(&self) -> &str {
        &self.source_session_digest
    }

    pub fn required_checker_lanes(&self) -> &[String] {
        &self.required_checker_lanes
    }

    pub fn evidence_basis(&self) -> &[String] {
        &self.evidence_basis
    }

    pub fn expected_information_gain(&self) -> &str {
        &self.expected_information_gain
    }

    pub fn reactivation_obligations(&self) -> &[String] {
        &self.reactivation_obligations
    }

    pub fn actions(&self) -> &[TilingIterationAction] {
        &self.actions
    }

    pub fn counters(&self) -> &TilingIterationCounters {
        &self.counters
    }

    pub fn query_readiness_checks(&self) -> usize {
        self.counters.query_readiness_rows()
    }

    pub fn is_replayable(&self) -> bool {
        true
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }

    pub fn executes_checker_work(&self) -> bool {
        false
    }

    pub fn registers_query_invariant_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(TilingIterationPacket, core);

fn action_parent_references(
    session: &crate::research_cockpit::ResearchCockpitSession,
) -> Vec<HadwigerArtifactReference> {
    let mut references = vec![session.corpus().reference(), session.frontier().reference()];
    if let Some(derived) = session.derived_frontier_state() {
        references.push(derived.reference());
    }
    references
}

fn packet_payload(
    request: &TilingIterationPacketRequest,
    source_session_digest: &str,
    evidence_basis: &[String],
    required_checker_lanes: &[String],
    expected_information_gain: &str,
    reactivation_obligations: &[String],
    actions: &[TilingIterationAction],
    counters: &TilingIterationCounters,
) -> Vec<HadwigerArtifactPayloadEntry> {
    let mut payload = vec![
        HadwigerArtifactPayloadEntry::text("packet_id", request.packet_id()),
        HadwigerArtifactPayloadEntry::text("packet_kind", request.packet_kind().as_str()),
        HadwigerArtifactPayloadEntry::text("source_session_digest", source_session_digest),
        HadwigerArtifactPayloadEntry::text("expected_information_gain", expected_information_gain),
    ];
    for basis in evidence_basis {
        payload.push(HadwigerArtifactPayloadEntry::text("evidence_basis", basis));
    }
    for lane in required_checker_lanes {
        payload.push(HadwigerArtifactPayloadEntry::text(
            "required_checker_lane",
            lane,
        ));
    }
    for obligation in reactivation_obligations {
        payload.push(HadwigerArtifactPayloadEntry::text(
            "reactivation_obligation",
            obligation,
        ));
    }
    for action in actions {
        payload.push(HadwigerArtifactPayloadEntry::text(
            "action",
            format!(
                "{}:{}:{}:{}:{}",
                action.action_id(),
                action.action_kind().as_str(),
                action.eligibility().as_str(),
                action
                    .blocker()
                    .map(TilingIterationBlocker::as_str)
                    .unwrap_or("none"),
                action.equivalence_basis()
            ),
        ));
    }
    push_counter_payload(&mut payload, counters);
    payload
}

fn push_counter_payload(
    payload: &mut Vec<HadwigerArtifactPayloadEntry>,
    counters: &TilingIterationCounters,
) {
    payload.push(HadwigerArtifactPayloadEntry::unsigned(
        "query_declarations_checked",
        counters.query_declarations_checked() as u128,
    ));
    payload.push(HadwigerArtifactPayloadEntry::unsigned(
        "query_readiness_rows",
        counters.query_readiness_rows() as u128,
    ));
    payload.push(HadwigerArtifactPayloadEntry::unsigned(
        "blocked_actions",
        counters.blocked_actions() as u128,
    ));
    payload.push(HadwigerArtifactPayloadEntry::unsigned(
        "equivalence_basis_rows",
        counters.equivalence_basis_rows() as u128,
    ));
}
