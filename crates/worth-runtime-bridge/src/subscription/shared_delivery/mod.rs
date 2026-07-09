mod acknowledgement;
mod bundle;
mod layout;
mod plan;
mod projection;

use crate::subscription::{BridgeSubscriptionDeliveryFamily, BridgeSubscriptionDeliveryFamilyKind};

pub(super) fn canonical_bundle_family_token(delivery_family_identity: &str) -> &str {
    let canonical_member = BridgeSubscriptionDeliveryFamily::select(
        BridgeSubscriptionDeliveryFamilyKind::CanonicalMember,
    );
    let admitted_coalesced = BridgeSubscriptionDeliveryFamily::select(
        BridgeSubscriptionDeliveryFamilyKind::AdmittedCoalesced,
    );

    if delivery_family_identity == canonical_member.delivery_family_identity().as_str()
        || delivery_family_identity == admitted_coalesced.delivery_family_identity().as_str()
    {
        "publishable_canonical_bundle"
    } else {
        delivery_family_identity
    }
}

pub use acknowledgement::{
    BridgeSharedDeliveryAcknowledgementFrontier,
    BridgeSharedDeliveryAcknowledgementFrontierRejection,
    BridgeSharedDeliveryAcknowledgementFrontierRejectionKind,
};
pub use bundle::{
    BridgeSharedConsumerDeliveryBundleDraft, BridgeSharedConsumerDeliveryBundleSealed,
};
pub use layout::BridgeSharedConsumerDeliveryLayout;
pub use plan::{
    BridgeSharedConsumerDeliveryPlan, BridgeSharedConsumerDeliveryPlanRejection,
    BridgeSharedConsumerDeliveryPlanRejectionKind,
};
pub use projection::{
    BridgeSharedConsumerDeliveryProjection, BridgeSharedConsumerDeliveryProjectionPosture,
    BridgeSharedConsumerDeliveryProjectionRejection,
    BridgeSharedConsumerDeliveryProjectionRejectionKind,
};
