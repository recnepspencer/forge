use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::super::{
    BridgeSubscriptionCounters, BridgeSubscriptionFanoutDeliveryProjectionSetIdentity,
    BridgeSubscriptionFanoutLayoutIdentity, BridgeSubscriptionFanoutProjectionValidationIdentity,
};
use super::{BridgeSubscriptionFanoutDeliveryProjectionSet, BridgeSubscriptionFanoutLayout};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSubscriptionFanoutProjectionValidationRejectionKind {
    LayoutIdentityMismatch,
    ActiveSubscriptionMismatch,
    DeliveryFamilyMismatch,
    CostProfileMismatch,
    SharingEligibilityMismatch,
}

impl BridgeSubscriptionFanoutProjectionValidationRejectionKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LayoutIdentityMismatch => "layout_identity_mismatch",
            Self::ActiveSubscriptionMismatch => "active_subscription_mismatch",
            Self::DeliveryFamilyMismatch => "delivery_family_mismatch",
            Self::CostProfileMismatch => "cost_profile_mismatch",
            Self::SharingEligibilityMismatch => "sharing_eligibility_mismatch",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionFanoutProjectionValidationRejection {
    rejection_kind: BridgeSubscriptionFanoutProjectionValidationRejectionKind,
    fanout_layout_identity: BridgeSubscriptionFanoutLayoutIdentity,
    projection_set_identity: BridgeSubscriptionFanoutDeliveryProjectionSetIdentity,
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
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-fanout-projection-validation-rejection|kind={}|layout={}|projection-set={}",
            rejection_kind.as_str(),
            layout.fanout_layout_identity().as_str(),
            projection_set.fanout_delivery_projection_set_identity().as_str(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            rejection_kind,
            fanout_layout_identity: layout.fanout_layout_identity().clone(),
            projection_set_identity: projection_set
                .fanout_delivery_projection_set_identity()
                .clone(),
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
