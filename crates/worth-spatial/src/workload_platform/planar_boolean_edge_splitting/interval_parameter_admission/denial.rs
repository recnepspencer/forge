#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanSplitIntervalAdmissionDenialKind {
    NonFiniteRange,
    OutOfDomainRange,
    CollapsedInterval,
    ContradictoryIntervalSense,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanSplitIntervalAdmissionDenial {
    kind: PlanarBooleanSplitIntervalAdmissionDenialKind,
    evidence_identity: String,
    human_reason: String,
    rejected_non_finite_intervals: usize,
    rejected_out_of_domain_intervals: usize,
    rejected_collapsed_intervals: usize,
    rejected_contradictory_sense_intervals: usize,
}

impl PlanarBooleanSplitIntervalAdmissionDenial {
    pub(crate) fn non_finite_range(
        evidence_identity: impl Into<String>,
        human_reason: impl Into<String>,
    ) -> Self {
        Self {
            kind: PlanarBooleanSplitIntervalAdmissionDenialKind::NonFiniteRange,
            evidence_identity: evidence_identity.into(),
            human_reason: human_reason.into(),
            rejected_non_finite_intervals: 1,
            rejected_out_of_domain_intervals: 0,
            rejected_collapsed_intervals: 0,
            rejected_contradictory_sense_intervals: 0,
        }
    }

    pub(crate) fn out_of_domain_range(
        evidence_identity: impl Into<String>,
        human_reason: impl Into<String>,
    ) -> Self {
        Self {
            kind: PlanarBooleanSplitIntervalAdmissionDenialKind::OutOfDomainRange,
            evidence_identity: evidence_identity.into(),
            human_reason: human_reason.into(),
            rejected_non_finite_intervals: 0,
            rejected_out_of_domain_intervals: 1,
            rejected_collapsed_intervals: 0,
            rejected_contradictory_sense_intervals: 0,
        }
    }

    pub(crate) fn collapsed_interval(
        evidence_identity: impl Into<String>,
        human_reason: impl Into<String>,
    ) -> Self {
        Self {
            kind: PlanarBooleanSplitIntervalAdmissionDenialKind::CollapsedInterval,
            evidence_identity: evidence_identity.into(),
            human_reason: human_reason.into(),
            rejected_non_finite_intervals: 0,
            rejected_out_of_domain_intervals: 0,
            rejected_collapsed_intervals: 1,
            rejected_contradictory_sense_intervals: 0,
        }
    }

    pub(crate) fn contradictory_interval_sense(
        evidence_identity: impl Into<String>,
        human_reason: impl Into<String>,
    ) -> Self {
        Self {
            kind: PlanarBooleanSplitIntervalAdmissionDenialKind::ContradictoryIntervalSense,
            evidence_identity: evidence_identity.into(),
            human_reason: human_reason.into(),
            rejected_non_finite_intervals: 0,
            rejected_out_of_domain_intervals: 0,
            rejected_collapsed_intervals: 0,
            rejected_contradictory_sense_intervals: 1,
        }
    }

    pub fn kind(&self) -> PlanarBooleanSplitIntervalAdmissionDenialKind {
        self.kind
    }

    pub fn evidence_identity(&self) -> &str {
        &self.evidence_identity
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }

    pub fn rejected_non_finite_intervals(&self) -> usize {
        self.rejected_non_finite_intervals
    }

    pub fn rejected_out_of_domain_intervals(&self) -> usize {
        self.rejected_out_of_domain_intervals
    }

    pub fn rejected_collapsed_intervals(&self) -> usize {
        self.rejected_collapsed_intervals
    }

    pub fn rejected_contradictory_sense_intervals(&self) -> usize {
        self.rejected_contradictory_sense_intervals
    }
}
