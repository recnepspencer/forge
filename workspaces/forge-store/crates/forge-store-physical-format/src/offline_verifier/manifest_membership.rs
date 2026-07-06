use super::codec::DecodedOfflineManifestSections;
use crate::{
    OfflineVerifierCounterSnapshot, OfflineVerifierDenial, OfflineVerifierDenialKind,
    PhysicalManifestUniverseBuilder, PhysicalRootManifest, PhysicalSegmentId,
};

pub(crate) fn verify_membership_posture(
    decoded: &DecodedOfflineManifestSections,
    counters: OfflineVerifierCounterSnapshot,
) -> Result<(), OfflineVerifierDenial> {
    for page_slot in &decoded.page_slots {
        if !contains_segment(decoded, page_slot.page_slot().segment_id()) {
            return malformed_membership(counters);
        }
    }
    for extent in &decoded.extents {
        if !contains_segment(decoded, extent.extent().segment_id()) {
            return malformed_membership(counters);
        }
    }
    for free_space in &decoded.free_space {
        if !free_space_segment_is_manifested(decoded, free_space.reuse_cell()) {
            return malformed_membership(counters);
        }
    }
    Ok(())
}

pub(crate) fn build_root_manifest(decoded: &DecodedOfflineManifestSections) -> PhysicalRootManifest {
    let mut builder = PhysicalManifestUniverseBuilder::s1(decoded.root);
    for segment in &decoded.segments {
        builder = builder.segment(segment.segment());
    }
    for page_slot in &decoded.page_slots {
        builder = builder.ordinary_page(page_slot.page_slot());
    }
    for extent in &decoded.extents {
        builder = builder.extent(extent.extent());
    }
    for allocation in &decoded.allocation_classes {
        builder = builder.allocation_class(allocation.allocation_class());
    }
    for free_space in &decoded.free_space {
        builder = builder.free_space_reuse(free_space.reuse_cell());
    }
    builder.publish()
}

fn malformed_membership<T>(
    counters: OfflineVerifierCounterSnapshot,
) -> Result<T, OfflineVerifierDenial> {
    Err(OfflineVerifierDenial::new(
        OfflineVerifierDenialKind::MalformedManifestMembership,
        counters,
    ))
}

fn contains_segment(
    decoded: &DecodedOfflineManifestSections,
    segment_id: PhysicalSegmentId,
) -> bool {
    decoded
        .segments
        .iter()
        .any(|entry| entry.segment().segment_id() == segment_id)
}

fn free_space_segment_is_manifested(
    decoded: &DecodedOfflineManifestSections,
    cell: crate::FreeSpaceReuseCell,
) -> bool {
    match cell.address() {
        crate::FreeSpaceReuseAddress::PageSlot { segment_id, .. }
        | crate::FreeSpaceReuseAddress::Extent { segment_id, .. } => {
            contains_segment(decoded, segment_id)
        }
    }
}