use super::super::{
    SubscriptionSupportCertificationLaneOutcome, SubscriptionSupportClassificationReport,
};

#[derive(Default)]
pub(super) struct CertificationMatrixEvidence {
    classification_reports: Vec<SubscriptionSupportClassificationReport>,
    lane_outcomes: Vec<SubscriptionSupportCertificationLaneOutcome>,
}

impl CertificationMatrixEvidence {
    pub(super) fn new() -> Self {
        Self::default()
    }

    pub(super) fn record_classification_report(
        &mut self,
        report: SubscriptionSupportClassificationReport,
    ) {
        self.classification_reports.push(report);
    }

    pub(super) fn record_lane_outcome(
        &mut self,
        outcome: SubscriptionSupportCertificationLaneOutcome,
    ) {
        self.lane_outcomes.push(outcome);
    }

    pub(super) fn classification_reports(&self) -> &[SubscriptionSupportClassificationReport] {
        &self.classification_reports
    }

    pub(super) fn lane_outcomes(&self) -> &[SubscriptionSupportCertificationLaneOutcome] {
        &self.lane_outcomes
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        Vec<SubscriptionSupportClassificationReport>,
        Vec<SubscriptionSupportCertificationLaneOutcome>,
    ) {
        (self.classification_reports, self.lane_outcomes)
    }
}
