use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{
    assemble_reference_bundle, precedence_stage_for_boundary, reference_manifest,
    BridgeSubscriptionCertificationComparisonPlan,
    BridgeSubscriptionCertificationComparisonRelationship,
    BridgeSubscriptionCertificationComparisonReport,
    BridgeSubscriptionCertificationCounterSnapshot, BridgeSubscriptionCertificationFailureBoundary,
    BridgeSubscriptionCertificationFailurePrecedenceStage,
    BridgeSubscriptionCertificationReportBundleScenario,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionCertificationBundleInsufficiencyReport {
    complete_bundle_digest: Arc<str>,
    insufficient_bundle_digest: Arc<str>,
    complete_completeness_report_digest: Arc<str>,
    insufficient_completeness_report_digest: Arc<str>,
    comparison_report_digest: Arc<str>,
    primary_failure_boundary: BridgeSubscriptionCertificationFailureBoundary,
    primary_failure_precedence_stage: BridgeSubscriptionCertificationFailurePrecedenceStage,
    insufficiency_is_primary_without_semantic_drift: bool,
    counters: BridgeSubscriptionCertificationCounterSnapshot,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionCertificationBundleInsufficiencyReport {
    pub(crate) fn certify() -> Self {
        let manifest = reference_manifest();
        let complete = assemble_reference_bundle(
            &manifest,
            BridgeSubscriptionCertificationReportBundleScenario::StableAdmitted,
        );
        let insufficient = assemble_reference_bundle(
            &manifest,
            BridgeSubscriptionCertificationReportBundleScenario::BundleInsufficiency,
        );
        let plan = BridgeSubscriptionCertificationComparisonPlan::admit(
            BridgeSubscriptionCertificationComparisonRelationship::BundleCompleteness,
            None,
            None,
        )
        .expect("bundle completeness comparison plan should admit");
        let comparison = BridgeSubscriptionCertificationComparisonReport::compare(
            plan,
            &complete,
            &insufficient,
        );
        Self::from_parts(complete, insufficient, comparison)
    }

    fn from_parts(
        complete: super::BridgeSubscriptionCertificationBundleSealed,
        insufficient: super::BridgeSubscriptionCertificationBundleSealed,
        comparison: BridgeSubscriptionCertificationComparisonReport,
    ) -> Self {
        let primary_failure_boundary = comparison
            .primary_failure_boundary()
            .expect("bundle insufficiency comparison must localize a primary failure");
        let primary_failure_precedence_stage = comparison
            .primary_failure_precedence_stage()
            .expect("bundle insufficiency comparison must expose precedence");
        let insufficiency_is_primary_without_semantic_drift = primary_failure_boundary
            == BridgeSubscriptionCertificationFailureBoundary::BundleInsufficiency
            && primary_failure_precedence_stage
                == precedence_stage_for_boundary(
                    BridgeSubscriptionCertificationFailureBoundary::BundleInsufficiency,
                )
            && complete.semantic_digests().digest() == insufficient.semantic_digests().digest()
            && complete.counters().digest() == insufficient.counters().digest()
            && comparison.mismatch_count() == 1;
        let counters = BridgeSubscriptionCertificationCounterSnapshot::combine([
            *comparison.counters(),
            BridgeSubscriptionCertificationCounterSnapshot::from_bundle_insufficiency_report(),
        ]);
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-certification-bundle-insufficiency-report|complete={}|insufficient={}|complete-completeness={}|insufficient-completeness={}|comparison={}|primary={}|stage={}|insufficiency-primary={insufficiency_is_primary_without_semantic_drift}|counters={}",
            complete.digest(),
            insufficient.digest(),
            complete.completeness_report().digest(),
            insufficient.completeness_report().digest(),
            comparison.digest(),
            primary_failure_boundary.as_str(),
            primary_failure_precedence_stage.as_str(),
            counters.digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            complete_bundle_digest: Arc::from(complete.digest()),
            insufficient_bundle_digest: Arc::from(insufficient.digest()),
            complete_completeness_report_digest: Arc::from(complete.completeness_report().digest()),
            insufficient_completeness_report_digest: Arc::from(
                insufficient.completeness_report().digest(),
            ),
            comparison_report_digest: Arc::from(comparison.digest()),
            primary_failure_boundary,
            primary_failure_precedence_stage,
            insufficiency_is_primary_without_semantic_drift,
            counters,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-certification-bundle-insufficiency-report:sha256:{digest:x}"
            )),
        }
    }

    pub fn complete_bundle_digest(&self) -> &str {
        self.complete_bundle_digest.as_ref()
    }

    pub fn insufficient_bundle_digest(&self) -> &str {
        self.insufficient_bundle_digest.as_ref()
    }

    pub fn complete_completeness_report_digest(&self) -> &str {
        self.complete_completeness_report_digest.as_ref()
    }

    pub fn insufficient_completeness_report_digest(&self) -> &str {
        self.insufficient_completeness_report_digest.as_ref()
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

    pub fn insufficiency_is_primary_without_semantic_drift(&self) -> bool {
        self.insufficiency_is_primary_without_semantic_drift
    }

    pub fn counters(&self) -> &BridgeSubscriptionCertificationCounterSnapshot {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
