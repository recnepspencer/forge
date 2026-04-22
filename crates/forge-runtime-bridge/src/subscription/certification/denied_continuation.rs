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
pub struct BridgeSubscriptionCertificationDeniedContinuationReport {
    admitted_bundle_digest: Arc<str>,
    denied_bundle_digest: Arc<str>,
    comparison_report_digest: Arc<str>,
    primary_failure_boundary: BridgeSubscriptionCertificationFailureBoundary,
    primary_failure_precedence_stage: BridgeSubscriptionCertificationFailurePrecedenceStage,
    denied_before_delivery_drift: bool,
    counters: BridgeSubscriptionCertificationCounterSnapshot,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionCertificationDeniedContinuationReport {
    pub(crate) fn certify() -> Self {
        let manifest = reference_manifest();
        let admitted = assemble_reference_bundle(
            &manifest,
            BridgeSubscriptionCertificationReportBundleInput::stable(),
        );
        let mut denied_input = BridgeSubscriptionCertificationReportBundleInput::stable();
        denied_input.continuation_digest = "report-continuation-digest-authority-denied";
        let denied = assemble_reference_bundle(&manifest, denied_input);
        let plan = BridgeSubscriptionCertificationComparisonPlan::admit(
            BridgeSubscriptionCertificationComparisonRelationship::ExpectedRejection,
            Some(BridgeSubscriptionCertificationFailureBoundary::ContinuationDenialOrAmbiguity),
            None,
        )
        .expect("denied continuation report names continuation boundary");
        let comparison =
            BridgeSubscriptionCertificationComparisonReport::compare(plan, &admitted, &denied);
        let primary_failure_boundary = comparison
            .primary_failure_boundary()
            .expect("denied continuation comparison must localize failure");
        let primary_failure_precedence_stage = comparison
            .primary_failure_precedence_stage()
            .expect("denied continuation comparison must expose precedence");
        let denied_before_delivery_drift = primary_failure_boundary
            == BridgeSubscriptionCertificationFailureBoundary::ContinuationDenialOrAmbiguity
            && !comparison
                .suppressed_failure_boundaries()
                .contains(&BridgeSubscriptionCertificationFailureBoundary::DeliveryDigestDrift)
            && comparison.mismatch_count() == 1;
        let counters = BridgeSubscriptionCertificationCounterSnapshot::combine([
            *comparison.counters(),
            BridgeSubscriptionCertificationCounterSnapshot::from_denied_continuation_report(),
        ]);
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-certification-denied-continuation-report|admitted={}|denied={}|comparison={}|primary={}|stage={}|before-delivery={denied_before_delivery_drift}|counters={}",
            admitted.digest(),
            denied.digest(),
            comparison.digest(),
            primary_failure_boundary.as_str(),
            primary_failure_precedence_stage.as_str(),
            counters.digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            admitted_bundle_digest: Arc::from(admitted.digest()),
            denied_bundle_digest: Arc::from(denied.digest()),
            comparison_report_digest: Arc::from(comparison.digest()),
            primary_failure_boundary,
            primary_failure_precedence_stage,
            denied_before_delivery_drift,
            counters,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-certification-denied-continuation-report:sha256:{digest:x}"
            )),
        }
    }

    pub fn admitted_bundle_digest(&self) -> &str {
        self.admitted_bundle_digest.as_ref()
    }

    pub fn denied_bundle_digest(&self) -> &str {
        self.denied_bundle_digest.as_ref()
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

    pub fn denied_before_delivery_drift(&self) -> bool {
        self.denied_before_delivery_drift
    }

    pub fn counters(&self) -> &BridgeSubscriptionCertificationCounterSnapshot {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
