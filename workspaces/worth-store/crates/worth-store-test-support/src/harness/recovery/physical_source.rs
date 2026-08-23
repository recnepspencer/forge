use std::num::NonZeroU64;
use std::path::Path;

use worth_store_physical_format::{
    durable_artifact_checksum, inspect_checkpoint_stream, CheckpointBindingCompactionHeader,
    CheckpointRootBasis, CheckpointStreamEncoder, CheckpointWalSourceRange,
    CurrentPhysicalRecordPlacement, DurableInlineRecordPlacement, DurablePhysicalRootManifest,
    DurableRootSelector, FreeSpaceBlockReference, FreeSpaceKey, PersistedRecordIdentity,
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId,
    PhysicalRecordFormatDeclaration, PhysicalRecordSlot, PhysicalRootRoutingBlock,
    PhysicalSegmentId, RootSelectorIdentity, RootSelectorRole,
};
use worth_store_recovery_physics::{
    admit_physical_page_facts, admit_physical_root_slot, admit_physical_wal_tail,
    select_current_previous_root, select_physical_recovery_sources, PhysicalManifestBlockCandidate,
    PhysicalSourceSelection, PhysicalWalSegmentCandidate, SelectedCompactionProduct,
};
use worth_store_wal::{
    inspect_verified_wal_segment, prepare_wal_frame_append, WalSegmentArtifactIdentity,
    WalSegmentGeneration, WalSegmentId,
};

/// Builds the exact current recovery-physics source selection required by the
/// layout replay fixture. The selected root/checkpoint/tail are real typed
/// admissions; the replay artifact itself remains owned by the layout fixture.
pub fn deterministic_checkpoint_plus_tail_source() -> PhysicalSourceSelection {
    let format = PhysicalRecordFormatDeclaration::builder().admit().unwrap();
    let store = stable_store();
    let (manifest_bytes, block_reference, block_bytes) = root_manifest(format);
    let selector = DurableRootSelector::new(
        store,
        format,
        RootSelectorIdentity::new(1).unwrap(),
        RootSelectorRole::Current,
        1,
        None,
        None,
    )
    .unwrap();
    let current = admit_physical_root_slot(
        store,
        RootSelectorRole::Current,
        Some(&selector.encode()),
        Some(&manifest_bytes),
        64,
    );
    let root = select_current_previous_root(
        current,
        worth_store_recovery_physics::PhysicalRootSlotObservation::Absent,
        None,
    )
    .unwrap();
    let page_facts = admit_physical_page_facts(
        root.selected(),
        vec![PhysicalManifestBlockCandidate::new(
            block_reference,
            block_bytes,
        )],
        1,
        1,
    )
    .unwrap();
    let checkpoint = checkpoint_base(format, store);
    let wal_tail = wal_tail(checkpoint.wal_tail_begin_lsn());
    select_physical_recovery_sources(
        root,
        page_facts,
        None,
        Some(checkpoint),
        wal_tail,
        None,
        Vec::new(),
    )
    .unwrap()
}

pub fn deterministic_selected_compaction_product() -> SelectedCompactionProduct {
    let format = PhysicalRecordFormatDeclaration::builder().admit().unwrap();
    let checkpoint = checkpoint_base(format, stable_store());
    SelectedCompactionProduct::admit(&checkpoint)
}

fn root_manifest(
    format: PhysicalRecordFormatDeclaration,
) -> (
    Vec<u8>,
    worth_store_physical_format::ManifestBlockReference,
    Vec<u8>,
) {
    let record = PersistedRecordIdentity::new([1; 16], 1).unwrap();
    let authority = PhysicalGenerationAuthority::for_canonical_physical_format();
    let segment = authority
        .segment_cell(PhysicalSegmentId::from_raw(1).unwrap())
        .with_segment_generation(PhysicalGeneration::from_raw(1).unwrap());
    let page = authority
        .page_cell(
            PhysicalSegmentId::from_raw(1).unwrap(),
            PhysicalPageId::from_raw(1).unwrap(),
        )
        .with_page_generation(PhysicalGeneration::from_raw(1).unwrap());
    let slot = authority
        .slot_cell(
            PhysicalSegmentId::from_raw(1).unwrap(),
            PhysicalPageId::from_raw(1).unwrap(),
            PhysicalRecordSlot::from_raw(1).unwrap(),
        )
        .with_slot_generation(PhysicalGeneration::from_raw(1).unwrap());
    let placement = DurableInlineRecordPlacement::new(record, segment, page, slot, 1, 1).unwrap();
    let block = PhysicalRootRoutingBlock::leaf(
        1,
        1,
        1,
        vec![CurrentPhysicalRecordPlacement::Inline(placement)],
        2,
    )
    .unwrap();
    let block_bytes = block.encode(format);
    let block_reference = block.reference(durable_artifact_checksum(&block_bytes));
    let free_space_key = FreeSpaceKey::new(
        worth_store_physical_format::RecordAllocationClass::InlinePage,
        1,
    )
    .unwrap();
    let free_space = FreeSpaceBlockReference::new(1, 1, 0, 1, free_space_key, free_space_key);
    let manifest = DurablePhysicalRootManifest::builder(1, 1, 2, 1)
        .record_count(1)
        .next_block(2)
        .routing_root(Some(block_reference))
        .free_space_root(free_space)
        .admit()
        .unwrap();
    (manifest.encode(format), block_reference, block_bytes)
}

fn checkpoint_base(
    _format: PhysicalRecordFormatDeclaration,
    store: worth_store_physical_format::store_namespace::StableStoreIdentity,
) -> worth_store_recovery_physics::PhysicalCheckpointBase {
    let identity = worth_store_physical_format::PhysicalCheckpointIdentity::new(
        store,
        NonZeroU64::new(1).unwrap(),
    );
    let source = worth_store_physical_format::PhysicalCheckpointSource::concurrent(
        identity,
        CheckpointWalSourceRange::new(10, 20).unwrap(),
        CheckpointRootBasis::new(1, 1),
        1,
    );
    let (encoder, header) = CheckpointStreamEncoder::begin(source);
    let (encoder, compaction) =
        encoder.begin_binding_compaction(CheckpointBindingCompactionHeader::new(1, 20).unwrap());
    let (_, footer) = encoder.finish();
    let mut bytes = header;
    bytes.extend_from_slice(&compaction);
    bytes.extend_from_slice(&footer);
    let verified = inspect_checkpoint_stream(&bytes, 0, 0).unwrap();
    worth_store_recovery_physics::PhysicalCheckpointBase::admit(
        &root_for_checkpoint(store),
        verified,
    )
    .unwrap()
}

fn root_for_checkpoint(
    store: worth_store_physical_format::store_namespace::StableStoreIdentity,
) -> worth_store_recovery_physics::SelectedPhysicalRoot {
    let format = PhysicalRecordFormatDeclaration::builder().admit().unwrap();
    let (manifest, _, _) = root_manifest(format);
    let selector = DurableRootSelector::new(
        store,
        format,
        RootSelectorIdentity::new(1).unwrap(),
        RootSelectorRole::Current,
        1,
        None,
        None,
    )
    .unwrap();
    select_current_previous_root(
        admit_physical_root_slot(
            store,
            RootSelectorRole::Current,
            Some(&selector.encode()),
            Some(&manifest),
            64,
        ),
        worth_store_recovery_physics::PhysicalRootSlotObservation::Absent,
        None,
    )
    .unwrap()
}

fn wal_tail(frontier: u64) -> worth_store_recovery_physics::SelectedPhysicalWalTail {
    let frame = prepare_wal_frame_append(
        Path::new("recovery-tail-fixture"),
        1,
        1,
        frontier,
        frontier + 10,
        "layout-replay-tail",
        b"payload",
    )
    .unwrap();
    let identity = WalSegmentArtifactIdentity::new(
        WalSegmentId::new(1).unwrap(),
        WalSegmentGeneration::new(1).unwrap(),
    );
    let inspection = inspect_verified_wal_segment(identity, frame.encoded_frame())
        .unwrap()
        .inspection();
    admit_physical_wal_tail(
        frontier,
        vec![PhysicalWalSegmentCandidate::verified(inspection, None)],
    )
    .unwrap()
}

fn stable_store() -> worth_store_physical_format::store_namespace::StableStoreIdentity {
    worth_store_physical_format::store_namespace::StoreNamespaceIdentityRecord::new(
        worth_store_physical_format::store_namespace::StoreNamespaceVersion::CURRENT,
        worth_store_physical_format::store_namespace::ProposedStoreIdentity::from_nonzero_bytes(
            [7; 16],
        )
        .unwrap(),
    )
    .published_identity()
}
