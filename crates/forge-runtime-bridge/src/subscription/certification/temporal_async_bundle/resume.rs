use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::subscription::{AdmittedBridgeSubscriptionResumeBasis, BridgeActiveSubscription};

use super::bundle::{
    BridgeTemporalAsyncCertificationBundleRejection,
    BridgeTemporalAsyncCertificationBundleRejectionKind,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeTemporalAsyncCertificationResumeSection {
    bridge_owner: Arc<str>,
    active_subscription_identity: Arc<str>,
    admitted_subscription_identity: Arc<str>,
    retained_resume_basis_digest: Arc<str>,
    temporal_resume_basis_digest: Arc<str>,
    inflight_async_resume_basis_digest: Arc<str>,
    delivery_resume_basis_digest: Arc<str>,
    semantic_digest: Arc<str>,
    digest: Arc<str>,
}

impl BridgeTemporalAsyncCertificationResumeSection {
    pub(crate) fn collect(
        active_subscription: &BridgeActiveSubscription,
        resume_basis: &AdmittedBridgeSubscriptionResumeBasis,
    ) -> Result<Self, BridgeTemporalAsyncCertificationBundleRejection> {
        let retained = resume_basis.retained_basis();
        if retained.active_subscription_identity()
            != active_subscription.active_subscription_identity().as_str()
            || retained.admitted_subscription_identity()
                != active_subscription
                    .activation_ready()
                    .admitted()
                    .admitted_subscription_identity()
                    .as_str()
        {
            return Err(BridgeTemporalAsyncCertificationBundleRejection::new(
                BridgeTemporalAsyncCertificationBundleRejectionKind::ResumeSubscriptionMismatch,
                "resume basis must retain the same active/admitted subscription identities",
            ));
        }
        let temporal_resume_basis_digest = retained
            .temporal_resume_basis()
            .map(|basis| basis.digest())
            .unwrap_or("-");
        let inflight_async_resume_basis_digest = retained
            .inflight_async_resume_basis()
            .map(|basis| basis.digest())
            .unwrap_or("-");
        let delivery_resume_basis_digest = retained
            .delivery_resume_basis()
            .map(|basis| basis.digest())
            .unwrap_or("-");
        let semantic_basis = format!(
            "bridge-temporal-async-certification-resume-section|retained={}|temporal={}|async={}|delivery={}",
            retained.digest(),
            temporal_resume_basis_digest,
            inflight_async_resume_basis_digest,
            delivery_resume_basis_digest,
        );
        let semantic_digest = Sha256::digest(semantic_basis.as_bytes());
        let digest = Sha256::digest(
            format!("{semantic_basis}|bridge-owner=forge-runtime-bridge").as_bytes(),
        );
        Ok(Self {
            bridge_owner: Arc::from("forge-runtime-bridge"),
            active_subscription_identity: Arc::from(
                retained.active_subscription_identity().to_owned(),
            ),
            admitted_subscription_identity: Arc::from(
                retained.admitted_subscription_identity().to_owned(),
            ),
            retained_resume_basis_digest: Arc::from(retained.digest().to_owned()),
            temporal_resume_basis_digest: Arc::from(temporal_resume_basis_digest.to_owned()),
            inflight_async_resume_basis_digest: Arc::from(
                inflight_async_resume_basis_digest.to_owned(),
            ),
            delivery_resume_basis_digest: Arc::from(delivery_resume_basis_digest.to_owned()),
            semantic_digest: Arc::from(format!(
                "bridge-temporal-async-certification-resume-section-semantic:sha256:{semantic_digest:x}"
            )),
            digest: Arc::from(format!(
                "bridge-temporal-async-certification-resume-section:sha256:{digest:x}"
            )),
        })
    }

    pub fn bridge_owner(&self) -> &str {
        self.bridge_owner.as_ref()
    }

    pub fn retained_resume_basis_digest(&self) -> &str {
        self.retained_resume_basis_digest.as_ref()
    }

    pub fn semantic_digest(&self) -> &str {
        self.semantic_digest.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
