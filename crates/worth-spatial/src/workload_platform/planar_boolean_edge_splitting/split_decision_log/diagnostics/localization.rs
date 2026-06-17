use super::affected_artifact::PlanarBooleanSplitAffectedArtifact;
use super::kind::PlanarBooleanSplitDecisionKind;
use super::phase::PlanarBooleanSplitDecisionPhase;
use super::row::PlanarBooleanSplitDecisionRow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanSplitFailureLocalization {
    localization_identity: String,
    decision_identity: String,
    phase: PlanarBooleanSplitDecisionPhase,
    kind: PlanarBooleanSplitDecisionKind,
    affected_artifact: PlanarBooleanSplitAffectedArtifact,
    affected_artifact_identity: String,
    source_edge_identity: String,
    carrier_identity: String,
    event_identities: Vec<String>,
    event_group_identities: Vec<String>,
    policy_or_denial_kind: Option<String>,
}

impl PlanarBooleanSplitFailureLocalization {
    pub(crate) fn from_row(row: &PlanarBooleanSplitDecisionRow) -> Self {
        let localization_identity = format!("edge-split-localization:{}", row.decision_identity());
        Self {
            localization_identity,
            decision_identity: row.decision_identity().to_string(),
            phase: row.phase(),
            kind: row.kind(),
            affected_artifact: row.affected_artifact(),
            affected_artifact_identity: row.affected_artifact_identity().to_string(),
            source_edge_identity: row.source_edge_identity().to_string(),
            carrier_identity: row.carrier_identity().to_string(),
            event_identities: row.event_identities().to_vec(),
            event_group_identities: row.event_group_identities().to_vec(),
            policy_or_denial_kind: row.policy_or_denial_kind().map(str::to_string),
        }
    }
    pub fn localization_identity(&self) -> &str {
        &self.localization_identity
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
    pub fn policy_or_denial_kind(&self) -> Option<&str> {
        self.policy_or_denial_kind.as_deref()
    }
}
