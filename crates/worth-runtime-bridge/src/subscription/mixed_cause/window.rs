use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::subscription::{
    BridgeSubscriptionCounters, BridgeSubscriptionDeliveryFamily,
    BridgeSubscriptionDeliveryFamilyKind, BridgeSubscriptionMixedCauseDeliveryWindowIdentity,
};

use super::ordering::{BridgeMixedCauseOrdering, BridgeOrderedMixedCause};
use super::request::BridgeMixedCauseOrderingLaneKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeMixedCauseDeliveryWindowRejectionKind {
    NoOrderedCauses,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeMixedCauseDeliveryWindowRejection {
    kind: BridgeMixedCauseDeliveryWindowRejectionKind,
    detail: Arc<str>,
}

impl BridgeMixedCauseDeliveryWindowRejection {
    fn new(kind: BridgeMixedCauseDeliveryWindowRejectionKind, detail: impl Into<Arc<str>>) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub fn kind(&self) -> BridgeMixedCauseDeliveryWindowRejectionKind {
        self.kind
    }
    pub fn detail(&self) -> &str {
        self.detail.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeMixedCauseDeliveryWindowPlan {
    delivery_window_identity: BridgeSubscriptionMixedCauseDeliveryWindowIdentity,
    ordering_identity: Arc<str>,
    lane_kind: BridgeMixedCauseOrderingLaneKind,
    delivery_family: BridgeSubscriptionDeliveryFamily,
    ordered_causes: Vec<BridgeOrderedMixedCause>,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeMixedCauseDeliveryWindowPlan {
    pub(crate) fn plan(
        ordering: &BridgeMixedCauseOrdering,
        delivery_family_kind: BridgeSubscriptionDeliveryFamilyKind,
    ) -> Result<Self, BridgeMixedCauseDeliveryWindowRejection> {
        if ordering.ordered().is_empty() {
            return Err(BridgeMixedCauseDeliveryWindowRejection::new(
                BridgeMixedCauseDeliveryWindowRejectionKind::NoOrderedCauses,
                "mixed-cause delivery planning requires at least one ordered cause",
            ));
        }
        let delivery_family = BridgeSubscriptionDeliveryFamily::select(delivery_family_kind);
        let ordered_causes = ordering.ordered().to_vec();
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-mixed-cause-delivery-window|ordering={}|delivery-family={}|ordered={}",
            ordering.ordering_identity().as_str(),
            delivery_family.delivery_family_identity().as_str(),
            ordered_causes
                .iter()
                .map(BridgeOrderedMixedCause::digest)
                .collect::<Vec<_>>()
                .join(","),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Ok(Self {
            delivery_window_identity:
                BridgeSubscriptionMixedCauseDeliveryWindowIdentity::admit_bridge_owned(format!(
                    "bridge-mixed-cause-delivery-window-id:sha256:{digest:x}"
                )),
            ordering_identity: Arc::from(ordering.ordering_identity().as_str().to_owned()),
            lane_kind: ordering.lane_kind(),
            delivery_family,
            ordered_causes,
            counters: BridgeSubscriptionCounters::from_mixed_cause_delivery_window_plan(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-mixed-cause-delivery-window:sha256:{digest:x}"
            )),
        })
    }

    pub fn delivery_window_identity(&self) -> &BridgeSubscriptionMixedCauseDeliveryWindowIdentity {
        &self.delivery_window_identity
    }

    pub fn delivery_family(&self) -> &BridgeSubscriptionDeliveryFamily {
        &self.delivery_family
    }
    pub fn lane_kind(&self) -> BridgeMixedCauseOrderingLaneKind {
        self.lane_kind
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
