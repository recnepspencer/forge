#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanOrderedEdgeSplitScheduleDenialKind {
    NonFiniteScheduleParameter,
    MissingTieBreakerIdentity,
    OrderedScheduleInvariantMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOrderedEdgeSplitScheduleDenial {
    kind: PlanarBooleanOrderedEdgeSplitScheduleDenialKind,
    evidence_identity: String,
    human_reason: String,
}

impl PlanarBooleanOrderedEdgeSplitScheduleDenial {
    pub(crate) fn new(
        kind: PlanarBooleanOrderedEdgeSplitScheduleDenialKind,
        evidence_identity: impl Into<String>,
        human_reason: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            evidence_identity: evidence_identity.into(),
            human_reason: human_reason.into(),
        }
    }

    pub fn kind(&self) -> PlanarBooleanOrderedEdgeSplitScheduleDenialKind {
        self.kind
    }

    pub fn evidence_identity(&self) -> &str {
        &self.evidence_identity
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }
}
