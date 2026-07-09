use crate::history::data::{BranchId, CommitId};
use crate::logic::runtime::RelationalReplayRecord;
use crate::publication::bundle::PublicationBundle;
use crate::publication::patch::data::{
    PublishedAuthoritativePatchEnvelope, PublishedAuthoritativeRecordPatch, RecordStructuralChange,
};
use crate::replay::data::CanonicalCommitEnvelope;
use worth_foundational::facade::{AspectKey, AspectLocator, LocatorAuthority};
use worth_runtime_bridge::facade::{
    BridgeCommittedPatchEnvelope, BridgeCommittedPatchEnvelopeIdentity, BridgeCommittedPatchItem,
    BridgeCommittedPatchTarget, TruthBranchIdentity, TruthCommitIdentity, TruthDeltaSurfaceKind,
    TruthPatchIdentity, TruthSnapshotIdentity,
};

use super::identities::{
    bridge_snapshot_identity_for_commit, bridge_snapshot_identity_for_handle, record_ref_identity,
};

pub fn publication_patch_to_bridge_envelope(
    commit_id: CommitId,
    branch_id: &BranchId,
    snapshot_identity: TruthSnapshotIdentity,
    patch: &PublishedAuthoritativePatchEnvelope,
) -> BridgeCommittedPatchEnvelope {
    let envelope_identity = BridgeCommittedPatchEnvelopeIdentity::new(
        TruthCommitIdentity::from_relational_commit_id(commit_id.0),
        TruthPatchIdentity::from_relational_patch_position(patch.position.0),
        snapshot_identity,
        TruthBranchIdentity::from_relational_branch_id(branch_id.0.clone()),
    );
    BridgeCommittedPatchEnvelope::new(
        envelope_identity,
        bridge_patch_items(&patch.canonicalized().authoritative_record_patches),
    )
    .expect("relational publication must export native bridge committed patch targets")
}

pub fn publication_bundle_to_bridge_envelope(
    bundle: &PublicationBundle<RelationalReplayRecord>,
) -> BridgeCommittedPatchEnvelope {
    publication_patch_to_bridge_envelope(
        bundle.commit.commit_id,
        &bundle.commit.branch_id,
        bridge_snapshot_identity_for_handle(&bundle.snapshot),
        &bundle.patch,
    )
}

pub fn commit_envelope_to_bridge_envelope(
    envelope: &CanonicalCommitEnvelope,
) -> BridgeCommittedPatchEnvelope {
    publication_patch_to_bridge_envelope(
        envelope.commit.commit_id,
        &envelope.commit.branch_id,
        bridge_snapshot_identity_for_commit(envelope.commit.commit_id, envelope.commit.version_id),
        &envelope.patch,
    )
}

fn bridge_patch_items(
    authoritative_record_patches: &[PublishedAuthoritativeRecordPatch],
) -> Vec<BridgeCommittedPatchItem> {
    let mut items = Vec::new();
    for record in authoritative_record_patches {
        let record_identity = record_ref_identity(&record.target);
        let changed_aspects = record.authoritative_changed_aspects();
        if changed_aspects.is_empty() {
            items.push(BridgeCommittedPatchItem::with_relational_record_target(
                record_identity,
                BridgeCommittedPatchTarget::entity_facet(authoritative_aspect_locator(
                    lifecycle_aspect_key(),
                )),
            ));
            continue;
        }

        for aspect in changed_aspects.iter() {
            items.push(BridgeCommittedPatchItem::with_relational_record_target(
                record_identity,
                bridge_whole_aspect_target(
                    authoritative_aspect_locator(aspect.clone()),
                    structural_change_surface_kind(record),
                ),
            ));
        }
    }
    items
}

fn authoritative_aspect_locator(aspect_key: AspectKey) -> AspectLocator {
    AspectLocator::new(LocatorAuthority::Authoritative, aspect_key)
}

fn bridge_whole_aspect_target(
    aspect_locator: AspectLocator,
    surface_kind: TruthDeltaSurfaceKind,
) -> BridgeCommittedPatchTarget {
    match surface_kind {
        TruthDeltaSurfaceKind::EntityRelationEndpoint => {
            BridgeCommittedPatchTarget::entity_relation_endpoint(aspect_locator)
        }
        TruthDeltaSurfaceKind::EntityRegion => {
            BridgeCommittedPatchTarget::entity_region(aspect_locator)
        }
        TruthDeltaSurfaceKind::EntityPartition => {
            BridgeCommittedPatchTarget::entity_partition(aspect_locator)
        }
        TruthDeltaSurfaceKind::EntityFacet => {
            BridgeCommittedPatchTarget::entity_facet(aspect_locator)
        }
        TruthDeltaSurfaceKind::EntityField => {
            panic!("relational structural publication cannot emit field targets")
        }
    }
}

fn lifecycle_aspect_key() -> AspectKey {
    AspectKey::new("lifecycle").expect("lifecycle is a valid bridge aspect key")
}

fn structural_change_surface_kind(
    record: &PublishedAuthoritativeRecordPatch,
) -> TruthDeltaSurfaceKind {
    match record.structural_change {
        RecordStructuralChange::Created
        | RecordStructuralChange::Updated
        | RecordStructuralChange::Deleted => TruthDeltaSurfaceKind::EntityRegion,
        RecordStructuralChange::RetainedForAudit => TruthDeltaSurfaceKind::EntityFacet,
    }
}
