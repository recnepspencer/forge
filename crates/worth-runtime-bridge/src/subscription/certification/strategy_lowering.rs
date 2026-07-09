use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{
    assemble_reference_bundle, reference_manifest, BridgeSubscriptionCertificationComparisonPlan,
    BridgeSubscriptionCertificationComparisonRelationship,
    BridgeSubscriptionCertificationComparisonReport,
    BridgeSubscriptionCertificationCounterSnapshot, BridgeSubscriptionCertificationDivergenceAxis,
    BridgeSubscriptionCertificationFailureBoundary,
    BridgeSubscriptionCertificationFailurePrecedenceStage,
    BridgeSubscriptionCertificationReportBundleScenario,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionCertificationStrategyLoweringReport {
    detail_bundle_digest: Arc<str>,
    collection_bundle_digest: Arc<str>,
    comparison_report_digest: Arc<str>,
    primary_failure_boundary: BridgeSubscriptionCertificationFailureBoundary,
    primary_failure_precedence_stage: BridgeSubscriptionCertificationFailurePrecedenceStage,
    strategy_lowering_is_distinct_without_signal_rediscovery: bool,
    counters: BridgeSubscriptionCertificationCounterSnapshot,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionCertificationStrategyLoweringReport {
    pub(crate) fn certify() -> Self {
        let manifest = reference_manifest();
        let detail = assemble_reference_bundle(
            &manifest,
            BridgeSubscriptionCertificationReportBundleScenario::StableAdmitted,
        );
        let collection = assemble_reference_bundle(
            &manifest,
            BridgeSubscriptionCertificationReportBundleScenario::CollectionMembershipStrategyLowering,
        );
        let plan = BridgeSubscriptionCertificationComparisonPlan::admit(
            BridgeSubscriptionCertificationComparisonRelationship::IntentionalDivergence,
            None,
            Some(BridgeSubscriptionCertificationDivergenceAxis::StrategyLowering),
        )
        .expect("strategy lowering divergence plan should admit");
        let comparison =
            BridgeSubscriptionCertificationComparisonReport::compare(plan, &detail, &collection);
        let primary_failure_boundary = comparison
            .primary_failure_boundary()
            .expect("strategy lowering comparison must localize drift");
        let primary_failure_precedence_stage = comparison
            .primary_failure_precedence_stage()
            .expect("strategy lowering comparison must expose precedence");
        let strategy_lowering_is_distinct_without_signal_rediscovery =
            detail.semantic_digests().strategy_lowering_digest()
                != collection.semantic_digests().strategy_lowering_digest()
                && detail.counters().global_history_scan_count() == 0
                && collection.counters().global_subscription_scan_count() == 0
                && comparison.mismatch_count() == 1;
        let counters = BridgeSubscriptionCertificationCounterSnapshot::combine([
            *comparison.counters(),
            BridgeSubscriptionCertificationCounterSnapshot::from_strategy_lowering_report(),
        ]);
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-certification-strategy-lowering-report|detail={}|collection={}|comparison={}|primary={}|stage={}|distinct-without-rediscovery={strategy_lowering_is_distinct_without_signal_rediscovery}|counters={}",
            detail.digest(),
            collection.digest(),
            comparison.digest(),
            primary_failure_boundary.as_str(),
            primary_failure_precedence_stage.as_str(),
            counters.digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            detail_bundle_digest: Arc::from(detail.digest()),
            collection_bundle_digest: Arc::from(collection.digest()),
            comparison_report_digest: Arc::from(comparison.digest()),
            primary_failure_boundary,
            primary_failure_precedence_stage,
            strategy_lowering_is_distinct_without_signal_rediscovery,
            counters,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-certification-strategy-lowering-report:sha256:{digest:x}"
            )),
        }
    }

    pub fn detail_bundle_digest(&self) -> &str {
        self.detail_bundle_digest.as_ref()
    }

    pub fn collection_bundle_digest(&self) -> &str {
        self.collection_bundle_digest.as_ref()
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

    pub fn strategy_lowering_is_distinct_without_signal_rediscovery(&self) -> bool {
        self.strategy_lowering_is_distinct_without_signal_rediscovery
    }

    pub fn counters(&self) -> &BridgeSubscriptionCertificationCounterSnapshot {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
