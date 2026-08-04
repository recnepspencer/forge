use super::super::counter_snapshot::SubscriptionSupportCounterSnapshot;
use super::super::lane::SubscriptionSupportCertificationLaneKind;
use super::SubscriptionSupportCertificationLaneOutcome;
use crate::failure::StoreError;
use crate::failure::StoreErrorKind;
use crate::subscription_support::digest::stable_digest;
use crate::SubscriptionSupportAccessStructureReport;

impl SubscriptionSupportCertificationLaneOutcome {
    pub fn from_access_structure_debt(
        lane: SubscriptionSupportCertificationLaneKind,
        report: &SubscriptionSupportAccessStructureReport,
        counter_snapshot: SubscriptionSupportCounterSnapshot,
    ) -> Result<Self, StoreError> {
        Ok(Self {
            lane,
            classification: None,
            primary_cause: None,
            suppressed_causes: Vec::new(),
            truth_digest: stable_digest(&(lane, report.has_debt()))?,
            artifact_digest: stable_digest(&report.required().to_vec())?,
            subscription_support_digest: stable_digest(&report.debted().to_vec())?,
            replay_digest: stable_digest(&(lane, &counter_snapshot))?,
            diagnostics_digest: stable_digest(report)?,
            counter_digest: stable_digest(&counter_snapshot)?,
            cost_surface: None,
            batch_receipt_reuse_report: None,
            counter_snapshot,
        })
    }

    pub fn from_typed_rejection(
        lane: SubscriptionSupportCertificationLaneKind,
        error_kind: StoreErrorKind,
        counter_snapshot: SubscriptionSupportCounterSnapshot,
    ) -> Result<Self, StoreError> {
        Ok(Self {
            lane,
            classification: None,
            primary_cause: None,
            suppressed_causes: Vec::new(),
            truth_digest: stable_digest(&(lane, &error_kind))?,
            artifact_digest: stable_digest(&(lane, "typed-rejection"))?,
            subscription_support_digest: stable_digest(&(lane, &error_kind, &counter_snapshot))?,
            replay_digest: stable_digest(&(lane, &counter_snapshot))?,
            diagnostics_digest: stable_digest(&(lane, &error_kind))?,
            counter_digest: stable_digest(&counter_snapshot)?,
            cost_surface: None,
            batch_receipt_reuse_report: None,
            counter_snapshot,
        })
    }
}
