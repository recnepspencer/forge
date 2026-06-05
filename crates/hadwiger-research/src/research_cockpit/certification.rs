use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactReference, HadwigerArtifactShapeError,
    HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::HadwigerCanonicalArtifact;

use super::actions::{ResearchCockpitActionPacket, ResearchCockpitEquivalenceScope};
use super::session::ResearchCockpitSession;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HadwigerCertificationScenario {
    name: &'static str,
    retained_digest: String,
}

impl HadwigerCertificationScenario {
    pub(crate) fn new(name: &'static str, retained_digest: impl Into<String>) -> Self {
        Self {
            name,
            retained_digest: retained_digest.into(),
        }
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn retained_digest(&self) -> &str {
        &self.retained_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HadwigerCertificationDigestInventory {
    rows: Vec<(String, String)>,
}

impl HadwigerCertificationDigestInventory {
    pub(crate) fn new(mut rows: Vec<(String, String)>) -> Self {
        rows.sort();
        rows.dedup();
        Self { rows }
    }

    pub fn rows(&self) -> &[(String, String)] {
        &self.rows
    }

    pub fn contains_discovery_frontier_digest(&self) -> bool {
        self.rows
            .iter()
            .any(|(name, _)| name == "discovery_frontier")
    }

    pub(crate) fn stable_token(&self) -> String {
        self.rows
            .iter()
            .map(|(name, digest)| format!("{name}={digest}"))
            .collect::<Vec<_>>()
            .join(";")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HadwigerCertificationBundle {
    core: HadwigerArtifactCore,
    digest_inventory: HadwigerCertificationDigestInventory,
    scenarios: Vec<HadwigerCertificationScenario>,
}

impl HadwigerCertificationBundle {
    pub(crate) fn new(
        session: &ResearchCockpitSession,
        packet: &ResearchCockpitActionPacket,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let digest_inventory = digest_inventory(session, packet);
        let scenarios = certification_scenarios(session, packet);
        let core = artifact_core(
            HadwigerArtifactKind::HadwigerCertificationBundle,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "hadwiger_milestone_one_certification_bundle".to_string(),
            },
            certification_parents(session, packet),
            certification_payload(&digest_inventory, &scenarios),
        )?;
        Ok(Self {
            core,
            digest_inventory,
            scenarios,
        })
    }

    pub fn digest_inventory(&self) -> &HadwigerCertificationDigestInventory {
        &self.digest_inventory
    }

    pub fn scenarios(&self) -> &[HadwigerCertificationScenario] {
        &self.scenarios
    }

    pub fn registers_query_invariant_authority(&self) -> bool {
        false
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(HadwigerCertificationBundle, core);

fn digest_inventory(
    session: &ResearchCockpitSession,
    packet: &ResearchCockpitActionPacket,
) -> HadwigerCertificationDigestInventory {
    let mut rows = vec![
        (
            "research_cockpit_session".to_string(),
            session.artifact_digest().stable_token().to_string(),
        ),
        (
            "research_cockpit_action_packet".to_string(),
            packet.artifact_digest().stable_token().to_string(),
        ),
        (
            "research_cockpit_report".to_string(),
            packet.report().artifact_digest().stable_token().to_string(),
        ),
        (
            "evidence_corpus".to_string(),
            session
                .corpus()
                .artifact_digest()
                .stable_token()
                .to_string(),
        ),
        (
            "discovery_frontier".to_string(),
            session
                .frontier()
                .artifact_digest()
                .stable_token()
                .to_string(),
        ),
        (
            "research_graph_invariant_catalog".to_string(),
            session
                .invariant_catalog()
                .artifact_digest()
                .stable_token()
                .to_string(),
        ),
        (
            "counter_snapshot".to_string(),
            packet.counters().stable_token(),
        ),
    ];
    if let Some(agent) = session.agent_admission() {
        rows.push((
            "agent_advisory".to_string(),
            agent.batch().artifact_digest().stable_token().to_string(),
        ));
    }
    if let Some(derived) = session.derived_frontier_state() {
        rows.push((
            "derived_frontier_state".to_string(),
            derived.artifact_digest().stable_token().to_string(),
        ));
    }
    for (index, evidence) in session.corpus().evidence_references().iter().enumerate() {
        rows.push((
            format!("retained_evidence:{index:04}"),
            evidence.stable_token(),
        ));
    }
    for class in packet.equivalence_classes() {
        rows.push((
            format!("equivalence:{}", class.scope().as_str()),
            class.artifact_digest().stable_token().to_string(),
        ));
    }
    HadwigerCertificationDigestInventory::new(rows)
}

fn certification_scenarios(
    session: &ResearchCockpitSession,
    packet: &ResearchCockpitActionPacket,
) -> Vec<HadwigerCertificationScenario> {
    let report_digest = packet.report().artifact_digest().stable_token().to_string();
    let agent_digest = session
        .agent_admission()
        .map(|agent| agent.batch().artifact_digest().stable_token().to_string())
        .unwrap_or_else(|| report_digest.clone());
    let tile_digest = packet
        .equivalence_classes()
        .iter()
        .find(|class| class.scope() == ResearchCockpitEquivalenceScope::TileContact)
        .map(|class| class.artifact_digest().stable_token().to_string())
        .unwrap_or_else(|| report_digest.clone());
    vec![
        HadwigerCertificationScenario::new("dead-end suppression", report_digest.clone()),
        HadwigerCertificationScenario::new("agent advisory boundary", agent_digest),
        HadwigerCertificationScenario::new(
            "research graph invariant registration blocked",
            session
                .invariant_catalog()
                .artifact_digest()
                .stable_token()
                .to_string(),
        ),
        HadwigerCertificationScenario::new(
            "research cockpit replay",
            packet.artifact_digest().stable_token().to_string(),
        ),
        HadwigerCertificationScenario::new("tile equivalence advisory", tile_digest),
    ]
}

fn certification_parents(
    session: &ResearchCockpitSession,
    packet: &ResearchCockpitActionPacket,
) -> Vec<HadwigerArtifactReference> {
    let mut parents = vec![
        session.reference(),
        packet.reference(),
        packet.report().reference(),
    ];
    parents.extend(
        packet
            .equivalence_classes()
            .iter()
            .map(HadwigerCanonicalArtifact::reference),
    );
    parents
}

fn certification_payload(
    inventory: &HadwigerCertificationDigestInventory,
    scenarios: &[HadwigerCertificationScenario],
) -> Vec<HadwigerArtifactPayloadEntry> {
    let mut payload = vec![HadwigerArtifactPayloadEntry::text(
        "digest_inventory",
        inventory.stable_token(),
    )];
    for scenario in scenarios {
        payload.push(HadwigerArtifactPayloadEntry::text(
            "scenario",
            format!("{}={}", scenario.name(), scenario.retained_digest()),
        ));
    }
    payload
}
