use forge_foundational::facade::{AspectKey, FieldKey};

use crate::input::envelope::{
    BridgeCommittedPatchEnvelope, BridgeCommittedPatchItem, BridgeProducerMetadata,
    TruthBranchIdentity, TruthPatchIdentity,
};
use crate::mapping::TruthDeltaSurfaceKind;
use crate::snapshot::TruthSnapshotIdentity;

use super::{
    derive_normalized_truth_delta_surface_set, truth_delta_surface_count,
    truth_delta_surface_target_mask_identity,
};

fn envelope(items: Vec<BridgeCommittedPatchItem>) -> BridgeCommittedPatchEnvelope {
    BridgeCommittedPatchEnvelope::new(
        crate::input::envelope::BridgeCommittedPatchEnvelopeIdentity::new_with_metadata(
            BridgeProducerMetadata::bridge_harness_fixture(),
            crate::facade::TruthCommitIdentity::new("commit-a"),
            TruthPatchIdentity::new("patch-a"),
            TruthSnapshotIdentity::new("snapshot-a"),
            TruthBranchIdentity::new("main"),
        ),
        items,
    )
    .expect("fixture envelopes should validate")
}

#[test]
fn derives_default_field_surface_without_prefix() {
    let profile_aspect = aspect_key("profile");
    let name_field = field_key("name");
    let normalized = derive_normalized_truth_delta_surface_set(&envelope(vec![field_item(
        "user",
        profile_aspect.clone(),
        name_field.clone(),
    )]));

    assert_eq!(
        truth_delta_surface_count(&envelope(vec![field_item(
            "user",
            profile_aspect,
            name_field.clone(),
        )])),
        1
    );
    assert_eq!(normalized.len(), 1);
    let surface = &normalized.surfaces[0];
    assert_eq!(surface.surface_kind(), TruthDeltaSurfaceKind::EntityField);
    assert_eq!(
        surface.native_target_basis(),
        expected_field_target_basis(&name_field),
    );
}

#[test]
fn derives_explicit_whole_aspect_surface_matrix() {
    let cases = [
        (
            crate::facade::BridgeCommittedPatchTarget::entity_relation_endpoint(aspect_locator(
                aspect_key("profile"),
            )),
            TruthDeltaSurfaceKind::EntityRelationEndpoint,
            "entity-relation-endpoint",
        ),
        (
            crate::facade::BridgeCommittedPatchTarget::entity_region(aspect_locator(aspect_key(
                "profile",
            ))),
            TruthDeltaSurfaceKind::EntityRegion,
            "entity-region",
        ),
        (
            crate::facade::BridgeCommittedPatchTarget::entity_partition(aspect_locator(
                aspect_key("profile"),
            )),
            TruthDeltaSurfaceKind::EntityPartition,
            "entity-partition",
        ),
        (
            crate::facade::BridgeCommittedPatchTarget::entity_facet(aspect_locator(aspect_key(
                "profile",
            ))),
            TruthDeltaSurfaceKind::EntityFacet,
            "entity-facet",
        ),
    ];

    for (target, expected_kind, expected_label) in cases {
        let normalized = derive_normalized_truth_delta_surface_set(&envelope(vec![
            BridgeCommittedPatchItem::with_target("user", target),
        ]));

        let surface = &normalized.surfaces[0];
        assert_eq!(surface.surface_kind(), expected_kind);
        assert_eq!(
            surface.native_target_basis(),
            expected_whole_target_basis(expected_label),
        );
        assert_digest_shaped_surface_identity(surface.surface_identity.as_str());
        assert!(!surface.surface_identity.as_str().contains(expected_label));
    }
}

#[test]
fn foundational_field_keys_are_plain_field_targets_not_route_prefixes() {
    let field_name = field_key("field_name");
    let normalized = derive_normalized_truth_delta_surface_set(&envelope(vec![field_item(
        "user",
        aspect_key("profile"),
        field_name.clone(),
    )]));

    assert_eq!(
        normalized.surfaces[0].surface_kind(),
        TruthDeltaSurfaceKind::EntityField
    );
    assert_eq!(
        normalized.surfaces[0].native_target_basis(),
        expected_field_target_basis(&field_name),
    );
}

#[test]
fn normalization_uses_committed_patch_target_not_aspect_registration() {
    let normalized = derive_normalized_truth_delta_surface_set(&envelope(vec![
        BridgeCommittedPatchItem::with_target(
            "user",
            crate::facade::BridgeCommittedPatchTarget::entity_region(aspect_locator(aspect_key(
                "profile",
            ))),
        ),
    ]));

    assert_eq!(
        normalized.surfaces[0].surface_kind(),
        TruthDeltaSurfaceKind::EntityRegion
    );
    assert_eq!(
        normalized.surfaces[0].native_target_basis(),
        expected_whole_target_basis("entity-region"),
    );
}

#[test]
fn deduplicates_repeated_normalized_surfaces() {
    let profile_aspect = aspect_key("profile");
    let name_field = field_key("name");
    let normalized = derive_normalized_truth_delta_surface_set(&envelope(vec![
        field_item("user", profile_aspect.clone(), name_field.clone()),
        field_item("user", profile_aspect, name_field),
    ]));

    assert_eq!(normalized.len(), 1);
    assert_digest_shaped_surface_identity(normalized.surfaces[0].surface_identity.as_str());
}

#[test]
fn surface_identity_consumes_target_mask_proof_not_native_target_basis_text() {
    let profile_aspect = aspect_key("profile");
    let name_field = field_key("name");
    let email_field = field_key("email");
    let name_surface = derive_normalized_truth_delta_surface_set(&envelope(vec![field_item(
        "user",
        profile_aspect.clone(),
        name_field,
    )]))
    .surfaces[0]
        .clone();
    let email_surface = derive_normalized_truth_delta_surface_set(&envelope(vec![field_item(
        "user",
        profile_aspect,
        email_field,
    )]))
    .surfaces[0]
        .clone();

    assert_digest_shaped_surface_identity(name_surface.surface_identity.as_str());
    let target_mask_identity = truth_delta_surface_target_mask_identity(name_surface.target());
    assert_digest_shaped_target_mask_identity(target_mask_identity.as_str());
    assert_ne!(
        name_surface.surface_identity.as_str(),
        email_surface.surface_identity.as_str()
    );
    assert!(!name_surface
        .surface_identity
        .as_str()
        .contains(name_surface.native_target_basis()));
    assert!(!name_surface
        .surface_identity
        .as_str()
        .contains(target_mask_identity.as_str()));
    assert!(!target_mask_identity
        .as_str()
        .contains("committed-patch-target"));
}

fn expected_field_target_basis(field: &FieldKey) -> String {
    let field = field.as_str();
    format!(
        "committed-patch-target|locator=version=bridge.committed-patch-target.v1;domain=locator;entries=[locus=named:aspect_field.aspect_key,kind=locator,value=exact-text:profile;locus=named:aspect_field.authority,kind=locator,value=exact-text:authoritative;locus=named:aspect_field.field_path,kind=locator,value=exact-text:{field};locus=named:aspect_field.kind,kind=locator,value=exact-text:aspect]|mutation-mask=version=bridge.committed-patch-target.v1;domain=aspect-mask;entries=[locus=named:profile.mutation.field.{field},kind=mask,value=exact-text:{field}]|projection-mask=version=bridge.committed-patch-target.v1;domain=aspect-mask;entries=[locus=named:profile.projection.field.{field},kind=mask,value=exact-text:{field}]|kind=entity-field"
    )
}

fn expected_whole_target_basis(kind: &str) -> String {
    format!(
        "committed-patch-target|locator=version=bridge.committed-patch-target.v1;domain=locator;entries=[locus=named:aspect.aspect_key,kind=locator,value=exact-text:profile;locus=named:aspect.authority,kind=locator,value=exact-text:authoritative;locus=named:aspect.kind,kind=locator,value=exact-text:aspect]|mutation-mask=version=bridge.committed-patch-target.v1;domain=aspect-mask;entries=[locus=named:profile.mutation.whole,kind=mask,value=exact-text:whole]|projection-mask=version=bridge.committed-patch-target.v1;domain=aspect-mask;entries=[locus=named:profile.projection.whole,kind=mask,value=exact-text:whole]|kind={kind}"
    )
}

fn assert_digest_shaped_surface_identity(identity: &str) {
    assert!(identity.starts_with("truth-delta-surface:sha256:"));
    assert_eq!(identity.len(), "truth-delta-surface:sha256:".len() + 64);
}

fn assert_digest_shaped_target_mask_identity(identity: &str) {
    assert!(identity.starts_with("truth-delta-surface-target-mask:sha256:"));
    assert_eq!(
        identity.len(),
        "truth-delta-surface-target-mask:sha256:".len() + 64
    );
}

fn field_item(entity: &str, aspect: AspectKey, field: FieldKey) -> BridgeCommittedPatchItem {
    BridgeCommittedPatchItem::with_target(
        entity,
        crate::facade::BridgeCommittedPatchTarget::entity_field_path(
            aspect_locator(aspect),
            forge_foundational::facade::CanonicalFieldPath::single(field),
        ),
    )
}

fn aspect_locator(aspect_key: AspectKey) -> forge_foundational::facade::AspectLocator {
    forge_foundational::facade::AspectLocator::new(
        forge_foundational::facade::LocatorAuthority::Authoritative,
        aspect_key,
    )
}

fn aspect_key(value: &str) -> AspectKey {
    AspectKey::new(value).expect("valid bridge patch aspect key")
}

fn field_key(value: &str) -> FieldKey {
    FieldKey::new(value.to_owned()).expect("valid foundational field key")
}
