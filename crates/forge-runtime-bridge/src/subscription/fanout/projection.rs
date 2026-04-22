use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::super::{
    BridgeActiveSubscriptionIdentity, BridgeSubscriptionCounters,
    BridgeSubscriptionDeliveryCostProfileIdentity, BridgeSubscriptionDeliveryFamilyIdentity,
    BridgeSubscriptionDeliveryWindowIdentity, BridgeSubscriptionDeliveryWindowSealed,
    BridgeSubscriptionFanoutConsumerBindingIdentity,
    BridgeSubscriptionFanoutDeliveryProjectionIdentity,
    BridgeSubscriptionFanoutDeliveryProjectionSetIdentity, BridgeSubscriptionFanoutLayoutIdentity,
};
use super::{BridgeSubscriptionFanoutConsumerBinding, BridgeSubscriptionFanoutLayout};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSubscriptionFanoutProjectionRejectionKind {
    ActiveSubscriptionMismatch,
    DeliveryFamilyMismatch,
}

impl BridgeSubscriptionFanoutProjectionRejectionKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActiveSubscriptionMismatch => "active_subscription_mismatch",
            Self::DeliveryFamilyMismatch => "delivery_family_mismatch",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionFanoutProjectionRejection {
    rejection_kind: BridgeSubscriptionFanoutProjectionRejectionKind,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionFanoutProjectionRejection {
    fn new(rejection_kind: BridgeSubscriptionFanoutProjectionRejectionKind) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-fanout-projection-rejection|kind={}",
            rejection_kind.as_str()
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            rejection_kind,
            counters: BridgeSubscriptionCounters::from_fanout_delivery_projection_rejection(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-fanout-projection-rejection:sha256:{digest:x}"
            )),
        }
    }

    pub fn rejection_kind(&self) -> BridgeSubscriptionFanoutProjectionRejectionKind {
        self.rejection_kind
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionFanoutDeliveryProjection {
    fanout_delivery_projection_identity: BridgeSubscriptionFanoutDeliveryProjectionIdentity,
    fanout_layout_identity: BridgeSubscriptionFanoutLayoutIdentity,
    fanout_consumer_binding_identity: BridgeSubscriptionFanoutConsumerBindingIdentity,
    delivery_window_identity: BridgeSubscriptionDeliveryWindowIdentity,
    delivery_family_identity: BridgeSubscriptionDeliveryFamilyIdentity,
    canonical_member_count: usize,
    canonical_member_digest_basis: Arc<str>,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionFanoutDeliveryProjection {
    fn new(
        layout: &BridgeSubscriptionFanoutLayout,
        binding: &BridgeSubscriptionFanoutConsumerBinding,
        sealed_window: &BridgeSubscriptionDeliveryWindowSealed,
        canonical_member_digest_basis: Arc<str>,
    ) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-fanout-delivery-projection|layout={}|binding={}|window={}|family={}|member-count={}|members={}",
            layout.fanout_layout_identity().as_str(),
            binding.fanout_consumer_binding_identity().as_str(),
            sealed_window.delivery_window_identity().as_str(),
            sealed_window.delivery_family().delivery_family_identity().as_str(),
            sealed_window.members().len(),
            canonical_member_digest_basis.as_ref(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            fanout_delivery_projection_identity:
                BridgeSubscriptionFanoutDeliveryProjectionIdentity::new(format!(
                    "bridge-subscription-fanout-delivery-projection-id:sha256:{digest:x}"
                )),
            fanout_layout_identity: layout.fanout_layout_identity().clone(),
            fanout_consumer_binding_identity: binding.fanout_consumer_binding_identity().clone(),
            delivery_window_identity: sealed_window.delivery_window_identity().clone(),
            delivery_family_identity: sealed_window
                .delivery_family()
                .delivery_family_identity()
                .clone(),
            canonical_member_count: sealed_window.members().len(),
            canonical_member_digest_basis,
            counters: BridgeSubscriptionCounters::zero(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-fanout-delivery-projection:sha256:{digest:x}"
            )),
        }
    }

    pub(crate) fn project(
        layout: &BridgeSubscriptionFanoutLayout,
        sealed_window: &BridgeSubscriptionDeliveryWindowSealed,
    ) -> Result<
        BridgeSubscriptionFanoutDeliveryProjectionSet,
        BridgeSubscriptionFanoutProjectionRejection,
    > {
        if layout.active_subscription_identity() != sealed_window.active_subscription_identity() {
            return Err(BridgeSubscriptionFanoutProjectionRejection::new(
                BridgeSubscriptionFanoutProjectionRejectionKind::ActiveSubscriptionMismatch,
            ));
        }
        if layout.delivery_family().delivery_family_identity()
            != sealed_window.delivery_family().delivery_family_identity()
        {
            return Err(BridgeSubscriptionFanoutProjectionRejection::new(
                BridgeSubscriptionFanoutProjectionRejectionKind::DeliveryFamilyMismatch,
            ));
        }
        let canonical_member_digest_basis = Arc::<str>::from(
            sealed_window
                .members()
                .iter()
                .map(|member| member.digest())
                .collect::<Vec<_>>()
                .join(","),
        );
        let projections = layout
            .consumer_bindings()
            .iter()
            .map(|binding| {
                Self::new(
                    layout,
                    binding,
                    sealed_window,
                    canonical_member_digest_basis.clone(),
                )
            })
            .collect::<Vec<_>>()
            .into();
        Ok(BridgeSubscriptionFanoutDeliveryProjectionSet::new(
            layout,
            sealed_window,
            canonical_member_digest_basis,
            projections,
        ))
    }

    pub fn fanout_delivery_projection_identity(
        &self,
    ) -> &BridgeSubscriptionFanoutDeliveryProjectionIdentity {
        &self.fanout_delivery_projection_identity
    }

    pub fn fanout_consumer_binding_identity(
        &self,
    ) -> &BridgeSubscriptionFanoutConsumerBindingIdentity {
        &self.fanout_consumer_binding_identity
    }

    pub fn fanout_layout_identity(&self) -> &BridgeSubscriptionFanoutLayoutIdentity {
        &self.fanout_layout_identity
    }

    pub fn delivery_window_identity(&self) -> &BridgeSubscriptionDeliveryWindowIdentity {
        &self.delivery_window_identity
    }

    pub fn delivery_family_identity(&self) -> &BridgeSubscriptionDeliveryFamilyIdentity {
        &self.delivery_family_identity
    }

    pub fn canonical_member_count(&self) -> usize {
        self.canonical_member_count
    }

    pub fn canonical_member_digest_basis(&self) -> &str {
        self.canonical_member_digest_basis.as_ref()
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionFanoutDeliveryProjectionSet {
    fanout_delivery_projection_set_identity: BridgeSubscriptionFanoutDeliveryProjectionSetIdentity,
    fanout_layout_identity: BridgeSubscriptionFanoutLayoutIdentity,
    active_subscription_identity: BridgeActiveSubscriptionIdentity,
    cost_profile_identity: BridgeSubscriptionDeliveryCostProfileIdentity,
    sharing_eligibility_digest: Arc<str>,
    delivery_window_identity: BridgeSubscriptionDeliveryWindowIdentity,
    delivery_family_identity: BridgeSubscriptionDeliveryFamilyIdentity,
    canonical_member_digest_basis: Arc<str>,
    consumer_binding_digest_basis: Arc<str>,
    projections: Arc<[BridgeSubscriptionFanoutDeliveryProjection]>,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionFanoutDeliveryProjectionSet {
    fn new(
        layout: &BridgeSubscriptionFanoutLayout,
        sealed_window: &BridgeSubscriptionDeliveryWindowSealed,
        canonical_member_digest_basis: Arc<str>,
        projections: Arc<[BridgeSubscriptionFanoutDeliveryProjection]>,
    ) -> Self {
        let projection_basis = projections
            .iter()
            .map(BridgeSubscriptionFanoutDeliveryProjection::digest)
            .collect::<Vec<_>>()
            .join(",");
        let consumer_binding_digest_basis = Arc::<str>::from(
            layout
                .consumer_bindings()
                .iter()
                .map(BridgeSubscriptionFanoutConsumerBinding::digest)
                .collect::<Vec<_>>()
                .join(","),
        );
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-fanout-delivery-projection-set|layout={}|active={}|cost-profile={}|sharing={}|window={}|family={}|member-basis={}|bindings={}|projection-count={}|projections={}",
            layout.fanout_layout_identity().as_str(),
            layout.active_subscription_identity().as_str(),
            layout.cost_profile_identity().as_str(),
            layout.sharing_eligibility_digest(),
            sealed_window.delivery_window_identity().as_str(),
            sealed_window.delivery_family().delivery_family_identity().as_str(),
            canonical_member_digest_basis.as_ref(),
            consumer_binding_digest_basis.as_ref(),
            projections.len(),
            projection_basis,
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            fanout_delivery_projection_set_identity:
                BridgeSubscriptionFanoutDeliveryProjectionSetIdentity::new(format!(
                    "bridge-subscription-fanout-delivery-projection-set-id:sha256:{digest:x}"
                )),
            fanout_layout_identity: layout.fanout_layout_identity().clone(),
            active_subscription_identity: layout.active_subscription_identity().clone(),
            cost_profile_identity: layout.cost_profile_identity().clone(),
            sharing_eligibility_digest: Arc::from(layout.sharing_eligibility_digest().to_owned()),
            delivery_window_identity: sealed_window.delivery_window_identity().clone(),
            delivery_family_identity: sealed_window
                .delivery_family()
                .delivery_family_identity()
                .clone(),
            canonical_member_digest_basis,
            consumer_binding_digest_basis,
            counters: BridgeSubscriptionCounters::from_fanout_delivery_projection(
                projections.len(),
            ),
            projections,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-fanout-delivery-projection-set:sha256:{digest:x}"
            )),
        }
    }

    pub fn fanout_delivery_projection_set_identity(
        &self,
    ) -> &BridgeSubscriptionFanoutDeliveryProjectionSetIdentity {
        &self.fanout_delivery_projection_set_identity
    }

    pub fn fanout_layout_identity(&self) -> &BridgeSubscriptionFanoutLayoutIdentity {
        &self.fanout_layout_identity
    }

    pub fn active_subscription_identity(&self) -> &BridgeActiveSubscriptionIdentity {
        &self.active_subscription_identity
    }

    pub fn cost_profile_identity(&self) -> &BridgeSubscriptionDeliveryCostProfileIdentity {
        &self.cost_profile_identity
    }

    pub fn sharing_eligibility_digest(&self) -> &str {
        self.sharing_eligibility_digest.as_ref()
    }

    pub fn delivery_window_identity(&self) -> &BridgeSubscriptionDeliveryWindowIdentity {
        &self.delivery_window_identity
    }

    pub fn delivery_family_identity(&self) -> &BridgeSubscriptionDeliveryFamilyIdentity {
        &self.delivery_family_identity
    }

    pub fn projections(&self) -> &[BridgeSubscriptionFanoutDeliveryProjection] {
        &self.projections
    }

    pub fn len(&self) -> usize {
        self.projections.len()
    }

    pub fn is_empty(&self) -> bool {
        self.projections.is_empty()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, BridgeSubscriptionFanoutDeliveryProjection> {
        self.projections.iter()
    }

    pub fn canonical_member_digest_basis(&self) -> &str {
        self.canonical_member_digest_basis.as_ref()
    }

    pub fn consumer_binding_digest_basis(&self) -> &str {
        self.consumer_binding_digest_basis.as_ref()
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }

    #[cfg(test)]
    pub(crate) fn with_canonical_member_digest_basis_for_test(
        &self,
        canonical_member_digest_basis: impl Into<Arc<str>>,
    ) -> Self {
        let mut tampered = self.clone();
        tampered.canonical_member_digest_basis = canonical_member_digest_basis.into();
        tampered
    }

    #[cfg(test)]
    pub(crate) fn with_consumer_binding_digest_basis_for_test(
        &self,
        consumer_binding_digest_basis: impl Into<Arc<str>>,
    ) -> Self {
        let mut tampered = self.clone();
        tampered.consumer_binding_digest_basis = consumer_binding_digest_basis.into();
        tampered
    }
}

impl std::ops::Deref for BridgeSubscriptionFanoutDeliveryProjectionSet {
    type Target = [BridgeSubscriptionFanoutDeliveryProjection];

    fn deref(&self) -> &Self::Target {
        self.projections()
    }
}
