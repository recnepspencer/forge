use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{
    BridgeSubscriptionCertificationCounterSnapshot, BridgeSubscriptionOfflineAuditOutcomeSummary,
    BridgeSubscriptionOfflineAuditPlan,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSubscriptionOfflineAuditOutcome {
    DiagnosedOffline,
}

impl BridgeSubscriptionOfflineAuditOutcome {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DiagnosedOffline => "diagnosed_offline",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionOfflineAuditReport {
    outcome: BridgeSubscriptionOfflineAuditOutcome,
    bundle_index_digest: Arc<str>,
    comparison_report_count: usize,
    outcome_summary: BridgeSubscriptionOfflineAuditOutcomeSummary,
    counters: BridgeSubscriptionCertificationCounterSnapshot,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionOfflineAuditReport {
    pub(crate) fn audit(plan: BridgeSubscriptionOfflineAuditPlan) -> Self {
        let counters = BridgeSubscriptionCertificationCounterSnapshot::combine([
            *plan.counters(),
            BridgeSubscriptionCertificationCounterSnapshot::from_offline_audit_report(),
        ]);
        let outcome = BridgeSubscriptionOfflineAuditOutcome::DiagnosedOffline;
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-offline-audit-report|plan={}|outcome={}|bundle-index={}|comparison-reports={}|outcomes={}|counters={}",
            plan.digest(),
            outcome.as_str(),
            plan.bundle_index_digest(),
            plan.comparison_report_count(),
            plan.outcome_summary().digest(),
            counters.digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            outcome,
            bundle_index_digest: Arc::from(plan.bundle_index_digest()),
            comparison_report_count: plan.comparison_report_count(),
            outcome_summary: plan.outcome_summary().clone(),
            counters,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-offline-audit-report:sha256:{digest:x}"
            )),
        }
    }

    pub fn outcome(&self) -> BridgeSubscriptionOfflineAuditOutcome {
        self.outcome
    }

    pub fn bundle_index_digest(&self) -> &str {
        self.bundle_index_digest.as_ref()
    }

    pub fn comparison_report_count(&self) -> usize {
        self.comparison_report_count
    }

    pub fn outcome_summary(&self) -> &BridgeSubscriptionOfflineAuditOutcomeSummary {
        &self.outcome_summary
    }

    pub fn counters(&self) -> &BridgeSubscriptionCertificationCounterSnapshot {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
