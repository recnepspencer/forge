use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::subscription::{
    BridgeActiveSubscription, BridgeSubscriptionAdmittedResumeBasisIdentity,
    BridgeSubscriptionCounters, BridgeSubscriptionResumeAdmission,
};

use super::basis::BridgeRetainedSubscriptionResumeBasis;
use super::rejection::{
    BridgeSubscriptionResumeBasisRejection, BridgeSubscriptionResumeBasisRejectionKind,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmittedBridgeSubscriptionResumeBasis {
    admitted_resume_basis_identity: BridgeSubscriptionAdmittedResumeBasisIdentity,
    retained_basis: BridgeRetainedSubscriptionResumeBasis,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl AdmittedBridgeSubscriptionResumeBasis {
    pub(crate) fn admit(
        retained_basis: &BridgeRetainedSubscriptionResumeBasis,
    ) -> Result<Self, BridgeSubscriptionResumeBasisRejection> {
        if !retained_basis.retention_complete()
            || retained_basis
                .temporal_resume_basis()
                .is_some_and(|basis| !basis.retention_complete())
            || retained_basis
                .inflight_async_resume_basis()
                .is_some_and(|basis| !basis.retention_complete())
            || retained_basis
                .delivery_resume_basis()
                .is_some_and(|basis| !basis.retention_complete())
        {
            return Err(BridgeSubscriptionResumeBasisRejection::new(
                BridgeSubscriptionResumeBasisRejectionKind::RetentionTruncated,
            ));
        }
        if retained_basis.active_subscription_identity().is_empty() {
            return Err(BridgeSubscriptionResumeBasisRejection::new(
                BridgeSubscriptionResumeBasisRejectionKind::ActiveSubscriptionMismatch,
            ));
        }
        if retained_basis.admitted_subscription_identity().is_empty() {
            return Err(BridgeSubscriptionResumeBasisRejection::new(
                BridgeSubscriptionResumeBasisRejectionKind::AdmittedSubscriptionMismatch,
            ));
        }
        if retained_basis.basis_identity().is_empty() {
            return Err(BridgeSubscriptionResumeBasisRejection::new(
                BridgeSubscriptionResumeBasisRejectionKind::BasisMismatch,
            ));
        }
        if retained_basis.cost_profile_identity().is_empty() {
            return Err(BridgeSubscriptionResumeBasisRejection::new(
                BridgeSubscriptionResumeBasisRejectionKind::CostProfileMismatch,
            ));
        }
        if retained_basis.consumer_contract_identity().is_empty() {
            return Err(BridgeSubscriptionResumeBasisRejection::new(
                BridgeSubscriptionResumeBasisRejectionKind::ConsumerContractMismatch,
            ));
        }
        if retained_basis.fanout_layout_identity().is_some()
            && retained_basis.delivery_resume_basis().is_none()
        {
            return Err(BridgeSubscriptionResumeBasisRejection::new(
                BridgeSubscriptionResumeBasisRejectionKind::DeliveryBasisMissing,
            ));
        }
        if retained_basis.temporal_resume_basis().is_some()
            && retained_basis
                .temporal_resume_basis()
                .is_some_and(|basis| basis.digest().is_empty())
        {
            return Err(BridgeSubscriptionResumeBasisRejection::new(
                BridgeSubscriptionResumeBasisRejectionKind::TemporalBasisMissing,
            ));
        }
        if retained_basis.inflight_async_resume_basis().is_some()
            && retained_basis
                .inflight_async_resume_basis()
                .is_some_and(|basis| basis.request_generation().is_none())
        {
            return Err(BridgeSubscriptionResumeBasisRejection::new(
                BridgeSubscriptionResumeBasisRejectionKind::InflightAsyncGenerationMissing,
            ));
        }

        let temporal_branch = retained_basis
            .temporal_resume_basis()
            .map(|basis| basis.truth_branch_identity());
        let async_branch = retained_basis
            .inflight_async_resume_basis()
            .and_then(|basis| basis.truth_branch_identity());
        if matches!((temporal_branch, async_branch), (Some(left), Some(right)) if left != right) {
            return Err(BridgeSubscriptionResumeBasisRejection::new(
                BridgeSubscriptionResumeBasisRejectionKind::CrossBranchResumeRejected,
            ));
        }

        let canonical_basis = Arc::<str>::from(format!(
            "bridge-admitted-subscription-resume-basis|retained={}|checkpoint={}|basis={}|next-sequence={}|temporal={}|async={}|delivery={}",
            retained_basis.digest(),
            retained_basis.checkpoint_identity(),
            retained_basis.basis_identity(),
            retained_basis.expected_next_canonical_sequence(),
            retained_basis
                .temporal_resume_basis()
                .map(|basis| basis.digest())
                .unwrap_or("-"),
            retained_basis
                .inflight_async_resume_basis()
                .map(|basis| basis.digest())
                .unwrap_or("-"),
            retained_basis
                .delivery_resume_basis()
                .map(|basis| basis.digest())
                .unwrap_or("-"),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Ok(Self {
            admitted_resume_basis_identity: BridgeSubscriptionAdmittedResumeBasisIdentity::new(
                format!("bridge-admitted-subscription-resume-basis-id:sha256:{digest:x}"),
            ),
            retained_basis: retained_basis.clone(),
            counters: BridgeSubscriptionCounters::from_resume_basis_admission(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-admitted-subscription-resume-basis:sha256:{digest:x}"
            )),
        })
    }

    pub fn admitted_resume_basis_identity(&self) -> &BridgeSubscriptionAdmittedResumeBasisIdentity {
        &self.admitted_resume_basis_identity
    }

    pub fn retained_basis(&self) -> &BridgeRetainedSubscriptionResumeBasis {
        &self.retained_basis
    }

    pub(crate) fn lower_resume_admission(
        &self,
        active_subscription: &BridgeActiveSubscription,
        replay_readiness: &super::readiness::BridgeSubscriptionReplayReadiness,
    ) -> Result<BridgeSubscriptionResumeAdmission, BridgeSubscriptionResumeBasisRejection> {
        if replay_readiness.admitted_resume_basis_identity()
            != self.admitted_resume_basis_identity().as_str()
        {
            return Err(BridgeSubscriptionResumeBasisRejection::new(
                BridgeSubscriptionResumeBasisRejectionKind::BasisMismatch,
            ));
        }
        if replay_readiness.retained_resume_basis_digest() != self.retained_basis().digest() {
            return Err(BridgeSubscriptionResumeBasisRejection::new(
                BridgeSubscriptionResumeBasisRejectionKind::BasisMismatch,
            ));
        }
        BridgeSubscriptionResumeAdmission::from_retained_resume_basis(
            active_subscription,
            self.retained_basis(),
        )
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
