use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::subscription::{
    BridgeSharedConsumerDeliveryBundleSealed, BridgeSharedConsumerDeliveryProjection,
    BridgeSharedDeliveryAcknowledgementFrontier, BridgeSubscriptionCounters,
    BridgeSubscriptionRetainedDeliveryResumeBasisIdentity,
};

use super::rejection::{
    BridgeSubscriptionResumeBasisRejection, BridgeSubscriptionResumeBasisRejectionKind,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeRetainedDeliveryResumeBasis {
    retained_delivery_resume_basis_identity: BridgeSubscriptionRetainedDeliveryResumeBasisIdentity,
    bundle_identity: Arc<str>,
    projection_identity: Arc<str>,
    consumer_contract_identity: Arc<str>,
    acknowledged_ordered_cause_sequence: usize,
    acknowledged_prefix_digest: Arc<str>,
    retention_complete: bool,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeRetainedDeliveryResumeBasis {
    pub(crate) fn capture(
        bundle: &BridgeSharedConsumerDeliveryBundleSealed,
        projection: &BridgeSharedConsumerDeliveryProjection,
        acknowledgement: &BridgeSharedDeliveryAcknowledgementFrontier,
        retention_complete: bool,
    ) -> Result<Self, BridgeSubscriptionResumeBasisRejection> {
        if projection.bundle_identity() != bundle.shared_delivery_bundle_sealed_identity().as_str()
            || acknowledgement.bundle_identity()
                != bundle.shared_delivery_bundle_sealed_identity().as_str()
            || acknowledgement.projection_identity()
                != projection.shared_delivery_projection_identity().as_str()
            || acknowledgement.consumer_contract_identity()
                != projection.consumer_contract_identity()
        {
            return Err(BridgeSubscriptionResumeBasisRejection::new(
                BridgeSubscriptionResumeBasisRejectionKind::DeliveryBasisMismatch,
            ));
        }

        let canonical_basis = Arc::<str>::from(format!(
            "bridge-retained-delivery-resume-basis|bundle={}|projection={}|consumer={}|ack-sequence={}|prefix={}|retention-complete={retention_complete}",
            bundle.shared_delivery_bundle_sealed_identity().as_str(),
            projection.shared_delivery_projection_identity().as_str(),
            projection.consumer_contract_identity(),
            acknowledgement.acknowledged_ordered_cause_sequence(),
            acknowledgement.acknowledged_prefix_digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Ok(Self {
            retained_delivery_resume_basis_identity:
                BridgeSubscriptionRetainedDeliveryResumeBasisIdentity::new(format!(
                    "bridge-retained-delivery-resume-basis-id:sha256:{digest:x}"
                )),
            bundle_identity: Arc::from(
                bundle
                    .shared_delivery_bundle_sealed_identity()
                    .as_str()
                    .to_owned(),
            ),
            projection_identity: Arc::from(
                projection
                    .shared_delivery_projection_identity()
                    .as_str()
                    .to_owned(),
            ),
            consumer_contract_identity: Arc::from(
                projection.consumer_contract_identity().to_owned(),
            ),
            acknowledged_ordered_cause_sequence: acknowledgement
                .acknowledged_ordered_cause_sequence(),
            acknowledged_prefix_digest: Arc::from(
                acknowledgement.acknowledged_prefix_digest().to_owned(),
            ),
            retention_complete,
            counters: BridgeSubscriptionCounters::from_resume_delivery_basis(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-retained-delivery-resume-basis:sha256:{digest:x}"
            )),
        })
    }

    pub fn acknowledged_ordered_cause_sequence(&self) -> usize {
        self.acknowledged_ordered_cause_sequence
    }

    pub fn retention_complete(&self) -> bool {
        self.retention_complete
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
