use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::subscription::{
    BridgeSubscriptionCounters, BridgeSubscriptionDeliveryContentOmissionReason,
    BridgeSubscriptionDeliveryFamily, BridgeSubscriptionDeliveryFamilyKind,
    BridgeSubscriptionDeliveryMemberClass, BridgeSubscriptionDeliveryMemberInput,
    BridgeSubscriptionTemporalCauseRecordIdentity, BridgeSubscriptionTemporalDeliveryPlanIdentity,
};

use super::BridgeTemporalCauseRecord;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeTemporalDeliveryWindowPlan {
    delivery_plan_identity: BridgeSubscriptionTemporalDeliveryPlanIdentity,
    cause_record_identity: BridgeSubscriptionTemporalCauseRecordIdentity,
    delivery_family: BridgeSubscriptionDeliveryFamily,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeTemporalDeliveryWindowPlan {
    pub(crate) fn plan(
        cause_record: &BridgeTemporalCauseRecord,
        delivery_family_kind: BridgeSubscriptionDeliveryFamilyKind,
    ) -> Self {
        let delivery_family = BridgeSubscriptionDeliveryFamily::select(delivery_family_kind);
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-temporal-delivery-window-plan|cause={}|classification={}|delivery-family={}",
            cause_record.cause_record_identity().as_str(),
            cause_record.classification().as_str(),
            delivery_family.delivery_family_identity().as_str(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            delivery_plan_identity: BridgeSubscriptionTemporalDeliveryPlanIdentity::new(format!(
                "bridge-temporal-delivery-window-plan-id:sha256:{digest:x}"
            )),
            cause_record_identity: cause_record.cause_record_identity().clone(),
            delivery_family,
            counters: BridgeSubscriptionCounters::from_temporal_delivery_plan(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-temporal-delivery-window-plan:sha256:{digest:x}"
            )),
        }
    }

    pub fn delivery_plan_identity(&self) -> &BridgeSubscriptionTemporalDeliveryPlanIdentity {
        &self.delivery_plan_identity
    }

    pub fn cause_record_identity(&self) -> &BridgeSubscriptionTemporalCauseRecordIdentity {
        &self.cause_record_identity
    }

    pub fn delivery_family(&self) -> &BridgeSubscriptionDeliveryFamily {
        &self.delivery_family
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }

    pub fn to_member_input(
        &self,
        member_class: BridgeSubscriptionDeliveryMemberClass,
        omission_reason: BridgeSubscriptionDeliveryContentOmissionReason,
    ) -> BridgeSubscriptionDeliveryMemberInput {
        BridgeSubscriptionDeliveryMemberInput::omitted_content(
            self.cause_record_identity().as_str(),
            self.digest(),
            member_class,
            omission_reason,
        )
    }
}
