use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::subscription::{
    BridgeOrderedMixedCause, BridgeSubscriptionCounters, BridgeSubscriptionDeliveryFamilyKind,
    BridgeSubscriptionSharedDeliveryProjectionIdentity,
};

use super::BridgeSharedConsumerDeliveryBundleSealed;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSharedConsumerDeliveryProjectionPosture {
    SparseCanonical,
    CoalescedCanonical,
    DescriptorOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSharedConsumerDeliveryProjectionRejectionKind {
    ConsumerProjectionOrdinalOutOfRange,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSharedConsumerDeliveryProjectionRejection {
    rejection_kind: BridgeSharedConsumerDeliveryProjectionRejectionKind,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSharedConsumerDeliveryProjectionRejection {
    fn new(rejection_kind: BridgeSharedConsumerDeliveryProjectionRejectionKind) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-shared-consumer-delivery-projection-rejection|kind={rejection_kind:?}"
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            rejection_kind,
            counters: BridgeSubscriptionCounters::from_shared_delivery_projection_rejection(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-shared-consumer-delivery-projection-rejection:sha256:{digest:x}"
            )),
        }
    }

    pub fn rejection_kind(&self) -> BridgeSharedConsumerDeliveryProjectionRejectionKind {
        self.rejection_kind
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSharedConsumerDeliveryProjection {
    shared_delivery_projection_identity: BridgeSubscriptionSharedDeliveryProjectionIdentity,
    bundle_identity: Arc<str>,
    consumer_projection_ordinal: usize,
    consumer_contract_identity: Arc<str>,
    projection_posture: BridgeSharedConsumerDeliveryProjectionPosture,
    ordered_causes: Arc<[BridgeOrderedMixedCause]>,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSharedConsumerDeliveryProjection {
    pub(crate) fn project(
        bundle: &BridgeSharedConsumerDeliveryBundleSealed,
        consumer_projection_ordinal: usize,
    ) -> Result<Self, BridgeSharedConsumerDeliveryProjectionRejection> {
        let Some(consumer_contract_identity) = bundle
            .consumer_contract_identities()
            .get(consumer_projection_ordinal)
        else {
            return Err(BridgeSharedConsumerDeliveryProjectionRejection::new(
                BridgeSharedConsumerDeliveryProjectionRejectionKind::ConsumerProjectionOrdinalOutOfRange,
            ));
        };
        let projection_posture = match bundle.delivery_family_identity() {
            family
                if family
                    == crate::subscription::BridgeSubscriptionDeliveryFamily::select(
                        BridgeSubscriptionDeliveryFamilyKind::CanonicalMember,
                    )
                    .delivery_family_identity()
                    .as_str() =>
            {
                BridgeSharedConsumerDeliveryProjectionPosture::SparseCanonical
            }
            family
                if family
                    == crate::subscription::BridgeSubscriptionDeliveryFamily::select(
                        BridgeSubscriptionDeliveryFamilyKind::AdmittedCoalesced,
                    )
                    .delivery_family_identity()
                    .as_str() =>
            {
                BridgeSharedConsumerDeliveryProjectionPosture::CoalescedCanonical
            }
            _ => BridgeSharedConsumerDeliveryProjectionPosture::DescriptorOnly,
        };
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-shared-consumer-delivery-projection|bundle={}|ordinal={}|consumer={}|posture={projection_posture:?}|ordered={}",
            bundle.shared_delivery_bundle_sealed_identity().as_str(),
            consumer_projection_ordinal,
            consumer_contract_identity.as_ref(),
            bundle
                .ordered_causes()
                .iter()
                .map(BridgeOrderedMixedCause::digest)
                .collect::<Vec<_>>()
                .join(","),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Ok(Self {
            shared_delivery_projection_identity:
                BridgeSubscriptionSharedDeliveryProjectionIdentity::admit_bridge_owned(format!(
                    "bridge-shared-consumer-delivery-projection-id:sha256:{digest:x}"
                )),
            bundle_identity: Arc::from(
                bundle
                    .shared_delivery_bundle_sealed_identity()
                    .as_str()
                    .to_owned(),
            ),
            consumer_projection_ordinal,
            consumer_contract_identity: Arc::clone(consumer_contract_identity),
            projection_posture,
            ordered_causes: bundle.ordered_causes().to_vec().into(),
            counters: BridgeSubscriptionCounters::from_shared_delivery_projection(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-shared-consumer-delivery-projection:sha256:{digest:x}"
            )),
        })
    }

    pub fn shared_delivery_projection_identity(
        &self,
    ) -> &BridgeSubscriptionSharedDeliveryProjectionIdentity {
        &self.shared_delivery_projection_identity
    }

    pub fn consumer_projection_ordinal(&self) -> usize {
        self.consumer_projection_ordinal
    }

    pub fn consumer_contract_identity(&self) -> &str {
        self.consumer_contract_identity.as_ref()
    }

    pub fn bundle_identity(&self) -> &str {
        self.bundle_identity.as_ref()
    }

    pub fn projection_posture(&self) -> BridgeSharedConsumerDeliveryProjectionPosture {
        self.projection_posture
    }

    pub fn ordered_causes(&self) -> &[BridgeOrderedMixedCause] {
        &self.ordered_causes
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
