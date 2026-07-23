use std::collections::BTreeMap;

use worth_store_physical_format::{
    durable_artifact_checksum, CurrentPhysicalRecordPlacement, DurableFreeSpaceManifestHeader,
    DurablePhysicalRootManifest, PersistedRecordIdentity, RecordArtifactFile,
    RecordSegmentPageManifestEntry, SegmentPageKey,
};

use super::super::{
    planning::batch_placement::classify_batch,
    planning::extent_placement::lower_extents,
    planning::free_space_projection::{project_successor_free_space, FreeSpaceProjectionContext},
    planning::inline_segment_plan::{plan_inline_segments, InlineSegmentPlanningContext},
    publication::append_observation::PublicationObservation,
    publication::batch::AdmittedRecordAppendBatch,
    publication::segment_publication::SegmentDataPlan,
    publication::CandidateDataArtifact,
    RecordAppendDenial, RecordAppendError,
};

pub(in crate::physical_runtime::record_serving) struct LoweredRecordPlacementPlan {
    pub(in crate::physical_runtime::record_serving) records: Vec<PersistedRecordIdentity>,
    pub(in crate::physical_runtime::record_serving) data: Vec<CandidateDataArtifact>,
    pub(in crate::physical_runtime::record_serving) manifests: Vec<(RecordArtifactFile, Vec<u8>)>,
    pub(in crate::physical_runtime::record_serving) manifest: DurablePhysicalRootManifest,
    pub(in crate::physical_runtime::record_serving) free_space: DurableFreeSpaceManifestHeader,
    pub(in crate::physical_runtime::record_serving) observation: PublicationObservation,
}

pub(in crate::physical_runtime::record_serving) fn lower_batch(
    context: super::super::planning::placement_context::PlacementPlanningContext<'_>,
    batch: AdmittedRecordAppendBatch,
) -> Result<LoweredRecordPlacementPlan, RecordAppendError> {
    let super::super::planning::placement_context::PlacementPlanningContext {
        media,
        format,
        access,
        current_root,
        current_free_space,
        frontier,
        placement,
        generation,
        frame_load,
    } = context;
    let reader = super::super::access::manifest_routing::ManifestReader::with_loader(
        media,
        frame_load,
        format,
        access,
        current_root,
    );
    let classified = classify_batch(&reader, placement, batch)?;
    let mut data = Vec::new();
    let mut manifests = Vec::new();
    let mut new_placements = BTreeMap::new();
    let inline_plan = plan_inline_segments(
        InlineSegmentPlanningContext {
            media,
            format,
            access,
            current_root,
            current_free_space,
            frontier,
            placement,
            placements: &mut new_placements,
            frame_load,
        },
        classified.inline,
    )?;
    lower_extents(
        format,
        frontier,
        classified.extents,
        &mut data,
        &mut manifests,
        &mut new_placements,
    )?;
    let free_space = project_successor_free_space(
        FreeSpaceProjectionContext {
            media,
            frame_load,
            format,
            access,
            current: current_free_space,
            successor_generation: generation,
            successor_capacity: placement.manifest_capacity().get(),
            frontier,
        },
        &inline_plan.segments,
    )?;
    let peak_read_width = inline_plan.peak_read_width;
    let source_copy_count = inline_plan.source_copy_count;
    let source_copied_bytes = inline_plan.source_copied_bytes;
    let segment_updates = lower_segments(inline_plan.segments, &mut data)?;
    let super::super::planning::free_space_routing::FreeSpacePublicationPlan {
        header: free_space,
        blocks: free_space_blocks,
        discovery: free_space_discovery,
    } = free_space;
    manifests.extend(free_space_blocks);
    let free_space_bytes = free_space.encode(format.declaration());
    manifests.push((
        RecordArtifactFile::FreeSpaceManifest { generation },
        free_space_bytes.clone(),
    ));
    let last_inline_tail = new_placements
        .values()
        .filter_map(|entry| match entry {
            CurrentPhysicalRecordPlacement::Inline(value) => Some((
                value.segment().get(),
                value.page().get(),
                value.slot().get(),
                value.record(),
                value.segment_cell(),
            )),
            CurrentPhysicalRecordPlacement::Extent(_) => None,
        })
        .max_by_key(|value| (value.0, value.1, value.2))
        .map(|value| (Some(value.3), Some(value.4)))
        .unwrap_or((
            current_root.last_inline_record(),
            current_root.last_inline_segment(),
        ));
    let segment_routed = super::super::access::segment_membership::plan_segment_membership_updates(
        super::super::access::segment_membership::SegmentMembershipUpdateContext {
            media,
            frame_load,
            format,
            access,
            current: current_root,
            successor_generation: generation,
            successor_capacity: placement.manifest_capacity().get(),
        },
        segment_updates,
    )
    .map_err(|_| RecordAppendError::Denied(RecordAppendDenial::PublishedLayoutDamaged))?;
    let super::super::access::segment_membership::SegmentMembershipPublicationPlan {
        root: segment_root,
        next_block: next_segment_block,
        blocks: segment_blocks,
        discovery: segment_discovery,
    } = segment_routed;
    manifests.extend(segment_blocks);
    let routed = super::super::access::manifest_routing::plan_manifest_updates(
        &reader,
        current_root,
        super::super::access::manifest_routing::RootManifestUpdateRequest {
            successor_generation: generation,
            successor_capacity: placement.manifest_capacity().get(),
            free_space_checksum: durable_artifact_checksum(&free_space_bytes),
            free_space_root: free_space.root(),
            segment_root,
            next_segment_block,
            placements: new_placements,
            last_inline_record: last_inline_tail.0,
            last_inline_segment: last_inline_tail.1,
        },
    )
    .map_err(|_| RecordAppendError::Denied(RecordAppendDenial::PublishedLayoutDamaged))?;
    manifests.extend(routed.blocks);
    let manifest = routed.root;
    let observation = PublicationObservation {
        records: classified.identities.len() as u64,
        logical_bytes: classified.logical_bytes,
        completed_bytes: 0,
        segment_artifacts: count_segments(&data),
        extent_artifacts: count_extents(&data),
        transfer_count: 0,
        peak_transfer_width: peak_read_width as u64,
        explicit_copy_count: source_copy_count,
        copied_bytes: source_copied_bytes,
        peak_scratch_bytes: 0,
        manifest_blocks_read: routed
            .discovery
            .blocks_read()
            .saturating_add(segment_discovery.blocks_read())
            .saturating_add(free_space_discovery.blocks_read()),
        manifest_comparisons: routed
            .discovery
            .comparisons()
            .saturating_add(segment_discovery.comparisons())
            .saturating_add(free_space_discovery.comparisons()),
        manifest_bytes_read: routed
            .discovery
            .bytes_read()
            .saturating_add(segment_discovery.bytes_read())
            .saturating_add(free_space_discovery.bytes_read()),
    };
    Ok(LoweredRecordPlacementPlan {
        records: classified.identities,
        data,
        manifests,
        manifest,
        free_space,
        observation,
    })
}

fn lower_segments(
    segments: Vec<super::super::planning::inline_segment_plan::WorkingSegment>,
    data: &mut Vec<CandidateDataArtifact>,
) -> Result<BTreeMap<SegmentPageKey, RecordSegmentPageManifestEntry>, RecordAppendError> {
    let mut updates = BTreeMap::new();
    for segment in segments {
        for entry in segment.membership_updates.iter().copied() {
            updates.insert(SegmentPageKey::from(entry), entry);
        }
        data.push(CandidateDataArtifact::Segment(SegmentDataPlan {
            artifact: RecordArtifactFile::Segment {
                segment: segment.segment.segment_id().get(),
                generation: segment.segment.generation().get(),
            },
            pages: segment.data_pages,
        }));
    }
    Ok(updates)
}

fn count_segments(data: &[CandidateDataArtifact]) -> u64 {
    data.iter()
        .filter(|value| matches!(value, CandidateDataArtifact::Segment(_)))
        .count() as u64
}

fn count_extents(data: &[CandidateDataArtifact]) -> u64 {
    data.iter()
        .filter(|value| matches!(value, CandidateDataArtifact::Extent(_)))
        .count() as u64
}
