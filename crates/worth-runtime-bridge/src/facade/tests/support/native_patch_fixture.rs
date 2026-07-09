use crate::input::envelope::{
    BridgeCommittedPatchEnvelope, BridgeCommittedPatchEnvelopeIdentity, BridgeCommittedPatchItem,
    BridgeProducerMetadata, TruthBranchIdentity, TruthCommitIdentity, TruthPatchIdentity,
};
use crate::snapshot::TruthSnapshotIdentity;
use worth_foundational::facade::{AspectKey, AspectLocator, CanonicalFieldPath, FieldKey};

pub(in crate::facade::tests) fn canonical_envelope(
    branch: TruthBranchIdentity,
    commit: TruthCommitIdentity,
    patch: TruthPatchIdentity,
    snapshot: TruthSnapshotIdentity,
) -> BridgeCommittedPatchEnvelope {
    BridgeCommittedPatchEnvelope::new(
        BridgeCommittedPatchEnvelopeIdentity::new_with_metadata(
            BridgeProducerMetadata::bridge_harness_fixture(),
            commit,
            patch,
            snapshot,
            branch,
        ),
        vec![native_profile_name_patch_item()],
    )
    .expect("fixture envelopes should validate")
}

pub(in crate::facade::tests) fn native_profile_name_patch_item() -> BridgeCommittedPatchItem {
    BridgeCommittedPatchItem::with_target(
        "entity-1",
        crate::facade::BridgeCommittedPatchTarget::entity_field_path(
            AspectLocator::new(
                worth_foundational::facade::LocatorAuthority::Authoritative,
                AspectKey::new("profile").expect("valid bridge patch aspect key"),
            ),
            CanonicalFieldPath::single(
                FieldKey::new("name".to_owned()).expect("valid foundational field key"),
            ),
        ),
    )
}
