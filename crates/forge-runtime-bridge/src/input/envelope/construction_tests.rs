use forge_foundational::facade::{AspectKey, AspectLocator, FieldKey, LocatorAuthority};

use crate::error::BridgeRouteErrorKind;
use crate::input::envelope::{
    BridgeCommittedPatchEnvelopeIdentity, BridgeCommittedPatchItem, BridgeCommittedPatchTarget,
    BridgeProducerAuthorityKind, BridgeProducerMetadata, TruthBranchIdentity, TruthPatchIdentity,
};
use crate::snapshot::TruthSnapshotIdentity;

use super::construct_committed_patch_envelope;

#[test]
fn construction_sorts_and_deduplicates_patch_items() {
    let avatar_field = field_key("avatar");
    let name_field = field_key("name");
    let envelope = construct_committed_patch_envelope(
        BridgeCommittedPatchEnvelopeIdentity::new_with_metadata(
            BridgeProducerMetadata::bridge_harness_fixture(),
            crate::facade::TruthCommitIdentity::new("commit"),
            TruthPatchIdentity::new("patch"),
            TruthSnapshotIdentity::new("snapshot"),
            TruthBranchIdentity::new("branch"),
        ),
        vec![
            entity_field_patch_item("user", name_field.clone()),
            entity_field_patch_item("user", avatar_field.clone()),
            entity_field_patch_item("user", name_field.clone()),
        ],
    )
    .expect("valid committed patch envelope should construct");

    assert_eq!(envelope.patch_summary().patch_item_count(), 3);
    assert_eq!(envelope.patch_summary().normalized_patch_item_count(), 2);
    assert_eq!(
        envelope.patch_body().canonical_items()[0].target_canonical_basis(),
        expected_field_target_basis(&avatar_field),
    );
    assert_eq!(
        envelope.patch_body().canonical_items()[1].target_canonical_basis(),
        expected_field_target_basis(&name_field),
    );
}

#[test]
fn construction_digest_is_digest_shaped_not_raw_patch_basis() {
    let envelope = construct_committed_patch_envelope(
        BridgeCommittedPatchEnvelopeIdentity::new_with_metadata(
            BridgeProducerMetadata::bridge_harness_fixture(),
            crate::facade::TruthCommitIdentity::new("commit-a"),
            TruthPatchIdentity::new("patch-a"),
            TruthSnapshotIdentity::new("snapshot-a"),
            TruthBranchIdentity::new("main"),
        ),
        vec![entity_field_patch_item("entity-1", field_key("name"))],
    )
    .expect("valid committed patch envelope should construct");

    let digest = envelope.digest().as_str();
    assert!(
        digest.starts_with("patch:sha256:"),
        "committed patch digest must be digest-shaped: {digest}"
    );
    assert!(
        !digest.contains("committed-patch-target")
            && !digest.contains("entity-1")
            && !digest.contains("profile")
            && !digest.contains("patch|commit="),
        "committed patch digest must not expose raw patch basis: {digest}"
    );
}

#[test]
fn construction_rejects_empty_identity_bearing_fields() {
    let error = construct_committed_patch_envelope(
        BridgeCommittedPatchEnvelopeIdentity::new_with_metadata(
            BridgeProducerMetadata::bridge_harness_fixture(),
            crate::facade::TruthCommitIdentity::new("  "),
            TruthPatchIdentity::new("patch-a"),
            TruthSnapshotIdentity::new("snapshot-a"),
            TruthBranchIdentity::new("main"),
        ),
        vec![entity_field_patch_item("entity-1", field_key("name"))],
    )
    .expect_err("empty canonical identities must be rejected");

    assert_eq!(
        error.kind(),
        BridgeRouteErrorKind::UnsupportedTruthPatchScope
    );
    assert_eq!(error.context().patch_target_coordinate(), None);
}

#[test]
fn construction_rejects_non_authoritative_patch_item_target_locator() {
    let error = construct_committed_patch_envelope(
        BridgeCommittedPatchEnvelopeIdentity::new_with_metadata(
            BridgeProducerMetadata::bridge_harness_fixture(),
            crate::facade::TruthCommitIdentity::new("commit-a"),
            TruthPatchIdentity::new("patch-a"),
            TruthSnapshotIdentity::new("snapshot-a"),
            TruthBranchIdentity::new("main"),
        ),
        vec![BridgeCommittedPatchItem::with_target(
            "entity-1",
            BridgeCommittedPatchTarget::entity_region(AspectLocator::new(
                LocatorAuthority::Planned,
                aspect_key("profile"),
            )),
        )],
    )
    .expect_err("non-authoritative patch target locators must be rejected");

    assert_eq!(
        error.kind(),
        BridgeRouteErrorKind::UnsupportedTruthDeltaSurface
    );
    let patch_target_coordinate = error
        .context()
        .patch_target_coordinate()
        .expect("patch target locator denial should retain native patch target coordinate");
    assert_eq!(patch_target_coordinate.entity_identity(), "entity-1");
    assert_eq!(patch_target_coordinate.aspect_key().as_str(), "profile");
    assert_eq!(
        patch_target_coordinate.aspect_locator().authority(),
        LocatorAuthority::Planned
    );
    assert_eq!(patch_target_coordinate.field_locator(), None);
    assert_eq!(
        patch_target_coordinate.surface_kind(),
        crate::mapping::TruthDeltaSurfaceKind::EntityRegion
    );
    assert_eq!(
        patch_target_coordinate.target().surface_kind(),
        crate::mapping::TruthDeltaSurfaceKind::EntityRegion
    );
    assert!(patch_target_coordinate
        .target()
        .projection_mask()
        .is_whole_aspect());
    assert_eq!(
        patch_target_coordinate.target_canonical_basis(),
        "committed-patch-target|locator=version=bridge.committed-patch-target.v1;domain=locator;entries=[locus=named:aspect.aspect_key,kind=locator,value=exact-text:profile;locus=named:aspect.authority,kind=locator,value=exact-text:planned;locus=named:aspect.kind,kind=locator,value=exact-text:aspect]|mutation-mask=version=bridge.committed-patch-target.v1;domain=aspect-mask;entries=[locus=named:profile.mutation.whole,kind=mask,value=exact-text:whole]|projection-mask=version=bridge.committed-patch-target.v1;domain=aspect-mask;entries=[locus=named:profile.projection.whole,kind=mask,value=exact-text:whole]|kind=entity-region"
    );
}

#[test]
fn construction_rejects_unsupported_producer_schema() {
    let error = construct_committed_patch_envelope(
        BridgeCommittedPatchEnvelopeIdentity::new_with_metadata(
            BridgeProducerMetadata::new(
                BridgeProducerAuthorityKind::BridgeHarnessFixture,
                "forge-runtime-bridge.producer-envelope.v999",
            ),
            crate::facade::TruthCommitIdentity::new("commit-a"),
            TruthPatchIdentity::new("patch-a"),
            TruthSnapshotIdentity::new("snapshot-a"),
            TruthBranchIdentity::new("main"),
        ),
        vec![entity_field_patch_item("entity-1", field_key("name"))],
    )
    .expect_err("unsupported producer schemas must fail construction");

    assert_eq!(
        error.kind(),
        BridgeRouteErrorKind::UnsupportedProducerEnvelope
    );
}

fn entity_field_patch_item(entity: &str, field_key: FieldKey) -> BridgeCommittedPatchItem {
    BridgeCommittedPatchItem::with_target(
        entity,
        BridgeCommittedPatchTarget::entity_field_path(
            AspectLocator::new(LocatorAuthority::Authoritative, aspect_key("profile")),
            forge_foundational::facade::CanonicalFieldPath::single(field_key),
        ),
    )
}

fn aspect_key(value: &str) -> AspectKey {
    AspectKey::new(value).expect("valid bridge patch aspect key")
}

fn field_key(value: &str) -> FieldKey {
    FieldKey::new(value.to_owned()).expect("valid foundational field key")
}

fn expected_field_target_basis(field: &FieldKey) -> String {
    let field = field.as_str();
    format!(
        "committed-patch-target|locator=version=bridge.committed-patch-target.v1;domain=locator;entries=[locus=named:aspect_field.aspect_key,kind=locator,value=exact-text:profile;locus=named:aspect_field.authority,kind=locator,value=exact-text:authoritative;locus=named:aspect_field.field_path,kind=locator,value=exact-text:{field};locus=named:aspect_field.kind,kind=locator,value=exact-text:aspect]|mutation-mask=version=bridge.committed-patch-target.v1;domain=aspect-mask;entries=[locus=named:profile.mutation.field.{field},kind=mask,value=exact-text:{field}]|projection-mask=version=bridge.committed-patch-target.v1;domain=aspect-mask;entries=[locus=named:profile.projection.field.{field},kind=mask,value=exact-text:{field}]|kind=entity-field"
    )
}
