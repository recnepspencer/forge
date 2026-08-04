use super::super::counter_snapshot::SubscriptionSupportCounterSnapshot;
use super::super::lane::SubscriptionSupportCertificationLaneKind;
use super::verdict::operational_verdict_classification;
use super::SubscriptionSupportCertificationLaneOutcome;
use crate::failure::StoreError;
use crate::subscription_support::digest::stable_digest;
use crate::{SubscriptionSupportActionPublicationRecoveryReport, SupportBatchReceiptReuseReport};

impl SubscriptionSupportCertificationLaneOutcome {
    pub fn from_batch_receipt_reuse_report(
        lane: SubscriptionSupportCertificationLaneKind,
        report: &SupportBatchReceiptReuseReport,
        counter_snapshot: SubscriptionSupportCounterSnapshot,
    ) -> Result<Self, StoreError> {
        Ok(Self {
            lane,
            classification: None,
            primary_cause: None,
            suppressed_causes: Vec::new(),
            truth_digest: stable_digest(&(
                lane,
                report.density_class(),
                report.affected_entries(),
            ))?,
            artifact_digest: stable_digest(&(lane, report.reused_proofs()))?,
            subscription_support_digest: stable_digest(&(lane, report, &counter_snapshot))?,
            replay_digest: stable_digest(&(lane, &counter_snapshot))?,
            diagnostics_digest: stable_digest(report)?,
            counter_digest: stable_digest(&counter_snapshot)?,
            cost_surface: None,
            batch_receipt_reuse_report: Some(report.clone()),
            counter_snapshot,
        })
    }

    pub fn from_action_publication_recovery(
        lane: SubscriptionSupportCertificationLaneKind,
        report: &SubscriptionSupportActionPublicationRecoveryReport,
        counter_snapshot: SubscriptionSupportCounterSnapshot,
    ) -> Result<Self, StoreError> {
        Ok(Self {
            lane,
            classification: operational_verdict_classification(report.verdict()),
            primary_cause: None,
            suppressed_causes: Vec::new(),
            truth_digest: stable_digest(&(
                lane,
                report.action_id(),
                report.recovery_disposition(),
                report.verdict(),
            ))?,
            artifact_digest: stable_digest(&(report.action_id(), report.artifact_id()))?,
            subscription_support_digest: stable_digest(report)?,
            replay_digest: stable_digest(&(lane, &counter_snapshot))?,
            diagnostics_digest: stable_digest(&(
                report.action_origin(),
                report.completed_action().map(|action| action.envelope()),
            ))?,
            counter_digest: stable_digest(&counter_snapshot)?,
            cost_surface: None,
            batch_receipt_reuse_report: None,
            counter_snapshot,
        })
    }
}
