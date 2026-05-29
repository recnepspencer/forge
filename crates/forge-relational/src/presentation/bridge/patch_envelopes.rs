use crate::history::data::CommitId;
use crate::logic::runtime::RelationalReplayRecord;
use crate::publication::bundle::PublicationBundle;
use crate::publication::patch::data::{
    AspectKey, PatchRecord, RecordStructuralChange, RelationalPatchRecord,
};
use crate::replay::data::CanonicalCommitEnvelope;
use forge_runtime_bridge::facade::{
    BridgeCommittedPatchItem, RawCommittedPatchEnvelope, TruthBranchIdentity, TruthCommitIdentity,
    TruthPatchIdentity, TruthSnapshotIdentity,
};

use super::identities::{
    bridge_snapshot_identity_for_commit, bridge_snapshot_identity_for_handle, record_ref_identity,
};

pub fn publication_patch_to_bridge_envelope(
    commit_id: CommitId,
    branch_identity: impl Into<String>,
    snapshot_identity: impl Into<String>,
    patch: &RelationalPatchRecord,
) -> RawCommittedPatchEnvelope {
    let snapshot_identity = TruthSnapshotIdentity::new(snapshot_identity.into());
    RawCommittedPatchEnvelope::new(
        TruthCommitIdentity::new(format!("commit-{}", commit_id.0)),
        TruthPatchIdentity::new(format!("patch-{}", patch.position.0)),
        snapshot_identity,
        TruthBranchIdentity::new(branch_identity.into()),
        bridge_patch_items(&patch.canonicalized().records),
    )
}

pub fn publication_bundle_to_bridge_envelope(
    bundle: &PublicationBundle<RelationalReplayRecord>,
) -> RawCommittedPatchEnvelope {
    publication_patch_to_bridge_envelope(
        bundle.commit.commit_id,
        bundle.commit.branch_id.0.clone(),
        bridge_snapshot_identity_for_handle(&bundle.snapshot)
            .as_str()
            .to_string(),
        &bundle.patch,
    )
}

pub fn commit_envelope_to_bridge_envelope(
    envelope: &CanonicalCommitEnvelope,
) -> RawCommittedPatchEnvelope {
    publication_patch_to_bridge_envelope(
        envelope.commit.commit_id,
        envelope.commit.branch_id.0.clone(),
        bridge_snapshot_identity_for_commit(envelope.commit.commit_id, envelope.commit.version_id)
            .as_str()
            .to_string(),
        &envelope.patch,
    )
}

fn bridge_patch_items(records: &[PatchRecord]) -> Vec<BridgeCommittedPatchItem> {
    let mut items = Vec::new();
    for record in records {
        let entity_identity = record_ref_identity(&record.target);
        let changed_aspects = record.authoritative_changed_aspects();
        if changed_aspects.is_empty() {
            items.push(BridgeCommittedPatchItem::new(
                entity_identity.clone(),
                structural_change_label(record),
                "structural",
            ));
            continue;
        }

        for aspect in changed_aspects.iter() {
            items.push(BridgeCommittedPatchItem::new(
                entity_identity.clone(),
                aspect_key_label(aspect),
                structural_change_label(record),
            ));
        }
    }
    items
}

fn aspect_key_label(aspect: &AspectKey) -> String {
    aspect.as_str().to_string()
}

fn structural_change_label(record: &PatchRecord) -> &'static str {
    match record.structural_change {
        RecordStructuralChange::Created => "created",
        RecordStructuralChange::Updated => "updated",
        RecordStructuralChange::Deleted => "deleted",
        RecordStructuralChange::RetainedForAudit => "retained_for_audit",
    }
}
