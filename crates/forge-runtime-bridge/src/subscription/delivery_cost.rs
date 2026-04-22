use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{BridgeSubscriptionCounters, BridgeSubscriptionDeliveryCostProfileIdentity};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeSubscriptionDeliveryDensityPosture {
    SparseMemberDelivery,
    BoundedCoalescedWindow,
    DenseRestartRequired,
    RejectedOverBudget,
}

impl BridgeSubscriptionDeliveryDensityPosture {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SparseMemberDelivery => "sparse_member_delivery",
            Self::BoundedCoalescedWindow => "bounded_coalesced_window",
            Self::DenseRestartRequired => "dense_restart_required",
            Self::RejectedOverBudget => "rejected_over_budget",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSubscriptionDeliveryCostProfileRejectionKind {
    EmptyMemberBudget,
    EmptyFanoutBudget,
    CoalescedWidthExceedsMemberBudget,
    OverBudgetPostureRejected,
}

impl BridgeSubscriptionDeliveryCostProfileRejectionKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyMemberBudget => "empty_member_budget",
            Self::EmptyFanoutBudget => "empty_fanout_budget",
            Self::CoalescedWidthExceedsMemberBudget => "coalesced_width_exceeds_member_budget",
            Self::OverBudgetPostureRejected => "over_budget_posture_rejected",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionDeliveryCostProfileRejection {
    rejection_kind: BridgeSubscriptionDeliveryCostProfileRejectionKind,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionDeliveryCostProfileRejection {
    fn new(rejection_kind: BridgeSubscriptionDeliveryCostProfileRejectionKind) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-delivery-cost-profile-rejection|kind={}",
            rejection_kind.as_str(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            rejection_kind,
            counters: BridgeSubscriptionCounters::from_delivery_cost_profile_rejection(matches!(
                rejection_kind,
                BridgeSubscriptionDeliveryCostProfileRejectionKind::OverBudgetPostureRejected
            )),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-delivery-cost-profile-rejection:sha256:{digest:x}"
            )),
        }
    }

    pub fn rejection_kind(&self) -> BridgeSubscriptionDeliveryCostProfileRejectionKind {
        self.rejection_kind
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionDeliveryCostProfile {
    cost_profile_identity: BridgeSubscriptionDeliveryCostProfileIdentity,
    density_posture: BridgeSubscriptionDeliveryDensityPosture,
    max_member_count: usize,
    max_coalesced_member_width: usize,
    max_fanout_width: usize,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionDeliveryCostProfile {
    pub(crate) fn admit(
        density_posture: BridgeSubscriptionDeliveryDensityPosture,
        max_member_count: usize,
        max_coalesced_member_width: usize,
        max_fanout_width: usize,
    ) -> Result<Self, BridgeSubscriptionDeliveryCostProfileRejection> {
        if max_member_count == 0 {
            return Err(BridgeSubscriptionDeliveryCostProfileRejection::new(
                BridgeSubscriptionDeliveryCostProfileRejectionKind::EmptyMemberBudget,
            ));
        }
        if max_fanout_width == 0 {
            return Err(BridgeSubscriptionDeliveryCostProfileRejection::new(
                BridgeSubscriptionDeliveryCostProfileRejectionKind::EmptyFanoutBudget,
            ));
        }
        if max_coalesced_member_width > max_member_count {
            return Err(BridgeSubscriptionDeliveryCostProfileRejection::new(
                BridgeSubscriptionDeliveryCostProfileRejectionKind::CoalescedWidthExceedsMemberBudget,
            ));
        }
        if density_posture == BridgeSubscriptionDeliveryDensityPosture::RejectedOverBudget {
            return Err(BridgeSubscriptionDeliveryCostProfileRejection::new(
                BridgeSubscriptionDeliveryCostProfileRejectionKind::OverBudgetPostureRejected,
            ));
        }

        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-delivery-cost-profile|posture={}|max-members={}|max-coalesced={}|max-fanout={}",
            density_posture.as_str(),
            max_member_count,
            max_coalesced_member_width,
            max_fanout_width,
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Ok(Self {
            cost_profile_identity: BridgeSubscriptionDeliveryCostProfileIdentity::new(format!(
                "bridge-subscription-delivery-cost-profile-id:sha256:{digest:x}"
            )),
            density_posture,
            max_member_count,
            max_coalesced_member_width,
            max_fanout_width,
            counters: BridgeSubscriptionCounters::from_delivery_cost_profile(density_posture),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-delivery-cost-profile:sha256:{digest:x}"
            )),
        })
    }

    pub fn cost_profile_identity(&self) -> &BridgeSubscriptionDeliveryCostProfileIdentity {
        &self.cost_profile_identity
    }

    pub fn density_posture(&self) -> BridgeSubscriptionDeliveryDensityPosture {
        self.density_posture
    }

    pub fn max_member_count(&self) -> usize {
        self.max_member_count
    }

    pub fn max_coalesced_member_width(&self) -> usize {
        self.max_coalesced_member_width
    }

    pub fn max_fanout_width(&self) -> usize {
        self.max_fanout_width
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
