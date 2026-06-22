use super::counters::PlanarBooleanLoopDecisionLogCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanLoopDecisionLogDenialKind {
    RequestIdentityMismatch,
    DuplicateDecisionIdentity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopDecisionLogDenial {
    kind: PlanarBooleanLoopDecisionLogDenialKind,
    detail_identity: String,
    counters: PlanarBooleanLoopDecisionLogCounters,
    human_reason: String,
}

impl PlanarBooleanLoopDecisionLogDenial {
    pub(crate) fn new(
        kind: PlanarBooleanLoopDecisionLogDenialKind,
        detail_identity: impl Into<String>,
        counters: PlanarBooleanLoopDecisionLogCounters,
        human_reason: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            detail_identity: detail_identity.into(),
            counters,
            human_reason: human_reason.into(),
        }
    }

    pub fn kind(&self) -> PlanarBooleanLoopDecisionLogDenialKind {
        self.kind
    }

    pub fn detail_identity(&self) -> &str {
        &self.detail_identity
    }

    pub fn counters(&self) -> PlanarBooleanLoopDecisionLogCounters {
        self.counters
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }
}
