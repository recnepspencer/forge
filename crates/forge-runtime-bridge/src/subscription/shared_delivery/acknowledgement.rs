use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::subscription::{
    BridgeSubscriptionCounters, BridgeSubscriptionSharedDeliveryAcknowledgementIdentity,
};

use super::{BridgeSharedConsumerDeliveryBundleSealed, BridgeSharedConsumerDeliveryProjection};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSharedDeliveryAcknowledgementFrontierRejectionKind {
    DescriptorOnlyFamilyCannotPublishAcknowledgement,
    ProjectionBundleMismatch,
    OrderedCauseSequenceOutOfRange,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSharedDeliveryAcknowledgementFrontierRejection {
    rejection_kind: BridgeSharedDeliveryAcknowledgementFrontierRejectionKind,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSharedDeliveryAcknowledgementFrontierRejection {
    fn new(rejection_kind: BridgeSharedDeliveryAcknowledgementFrontierRejectionKind) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-shared-delivery-acknowledgement-frontier-rejection|kind={rejection_kind:?}"
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            rejection_kind,
            counters: BridgeSubscriptionCounters::from_shared_delivery_acknowledgement_rejection(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-shared-delivery-acknowledgement-frontier-rejection:sha256:{digest:x}"
            )),
        }
    }

    pub fn rejection_kind(&self) -> BridgeSharedDeliveryAcknowledgementFrontierRejectionKind {
        self.rejection_kind
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSharedDeliveryAcknowledgementFrontier {
    shared_delivery_acknowledgement_identity:
        BridgeSubscriptionSharedDeliveryAcknowledgementIdentity,
    bundle_identity: Arc<str>,
    projection_identity: Arc<str>,
    consumer_contract_identity: Arc<str>,
    acknowledged_ordered_cause_sequence: usize,
    acknowledged_ordered_cause_digest: Arc<str>,
    acknowledged_prefix_digest: Arc<str>,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSharedDeliveryAcknowledgementFrontier {
    pub(crate) fn admit(
        bundle: &BridgeSharedConsumerDeliveryBundleSealed,
        projection: &BridgeSharedConsumerDeliveryProjection,
        acknowledged_ordered_cause_sequence: usize,
    ) -> Result<Self, BridgeSharedDeliveryAcknowledgementFrontierRejection> {
        if projection.bundle_identity() != bundle.shared_delivery_bundle_sealed_identity().as_str()
        {
            return Err(BridgeSharedDeliveryAcknowledgementFrontierRejection::new(
                BridgeSharedDeliveryAcknowledgementFrontierRejectionKind::ProjectionBundleMismatch,
            ));
        }
        if matches!(
            projection.projection_posture(),
            super::BridgeSharedConsumerDeliveryProjectionPosture::DescriptorOnly
        ) {
            return Err(BridgeSharedDeliveryAcknowledgementFrontierRejection::new(
                BridgeSharedDeliveryAcknowledgementFrontierRejectionKind::DescriptorOnlyFamilyCannotPublishAcknowledgement,
            ));
        }
        let Some(acknowledged_ordered_cause_digest) = bundle
            .ordered_causes()
            .get(acknowledged_ordered_cause_sequence)
        else {
            return Err(BridgeSharedDeliveryAcknowledgementFrontierRejection::new(
                BridgeSharedDeliveryAcknowledgementFrontierRejectionKind::OrderedCauseSequenceOutOfRange,
            ));
        };
        let prefix_basis = bundle
            .ordered_causes()
            .iter()
            .take(acknowledged_ordered_cause_sequence + 1)
            .map(|cause| cause.digest())
            .collect::<Vec<_>>()
            .join(",");
        let acknowledged_prefix_digest = Arc::<str>::from(format!(
            "bridge-shared-delivery-acknowledged-prefix:sha256:{:x}",
            Sha256::digest(prefix_basis.as_bytes())
        ));
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-shared-delivery-acknowledgement-frontier|bundle={}|projection={}|consumer={}|sequence={}|cause={}|prefix={}",
            bundle.shared_delivery_bundle_sealed_identity().as_str(),
            projection.shared_delivery_projection_identity().as_str(),
            projection.consumer_contract_identity(),
            acknowledged_ordered_cause_sequence,
            acknowledged_ordered_cause_digest.digest(),
            acknowledged_prefix_digest.as_ref(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Ok(Self {
            shared_delivery_acknowledgement_identity:
                BridgeSubscriptionSharedDeliveryAcknowledgementIdentity::new(format!(
                    "bridge-shared-delivery-acknowledgement-id:sha256:{digest:x}"
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
            acknowledged_ordered_cause_sequence,
            acknowledged_ordered_cause_digest: Arc::from(
                acknowledged_ordered_cause_digest.digest().to_owned(),
            ),
            acknowledged_prefix_digest,
            counters: BridgeSubscriptionCounters::from_shared_delivery_acknowledgement(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-shared-delivery-acknowledgement-frontier:sha256:{digest:x}"
            )),
        })
    }

    pub fn acknowledged_ordered_cause_sequence(&self) -> usize {
        self.acknowledged_ordered_cause_sequence
    }

    pub fn bundle_identity(&self) -> &str {
        self.bundle_identity.as_ref()
    }

    pub fn projection_identity(&self) -> &str {
        self.projection_identity.as_ref()
    }

    pub fn consumer_contract_identity(&self) -> &str {
        self.consumer_contract_identity.as_ref()
    }

    pub fn acknowledged_ordered_cause_digest(&self) -> &str {
        self.acknowledged_ordered_cause_digest.as_ref()
    }

    pub fn acknowledged_prefix_digest(&self) -> &str {
        self.acknowledged_prefix_digest.as_ref()
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }
}
