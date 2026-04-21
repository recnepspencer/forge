use std::sync::Arc;

use forge_runtime_bridge::facade::{
    BridgeSubscriptionDeclaration, BridgeSubscriptionDeclarationFamilyKind,
    BridgeSubscriptionDeclarationIdentity, BridgeSubscriptionDeliveryIntentClass,
    NormalizedSubscriptionSliceIntent,
};

fn main() {
    let _ = BridgeSubscriptionDeclaration {
        declaration_identity: BridgeSubscriptionDeclarationIdentity::new("subscription:literal"),
        requested_family_kind: BridgeSubscriptionDeclarationFamilyKind::DetailExact,
        delivery_intent_class: BridgeSubscriptionDeliveryIntentClass::None,
        normalized_slice_intents: Arc::<[NormalizedSubscriptionSliceIntent]>::from([]),
        canonical_basis: Arc::from("basis"),
        digest: Arc::from("digest"),
    };
}
