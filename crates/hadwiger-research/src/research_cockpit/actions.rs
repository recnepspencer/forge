use std::collections::BTreeSet;

use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactShapeError, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::HadwigerCanonicalArtifact;

use super::report::{ResearchCockpitCounters, ResearchCockpitReport};
use super::session::ResearchCockpitSession;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ResearchCockpitActionKind {
    CheckerWork,
    ProofAdmission,
    InvariantDenial,
    AgentAdvisory,
    UnsupportedWork,
}

impl ResearchCockpitActionKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CheckerWork => "checker_work",
            Self::ProofAdmission => "proof_admission",
            Self::InvariantDenial => "invariant_denial",
            Self::AgentAdvisory => "agent_advisory",
            Self::UnsupportedWork => "unsupported_work",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ResearchCockpitActionEligibility {
    Eligible,
    Blocked,
    AdvisoryOnly,
    Unsupported,
}

impl ResearchCockpitActionEligibility {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Eligible => "eligible",
            Self::Blocked => "blocked",
            Self::AdvisoryOnly => "advisory_only",
            Self::Unsupported => "unsupported",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ResearchCockpitActionBlocker {
    MissingCheckerEvidence,
    StaleDerivedFrontier,
    SuppressedDeadEndEquivalence,
    TileEquivalenceDuplicateCheckerWork,
    ResearchGraphInvariantLegality,
    AdvisoryOnlyAgentProposal,
    UnsupportedTypedEvidence,
}

impl ResearchCockpitActionBlocker {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingCheckerEvidence => "missing_checker_evidence",
            Self::StaleDerivedFrontier => "stale_derived_frontier",
            Self::SuppressedDeadEndEquivalence => "suppressed_dead_end_equivalence",
            Self::TileEquivalenceDuplicateCheckerWork => "tile_equivalence_duplicate_checker_work",
            Self::ResearchGraphInvariantLegality => "research_graph_invariant_legality",
            Self::AdvisoryOnlyAgentProposal => "advisory_only_agent_proposal",
            Self::UnsupportedTypedEvidence => "unsupported_typed_evidence",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResearchCockpitAction {
    action_id: String,
    kind: ResearchCockpitActionKind,
    eligibility: ResearchCockpitActionEligibility,
    blocker: Option<ResearchCockpitActionBlocker>,
    equivalence_token: String,
}

impl ResearchCockpitAction {
    pub(crate) fn new(
        action_id: impl Into<String>,
        kind: ResearchCockpitActionKind,
        eligibility: ResearchCockpitActionEligibility,
        blocker: Option<ResearchCockpitActionBlocker>,
        equivalence_token: impl Into<String>,
    ) -> Self {
        Self {
            action_id: action_id.into(),
            kind,
            eligibility,
            blocker,
            equivalence_token: equivalence_token.into(),
        }
    }

    pub fn action_id(&self) -> &str {
        &self.action_id
    }

    pub fn kind(&self) -> ResearchCockpitActionKind {
        self.kind
    }

    pub fn eligibility(&self) -> ResearchCockpitActionEligibility {
        self.eligibility
    }

    pub fn blocker(&self) -> Option<ResearchCockpitActionBlocker> {
        self.blocker
    }

    pub fn equivalence_token(&self) -> &str {
        &self.equivalence_token
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ResearchCockpitEquivalenceScope {
    ActionSuppression,
    CheckerInput,
    ProofAdmission,
    TileContact,
}

impl ResearchCockpitEquivalenceScope {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ActionSuppression => "action_suppression",
            Self::CheckerInput => "checker_input",
            Self::ProofAdmission => "proof_admission",
            Self::TileContact => "tile_contact",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResearchCockpitEquivalenceClass {
    core: HadwigerArtifactCore,
    scope: ResearchCockpitEquivalenceScope,
    equivalence_token: String,
    member_tokens: Vec<String>,
}

impl ResearchCockpitEquivalenceClass {
    pub(crate) fn new(
        session: &ResearchCockpitSession,
        scope: ResearchCockpitEquivalenceScope,
        equivalence_token: impl Into<String>,
        mut member_tokens: Vec<String>,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        member_tokens.sort();
        member_tokens.dedup();
        let equivalence_token = equivalence_token.into();
        let core = artifact_core(
            HadwigerArtifactKind::ResearchCockpitEquivalenceClass,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "research_cockpit_equivalence_class".to_string(),
            },
            vec![session.reference()],
            equivalence_payload(scope, &equivalence_token, &member_tokens),
        )?;
        Ok(Self {
            core,
            scope,
            equivalence_token,
            member_tokens,
        })
    }

    pub fn scope(&self) -> ResearchCockpitEquivalenceScope {
        self.scope
    }

    pub fn equivalence_token(&self) -> &str {
        &self.equivalence_token
    }

    pub fn member_tokens(&self) -> &[String] {
        &self.member_tokens
    }
}

impl_hadwiger_artifact!(ResearchCockpitEquivalenceClass, core);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResearchCockpitActionPacket {
    core: HadwigerArtifactCore,
    source_session_digest: String,
    actions: Vec<ResearchCockpitAction>,
    equivalence_classes: Vec<ResearchCockpitEquivalenceClass>,
    report: ResearchCockpitReport,
}

impl ResearchCockpitActionPacket {
    pub(crate) fn new(
        session: &ResearchCockpitSession,
        mut actions: Vec<ResearchCockpitAction>,
        mut equivalence_classes: Vec<ResearchCockpitEquivalenceClass>,
        counters: ResearchCockpitCounters,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        actions.sort_by(|left, right| left.action_id.cmp(&right.action_id));
        dedupe_actions(&mut actions);
        equivalence_classes.sort_by_key(|class| class.reference().stable_token());
        let report = ResearchCockpitReport::new(session, counters)?;
        let mut parents = vec![session.reference(), report.reference()];
        parents.extend(
            equivalence_classes
                .iter()
                .map(ResearchCockpitEquivalenceClass::reference),
        );
        let source_session_digest = session.session_digest().to_string();
        let core = artifact_core(
            HadwigerArtifactKind::ResearchCockpitActionPacket,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "research_cockpit_action_packet".to_string(),
            },
            parents,
            action_packet_payload(
                &source_session_digest,
                &actions,
                &equivalence_classes,
                &report,
            ),
        )?;
        Ok(Self {
            core,
            source_session_digest,
            actions,
            equivalence_classes,
            report,
        })
    }

    pub fn source_session_digest(&self) -> &str {
        &self.source_session_digest
    }

    pub fn actions(&self) -> &[ResearchCockpitAction] {
        &self.actions
    }

    pub fn equivalence_classes(&self) -> &[ResearchCockpitEquivalenceClass] {
        &self.equivalence_classes
    }

    pub fn report(&self) -> &ResearchCockpitReport {
        &self.report
    }

    pub fn counters(&self) -> &ResearchCockpitCounters {
        self.report.counters()
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(ResearchCockpitActionPacket, core);

pub(crate) fn action_equivalence_class(
    session: &ResearchCockpitSession,
    actions: &[ResearchCockpitAction],
    scope: ResearchCockpitEquivalenceScope,
    equivalence_token: &str,
) -> Result<Option<ResearchCockpitEquivalenceClass>, HadwigerArtifactShapeError> {
    let members = actions
        .iter()
        .filter(|action| action.equivalence_token() == equivalence_token)
        .map(|action| action.action_id().to_string())
        .collect::<Vec<_>>();
    if members.is_empty() {
        Ok(None)
    } else {
        ResearchCockpitEquivalenceClass::new(session, scope, equivalence_token, members).map(Some)
    }
}

fn dedupe_actions(actions: &mut Vec<ResearchCockpitAction>) {
    let mut seen = BTreeSet::new();
    actions.retain(|action| {
        seen.insert(format!(
            "{}:{}",
            action.kind().as_str(),
            action.equivalence_token()
        ))
    });
}

fn equivalence_payload(
    scope: ResearchCockpitEquivalenceScope,
    equivalence_token: &str,
    member_tokens: &[String],
) -> Vec<HadwigerArtifactPayloadEntry> {
    let mut payload = vec![
        HadwigerArtifactPayloadEntry::text("scope", scope.as_str()),
        HadwigerArtifactPayloadEntry::text("equivalence_token", equivalence_token),
    ];
    for member in member_tokens {
        payload.push(HadwigerArtifactPayloadEntry::text("member", member));
    }
    payload
}

fn action_packet_payload(
    source_session_digest: &str,
    actions: &[ResearchCockpitAction],
    equivalence_classes: &[ResearchCockpitEquivalenceClass],
    report: &ResearchCockpitReport,
) -> Vec<HadwigerArtifactPayloadEntry> {
    let mut payload = vec![
        HadwigerArtifactPayloadEntry::text("source_session_digest", source_session_digest),
        HadwigerArtifactPayloadEntry::text("report", report.reference().stable_token()),
    ];
    for action in actions {
        payload.push(HadwigerArtifactPayloadEntry::text(
            "action",
            format!(
                "{}:{}:{}:{}",
                action.action_id(),
                action.kind().as_str(),
                action.eligibility().as_str(),
                action
                    .blocker()
                    .map(ResearchCockpitActionBlocker::as_str)
                    .unwrap_or("none")
            ),
        ));
    }
    for class in equivalence_classes {
        payload.push(HadwigerArtifactPayloadEntry::text(
            "equivalence_class",
            class.reference().stable_token(),
        ));
    }
    payload
}
