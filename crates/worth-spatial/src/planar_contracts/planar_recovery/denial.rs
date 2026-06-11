#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarRecoveryPostureDenialKind {
    MissingRecoverySource,
    SummarySourceNotAuthority,
    MissingRetainedPlanarFacts,
    MissingProjectionConsumedPlanarFacts,
    MismatchedRetainedProjectionBasis,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarRecoveryPostureDenial {
    kind: PlanarRecoveryPostureDenialKind,
    reason: String,
}

impl PlanarRecoveryPostureDenial {
    pub(crate) fn new(kind: PlanarRecoveryPostureDenialKind, reason: impl Into<String>) -> Self {
        Self {
            kind,
            reason: reason.into(),
        }
    }

    pub fn kind(&self) -> PlanarRecoveryPostureDenialKind {
        self.kind
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }
}
