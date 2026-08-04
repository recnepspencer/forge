use super::super::counter_snapshot::SubscriptionSupportCounterSnapshot;
use super::super::lane::SubscriptionSupportCertificationLaneKind;
use super::SubscriptionSupportCertificationLaneOutcome;
use crate::failure::StoreError;
use crate::subscription_support::digest::stable_digest;
use crate::subscription_support::{
    SubscriptionSupportClassificationReport, SubscriptionSupportMissingSupportRecoveryReport,
};

impl SubscriptionSupportCertificationLaneOutcome {
    pub fn from_classification_report(
        lane: SubscriptionSupportCertificationLaneKind,
        report: &SubscriptionSupportClassificationReport,
    ) -> Result<Self, StoreError> {
        Ok(Self {
            lane,
            classification: Some(report.classification()),
            primary_cause: report.primary_cause(),
            suppressed_causes: report.suppressed_causes().to_vec(),
            truth_digest: stable_digest(&(
                lane,
                report.classification(),
                report.primary_cause(),
                report.suppressed_causes(),
            ))?,
            artifact_digest: stable_digest(&(report.artifact_id(), report.declaration_digest()))?,
            subscription_support_digest: stable_digest(&(
                report.artifact_id(),
                report.classification(),
                report.cost_surface(),
            ))?,
            replay_digest: stable_digest(&(
                lane,
                report.cost_surface(),
                report.counter_snapshot(),
            ))?,
            diagnostics_digest: stable_digest(&(
                report.primary_cause(),
                report.suppressed_causes(),
            ))?,
            counter_digest: stable_digest(report.counter_snapshot())?,
            cost_surface: Some(report.cost_surface()),
            batch_receipt_reuse_report: None,
            counter_snapshot: report.counter_snapshot().clone(),
        })
    }

    pub fn from_missing_support_recovery(
        lane: SubscriptionSupportCertificationLaneKind,
        report: &SubscriptionSupportMissingSupportRecoveryReport,
        counter_snapshot: SubscriptionSupportCounterSnapshot,
    ) -> Result<Self, StoreError> {
        Ok(Self {
            lane,
            classification: Some(report.classification()),
            primary_cause: Some(report.primary_cause()),
            suppressed_causes: Vec::new(),
            truth_digest: stable_digest(&(lane, report.classification(), report.primary_cause()))?,
            artifact_digest: stable_digest(report)?,
            subscription_support_digest: stable_digest(&(report, report.maintenance_report()))?,
            replay_digest: stable_digest(&(lane, &counter_snapshot))?,
            diagnostics_digest: stable_digest(&(
                report.primary_cause(),
                report.maintenance_report(),
            ))?,
            counter_digest: stable_digest(&counter_snapshot)?,
            cost_surface: None,
            batch_receipt_reuse_report: None,
            counter_snapshot,
        })
    }
}
