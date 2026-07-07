use crate::offline_verifier::codec::{
    encode_allocation_class, EXTENT_MAGIC, FREE_MAGIC, ROOT_BODY_LENGTH, ROOT_MAGIC, SEGMENT_MAGIC,
};
use crate::{
    AllocationClassManifestEntry, ExtentManifestEntry, FreeSpaceManifestEntry, FreeSpaceReuseCell,
    PhysicalByteOrder, RootPublicationCell, SegmentManifestEntry, SegmentPageManifestEntry,
};

pub(crate) fn encode_root_manifest(
    byte_order: PhysicalByteOrder,
    root: RootPublicationCell,
) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(ROOT_MAGIC.len() + ROOT_BODY_LENGTH);
    bytes.extend_from_slice(ROOT_MAGIC);
    bytes.extend_from_slice(&byte_order.write_u64(root.root_reference().get()));
    bytes.extend_from_slice(&byte_order.write_u64(root.generation().get()));
    bytes
}

pub(crate) fn encode_segment_manifest(
    byte_order: PhysicalByteOrder,
    segments: &[SegmentManifestEntry],
    page_slots: &[SegmentPageManifestEntry],
) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(SEGMENT_MAGIC);
    for segment in segments {
        bytes.push(0x01);
        bytes.extend_from_slice(&byte_order.write_u64(segment.segment().segment_id().get()));
        bytes.extend_from_slice(&byte_order.write_u64(segment.segment().generation().get()));
    }
    for page_slot in page_slots {
        let cell = page_slot.page_slot();
        bytes.push(0x02);
        bytes.extend_from_slice(&byte_order.write_u64(cell.segment_id().get()));
        bytes.extend_from_slice(&byte_order.write_u64(cell.page_id().get()));
        bytes.extend_from_slice(&byte_order.write_u16(cell.slot().get()));
        bytes.extend_from_slice(&byte_order.write_u64(cell.generation().get()));
    }
    bytes
}

pub(crate) fn encode_extent_manifest(
    byte_order: PhysicalByteOrder,
    extents: &[ExtentManifestEntry],
    allocation_classes: &[AllocationClassManifestEntry],
) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(EXTENT_MAGIC);
    for extent in extents {
        let cell = extent.extent();
        bytes.push(0x01);
        bytes.extend_from_slice(&byte_order.write_u64(cell.segment_id().get()));
        bytes.extend_from_slice(&byte_order.write_u64(cell.extent_id().get()));
        bytes.extend_from_slice(&byte_order.write_u64(cell.generation().get()));
    }
    for allocation_class in allocation_classes {
        bytes.push(0x02);
        bytes.push(encode_allocation_class(allocation_class.allocation_class()));
    }
    bytes
}

pub(crate) fn encode_free_space_map(
    byte_order: PhysicalByteOrder,
    entries: &[FreeSpaceManifestEntry],
) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(FREE_MAGIC);
    for entry in entries {
        encode_free_space_entry(byte_order, entry.reuse_cell(), &mut bytes);
    }
    bytes
}

fn encode_free_space_entry(
    byte_order: PhysicalByteOrder,
    cell: FreeSpaceReuseCell,
    bytes: &mut Vec<u8>,
) {
    bytes.push(match cell.address() {
        crate::FreeSpaceReuseAddress::PageSlot { .. } => 0x01,
        crate::FreeSpaceReuseAddress::Extent { .. } => 0x02,
    });
    bytes.push(encode_allocation_class(cell.allocation_class()));
    match cell.address() {
        crate::FreeSpaceReuseAddress::PageSlot {
            segment_id,
            page_id,
            slot,
        } => {
            bytes.extend_from_slice(&byte_order.write_u64(segment_id.get()));
            bytes.extend_from_slice(&byte_order.write_u64(page_id.get()));
            bytes.extend_from_slice(&byte_order.write_u16(slot.get()));
        }
        crate::FreeSpaceReuseAddress::Extent {
            segment_id,
            extent_id,
        } => {
            bytes.extend_from_slice(&byte_order.write_u64(segment_id.get()));
            bytes.extend_from_slice(&byte_order.write_u64(extent_id.get()));
            bytes.extend_from_slice(&byte_order.write_u16(0));
        }
    }
    bytes.extend_from_slice(&byte_order.write_u64(cell.generation().get()));
}
