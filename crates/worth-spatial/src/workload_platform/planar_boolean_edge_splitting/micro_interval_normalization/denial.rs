#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanIntervalSubdivisionNormalizationDenialKind {
    NonFiniteIntervalBoundary,
    CollapsedIntervalSubdivision,
    MicroIntervalBelowAdmittedPolicy,
    ContradictoryIntervalSubdivisionBasis,
    ForeignEndpointBoundarySchedule,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanIntervalSubdivisionNormalizationDenial {
    kind: PlanarBooleanIntervalSubdivisionNormalizationDenialKind,
    evidence_identity: String,
    human_reason: String,
}

impl PlanarBooleanIntervalSubdivisionNormalizationDenial {
    pub(crate) fn new(
        kind: PlanarBooleanIntervalSubdivisionNormalizationDenialKind,
        evidence_identity: impl Into<String>,
        human_reason: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            evidence_identity: evidence_identity.into(),
            human_reason: human_reason.into(),
        }
    }

    pub fn kind(&self) -> PlanarBooleanIntervalSubdivisionNormalizationDenialKind {
        self.kind
    }
    pub fn evidence_identity(&self) -> &str {
        &self.evidence_identity
    }
    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }
}
