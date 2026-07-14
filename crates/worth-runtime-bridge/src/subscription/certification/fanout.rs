use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{
    assemble_reference_bundle, reference_manifest, BridgeSubscriptionCertificationComparisonPlan,
    BridgeSubscriptionCertificationComparisonRelationship,
    BridgeSubscriptionCertificationComparisonReport,
    BridgeSubscriptionCertificationCounterSnapshot, BridgeSubscriptionCertificationFailureBoundary,
    BridgeSubscriptionCertificationReportBundleScenario,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionCertificationFanoutReport {
    shared_equivalence_report_digest: Arc<str>,
    divergent_rejection_report_digest: Arc<str>,
    shared_fanout_equivalent: bool,
    divergent_sharing_rejected_before_delivery: bool,
    counters: BridgeSubscriptionCertificationCounterSnapshot,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionCertificationFanoutReport {
    pub(crate) fn certify() -> Self {
        let manifest = reference_manifest();
        let control = assemble_reference_bundle(
            &manifest,
            BridgeSubscriptionCertificationReportBundleScenario::StableAdmitted,
        );
        let equivalent = assemble_reference_bundle(
            &manifest,
            BridgeSubscriptionCertificationReportBundleScenario::StableAdmitted,
        );
        let divergent = assemble_reference_bundle(
            &manifest,
            BridgeSubscriptionCertificationReportBundleScenario::DivergentFanout,
        );
        let equivalence_plan = BridgeSubscriptionCertificationComparisonPlan::admit(
            BridgeSubscriptionCertificationComparisonRelationship::SemanticEquivalence,
            None,
            None,
        )
        .expect("fanout equivalence plan should admit");
        let rejection_plan = BridgeSubscriptionCertificationComparisonPlan::admit(
            BridgeSubscriptionCertificationComparisonRelationship::ExpectedRejection,
            Some(BridgeSubscriptionCertificationFailureBoundary::IllegalSharingReuse),
            None,
        )
        .expect("divergent sharing rejection plan should admit");
        let shared_comparison = BridgeSubscriptionCertificationComparisonReport::compare(
            equivalence_plan,
            &control,
            &equivalent,
        );
        let divergent_comparison = BridgeSubscriptionCertificationComparisonReport::compare(
            rejection_plan,
            &control,
            &divergent,
        );
        let shared_fanout_equivalent = shared_comparison.mismatch_count() == 0;
        let divergent_sharing_rejected_before_delivery = divergent_comparison
            .primary_failure_boundary()
            == Some(BridgeSubscriptionCertificationFailureBoundary::IllegalSharingReuse)
            && !divergent_comparison
                .suppressed_failure_boundaries()
                .contains(&BridgeSubscriptionCertificationFailureBoundary::DeliveryDigestDrift);
        let counters = BridgeSubscriptionCertificationCounterSnapshot::combine([
            *shared_comparison.counters(),
            *divergent_comparison.counters(),
            BridgeSubscriptionCertificationCounterSnapshot::from_fanout_report(),
        ]);
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-certification-fanout-report|shared={}|divergent={}|shared-equivalent={shared_fanout_equivalent}|divergent-before-delivery={divergent_sharing_rejected_before_delivery}|counters={}",
            shared_comparison.digest(),
            divergent_comparison.digest(),
            counters.digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            shared_equivalence_report_digest: Arc::from(shared_comparison.digest()),
            divergent_rejection_report_digest: Arc::from(divergent_comparison.digest()),
            shared_fanout_equivalent,
            divergent_sharing_rejected_before_delivery,
            counters,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-certification-fanout-report:sha256:{digest:x}"
            )),
        }
    }

    pub fn shared_equivalence_report_digest(&self) -> &str {
        self.shared_equivalence_report_digest.as_ref()
    }

    pub fn divergent_rejection_report_digest(&self) -> &str {
        self.divergent_rejection_report_digest.as_ref()
    }

    pub fn shared_fanout_equivalent(&self) -> bool {
        self.shared_fanout_equivalent
    }

    pub fn divergent_sharing_rejected_before_delivery(&self) -> bool {
        self.divergent_sharing_rejected_before_delivery
    }

    pub fn counters(&self) -> &BridgeSubscriptionCertificationCounterSnapshot {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
