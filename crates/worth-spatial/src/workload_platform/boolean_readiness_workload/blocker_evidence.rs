#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanReadinessBlocker {
    PolicyRequired,
    CleanFailure,
    UnsupportedWorkloadFamily,
    PredicateUncertainty,
    OrientationFlipLocalization,
    KernelSummarySubstitution,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PlanarBooleanReadinessBlockerEvidence {
    blocker: PlanarBooleanReadinessBlocker,
    reason: String,
    evidence_digest: String,
}

impl PlanarBooleanReadinessBlockerEvidence {
    pub(crate) fn new(
        blocker: PlanarBooleanReadinessBlocker,
        reason: impl Into<String>,
        evidence_digest: impl Into<String>,
    ) -> Self {
        Self {
            blocker,
            reason: reason.into(),
            evidence_digest: evidence_digest.into(),
        }
    }

    pub(crate) fn blocker(&self) -> PlanarBooleanReadinessBlocker {
        self.blocker
    }

    pub(crate) fn reason(&self) -> &str {
        &self.reason
    }

    pub(crate) fn evidence_digest(&self) -> &str {
        &self.evidence_digest
    }
}
