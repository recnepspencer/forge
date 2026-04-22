use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::super::{
    BridgeSubscriptionCounters, BridgeSubscriptionFanoutDeliveryProjectionSetIdentity,
    BridgeSubscriptionFanoutLayoutIdentity, BridgeSubscriptionFanoutProjectionValidationIdentity,
};
use super::{
    BridgeSubscriptionFanoutConsumerBinding, BridgeSubscriptionFanoutDeliveryProjectionSet,
    BridgeSubscriptionFanoutLayout,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSubscriptionFanoutProjectionValidationRejectionKind {
    LayoutIdentityMismatch,
    ActiveSubscriptionMismatch,
    DeliveryFamilyMismatch,
    CostProfileMismatch,
    SharingEligibilityMismatch,
    ConsumerBindingOrderMismatch,
    ProjectionWidthMismatch,
    ProjectionDescriptorMismatch,
    CanonicalMemberDigestMismatch,
}

impl BridgeSubscriptionFanoutProjectionValidationRejectionKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LayoutIdentityMismatch => "layout_identity_mismatch",
            Self::ActiveSubscriptionMismatch => "active_subscription_mismatch",
            Self::DeliveryFamilyMismatch => "delivery_family_mismatch",
            Self::CostProfileMismatch => "cost_profile_mismatch",
            Self::SharingEligibilityMismatch => "sharing_eligibility_mismatch",
            Self::ConsumerBindingOrderMismatch => "consumer_binding_order_mismatch",
            Self::ProjectionWidthMismatch => "projection_width_mismatch",
            Self::ProjectionDescriptorMismatch => "projection_descriptor_mismatch",
            Self::CanonicalMemberDigestMismatch => "canonical_member_digest_mismatch",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionFanoutProjectionValidationRejection {
    rejection_kind: BridgeSubscriptionFanoutProjectionValidationRejectionKind,
    fanout_layout_identity: BridgeSubscriptionFanoutLayoutIdentity,
    projection_set_identity: BridgeSubscriptionFanoutDeliveryProjectionSetIdentity,
    rejected_projection_index: Option<usize>,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionFanoutProjectionValidationRejection {
    fn new(
        rejection_kind: BridgeSubscriptionFanoutProjectionValidationRejectionKind,
        layout: &BridgeSubscriptionFanoutLayout,
        projection_set: &BridgeSubscriptionFanoutDeliveryProjectionSet,
    ) -> Self {
        Self::new_with_projection_index(rejection_kind, layout, projection_set, None)
    }

    fn new_with_projection_index(
        rejection_kind: BridgeSubscriptionFanoutProjectionValidationRejectionKind,
        layout: &BridgeSubscriptionFanoutLayout,
        projection_set: &BridgeSubscriptionFanoutDeliveryProjectionSet,
        rejected_projection_index: Option<usize>,
    ) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-fanout-projection-validation-rejection|kind={}|layout={}|projection-set={}|projection-index={}",
            rejection_kind.as_str(),
            layout.fanout_layout_identity().as_str(),
            projection_set.fanout_delivery_projection_set_identity().as_str(),
            rejected_projection_index
                .map(|index| index.to_string())
                .unwrap_or_else(|| "none".to_owned()),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            rejection_kind,
            fanout_layout_identity: layout.fanout_layout_identity().clone(),
            projection_set_identity: projection_set
                .fanout_delivery_projection_set_identity()
                .clone(),
            rejected_projection_index,
            counters: BridgeSubscriptionCounters::from_fanout_projection_validation_rejection(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-fanout-projection-validation-rejection:sha256:{digest:x}"
            )),
        }
    }

    pub fn rejection_kind(&self) -> BridgeSubscriptionFanoutProjectionValidationRejectionKind {
        self.rejection_kind
    }

    pub fn fanout_layout_identity(&self) -> &BridgeSubscriptionFanoutLayoutIdentity {
        &self.fanout_layout_identity
    }

    pub fn projection_set_identity(
        &self,
    ) -> &BridgeSubscriptionFanoutDeliveryProjectionSetIdentity {
        &self.projection_set_identity
    }

    pub fn rejected_projection_index(&self) -> Option<usize> {
        self.rejected_projection_index
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionFanoutProjectionValidation {
    fanout_projection_validation_identity: BridgeSubscriptionFanoutProjectionValidationIdentity,
    fanout_layout_identity: BridgeSubscriptionFanoutLayoutIdentity,
    projection_set_identity: BridgeSubscriptionFanoutDeliveryProjectionSetIdentity,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionFanoutProjectionValidation {
    pub(crate) fn validate(
        layout: &BridgeSubscriptionFanoutLayout,
        projection_set: &BridgeSubscriptionFanoutDeliveryProjectionSet,
    ) -> Result<Self, BridgeSubscriptionFanoutProjectionValidationRejection> {
        if layout.fanout_layout_identity() != projection_set.fanout_layout_identity() {
            return Err(BridgeSubscriptionFanoutProjectionValidationRejection::new(
                BridgeSubscriptionFanoutProjectionValidationRejectionKind::LayoutIdentityMismatch,
                layout,
                projection_set,
            ));
        }
        if layout.active_subscription_identity() != projection_set.active_subscription_identity() {
            return Err(BridgeSubscriptionFanoutProjectionValidationRejection::new(
                BridgeSubscriptionFanoutProjectionValidationRejectionKind::ActiveSubscriptionMismatch,
                layout,
                projection_set,
            ));
        }
        if layout.cost_profile_identity() != projection_set.cost_profile_identity() {
            return Err(BridgeSubscriptionFanoutProjectionValidationRejection::new(
                BridgeSubscriptionFanoutProjectionValidationRejectionKind::CostProfileMismatch,
                layout,
                projection_set,
            ));
        }
        if layout.sharing_eligibility_digest() != projection_set.sharing_eligibility_digest() {
            return Err(BridgeSubscriptionFanoutProjectionValidationRejection::new(
                BridgeSubscriptionFanoutProjectionValidationRejectionKind::SharingEligibilityMismatch,
                layout,
                projection_set,
            ));
        }
        if layout.delivery_family().delivery_family_identity()
            != projection_set.delivery_family_identity()
        {
            return Err(BridgeSubscriptionFanoutProjectionValidationRejection::new(
                BridgeSubscriptionFanoutProjectionValidationRejectionKind::DeliveryFamilyMismatch,
                layout,
                projection_set,
            ));
        }
        let binding_basis = layout
            .consumer_bindings()
            .iter()
            .map(BridgeSubscriptionFanoutConsumerBinding::digest)
            .collect::<Vec<_>>()
            .join(",");
        if binding_basis != projection_set.consumer_binding_digest_basis() {
            return Err(BridgeSubscriptionFanoutProjectionValidationRejection::new(
                BridgeSubscriptionFanoutProjectionValidationRejectionKind::ConsumerBindingOrderMismatch,
                layout,
                projection_set,
            ));
        }
        if layout.consumer_bindings().len() != projection_set.projections().len() {
            return Err(BridgeSubscriptionFanoutProjectionValidationRejection::new(
                BridgeSubscriptionFanoutProjectionValidationRejectionKind::ProjectionWidthMismatch,
                layout,
                projection_set,
            ));
        }
        for (projection_index, (binding, projection)) in layout
            .consumer_bindings()
            .iter()
            .zip(projection_set.projections().iter())
            .enumerate()
        {
            if projection.fanout_layout_identity() != projection_set.fanout_layout_identity()
                || projection.fanout_consumer_binding_identity()
                    != binding.fanout_consumer_binding_identity()
                || projection.delivery_window_identity()
                    != projection_set.delivery_window_identity()
                || projection.delivery_family_identity()
                    != projection_set.delivery_family_identity()
            {
                return Err(BridgeSubscriptionFanoutProjectionValidationRejection::new_with_projection_index(
                    BridgeSubscriptionFanoutProjectionValidationRejectionKind::ProjectionDescriptorMismatch,
                    layout,
                    projection_set,
                    Some(projection_index),
                ));
            }
            if projection.canonical_member_digest_basis()
                != projection_set.canonical_member_digest_basis()
            {
                return Err(BridgeSubscriptionFanoutProjectionValidationRejection::new_with_projection_index(
                    BridgeSubscriptionFanoutProjectionValidationRejectionKind::CanonicalMemberDigestMismatch,
                    layout,
                    projection_set,
                    Some(projection_index),
                ));
            }
        }
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-fanout-projection-validation|layout={}|projection-set={}|window={}|member-basis={}|bindings={}",
            layout.fanout_layout_identity().as_str(),
            projection_set.fanout_delivery_projection_set_identity().as_str(),
            projection_set.delivery_window_identity().as_str(),
            projection_set.canonical_member_digest_basis(),
            projection_set.consumer_binding_digest_basis(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Ok(Self {
            fanout_projection_validation_identity:
                BridgeSubscriptionFanoutProjectionValidationIdentity::new(format!(
                    "bridge-subscription-fanout-projection-validation-id:sha256:{digest:x}"
                )),
            fanout_layout_identity: layout.fanout_layout_identity().clone(),
            projection_set_identity: projection_set
                .fanout_delivery_projection_set_identity()
                .clone(),
            counters: BridgeSubscriptionCounters::from_fanout_projection_validation(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-fanout-projection-validation:sha256:{digest:x}"
            )),
        })
    }

    pub fn fanout_projection_validation_identity(
        &self,
    ) -> &BridgeSubscriptionFanoutProjectionValidationIdentity {
        &self.fanout_projection_validation_identity
    }

    pub fn fanout_layout_identity(&self) -> &BridgeSubscriptionFanoutLayoutIdentity {
        &self.fanout_layout_identity
    }

    pub fn projection_set_identity(
        &self,
    ) -> &BridgeSubscriptionFanoutDeliveryProjectionSetIdentity {
        &self.projection_set_identity
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
