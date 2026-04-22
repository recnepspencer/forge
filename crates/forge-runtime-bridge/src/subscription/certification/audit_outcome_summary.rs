use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{
    BridgeSubscriptionCertificationComparisonOutcome,
    BridgeSubscriptionCertificationComparisonReport,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionOfflineAuditOutcomeSummary {
    equivalent_count: usize,
    intentionally_divergent_count: usize,
    expected_rejection_count: usize,
    unexpected_rejection_count: usize,
    diagnostics_only_count: usize,
    residue_mismatch_count: usize,
    replay_mismatch_count: usize,
    counter_contract_violation_count: usize,
    bundle_completeness_violation_count: usize,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionOfflineAuditOutcomeSummary {
    pub(crate) fn from_comparison_reports(
        reports: &[&BridgeSubscriptionCertificationComparisonReport],
    ) -> Self {
        let mut equivalent_count = 0;
        let mut intentionally_divergent_count = 0;
        let mut expected_rejection_count = 0;
        let mut unexpected_rejection_count = 0;
        let mut diagnostics_only_count = 0;
        let mut residue_mismatch_count = 0;
        let mut replay_mismatch_count = 0;
        let mut counter_contract_violation_count = 0;
        let mut bundle_completeness_violation_count = 0;
        for report in reports {
            match report.outcome() {
                BridgeSubscriptionCertificationComparisonOutcome::Equivalent => {
                    equivalent_count += 1;
                }
                BridgeSubscriptionCertificationComparisonOutcome::IntentionallyDivergent => {
                    intentionally_divergent_count += 1;
                }
                BridgeSubscriptionCertificationComparisonOutcome::RejectedAtExpectedBoundary => {
                    expected_rejection_count += 1;
                }
                BridgeSubscriptionCertificationComparisonOutcome::RejectedAtUnexpectedBoundary => {
                    unexpected_rejection_count += 1;
                }
                BridgeSubscriptionCertificationComparisonOutcome::DiagnosticsOnlyDifference => {
                    diagnostics_only_count += 1;
                }
                BridgeSubscriptionCertificationComparisonOutcome::ResidueMismatch => {
                    residue_mismatch_count += 1;
                }
                BridgeSubscriptionCertificationComparisonOutcome::ReplayMismatch => {
                    replay_mismatch_count += 1;
                }
                BridgeSubscriptionCertificationComparisonOutcome::CounterContractViolation => {
                    counter_contract_violation_count += 1;
                }
                BridgeSubscriptionCertificationComparisonOutcome::BundleCompletenessViolation => {
                    bundle_completeness_violation_count += 1;
                }
            }
        }
        let canonical_basis = Arc::<str>::from(format!(
            concat!(
                "bridge-subscription-offline-audit-outcome-summary|equivalent={}|",
                "intentional-divergence={}|expected-rejection={}|unexpected-rejection={}|",
                "diagnostics-only={}|residue-mismatch={}|replay-mismatch={}|",
                "counter-contract-violation={}|bundle-completeness-violation={}"
            ),
            equivalent_count,
            intentionally_divergent_count,
            expected_rejection_count,
            unexpected_rejection_count,
            diagnostics_only_count,
            residue_mismatch_count,
            replay_mismatch_count,
            counter_contract_violation_count,
            bundle_completeness_violation_count,
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            equivalent_count,
            intentionally_divergent_count,
            expected_rejection_count,
            unexpected_rejection_count,
            diagnostics_only_count,
            residue_mismatch_count,
            replay_mismatch_count,
            counter_contract_violation_count,
            bundle_completeness_violation_count,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-offline-audit-outcome-summary:sha256:{digest:x}"
            )),
        }
    }

    pub fn equivalent_count(&self) -> usize {
        self.equivalent_count
    }

    pub fn intentionally_divergent_count(&self) -> usize {
        self.intentionally_divergent_count
    }

    pub fn expected_rejection_count(&self) -> usize {
        self.expected_rejection_count
    }

    pub fn unexpected_rejection_count(&self) -> usize {
        self.unexpected_rejection_count
    }

    pub fn diagnostics_only_count(&self) -> usize {
        self.diagnostics_only_count
    }

    pub fn residue_mismatch_count(&self) -> usize {
        self.residue_mismatch_count
    }

    pub fn replay_mismatch_count(&self) -> usize {
        self.replay_mismatch_count
    }

    pub fn counter_contract_violation_count(&self) -> usize {
        self.counter_contract_violation_count
    }

    pub fn bundle_completeness_violation_count(&self) -> usize {
        self.bundle_completeness_violation_count
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
