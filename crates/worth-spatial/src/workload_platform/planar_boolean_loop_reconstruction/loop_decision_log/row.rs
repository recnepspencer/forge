use super::vocabulary::{
    PlanarBooleanLoopDecisionAffectedArtifact, PlanarBooleanLoopDecisionKind,
    PlanarBooleanLoopDecisionPhase, PlanarBooleanLoopDecisionReason,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopDecisionRow {
    decision_identity: String,
    phase: PlanarBooleanLoopDecisionPhase,
    kind: PlanarBooleanLoopDecisionKind,
    affected_artifact: PlanarBooleanLoopDecisionAffectedArtifact,
    affected_artifact_identity: String,
    source_loop_identities: Vec<String>,
    fragment_identities: Vec<String>,
    split_vertex_identities: Vec<String>,
    upstream_artifact_identities: Vec<String>,
    policy_or_denial_kind: Option<String>,
    reason: PlanarBooleanLoopDecisionReason,
    human_reason: String,
}

impl PlanarBooleanLoopDecisionRow {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        decision_identity: String,
        phase: PlanarBooleanLoopDecisionPhase,
        kind: PlanarBooleanLoopDecisionKind,
        affected_artifact: PlanarBooleanLoopDecisionAffectedArtifact,
        affected_artifact_identity: String,
        source_loop_identities: Vec<String>,
        fragment_identities: Vec<String>,
        split_vertex_identities: Vec<String>,
        upstream_artifact_identities: Vec<String>,
        policy_or_denial_kind: Option<String>,
        reason: PlanarBooleanLoopDecisionReason,
        human_reason: String,
    ) -> Self {
        Self {
            decision_identity,
            phase,
            kind,
            affected_artifact,
            affected_artifact_identity,
            source_loop_identities,
            fragment_identities,
            split_vertex_identities,
            upstream_artifact_identities,
            policy_or_denial_kind,
            reason,
            human_reason,
        }
    }

    pub fn decision_identity(&self) -> &str {
        &self.decision_identity
    }

    pub fn phase(&self) -> PlanarBooleanLoopDecisionPhase {
        self.phase
    }

    pub fn kind(&self) -> PlanarBooleanLoopDecisionKind {
        self.kind
    }

    pub fn affected_artifact(&self) -> PlanarBooleanLoopDecisionAffectedArtifact {
        self.affected_artifact
    }

    pub fn affected_artifact_identity(&self) -> &str {
        &self.affected_artifact_identity
    }

    pub fn source_loop_identities(&self) -> &[String] {
        &self.source_loop_identities
    }

    pub fn fragment_identities(&self) -> &[String] {
        &self.fragment_identities
    }

    pub fn split_vertex_identities(&self) -> &[String] {
        &self.split_vertex_identities
    }

    pub fn upstream_artifact_identities(&self) -> &[String] {
        &self.upstream_artifact_identities
    }

    pub fn policy_or_denial_kind(&self) -> Option<&str> {
        self.policy_or_denial_kind.as_deref()
    }

    pub fn reason(&self) -> PlanarBooleanLoopDecisionReason {
        self.reason
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }
}
