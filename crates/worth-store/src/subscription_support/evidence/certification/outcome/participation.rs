use super::super::counter_snapshot::SubscriptionSupportCounterSnapshot;
use super::super::lane::SubscriptionSupportCertificationLaneKind;
use super::verdict::operational_verdict_classification;
use super::SubscriptionSupportCertificationLaneOutcome;
use crate::failure::StoreError;
use crate::subscription_support::digest::stable_digest;
use crate::subscription_support::{
    SubscriptionResumeClassification, SubscriptionSupportCompatibilityOutcome,
    SubscriptionSupportCompatibilityReport, SubscriptionSupportDriftCause,
    SubscriptionSupportMaintenanceDebtReport, SubscriptionSupportMaintenanceReport,
    SubscriptionSupportPortabilityReport, SubscriptionSupportPostActionReport,
};

impl SubscriptionSupportCertificationLaneOutcome {
    pub fn from_compatibility_report(
        lane: SubscriptionSupportCertificationLaneKind,
        report: &SubscriptionSupportCompatibilityReport,
        counter_snapshot: SubscriptionSupportCounterSnapshot,
    ) -> Result<Self, StoreError> {
        let (classification, primary_cause) = match report.outcome() {
            SubscriptionSupportCompatibilityOutcome::ExactMigrated(_) => {
                (Some(SubscriptionResumeClassification::Exact), None)
            }
            SubscriptionSupportCompatibilityOutcome::Degraded(_) => (
                Some(SubscriptionResumeClassification::Degraded),
                Some(SubscriptionSupportDriftCause::SubscriptionSupportCompatibilityDrift),
            ),
            SubscriptionSupportCompatibilityOutcome::Rejected(_) => (None, None),
        };
        Ok(Self {
            lane,
            classification,
            primary_cause,
            suppressed_causes: Vec::new(),
            truth_digest: stable_digest(&(
                lane,
                report.participation_record().decision_kind(),
                report.participation_record().milestone12_relation(),
                report.participation_record().milestone12_rejection_kind(),
            ))?,
            artifact_digest: stable_digest(&(
                report.completed_action().envelope().action_id(),
                report.participation_record().affected_set_digest(),
            ))?,
            subscription_support_digest: stable_digest(report)?,
            replay_digest: stable_digest(&(
                lane,
                report.participation_record().milestone12_receipt_digest(),
                &counter_snapshot,
            ))?,
            diagnostics_digest: stable_digest(&(
                report.outcome().outcome_kind(),
                report.participation_record().milestone12_rejection_kind(),
            ))?,
            counter_digest: stable_digest(&counter_snapshot)?,
            cost_surface: Some(report.cost_surface()),
            batch_receipt_reuse_report: None,
            counter_snapshot,
        })
    }

    pub fn from_retention_report(
        lane: SubscriptionSupportCertificationLaneKind,
        report: &SubscriptionSupportPostActionReport,
        counter_snapshot: SubscriptionSupportCounterSnapshot,
    ) -> Result<Self, StoreError> {
        Ok(Self {
            lane,
            classification: operational_verdict_classification(report.retention_record().verdict()),
            primary_cause: None,
            suppressed_causes: Vec::new(),
            truth_digest: stable_digest(&(
                lane,
                report.retention_record().decision_kind(),
                report.retention_record().verdict(),
            ))?,
            artifact_digest: stable_digest(&(
                report.completed_action().envelope().action_id(),
                report.retention_record().affected_set_digest(),
            ))?,
            subscription_support_digest: stable_digest(report)?,
            replay_digest: stable_digest(&(lane, &counter_snapshot))?,
            diagnostics_digest: stable_digest(report.materialization())?,
            counter_digest: stable_digest(&counter_snapshot)?,
            cost_surface: Some(report.cost_surface()),
            batch_receipt_reuse_report: None,
            counter_snapshot,
        })
    }

    pub fn from_portability_report(
        lane: SubscriptionSupportCertificationLaneKind,
        report: &SubscriptionSupportPortabilityReport,
        counter_snapshot: SubscriptionSupportCounterSnapshot,
    ) -> Result<Self, StoreError> {
        Ok(Self {
            lane,
            classification: operational_verdict_classification(
                report.participation_record().verdict(),
            ),
            primary_cause: None,
            suppressed_causes: Vec::new(),
            truth_digest: stable_digest(&(
                lane,
                report.participation_record().decision_kind(),
                report.participation_record().verdict(),
            ))?,
            artifact_digest: stable_digest(&(
                report.completed_action().envelope().action_id(),
                report.participation_record().affected_set_digest(),
            ))?,
            subscription_support_digest: stable_digest(report)?,
            replay_digest: stable_digest(&(
                lane,
                report.manifest().manifest_digest(),
                &counter_snapshot,
            ))?,
            diagnostics_digest: stable_digest(report.outcome())?,
            counter_digest: stable_digest(&counter_snapshot)?,
            cost_surface: Some(report.cost_surface()),
            batch_receipt_reuse_report: None,
            counter_snapshot,
        })
    }

    pub fn from_maintenance_report(
        lane: SubscriptionSupportCertificationLaneKind,
        report: &SubscriptionSupportMaintenanceReport,
        counter_snapshot: SubscriptionSupportCounterSnapshot,
    ) -> Result<Self, StoreError> {
        Ok(Self {
            lane,
            classification: operational_verdict_classification(
                report.participation_record().verdict(),
            ),
            primary_cause: None,
            suppressed_causes: Vec::new(),
            truth_digest: stable_digest(&(
                lane,
                report.participation_record().decision_kind(),
                report.participation_record().verdict(),
            ))?,
            artifact_digest: stable_digest(&(
                report.completed_action().envelope().action_id(),
                report.participation_record().affected_set_digest(),
            ))?,
            subscription_support_digest: stable_digest(report)?,
            replay_digest: stable_digest(&(lane, &counter_snapshot))?,
            diagnostics_digest: stable_digest(&(
                report.participation_record().descriptor_count(),
                report.participation_record().coalesced_duplicate_count(),
            ))?,
            counter_digest: stable_digest(&counter_snapshot)?,
            cost_surface: Some(report.cost_surface()),
            batch_receipt_reuse_report: None,
            counter_snapshot,
        })
    }

    pub fn from_maintenance_debt_report(
        lane: SubscriptionSupportCertificationLaneKind,
        report: &SubscriptionSupportMaintenanceDebtReport,
        counter_snapshot: SubscriptionSupportCounterSnapshot,
    ) -> Result<Self, StoreError> {
        Ok(Self {
            lane,
            classification: operational_verdict_classification(report.debt_summary().verdict()),
            primary_cause: None,
            suppressed_causes: Vec::new(),
            truth_digest: stable_digest(&(
                lane,
                report.debt_summary().work_kind(),
                report.debt_summary().verdict(),
                report.debt_summary().delay_reason(),
            ))?,
            artifact_digest: stable_digest(&(
                report.debt_summary().action_id(),
                report.debt_summary().affected_set_digest(),
            ))?,
            subscription_support_digest: stable_digest(report)?,
            replay_digest: stable_digest(&(lane, &counter_snapshot))?,
            diagnostics_digest: stable_digest(report.debt_summary())?,
            counter_digest: stable_digest(&counter_snapshot)?,
            cost_surface: Some(report.cost_surface()),
            batch_receipt_reuse_report: None,
            counter_snapshot,
        })
    }
}
