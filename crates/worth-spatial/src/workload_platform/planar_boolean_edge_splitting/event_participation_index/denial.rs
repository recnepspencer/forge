#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanSplitEventParticipationDenialKind {
    MissingCarrierRows,
    CarrierSetIdentityMismatch,
    UnknownCarrierReference,
    UnknownGroupedPointEvent,
    UnknownGroupedIntervalEvent,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanSplitEventParticipationDenial {
    kind: PlanarBooleanSplitEventParticipationDenialKind,
    evidence_identity: String,
    human_reason: String,
    rejected_orphan_references: usize,
}

impl PlanarBooleanSplitEventParticipationDenial {
    pub(crate) fn new(
        kind: PlanarBooleanSplitEventParticipationDenialKind,
        evidence_identity: impl Into<String>,
        human_reason: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            evidence_identity: evidence_identity.into(),
            human_reason: human_reason.into(),
            rejected_orphan_references: 0,
        }
    }

    pub(crate) fn with_rejected_orphan_reference(
        kind: PlanarBooleanSplitEventParticipationDenialKind,
        evidence_identity: impl Into<String>,
        human_reason: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            evidence_identity: evidence_identity.into(),
            human_reason: human_reason.into(),
            rejected_orphan_references: 1,
        }
    }

    pub fn kind(&self) -> PlanarBooleanSplitEventParticipationDenialKind {
        self.kind
    }

    pub fn evidence_identity(&self) -> &str {
        &self.evidence_identity
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }

    pub fn rejected_orphan_references(&self) -> usize {
        self.rejected_orphan_references
    }
}
