use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{
    detect_failures, outcome_for, precedence_stage_for_boundary, primary_failure_boundary,
    BridgeSubscriptionCertificationBundleSealed, BridgeSubscriptionCertificationComparisonOutcome,
    BridgeSubscriptionCertificationComparisonPlan,
    BridgeSubscriptionCertificationComparisonRelationship,
    BridgeSubscriptionCertificationCounterSnapshot, BridgeSubscriptionCertificationFailureBoundary,
    BridgeSubscriptionCertificationFailurePrecedenceStage,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionCertificationComparisonReport {
    relationship: BridgeSubscriptionCertificationComparisonRelationship,
    left_bundle_digest: Arc<str>,
    right_bundle_digest: Arc<str>,
    outcome: BridgeSubscriptionCertificationComparisonOutcome,
    primary_failure_boundary: Option<BridgeSubscriptionCertificationFailureBoundary>,
    primary_failure_precedence_stage: Option<BridgeSubscriptionCertificationFailurePrecedenceStage>,
    suppressed_failure_boundaries: Vec<BridgeSubscriptionCertificationFailureBoundary>,
    mismatch_count: usize,
    counters: BridgeSubscriptionCertificationCounterSnapshot,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionCertificationComparisonReport {
    pub(crate) fn compare(
        plan: BridgeSubscriptionCertificationComparisonPlan,
        left: &BridgeSubscriptionCertificationBundleSealed,
        right: &BridgeSubscriptionCertificationBundleSealed,
    ) -> Self {
        let failures = detect_failures(left, right);
        let primary_failure_boundary = primary_failure_boundary(&failures);
        let suppressed_failure_boundaries = failures
            .iter()
            .copied()
            .filter(|failure| Some(*failure) != primary_failure_boundary)
            .collect::<Vec<_>>();
        let primary_failure_precedence_stage =
            primary_failure_boundary.map(precedence_stage_for_boundary);
        let mismatch_count = failures.len();
        let outcome = outcome_for(&plan, &failures, primary_failure_boundary);
        let counters = BridgeSubscriptionCertificationCounterSnapshot::combine([
            *plan.counters(),
            BridgeSubscriptionCertificationCounterSnapshot::from_bundle_comparison(
                mismatch_count,
                primary_failure_boundary.is_some(),
            ),
        ]);
        let suppressed = suppressed_failure_boundaries
            .iter()
            .map(|boundary| boundary.as_str())
            .collect::<Vec<_>>()
            .join(",");
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-certification-comparison-report|plan={}|left-bundle={}|right-bundle={}|relationship={}|outcome={}|primary={}|primary-stage={}|suppressed={suppressed}|mismatches={mismatch_count}|counters={}",
            plan.digest(),
            left.digest(),
            right.digest(),
            plan.relationship().as_str(),
            outcome.as_str(),
            primary_failure_boundary
                .map(BridgeSubscriptionCertificationFailureBoundary::as_str)
                .unwrap_or("none"),
            primary_failure_precedence_stage
                .map(BridgeSubscriptionCertificationFailurePrecedenceStage::as_str)
                .unwrap_or("none"),
            counters.digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            relationship: plan.relationship(),
            left_bundle_digest: Arc::from(left.digest()),
            right_bundle_digest: Arc::from(right.digest()),
            outcome,
            primary_failure_boundary,
            primary_failure_precedence_stage,
            suppressed_failure_boundaries,
            mismatch_count,
            counters,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-certification-comparison-report:sha256:{digest:x}"
            )),
        }
    }

    pub fn relationship(&self) -> BridgeSubscriptionCertificationComparisonRelationship {
        self.relationship
    }

    pub fn left_bundle_digest(&self) -> &str {
        self.left_bundle_digest.as_ref()
    }

    pub fn right_bundle_digest(&self) -> &str {
        self.right_bundle_digest.as_ref()
    }

    pub fn outcome(&self) -> BridgeSubscriptionCertificationComparisonOutcome {
        self.outcome
    }

    pub fn primary_failure_boundary(
        &self,
    ) -> Option<BridgeSubscriptionCertificationFailureBoundary> {
        self.primary_failure_boundary
    }

    pub fn primary_failure_precedence_stage(
        &self,
    ) -> Option<BridgeSubscriptionCertificationFailurePrecedenceStage> {
        self.primary_failure_precedence_stage
    }

    pub fn suppressed_failure_boundaries(
        &self,
    ) -> &[BridgeSubscriptionCertificationFailureBoundary] {
        &self.suppressed_failure_boundaries
    }

    pub fn mismatch_count(&self) -> usize {
        self.mismatch_count
    }

    pub fn counters(&self) -> &BridgeSubscriptionCertificationCounterSnapshot {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
