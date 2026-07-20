use worth_foundational::facade::{
    AspectFieldLocator, AspectKey, AspectLocator, CanonicalFieldPath, FieldKey, LocatorAuthority,
};

use super::{subscription_slice_target_identity, subscription_target_mask_identity};
use crate::input::envelope::BridgeCommittedPatchTarget;
use crate::mapping::SubscriptionSliceKind;

#[test]
fn slice_target_identity_consumes_committed_patch_target_proof_not_exported_basis_text() {
    let field_target = BridgeCommittedPatchTarget::entity_field(field_locator("profile", "name"));
    let field_identity = subscription_slice_target_identity(
        "entity-1",
        &field_target,
        &SubscriptionSliceKind::SignalField,
    );
    let region_target = BridgeCommittedPatchTarget::entity_region(aspect_locator("profile"));
    let region_identity = subscription_slice_target_identity(
        "entity-1",
        &region_target,
        &SubscriptionSliceKind::SignalRegion,
    );
    let field_target_mask_identity = subscription_target_mask_identity(&field_target);

    assert!(field_identity
        .as_str()
        .starts_with("subscription-slice-target:sha256:"));
    assert!(field_target_mask_identity
        .as_str()
        .starts_with("subscription-target-mask:sha256:"));
    assert_ne!(field_identity, region_identity);
    assert!(!field_identity.as_str().contains("committed-patch-target"));
    assert!(!field_identity
        .as_str()
        .contains(field_target_mask_identity.as_str()));
    assert!(!field_identity
        .as_str()
        .contains(field_target.canonical_basis().as_str()));
    assert!(field_target.canonical_basis().contains("projection-mask="));
}

fn aspect_key(value: &str) -> AspectKey {
    AspectKey::new(value).expect("valid subscription slice aspect key")
}

fn aspect_locator(value: &str) -> AspectLocator {
    AspectLocator::new(LocatorAuthority::Authoritative, aspect_key(value))
}

fn field_key(value: &str) -> FieldKey {
    FieldKey::new(value.to_owned()).expect("valid subscription slice field key")
}

fn field_locator(aspect: &str, field: &str) -> AspectFieldLocator {
    AspectFieldLocator::from_aspect(
        aspect_locator(aspect),
        CanonicalFieldPath::single(field_key(field)),
    )
}
