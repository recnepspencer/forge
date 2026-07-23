use std::path::Path;

use worth_store_offline_verifier::OfflineDurableManifestWalk;
use worth_store_physical_format::{
    DurableFreeSpaceManifestHeader, FreeSpaceBlockReference, PhysicalFreeSpaceMembershipBlock,
    PhysicalRecordFormatDeclaration, RecordFreeSpaceManifestEntry,
};

pub(super) struct DecodedFreeSpaceTree {
    pub(super) header: DurableFreeSpaceManifestHeader,
    pub(super) entries: Vec<RecordFreeSpaceManifestEntry>,
}

pub(super) fn decode_free_space_tree(
    store_root: &Path,
    generation: u64,
    format: PhysicalRecordFormatDeclaration,
    capacity: u16,
) -> DecodedFreeSpaceTree {
    let bytes = std::fs::read(store_root.join(format!(
        "families/records/free-space/free-space-{generation:016x}.manifest"
    )))
    .unwrap();
    let (header, found_format) = DurableFreeSpaceManifestHeader::decode(&bytes, capacity).unwrap();
    assert_eq!(found_format, format);
    let mut entries = Vec::new();
    if let Some(reference) = header.root() {
        walk_free_space_block(store_root, reference, format, capacity, &mut entries);
    }
    assert_eq!(entries.len() as u64, header.entry_count());
    DecodedFreeSpaceTree { header, entries }
}

fn walk_free_space_block(
    store_root: &Path,
    reference: FreeSpaceBlockReference,
    format: PhysicalRecordFormatDeclaration,
    capacity: u16,
    entries: &mut Vec<RecordFreeSpaceManifestEntry>,
) {
    let bytes = std::fs::read(store_root.join(format!(
        "families/records/free-space/free-space-{:016x}-block-{:016x}.manifest",
        reference.generation(),
        reference.block(),
    )))
    .unwrap();
    let (block, found_format) = PhysicalFreeSpaceMembershipBlock::decode(&bytes, capacity).unwrap();
    assert_eq!(found_format, format);
    assert_eq!(
        block.reference(worth_store_physical_format::durable_artifact_checksum(
            &bytes
        )),
        reference
    );
    if let Some(found) = block.entries() {
        entries.extend_from_slice(found);
    } else {
        for child in block.children().unwrap() {
            walk_free_space_block(store_root, *child, format, capacity, entries);
        }
    }
}

pub(super) fn decode_routing_tree(
    store_root: &Path,
    generation: u64,
    format: PhysicalRecordFormatDeclaration,
    capacity: u16,
) -> OfflineDurableManifestWalk {
    let observation =
        worth_store_offline_verifier::walk_current_durable_record_manifest(store_root, format)
            .unwrap();
    assert_eq!(observation.root_generation(), generation);
    assert_eq!(observation.node_capacity(), capacity);
    observation
}
