use crate::container_integrity_frame_header::reject_page_local_frame_header_mismatch;
use crate::{
    AmbiguousBoundaryDamage, ContainerIntegrityCounters, PhysicalBoundaryLocalization,
    PhysicalContainerIntegrityDenial, PhysicalContainerIntegrityDenialKind,
    SlotDirectoryIntegrityReport, TornFrameDenial,
};
use forge_store_physical_format::{
    PageRecordCounterSnapshot, PageRecordDenial, PageRecordDenialKind, PhysicalByteOrder,
    PhysicalRecordSlot, SlotDirectory, SlotDirectoryEntry, SlotDirectoryEntryState,
    PHYSICAL_HEADER_LENGTH,
};

pub(crate) fn inspect_record_slot_directory(
    page_body: &[u8],
    mut counters: ContainerIntegrityCounters,
) -> Result<
    (SlotDirectoryIntegrityReport, ContainerIntegrityCounters),
    PhysicalContainerIntegrityDenial,
> {
    counters = counters.with_slot_directory_read();
    let directory = SlotDirectory::decode(
        page_body,
        PhysicalByteOrder::LittleEndian,
        PageRecordCounterSnapshot::for_locate_attempt(),
    )
    .map_err(|denial| slot_directory_denial(denial, counters))?;
    let mut occupied_slots = 0u16;
    let mut free_or_reserved_slots = 0u16;
    for index in 1..=directory.slot_count() {
        counters = counters.with_slot_entry_inspected();
        let slot = PhysicalRecordSlot::from_raw(index)
            .expect("slot directory iteration starts at nonzero slot");
        let entry = directory
            .locate(
                slot,
                PhysicalByteOrder::LittleEndian,
                PageRecordCounterSnapshot::for_locate_attempt(),
            )
            .map_err(|denial| slot_directory_denial(denial, counters))?;
        let (state_report, next_counters) = inspect_slot_state(page_body, entry, counters)?;
        counters = next_counters;
        match state_report {
            SlotStateInspection::Occupied => occupied_slots += 1,
            SlotStateInspection::StructurallyEmpty => free_or_reserved_slots += 1,
        }
    }
    Ok((
        SlotDirectoryIntegrityReport::new(
            directory.slot_count(),
            occupied_slots,
            free_or_reserved_slots,
        ),
        counters,
    ))
}

fn inspect_slot_state(
    page_body: &[u8],
    entry: SlotDirectoryEntry,
    counters: ContainerIntegrityCounters,
) -> Result<(SlotStateInspection, ContainerIntegrityCounters), PhysicalContainerIntegrityDenial> {
    match entry.state() {
        SlotDirectoryEntryState::Occupied => {
            let frame = frame_slice(page_body, entry, counters)?;
            let counters = inspect_page_local_frame(frame, entry, counters)?;
            Ok((SlotStateInspection::Occupied, counters))
        }
        SlotDirectoryEntryState::Deleted
        | SlotDirectoryEntryState::Free
        | SlotDirectoryEntryState::Reserved => {
            if entry.offset() == 0 && entry.frame_length() == 0 {
                Ok((SlotStateInspection::StructurallyEmpty, counters))
            } else {
                Err(PhysicalContainerIntegrityDenial::new(
                    PhysicalContainerIntegrityDenialKind::SlotStateIntegrityFailure,
                    PhysicalBoundaryLocalization::SlotState(entry.slot()),
                    counters.with_skipped_record_view(),
                )
                .with_slot(entry.slot()))
            }
        }
        SlotDirectoryEntryState::Moved => Err(PhysicalContainerIntegrityDenial::new(
            PhysicalContainerIntegrityDenialKind::SlotStateIntegrityFailure,
            PhysicalBoundaryLocalization::SlotState(entry.slot()),
            counters.with_skipped_record_view(),
        )
        .with_slot(entry.slot())),
    }
}

fn frame_slice<'a>(
    page_body: &'a [u8],
    entry: SlotDirectoryEntry,
    counters: ContainerIntegrityCounters,
) -> Result<&'a [u8], PhysicalContainerIntegrityDenial> {
    let start = entry.offset() as usize;
    let end = start.saturating_add(entry.frame_length() as usize);
    if start >= page_body.len() || end > page_body.len() {
        return Err(PhysicalContainerIntegrityDenial::new(
            PhysicalContainerIntegrityDenialKind::FrameOutOfBounds,
            PhysicalBoundaryLocalization::SlotState(entry.slot()),
            counters.with_skipped_record_view(),
        )
        .with_slot(entry.slot())
        .with_lengths(end, page_body.len()));
    }
    Ok(&page_body[start..end])
}

fn inspect_page_local_frame(
    frame: &[u8],
    entry: SlotDirectoryEntry,
    counters: ContainerIntegrityCounters,
) -> Result<ContainerIntegrityCounters, PhysicalContainerIntegrityDenial> {
    let counters = counters
        .with_frame_boundary_check()
        .with_skipped_record_view();
    if frame.len() < PHYSICAL_HEADER_LENGTH as usize {
        return Err(PhysicalContainerIntegrityDenial::new(
            PhysicalContainerIntegrityDenialKind::TornFrame,
            PhysicalBoundaryLocalization::FrameHeader,
            counters,
        )
        .with_slot(entry.slot())
        .with_lengths(PHYSICAL_HEADER_LENGTH as usize, frame.len())
        .with_torn_frame(TornFrameDenial::new(
            PHYSICAL_HEADER_LENGTH as usize,
            frame.len(),
        )));
    }
    reject_page_local_frame_header_mismatch(frame, entry, counters)?;
    reject_page_local_frame_payload_length(frame, entry, counters)
}

fn reject_page_local_frame_payload_length(
    frame: &[u8],
    entry: SlotDirectoryEntry,
    counters: ContainerIntegrityCounters,
) -> Result<ContainerIntegrityCounters, PhysicalContainerIntegrityDenial> {
    let byte_order = PhysicalByteOrder::LittleEndian;
    let payload_len = byte_order.read_u32([frame[5], frame[6], frame[7], frame[8]]) as usize;
    let expected = PHYSICAL_HEADER_LENGTH as usize + payload_len;
    if expected == frame.len() {
        return Ok(counters);
    }
    let kind = if frame.len() < expected {
        PhysicalContainerIntegrityDenialKind::TornFrame
    } else {
        PhysicalContainerIntegrityDenialKind::MalformedFrame
    };
    Err(PhysicalContainerIntegrityDenial::new(
        kind,
        PhysicalBoundaryLocalization::FrameBody,
        counters,
    )
    .with_slot(entry.slot())
    .with_lengths(expected, frame.len())
    .with_torn_frame(TornFrameDenial::new(expected, frame.len())))
}

fn slot_directory_denial(
    denial: PageRecordDenial,
    counters: ContainerIntegrityCounters,
) -> PhysicalContainerIntegrityDenial {
    if is_ambiguous_slot_directory_damage(&denial) {
        return PhysicalContainerIntegrityDenial::new(
            PhysicalContainerIntegrityDenialKind::SlotDirectoryMalformed,
            PhysicalBoundaryLocalization::AmbiguousBoundary,
            counters.with_skipped_record_view(),
        )
        .with_ambiguous(AmbiguousBoundaryDamage::new(
            PhysicalBoundaryLocalization::SlotDirectory,
        ));
    }
    let localization = denial
        .slot()
        .map(PhysicalBoundaryLocalization::SlotState)
        .unwrap_or(PhysicalBoundaryLocalization::SlotDirectory);
    PhysicalContainerIntegrityDenial::new(
        non_ambiguous_slot_denial_kind(denial.kind()),
        localization,
        counters.with_skipped_record_view(),
    )
}

fn non_ambiguous_slot_denial_kind(
    kind: PageRecordDenialKind,
) -> PhysicalContainerIntegrityDenialKind {
    match kind {
        PageRecordDenialKind::FrameOutOfBounds => {
            PhysicalContainerIntegrityDenialKind::FrameOutOfBounds
        }
        _ => PhysicalContainerIntegrityDenialKind::SlotStateIntegrityFailure,
    }
}

fn is_ambiguous_slot_directory_damage(denial: &PageRecordDenial) -> bool {
    matches!(
        denial.kind(),
        PageRecordDenialKind::SlotDirectoryTooShort
            | PageRecordDenialKind::SlotDirectoryLengthMismatch
            | PageRecordDenialKind::SlotEntryMismatch
            | PageRecordDenialKind::SlotGenerationMismatch
    ) || (denial.kind() == PageRecordDenialKind::ReservedSlot && denial.slot().is_none())
}

enum SlotStateInspection {
    Occupied,
    StructurallyEmpty,
}
