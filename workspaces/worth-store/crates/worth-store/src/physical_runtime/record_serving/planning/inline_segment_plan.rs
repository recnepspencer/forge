use std::collections::{BTreeMap, VecDeque};

use worth_store_physical_backend::QualifiedFilesystemMedia;
use worth_store_physical_format::{
    inspect_inline_page_records, CurrentPhysicalRecordPlacement, DurableInlineRecordPlacement,
    DurablePhysicalRootManifest, PageGenerationCell, PersistedRecordIdentity,
    PhysicalGenerationAuthority, PhysicalRecordSlot, RecordSegmentPageManifestEntry,
    SegmentGenerationCell,
};

use super::super::{
    planning::batch_placement::{
        materialize_inline_inputs, MaterializedInlineInput, PendingInlineInput,
    },
    planning::inline_page_packing::{
        fitting_prefix, new_page_fill_capacity, remaining_policy_capacity,
    },
    planning::inline_plan_failure::admitted_generation as generation,
    planning::published_segment_reuse::{load_reusable_segment, ReusableSegmentContext},
    planning::published_tail_page::{load_published_tail_page, LoadedPublishedTailPage},
    planning::reusable_inline_tail::last_inline_placement,
    publication::segment_publication::PageDataPlan,
    residency::serving_artifacts::ServingRecordArtifacts,
    AdmittedPhysicalRecordFormat, AdmittedRecordPlacementPolicy, RecordAllocationFrontier,
    RecordAppendDenial, RecordAppendError,
};

pub(in crate::physical_runtime::record_serving) struct WorkingSegment {
    pub(in crate::physical_runtime::record_serving) segment: SegmentGenerationCell,
    pub(in crate::physical_runtime::record_serving) page_capacity: u32,
    pub(in crate::physical_runtime::record_serving) used_pages: u32,
    pub(in crate::physical_runtime::record_serving) membership_updates:
        Vec<RecordSegmentPageManifestEntry>,
    pub(in crate::physical_runtime::record_serving) data_pages: Vec<PageDataPlan>,
}

pub(in crate::physical_runtime::record_serving) struct PlannedInlineSegments {
    pub(in crate::physical_runtime::record_serving) segments: Vec<WorkingSegment>,
    pub(in crate::physical_runtime::record_serving) peak_read_width: usize,
    pub(in crate::physical_runtime::record_serving) source_copy_count: u64,
    pub(in crate::physical_runtime::record_serving) source_copied_bytes: u64,
}

pub(in crate::physical_runtime::record_serving) struct PlanningSegment {
    pub(in crate::physical_runtime::record_serving) segment: SegmentGenerationCell,
    pub(in crate::physical_runtime::record_serving) page_capacity: u32,
    pub(in crate::physical_runtime::record_serving) used_pages: u32,
    pub(in crate::physical_runtime::record_serving) last_published_page:
        Option<RecordSegmentPageManifestEntry>,
    pub(in crate::physical_runtime::record_serving) candidate_pages: Vec<PlannedPageMembership>,
    pub(in crate::physical_runtime::record_serving) data_pages: Vec<PageDataPlan>,
}

pub(in crate::physical_runtime::record_serving) enum PlannedPageMembership {
    Candidate {
        page: PageGenerationCell,
        frame_index: u32,
    },
}

pub(in crate::physical_runtime::record_serving) struct InlineSegmentPlanningContext<'plan> {
    pub(in crate::physical_runtime::record_serving) media: &'plan QualifiedFilesystemMedia,
    pub(in crate::physical_runtime::record_serving) format: AdmittedPhysicalRecordFormat,
    pub(in crate::physical_runtime::record_serving) access:
        super::super::AdmittedRecordAccessPolicy,
    pub(in crate::physical_runtime::record_serving) current_root:
        &'plan DurablePhysicalRootManifest,
    pub(in crate::physical_runtime::record_serving) current_free_space:
        &'plan worth_store_physical_format::DurableFreeSpaceManifestHeader,
    pub(in crate::physical_runtime::record_serving) frontier: &'plan mut RecordAllocationFrontier,
    pub(in crate::physical_runtime::record_serving) placement: AdmittedRecordPlacementPolicy,
    pub(in crate::physical_runtime::record_serving) placements:
        &'plan mut BTreeMap<PersistedRecordIdentity, CurrentPhysicalRecordPlacement>,
    pub(in crate::physical_runtime::record_serving) frame_load:
        &'plan (dyn super::super::residency::frame_ports::FrameLoadPort + Send + Sync),
}

pub(in crate::physical_runtime::record_serving) fn plan_inline_segments(
    context: InlineSegmentPlanningContext<'_>,
    inline: Vec<PendingInlineInput>,
) -> Result<PlannedInlineSegments, RecordAppendError> {
    if inline.is_empty() {
        return Ok(PlannedInlineSegments {
            segments: Vec::new(),
            peak_read_width: 0,
            source_copy_count: 0,
            source_copied_bytes: 0,
        });
    }
    let InlineSegmentPlanningContext {
        media,
        format,
        access,
        current_root,
        current_free_space,
        frontier,
        placement,
        placements,
        frame_load,
    } = context;
    let artifacts = ServingRecordArtifacts::new(media, frame_load);
    let last_inline =
        last_inline_placement(media, frame_load, format, access, current_root, placement)?;
    let (mut active, mut peak_read_width) = load_reusable_segment(
        ReusableSegmentContext {
            media,
            frame_load,
            format,
            access,
            current_root,
            current_free_space,
            placement,
        },
        last_inline,
    )?;
    let loaded_tail = if let (Some(last), Some(segment)) = (last_inline, active.as_ref()) {
        let loaded = load_published_tail_page(&artifacts, format, last, segment)?;
        peak_read_width = peak_read_width.max(loaded.page.len());
        Some(loaded)
    } else {
        None
    };
    let materialized = materialize_inline_inputs(inline)?;
    let mut inline = VecDeque::from(materialized.records);
    let mut plans = Vec::new();
    if let (Some(loaded), Some(segment)) = (loaded_tail, active.as_mut()) {
        append_to_last_page(format, placement, segment, loaded, &mut inline, placements)?;
    }
    while !inline.is_empty() {
        if active
            .as_ref()
            .is_some_and(|segment| segment.used_pages == segment.page_capacity)
        {
            let full = active.take().unwrap();
            if !full.data_pages.is_empty() {
                plans.push(finish_segment(full)?);
            }
        }
        if active.is_none() {
            active = Some(new_segment(frontier, placement)?);
        }
        append_new_page(
            format,
            placement,
            frontier,
            active.as_mut().unwrap(),
            &mut inline,
            placements,
        )?;
    }
    if let Some(active) = active {
        if !active.data_pages.is_empty() {
            plans.push(finish_segment(active)?);
        }
    }
    Ok(PlannedInlineSegments {
        segments: plans,
        peak_read_width,
        source_copy_count: materialized.copy_count,
        source_copied_bytes: materialized.copied_bytes,
    })
}

fn append_to_last_page(
    format: AdmittedPhysicalRecordFormat,
    placement: AdmittedRecordPlacementPolicy,
    segment: &mut PlanningSegment,
    loaded: LoadedPublishedTailPage,
    inline: &mut VecDeque<MaterializedInlineInput>,
    placements: &mut BTreeMap<PersistedRecordIdentity, CurrentPhysicalRecordPlacement>,
) -> Result<(), RecordAppendError> {
    let LoadedPublishedTailPage { page, geometry } = loaded;
    let count = fitting_prefix(
        inline.make_contiguous(),
        geometry.free_bytes() as usize,
        remaining_policy_capacity(format, placement, geometry.free_bytes() as usize),
    );
    if count == 0 {
        return Ok(());
    }
    let page_generation = generation(geometry.generation().checked_add(1))?;
    let candidate_page = PhysicalGenerationAuthority::for_canonical_physical_format()
        .page_cell(segment.segment.segment_id(), geometry.page())
        .with_page_generation(page_generation);
    remap_existing_page_records(format, &page, segment, candidate_page, placements)?;
    let candidate_frame_index = segment.data_pages.len() as u32;
    segment.last_published_page = None;
    segment
        .candidate_pages
        .push(PlannedPageMembership::Candidate {
            page: candidate_page,
            frame_index: candidate_frame_index,
        });
    let records = take_inline_records(
        count,
        inline,
        segment,
        candidate_page,
        geometry.slot_count(),
        placements,
    )?;
    segment.data_pages.push(PageDataPlan {
        page: candidate_page,
        existing_frame: Some(page),
        records,
    });
    Ok(())
}

fn remap_existing_page_records(
    format: AdmittedPhysicalRecordFormat,
    page: &[u8],
    segment: &PlanningSegment,
    candidate_page: PageGenerationCell,
    placements: &mut BTreeMap<PersistedRecordIdentity, CurrentPhysicalRecordPlacement>,
) -> Result<(), RecordAppendError> {
    let authority = PhysicalGenerationAuthority::for_canonical_physical_format();
    for descriptor in inspect_inline_page_records(format.declaration(), page)
        .map_err(|_| RecordAppendError::Denied(RecordAppendDenial::PublishedLayoutDamaged))?
    {
        let slot = authority
            .slot_cell(
                segment.segment.segment_id(),
                candidate_page.page_id(),
                descriptor.slot(),
            )
            .with_slot_generation(descriptor.slot_generation());
        let placement = DurableInlineRecordPlacement::new(
            descriptor.record(),
            segment.segment,
            candidate_page,
            slot,
            segment.page_capacity,
            u64::from(descriptor.payload_bytes()),
        )
        .ok_or(RecordAppendError::Denied(
            RecordAppendDenial::PublishedLayoutDamaged,
        ))?;
        placements.insert(
            descriptor.record(),
            CurrentPhysicalRecordPlacement::Inline(placement),
        );
    }
    Ok(())
}

fn append_new_page(
    format: AdmittedPhysicalRecordFormat,
    placement: AdmittedRecordPlacementPolicy,
    frontier: &mut RecordAllocationFrontier,
    segment: &mut PlanningSegment,
    inline: &mut VecDeque<MaterializedInlineInput>,
    placements: &mut BTreeMap<PersistedRecordIdentity, CurrentPhysicalRecordPlacement>,
) -> Result<(), RecordAppendError> {
    let free = new_page_fill_capacity(format, placement);
    let count = fitting_prefix(inline.make_contiguous(), free, free);
    if count == 0 {
        return Err(RecordAppendError::Denied(
            RecordAppendDenial::InlinePageFull,
        ));
    }
    let page_id = frontier.allocate_page().ok_or(RecordAppendError::Denied(
        RecordAppendDenial::PhysicalIdentityExhausted,
    ))?;
    let page = PhysicalGenerationAuthority::for_canonical_physical_format()
        .page_cell(segment.segment.segment_id(), page_id)
        .with_page_generation(generation(Some(1))?);
    let frame_index = segment.data_pages.len() as u32;
    let records = take_inline_records(count, inline, segment, page, 0, placements)?;
    segment
        .candidate_pages
        .push(PlannedPageMembership::Candidate { page, frame_index });
    segment.used_pages = segment
        .used_pages
        .checked_add(1)
        .ok_or(RecordAppendError::Denied(
            RecordAppendDenial::PhysicalIdentityExhausted,
        ))?;
    segment.data_pages.push(PageDataPlan {
        page,
        existing_frame: None,
        records,
    });
    Ok(())
}

fn take_inline_records(
    count: usize,
    inline: &mut VecDeque<MaterializedInlineInput>,
    segment: &PlanningSegment,
    page: PageGenerationCell,
    old_count: u16,
    placements: &mut BTreeMap<PersistedRecordIdentity, CurrentPhysicalRecordPlacement>,
) -> Result<
    Vec<(
        PersistedRecordIdentity,
        worth_store_physical_format::SlotGenerationCell,
        Vec<u8>,
    )>,
    RecordAppendError,
> {
    let authority = PhysicalGenerationAuthority::for_canonical_physical_format();
    let slot_generation = generation(Some(1))?;
    (0..count)
        .map(|index| {
            let input = inline.pop_front().expect("fitting prefix exists");
            let slot = PhysicalRecordSlot::from_raw(old_count + index as u16 + 1)
                .map_err(|_| RecordAppendError::Denied(RecordAppendDenial::InlinePageFull))?;
            let slot_cell = authority
                .slot_cell(segment.segment.segment_id(), page.page_id(), slot)
                .with_slot_generation(slot_generation);
            placements.insert(
                input.record,
                CurrentPhysicalRecordPlacement::Inline(
                    DurableInlineRecordPlacement::new(
                        input.record,
                        segment.segment,
                        page,
                        slot_cell,
                        segment.page_capacity,
                        input.bytes.len() as u64,
                    )
                    .expect("planner coordinates are consistent"),
                ),
            );
            Ok((input.record, slot_cell, input.bytes))
        })
        .collect()
}

fn new_segment(
    frontier: &mut RecordAllocationFrontier,
    placement: AdmittedRecordPlacementPolicy,
) -> Result<PlanningSegment, RecordAppendError> {
    let segment = frontier
        .allocate_segment()
        .ok_or(RecordAppendError::Denied(
            RecordAppendDenial::PhysicalIdentityExhausted,
        ))?;
    Ok(PlanningSegment {
        segment: PhysicalGenerationAuthority::for_canonical_physical_format()
            .segment_cell(segment)
            .with_segment_generation(generation(Some(1))?),
        page_capacity: placement.segment_pages().get(),
        used_pages: 0,
        last_published_page: None,
        candidate_pages: Vec::new(),
        data_pages: Vec::new(),
    })
}

fn finish_segment(segment: PlanningSegment) -> Result<WorkingSegment, RecordAppendError> {
    let data_page_count = u32::try_from(segment.data_pages.len())
        .map_err(|_| RecordAppendError::Denied(RecordAppendDenial::PublishedLayoutDamaged))?;
    let membership_updates = segment
        .candidate_pages
        .into_iter()
        .map(|membership| match membership {
            PlannedPageMembership::Candidate { page, frame_index } => {
                RecordSegmentPageManifestEntry::new(
                    page,
                    segment.segment,
                    data_page_count,
                    frame_index,
                )
            }
        })
        .collect::<Option<Vec<_>>>()
        .ok_or(RecordAppendError::Denied(
            RecordAppendDenial::PublishedLayoutDamaged,
        ))?;
    Ok(WorkingSegment {
        segment: segment.segment,
        page_capacity: segment.page_capacity,
        used_pages: segment.used_pages,
        membership_updates,
        data_pages: segment.data_pages,
    })
}
