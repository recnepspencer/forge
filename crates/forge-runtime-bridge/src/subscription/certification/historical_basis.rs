use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{
    assemble_reference_bundle, reference_manifest, BridgeSubscriptionCertificationComparisonPlan,
    BridgeSubscriptionCertificationComparisonRelationship,
    BridgeSubscriptionCertificationComparisonReport,
    BridgeSubscriptionCertificationCounterSnapshot, BridgeSubscriptionCertificationFailureBoundary,
    BridgeSubscriptionCertificationFailurePrecedenceStage,
    BridgeSubscriptionCertificationReportBundleInput,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionCertificationHistoricalBasisReport {
    retained_basis_bundle_digest: Arc<str>,
    latest_fallback_bundle_digest: Arc<str>,
    comparison_report_digest: Arc<str>,
    primary_failure_boundary: BridgeSubscriptionCertificationFailureBoundary,
    primary_failure_precedence_stage: BridgeSubscriptionCertificationFailurePrecedenceStage,
    retained_basis_is_explicit: bool,
    latest_truth_fallback_count: usize,
    counters: BridgeSubscriptionCertificationCounterSnapshot,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionCertificationHistoricalBasisReport {
    pub(crate) fn certify() -> Self {
        let manifest = reference_manifest();
        let retained = assemble_reference_bundle(
            &manifest,
            BridgeSubscriptionCertificationReportBundleInput::stable(),
        );
        let mut fallback_input = BridgeSubscriptionCertificationReportBundleInput::stable();
        fallback_input.basis_digest = "report-basis-digest-latest-fallback";
        let fallback = assemble_reference_bundle(&manifest, fallback_input);
        let plan = BridgeSubscriptionCertificationComparisonPlan::admit(
            BridgeSubscriptionCertificationComparisonRelationship::ExpectedRejection,
            Some(BridgeSubscriptionCertificationFailureBoundary::BasisDrift),
            None,
        )
        .expect("historical basis report names basis drift boundary");
        let comparison =
            BridgeSubscriptionCertificationComparisonReport::compare(plan, &retained, &fallback);
        let primary_failure_boundary = comparison
            .primary_failure_boundary()
            .expect("historical basis comparison must localize basis drift");
        let primary_failure_precedence_stage = comparison
            .primary_failure_precedence_stage()
            .expect("historical basis comparison must expose precedence");
        let retained_basis_is_explicit = retained.semantic_digests().subscription_basis_digest()
            != fallback.semantic_digests().subscription_basis_digest()
            && comparison.mismatch_count() == 1;
        let latest_truth_fallback_count = 0;
        let counters = BridgeSubscriptionCertificationCounterSnapshot::combine([
            *comparison.counters(),
            BridgeSubscriptionCertificationCounterSnapshot::from_historical_basis_report(),
        ]);
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-certification-historical-basis-report|retained={}|latest-fallback={}|comparison={}|primary={}|stage={}|retained-explicit={retained_basis_is_explicit}|latest-fallback-count={latest_truth_fallback_count}|counters={}",
            retained.digest(),
            fallback.digest(),
            comparison.digest(),
            primary_failure_boundary.as_str(),
            primary_failure_precedence_stage.as_str(),
            counters.digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            retained_basis_bundle_digest: Arc::from(retained.digest()),
            latest_fallback_bundle_digest: Arc::from(fallback.digest()),
            comparison_report_digest: Arc::from(comparison.digest()),
            primary_failure_boundary,
            primary_failure_precedence_stage,
            retained_basis_is_explicit,
            latest_truth_fallback_count,
            counters,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-certification-historical-basis-report:sha256:{digest:x}"
            )),
        }
    }

    pub fn retained_basis_bundle_digest(&self) -> &str {
        self.retained_basis_bundle_digest.as_ref()
    }

    pub fn latest_fallback_bundle_digest(&self) -> &str {
        self.latest_fallback_bundle_digest.as_ref()
    }

    pub fn comparison_report_digest(&self) -> &str {
        self.comparison_report_digest.as_ref()
    }

    pub fn primary_failure_boundary(&self) -> BridgeSubscriptionCertificationFailureBoundary {
        self.primary_failure_boundary
    }

    pub fn primary_failure_precedence_stage(
        &self,
    ) -> BridgeSubscriptionCertificationFailurePrecedenceStage {
        self.primary_failure_precedence_stage
    }

    pub fn retained_basis_is_explicit(&self) -> bool {
        self.retained_basis_is_explicit
    }

    pub fn latest_truth_fallback_count(&self) -> usize {
        self.latest_truth_fallback_count
    }

    pub fn counters(&self) -> &BridgeSubscriptionCertificationCounterSnapshot {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
