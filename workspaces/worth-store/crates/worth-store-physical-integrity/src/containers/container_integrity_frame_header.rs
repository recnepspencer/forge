use crate::{
    ContainerIntegrityCounters, PhysicalBoundaryLocalization, PhysicalContainerIntegrityDenial,
    PhysicalContainerIntegrityDenialKind,
};
use worth_store_physical_format::{
    PhysicalBinaryEncodingWitness, PhysicalCellReuseDomain, PhysicalFrameKind,
    PhysicalGenerationAuthority, PhysicalGenerationOwner, PhysicalHeaderAuthority,
    PhysicalHeaderDecodeDenial, PhysicalHeaderDecodeDenialKind, PhysicalReferenceAuthority,
    SlotDirectoryEntry,
};

pub(crate) fn reject_page_local_frame_header_mismatch(
    frame: &[u8],
    page_owner: PhysicalGenerationOwner,
    entry: SlotDirectoryEntry,
    counters: ContainerIntegrityCounters,
) -> Result<(), PhysicalContainerIntegrityDenial> {
    let cell =
        page_local_slot_cell(page_owner, entry).ok_or_else(|| header_mismatch(entry, counters))?;
    let references = PhysicalReferenceAuthority::for_canonical_physical_format();
    let reference = references
        .validate_page_slot(references.admit_page_slot(cell), cell)
        .map_err(|_| header_mismatch(entry, counters))?;
    let binary = PhysicalBinaryEncodingWitness::physical_format_canonical()
        .map_err(|_| header_mismatch(entry, counters))?;
    PhysicalHeaderAuthority::for_canonical_physical_format(binary)
        .decode_frame_header_prefix(reference, frame, PhysicalFrameKind::RecordFrame)
        .map(|_| ())
        .map_err(|denial| header_decode_denial(denial, entry, counters))
}

fn page_local_slot_cell(
    page_owner: PhysicalGenerationOwner,
    entry: SlotDirectoryEntry,
) -> Option<worth_store_physical_format::SlotGenerationCell> {
    if page_owner.domain() != PhysicalCellReuseDomain::Page {
        return None;
    }
    Some(
        PhysicalGenerationAuthority::for_canonical_physical_format()
            .slot_cell(
                page_owner.segment_id()?,
                page_owner.page_id()?,
                entry.slot(),
            )
            .with_slot_generation(entry.generation()),
    )
}

fn header_decode_denial(
    denial: PhysicalHeaderDecodeDenial,
    entry: SlotDirectoryEntry,
    counters: ContainerIntegrityCounters,
) -> PhysicalContainerIntegrityDenial {
    let localization = if denial.kind() == PhysicalHeaderDecodeDenialKind::HeaderLengthMismatch {
        PhysicalBoundaryLocalization::LengthField
    } else {
        PhysicalBoundaryLocalization::FrameHeader
    };
    let mut mapped = PhysicalContainerIntegrityDenial::new(
        PhysicalContainerIntegrityDenialKind::HeaderWitnessMismatch,
        localization,
        counters,
    )
    .with_slot(entry.slot());
    if let (Some(expected), Some(actual)) = (denial.expected_length(), denial.actual_length()) {
        mapped = mapped.with_lengths(expected, actual);
    }
    mapped
}

fn header_mismatch(
    entry: SlotDirectoryEntry,
    counters: ContainerIntegrityCounters,
) -> PhysicalContainerIntegrityDenial {
    PhysicalContainerIntegrityDenial::new(
        PhysicalContainerIntegrityDenialKind::HeaderWitnessMismatch,
        PhysicalBoundaryLocalization::FrameHeader,
        counters,
    )
    .with_slot(entry.slot())
}
