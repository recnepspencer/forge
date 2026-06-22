use super::affected_artifact::PlanarBooleanSplitAffectedArtifact;
use super::decision_reason::PlanarBooleanSplitDecisionReason;
use super::kind::PlanarBooleanSplitDecisionKind;
use super::phase::PlanarBooleanSplitDecisionPhase;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanSplitDecisionRow {
    decision_identity: String,
    phase: PlanarBooleanSplitDecisionPhase,
    kind: PlanarBooleanSplitDecisionKind,
    affected_artifact: PlanarBooleanSplitAffectedArtifact,
    affected_artifact_identity: String,
    source_edge_identity: String,
    carrier_identity: String,
    event_identities: Vec<String>,
    event_group_identities: Vec<String>,
    provenance_identities: Vec<String>,
    upstream_receipt_identity: String,
    decision_reason: PlanarBooleanSplitDecisionReason,
    policy_or_denial_kind: Option<String>,
}

impl PlanarBooleanSplitDecisionRow {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        decision_identity: String,
        phase: PlanarBooleanSplitDecisionPhase,
        kind: PlanarBooleanSplitDecisionKind,
        affected_artifact: PlanarBooleanSplitAffectedArtifact,
        affected_artifact_identity: String,
        source_edge_identity: String,
        carrier_identity: String,
        event_identities: Vec<String>,
        event_group_identities: Vec<String>,
        provenance_identities: Vec<String>,
        upstream_receipt_identity: String,
        decision_reason: PlanarBooleanSplitDecisionReason,
        policy_or_denial_kind: Option<String>,
    ) -> Self {
        Self {
            decision_identity,
            phase,
            kind,
            affected_artifact,
            affected_artifact_identity,
            source_edge_identity,
            carrier_identity,
            event_identities,
            event_group_identities,
            provenance_identities,
            upstream_receipt_identity,
            decision_reason,
            policy_or_denial_kind,
        }
    }
    pub fn decision_identity(&self) -> &str {
        &self.decision_identity
    }
    pub fn phase(&self) -> PlanarBooleanSplitDecisionPhase {
        self.phase
    }
    pub fn kind(&self) -> PlanarBooleanSplitDecisionKind {
        self.kind
    }
    pub fn affected_artifact(&self) -> PlanarBooleanSplitAffectedArtifact {
        self.affected_artifact
    }
    pub fn affected_artifact_identity(&self) -> &str {
        &self.affected_artifact_identity
    }
    pub fn source_edge_identity(&self) -> &str {
        &self.source_edge_identity
    }
    pub fn carrier_identity(&self) -> &str {
        &self.carrier_identity
    }
    pub fn event_identities(&self) -> &[String] {
        &self.event_identities
    }
    pub fn event_group_identities(&self) -> &[String] {
        &self.event_group_identities
    }
    pub fn provenance_identities(&self) -> &[String] {
        &self.provenance_identities
    }
    pub fn upstream_receipt_identity(&self) -> &str {
        &self.upstream_receipt_identity
    }
    pub fn decision_reason(&self) -> &PlanarBooleanSplitDecisionReason {
        &self.decision_reason
    }
    pub fn policy_or_denial_kind(&self) -> Option<&str> {
        self.policy_or_denial_kind.as_deref()
    }
}
