use crate::facade::history::CommitId;
use crate::facade::identity::{EntityId, PartitionId};
use crate::facade::publication::{
    PatchOrdering, PatchPublicationMode, PatchStreamPosition, PublishedAuthoritativePatchEnvelope,
    PublishedAuthoritativeRecordPatch, RecordStructuralChange,
};
use crate::publication::patch::data::{
    PatchDetail, PublishedAuthoritativePatch, PublishedAuthoritativePatchOperation,
};
use forge_foundational::facade::{AspectKey, AspectValue, ScalarAspectType};
use forge_runtime_bridge::facade::{
    CommittedPatchSource, SnapshotReadContract, SnapshotReadRecord, SnapshotReadRequest,
    SnapshotReadSource, TruthCommitIdentity, TruthSnapshotIdentity,
};

use super::{PublicationBridgeCatalog, PublicationBridgeSnapshot};

#[test]
fn publication_bridge_catalog_exposes_committed_patch_and_snapshot() {
    let catalog = PublicationBridgeCatalog::default();
    catalog.register_patch(
        CommitId(7),
        "main",
        "snapshot-a",
        &PublishedAuthoritativePatchEnvelope {
            ordering: PatchOrdering::CanonicalCommitOrder,
            publication_mode: PatchPublicationMode::CommitNative,
            position: PatchStreamPosition(11),
            authoritative_record_patches: vec![PublishedAuthoritativeRecordPatch {
                target: crate::transactions::data::RecordRef::Entity(EntityId::new(
                    PartitionId::main(),
                    4,
                    1,
                )),
                structural_change: RecordStructuralChange::Updated,
                authoritative_patch: PublishedAuthoritativePatch::new(vec![
                    PublishedAuthoritativePatchOperation::WholeAspectClear {
                        aspect_key: AspectKey::new("profile.name").unwrap(),
                    },
                ]),
                contains_opaque_aspect: false,
                detail: PatchDetail::DenseBitset(vec![1]),
            }],
        },
    );
    let snapshot_request = SnapshotReadRequest::for_coarse(
        "entity:0:4:1",
        SnapshotReadContract::scalar(
            AspectKey::new("profile.name").unwrap(),
            ScalarAspectType::String,
        ),
    );
    catalog.register_snapshot(PublicationBridgeSnapshot::new(
        TruthSnapshotIdentity::new("snapshot-a"),
        vec![SnapshotReadRecord::for_request(
            &snapshot_request,
            AspectValue::String("alice".into()),
        )],
    ));

    let envelope = catalog
        .load_committed_patch(
            forge_runtime_bridge::facade::RelationalCommittedPatchRequest::new(
                TruthCommitIdentity::new("commit-7"),
            ),
        )
        .expect("registered publication patch");
    let reader = catalog
        .open_snapshot(&TruthSnapshotIdentity::new("snapshot-a"))
        .expect("registered publication snapshot");

    assert_eq!(envelope.patch_identity().as_str(), "patch-11");
    assert_eq!(
        envelope.patch_body().canonical_items()[0]
            .aspect_key()
            .as_str(),
        "profile.name"
    );
    assert_eq!(reader.snapshot_identity().as_str(), "snapshot-a");
}
