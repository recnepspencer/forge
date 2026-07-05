use super::counters::PlanarBooleanOverlapCellClassificationCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanOverlapCellClassificationDenialKind {
    MissingOperandContainmentEvidenceDenied,
    ContradictoryOperandContainmentEvidenceDenied,
    WindingFieldInputMismatchDenied,
    NoOperandLocalWindingEvidenceDenied,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapCellClassificationDenial {
    kind: PlanarBooleanOverlapCellClassificationDenialKind,
    rejected_identity: String,
    counters: PlanarBooleanOverlapCellClassificationCounters,
    human_reason: &'static str,
}

impl PlanarBooleanOverlapCellClassificationDenial {
    pub(crate) fn new(
        kind: PlanarBooleanOverlapCellClassificationDenialKind,
        rejected_identity: impl Into<String>,
        counters: PlanarBooleanOverlapCellClassificationCounters,
        human_reason: &'static str,
    ) -> Self {
        Self {
            kind,
            rejected_identity: rejected_identity.into(),
            counters,
            human_reason,
        }
    }

    pub fn kind(&self) -> PlanarBooleanOverlapCellClassificationDenialKind {
        self.kind
    }

    pub fn rejected_identity(&self) -> &str {
        &self.rejected_identity
    }

    pub fn counters(&self) -> PlanarBooleanOverlapCellClassificationCounters {
        self.counters
    }

    pub fn human_reason(&self) -> &str {
        self.human_reason
    }
}
