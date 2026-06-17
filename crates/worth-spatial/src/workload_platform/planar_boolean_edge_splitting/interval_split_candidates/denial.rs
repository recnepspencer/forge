#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanIntervalSplitCandidateDenialKind {
    MissingParticipationRow,
    MissingSourceInterval,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanIntervalSplitCandidateDenial {
    kind: PlanarBooleanIntervalSplitCandidateDenialKind,
    evidence_identity: String,
    human_reason: String,
    rejected_missing_participation_rows: usize,
    rejected_missing_source_ranges: usize,
}

impl PlanarBooleanIntervalSplitCandidateDenial {
    pub(crate) fn missing_index_owned_interval_event(
        evidence_identity: impl Into<String>,
        human_reason: impl Into<String>,
    ) -> Self {
        Self {
            kind: PlanarBooleanIntervalSplitCandidateDenialKind::MissingParticipationRow,
            evidence_identity: evidence_identity.into(),
            human_reason: human_reason.into(),
            rejected_missing_participation_rows: 1,
            rejected_missing_source_ranges: 0,
        }
    }

    pub(crate) fn missing_source_interval_for_row_carrier(
        evidence_identity: impl Into<String>,
        human_reason: impl Into<String>,
    ) -> Self {
        Self {
            kind: PlanarBooleanIntervalSplitCandidateDenialKind::MissingSourceInterval,
            evidence_identity: evidence_identity.into(),
            human_reason: human_reason.into(),
            rejected_missing_participation_rows: 0,
            rejected_missing_source_ranges: 1,
        }
    }

    pub fn kind(&self) -> PlanarBooleanIntervalSplitCandidateDenialKind {
        self.kind
    }

    pub fn evidence_identity(&self) -> &str {
        &self.evidence_identity
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }

    pub fn rejected_missing_participation_rows(&self) -> usize {
        self.rejected_missing_participation_rows
    }

    pub fn rejected_missing_source_ranges(&self) -> usize {
        self.rejected_missing_source_ranges
    }
}
