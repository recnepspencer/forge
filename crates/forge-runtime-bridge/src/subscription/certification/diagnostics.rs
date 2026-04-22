use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{
    BridgeSubscriptionCertificationCounterSnapshot, BridgeSubscriptionOfflineAuditOutcome,
    BridgeSubscriptionOfflineAuditReport, BridgeSubscriptionReferenceWorkloadReport,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionCertificationInspection {
    audit_report_digest: Arc<str>,
    outcome: BridgeSubscriptionOfflineAuditOutcome,
    outcome_summary_digest: Arc<str>,
    counter_digest: Arc<str>,
    host_log_dependency_count: usize,
    live_state_dependency_count: usize,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionCertificationInspection {
    pub(crate) fn from_offline_audit(report: &BridgeSubscriptionOfflineAuditReport) -> Self {
        let counters = report.counters();
        let counter_digest = counters.digest();
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-certification-inspection|audit-report={}|outcome={}|outcome-summary={}|counters={counter_digest}|host-log-dependencies={}|live-state-dependencies={}",
            report.digest(),
            report.outcome().as_str(),
            report.outcome_summary().digest(),
            counters.host_log_dependency_count(),
            counters.live_state_dependency_count(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            audit_report_digest: Arc::from(report.digest()),
            outcome: report.outcome(),
            outcome_summary_digest: Arc::from(report.outcome_summary().digest()),
            counter_digest,
            host_log_dependency_count: counters.host_log_dependency_count(),
            live_state_dependency_count: counters.live_state_dependency_count(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-certification-inspection:sha256:{digest:x}"
            )),
        }
    }

    pub fn audit_report_digest(&self) -> &str {
        self.audit_report_digest.as_ref()
    }

    pub fn outcome(&self) -> BridgeSubscriptionOfflineAuditOutcome {
        self.outcome
    }

    pub fn counter_digest(&self) -> &str {
        self.counter_digest.as_ref()
    }

    pub fn outcome_summary_digest(&self) -> &str {
        self.outcome_summary_digest.as_ref()
    }

    pub fn host_log_dependency_count(&self) -> usize {
        self.host_log_dependency_count
    }

    pub fn live_state_dependency_count(&self) -> usize {
        self.live_state_dependency_count
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionReferenceWorkloadInspection {
    reference_workload_report_digest: Arc<str>,
    manifest_digest: Arc<str>,
    offline_audit_report_digest: Arc<str>,
    outcome_summary_digest: Arc<str>,
    coverage_report_digest: Arc<str>,
    counter_digest: Arc<str>,
    lane_report_digests: Vec<Arc<str>>,
    comparison_report_digests: Vec<Arc<str>>,
    lane_report_count: usize,
    comparison_report_count: usize,
    host_log_dependency_count: usize,
    live_state_dependency_count: usize,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionReferenceWorkloadInspection {
    pub(crate) fn from_reference_workload(
        report: &BridgeSubscriptionReferenceWorkloadReport,
    ) -> Self {
        let lane_report_digests = report
            .lane_reports()
            .iter()
            .map(|lane| Arc::<str>::from(lane.digest()))
            .collect::<Vec<_>>();
        let comparison_report_digests = report
            .comparison_reports()
            .iter()
            .map(|comparison| Arc::<str>::from(comparison.digest()))
            .collect::<Vec<_>>();
        let lane_digest_basis = lane_report_digests
            .iter()
            .map(Arc::as_ref)
            .collect::<Vec<_>>()
            .join(",");
        let comparison_digest_basis = comparison_report_digests
            .iter()
            .map(Arc::as_ref)
            .collect::<Vec<_>>()
            .join(",");
        let counters: BridgeSubscriptionCertificationCounterSnapshot = *report.counters();
        let counter_digest = counters.digest();
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-reference-workload-inspection|report={}|manifest={}|audit={}|outcome-summary={}|coverage={}|counters={counter_digest}|lane-count={}|comparison-count={}|lanes={lane_digest_basis}|comparisons={comparison_digest_basis}|host-log-dependencies={}|live-state-dependencies={}",
            report.digest(),
            report.manifest_digest(),
            report.offline_audit_report().digest(),
            report.outcome_summary().digest(),
            report.coverage_report().digest(),
            lane_report_digests.len(),
            comparison_report_digests.len(),
            counters.host_log_dependency_count(),
            counters.live_state_dependency_count(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            reference_workload_report_digest: Arc::from(report.digest()),
            manifest_digest: Arc::from(report.manifest_digest()),
            offline_audit_report_digest: Arc::from(report.offline_audit_report().digest()),
            outcome_summary_digest: Arc::from(report.outcome_summary().digest()),
            coverage_report_digest: Arc::from(report.coverage_report().digest()),
            counter_digest,
            lane_report_count: lane_report_digests.len(),
            comparison_report_count: comparison_report_digests.len(),
            host_log_dependency_count: counters.host_log_dependency_count(),
            live_state_dependency_count: counters.live_state_dependency_count(),
            lane_report_digests,
            comparison_report_digests,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-reference-workload-inspection:sha256:{digest:x}"
            )),
        }
    }

    pub fn reference_workload_report_digest(&self) -> &str {
        self.reference_workload_report_digest.as_ref()
    }

    pub fn manifest_digest(&self) -> &str {
        self.manifest_digest.as_ref()
    }

    pub fn offline_audit_report_digest(&self) -> &str {
        self.offline_audit_report_digest.as_ref()
    }

    pub fn outcome_summary_digest(&self) -> &str {
        self.outcome_summary_digest.as_ref()
    }

    pub fn coverage_report_digest(&self) -> &str {
        self.coverage_report_digest.as_ref()
    }

    pub fn counter_digest(&self) -> &str {
        self.counter_digest.as_ref()
    }

    pub fn lane_report_digests(&self) -> &[Arc<str>] {
        &self.lane_report_digests
    }

    pub fn comparison_report_digests(&self) -> &[Arc<str>] {
        &self.comparison_report_digests
    }

    pub fn lane_report_count(&self) -> usize {
        self.lane_report_count
    }

    pub fn comparison_report_count(&self) -> usize {
        self.comparison_report_count
    }

    pub fn host_log_dependency_count(&self) -> usize {
        self.host_log_dependency_count
    }

    pub fn live_state_dependency_count(&self) -> usize {
        self.live_state_dependency_count
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
