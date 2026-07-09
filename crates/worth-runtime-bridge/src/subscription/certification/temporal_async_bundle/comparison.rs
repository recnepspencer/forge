use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::bundle::BridgeTemporalAsyncCertificationBundleSealed;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeTemporalAsyncCertificationBundleComparisonOutcome {
    Equivalent,
    DiagnosticsRichnessOnlyDelta,
    Divergent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeTemporalAsyncCertificationBundleMismatchSection {
    SubscriptionIdentity,
    TemporalBasis,
    AsyncLifecycle,
    MixedCauseDelivery,
    ResumePosture,
    FailureLocalization,
    DiagnosticsRichness,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeTemporalAsyncCertificationBundleComparison {
    outcome: BridgeTemporalAsyncCertificationBundleComparisonOutcome,
    left_digest: Arc<str>,
    right_digest: Arc<str>,
    left_semantic_digest: Arc<str>,
    right_semantic_digest: Arc<str>,
    mismatched_sections: Arc<[BridgeTemporalAsyncCertificationBundleMismatchSection]>,
    digest: Arc<str>,
}

impl BridgeTemporalAsyncCertificationBundleComparison {
    pub(crate) fn compare(
        left: &BridgeTemporalAsyncCertificationBundleSealed,
        right: &BridgeTemporalAsyncCertificationBundleSealed,
    ) -> Self {
        let mut mismatched_sections = Vec::new();
        if left.active_subscription_identity() != right.active_subscription_identity()
            || left.admitted_subscription_identity() != right.admitted_subscription_identity()
        {
            mismatched_sections
                .push(BridgeTemporalAsyncCertificationBundleMismatchSection::SubscriptionIdentity);
        }
        if left.basis_section().semantic_digest() != right.basis_section().semantic_digest() {
            mismatched_sections
                .push(BridgeTemporalAsyncCertificationBundleMismatchSection::TemporalBasis);
        }
        if left.async_section().semantic_digest() != right.async_section().semantic_digest() {
            mismatched_sections
                .push(BridgeTemporalAsyncCertificationBundleMismatchSection::AsyncLifecycle);
        }
        if left.mixed_cause_section().semantic_digest()
            != right.mixed_cause_section().semantic_digest()
        {
            mismatched_sections
                .push(BridgeTemporalAsyncCertificationBundleMismatchSection::MixedCauseDelivery);
        }
        if left.resume_section().semantic_digest() != right.resume_section().semantic_digest() {
            mismatched_sections
                .push(BridgeTemporalAsyncCertificationBundleMismatchSection::ResumePosture);
        }
        if left.failure_section().semantic_digest() != right.failure_section().semantic_digest() {
            mismatched_sections
                .push(BridgeTemporalAsyncCertificationBundleMismatchSection::FailureLocalization);
        }
        if left.diagnostics_richness() != right.diagnostics_richness() {
            mismatched_sections
                .push(BridgeTemporalAsyncCertificationBundleMismatchSection::DiagnosticsRichness);
        }
        let outcome = if left.semantic_digest() == right.semantic_digest() {
            if left.digest() == right.digest() {
                BridgeTemporalAsyncCertificationBundleComparisonOutcome::Equivalent
            } else {
                BridgeTemporalAsyncCertificationBundleComparisonOutcome::DiagnosticsRichnessOnlyDelta
            }
        } else {
            BridgeTemporalAsyncCertificationBundleComparisonOutcome::Divergent
        };
        let canonical_basis = format!(
            "bridge-temporal-async-certification-bundle-comparison|outcome={outcome:?}|left={}|right={}|mismatches={}",
            left.digest(),
            right.digest(),
            mismatched_sections
                .iter()
                .map(|section| format!("{section:?}"))
                .collect::<Vec<_>>()
                .join(","),
        );
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            outcome,
            left_digest: Arc::from(left.digest().to_owned()),
            right_digest: Arc::from(right.digest().to_owned()),
            left_semantic_digest: Arc::from(left.semantic_digest().to_owned()),
            right_semantic_digest: Arc::from(right.semantic_digest().to_owned()),
            mismatched_sections: mismatched_sections.into(),
            digest: Arc::from(format!(
                "bridge-temporal-async-certification-bundle-comparison:sha256:{digest:x}"
            )),
        }
    }

    pub fn outcome(&self) -> BridgeTemporalAsyncCertificationBundleComparisonOutcome {
        self.outcome
    }

    pub fn diagnostics_richness_only_delta(&self) -> bool {
        self.outcome
            == BridgeTemporalAsyncCertificationBundleComparisonOutcome::DiagnosticsRichnessOnlyDelta
    }

    pub fn mismatched_sections(&self) -> &[BridgeTemporalAsyncCertificationBundleMismatchSection] {
        self.mismatched_sections.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
