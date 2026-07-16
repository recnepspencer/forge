use crate::{
    AllocationClassManifestEntry, ExtentManifestEntry, FreeSpaceManifestEntry,
    OfflineVerifierCounterSnapshot, OfflineVerifierDenial, PhysicalByteOrder, RootPublicationCell,
    SegmentManifestEntry, SegmentPageManifestEntry,
};

pub(crate) const ROOT_MAGIC: &[u8; 4] = b"F9RT";
pub(crate) const SEGMENT_MAGIC: &[u8; 4] = b"F9SG";
pub(crate) const EXTENT_MAGIC: &[u8; 4] = b"F9EX";
pub(crate) const FREE_MAGIC: &[u8; 4] = b"F9FS";
pub(crate) const ROOT_BODY_LENGTH: usize = 16;
pub(crate) const SEGMENT_ROW_LENGTH: usize = 17;
pub(crate) const PAGE_SLOT_ROW_LENGTH: usize = 27;
pub(crate) const EXTENT_ROW_LENGTH: usize = 25;
pub(crate) const ALLOCATION_ROW_LENGTH: usize = 1;
pub(crate) const FREE_ROW_LENGTH: usize = 28;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DecodedOfflineManifestSections {
    pub(crate) root: RootPublicationCell,
    pub(crate) segments: Vec<SegmentManifestEntry>,
    pub(crate) page_slots: Vec<SegmentPageManifestEntry>,
    pub(crate) extents: Vec<ExtentManifestEntry>,
    pub(crate) allocation_classes: Vec<AllocationClassManifestEntry>,
    pub(crate) free_space: Vec<FreeSpaceManifestEntry>,
    pub(crate) decoded_rows: u32,
}

pub struct OfflineManifestCodec;

impl OfflineManifestCodec {
    pub fn encode_root_manifest(
        byte_order: PhysicalByteOrder,
        root: RootPublicationCell,
    ) -> Vec<u8> {
        crate::offline_verifier::codec_encode::encode_root_manifest(byte_order, root)
    }

    pub fn encode_segment_manifest(
        byte_order: PhysicalByteOrder,
        segments: &[SegmentManifestEntry],
        page_slots: &[SegmentPageManifestEntry],
    ) -> Vec<u8> {
        crate::offline_verifier::codec_encode::encode_segment_manifest(
            byte_order, segments, page_slots,
        )
    }

    pub fn encode_extent_manifest(
        byte_order: PhysicalByteOrder,
        extents: &[ExtentManifestEntry],
        allocation_classes: &[AllocationClassManifestEntry],
    ) -> Vec<u8> {
        crate::offline_verifier::codec_encode::encode_extent_manifest(
            byte_order,
            extents,
            allocation_classes,
        )
    }

    pub fn encode_free_space_map(
        byte_order: PhysicalByteOrder,
        entries: &[FreeSpaceManifestEntry],
    ) -> Vec<u8> {
        crate::offline_verifier::codec_encode::encode_free_space_map(byte_order, entries)
    }

    pub fn decode_root_manifest(
        byte_order: PhysicalByteOrder,
        bytes: &[u8],
    ) -> Result<RootPublicationCell, OfflineVerifierDenial> {
        crate::offline_verifier::codec_decode::decode_root(
            byte_order,
            bytes,
            OfflineVerifierCounterSnapshot::empty().with_root_candidates_inspected(1),
        )
    }

    pub(crate) fn decode(
        byte_order: PhysicalByteOrder,
        root: &[u8],
        segment_manifest: &[u8],
        extent_manifest: &[u8],
        free_space_map: &[u8],
        counters: OfflineVerifierCounterSnapshot,
    ) -> Result<DecodedOfflineManifestSections, OfflineVerifierDenial> {
        crate::offline_verifier::codec_decode::decode(
            byte_order,
            root,
            segment_manifest,
            extent_manifest,
            free_space_map,
            counters,
        )
    }
}

pub(crate) fn encode_allocation_class(allocation_class: crate::AllocationClassKind) -> u8 {
    match allocation_class {
        crate::AllocationClassKind::OrdinaryRecordPage => 1,
        crate::AllocationClassKind::LargeRecordExtent => 2,
        crate::AllocationClassKind::RootManifest => 3,
        crate::AllocationClassKind::SegmentManifest => 4,
        crate::AllocationClassKind::ExtentManifest => 5,
        crate::AllocationClassKind::FreeSpaceMap => 6,
    }
}
