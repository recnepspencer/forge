use std::collections::BTreeMap;

use sha2::{Digest, Sha256};
use worth_store::physical_runtime::BoundedRecoveryFilesystemDiscovery;
use worth_store_physical_format::{
    decode_data_frame_page_lsn, decode_extent_chunk, inspect_inline_page, DurableExtentManifest,
    DurableExtentRecordPlacement, DurableFrameKind, DurableInlineRecordPlacement,
    ExtentChunkCoordinate, PhysicalRecordFormatDeclaration, RecordArtifactFile,
    RecordFrameCoordinate,
};
use worth_store_recovery_physics::{
    PhysicalRedoTarget, PhysicalRedoTargetIdentity, RecoveryPageObservation,
};

use super::{required, PageObservationFailure};
use crate::progression::RecoverySelectedSegmentPage;

pub(super) fn observe_inline(
    discovery: &mut BoundedRecoveryFilesystemDiscovery,
    placement: DurableInlineRecordPlacement,
    target: &PhysicalRedoTarget,
    format: PhysicalRecordFormatDeclaration,
    byte_limit: u64,
    entries: &BTreeMap<(u64, u64), RecoverySelectedSegmentPage>,
) -> Result<RecoveryPageObservation, PageObservationFailure> {
    let resolved = entries
        .get(&(placement.segment().get(), placement.page().get()))
        .copied()
        .filter(|resolved| {
            let entry = resolved.entry;
            entry.page_cell() == placement.page_cell()
                && entry.page_generation() == placement.page_generation()
                && entry.data_page_count() <= placement.segment_page_capacity()
        })
        .ok_or(PageObservationFailure::InvalidManifest {
            target: Some(target.identity()),
            artifact: entries
                .get(&(placement.segment().get(), placement.page().get()))
                .map_or(
                    RecordArtifactFile::RootManifest {
                        generation: placement.page_generation(),
                    },
                    |resolved| resolved.membership_artifact,
                ),
        })?;
    let entry = resolved.entry;
    let page_bytes = format.page_size().bytes();
    let offset = u64::from(entry.frame_index())
        .checked_mul(u64::from(page_bytes))
        .ok_or(PageObservationFailure::InvalidPage(target.identity()))?;
    let PhysicalRedoTargetIdentity::InlinePage { generation, .. } = target.identity() else {
        return Err(PageObservationFailure::InvalidTarget(target.identity()));
    };
    let RecordArtifactFile::Segment {
        segment: target_segment,
        generation: target_segment_generation,
    } = target.artifact()
    else {
        return Err(PageObservationFailure::InvalidTarget(target.identity()));
    };
    let materialized_target = target_segment_generation == entry.data_generation()
        && generation == placement.page_generation()
        && target.artifact_offset() == offset;
    let exact_successor = entry.data_generation().checked_add(1) == Some(target_segment_generation)
        && placement.page_generation().checked_add(1) == Some(generation);
    if target.artifact_length() != page_bytes
        || target_segment != placement.segment().get()
        || !(materialized_target || exact_successor)
    {
        return Err(PageObservationFailure::InvalidTarget(target.identity()));
    }
    let page = required(
        discovery.read_segment_range(
            placement.segment().get(),
            entry.data_generation(),
            offset,
            page_bytes,
            byte_limit,
        ),
        Some(target.identity()),
        RecordArtifactFile::Segment {
            segment: placement.segment().get(),
            generation: entry.data_generation(),
        },
    )?;
    let geometry = inspect_inline_page(format, &page)
        .map_err(|_| PageObservationFailure::InvalidPage(target.identity()))?;
    if geometry.page_cell() != placement.page_cell() {
        return Err(PageObservationFailure::InvalidPage(target.identity()));
    }
    let page_lsn = decode_data_frame_page_lsn(&page, DurableFrameKind::InlinePage)
        .map_err(|_| PageObservationFailure::InvalidPage(target.identity()))?;
    let source_coordinate = RecordFrameCoordinate::new(
        RecordArtifactFile::Segment {
            segment: placement.segment().get(),
            generation: entry.data_generation(),
        },
        offset,
        page_bytes,
    )
    .ok_or(PageObservationFailure::InvalidPage(target.identity()))?;
    Ok(RecoveryPageObservation::materialized(
        PhysicalRedoTargetIdentity::InlinePage {
            segment: placement.segment().get(),
            page: placement.page().get(),
            generation: placement.page_generation(),
        },
        page_lsn.get(),
        Sha256::digest(&page).into(),
        source_coordinate,
        resolved.routing_identity,
    ))
}

pub(super) fn observe_extent(
    discovery: &mut BoundedRecoveryFilesystemDiscovery,
    placement: DurableExtentRecordPlacement,
    target: &PhysicalRedoTarget,
    format: PhysicalRecordFormatDeclaration,
    byte_limit: u64,
    manifests: &mut BTreeMap<(u64, u64), DurableExtentManifest>,
) -> Result<RecoveryPageObservation, PageObservationFailure> {
    let key = (placement.extent().get(), placement.extent_generation());
    if !manifests.contains_key(&key) {
        let manifest_bytes = required(
            discovery.read_extent_manifest(key.0, key.1, byte_limit),
            Some(target.identity()),
            RecordArtifactFile::ExtentManifest {
                extent: key.0,
                generation: key.1,
            },
        )?;
        let (manifest, found_format) =
            DurableExtentManifest::decode(&manifest_bytes).map_err(|_| {
                PageObservationFailure::InvalidManifest {
                    target: Some(target.identity()),
                    artifact: RecordArtifactFile::ExtentManifest {
                        extent: key.0,
                        generation: key.1,
                    },
                }
            })?;
        if found_format != format
            || manifest.extent_cell() != placement.extent_cell()
            || manifest.record() != placement.record()
        {
            return Err(PageObservationFailure::InvalidManifest {
                target: Some(target.identity()),
                artifact: RecordArtifactFile::ExtentManifest {
                    extent: key.0,
                    generation: key.1,
                },
            });
        }
        manifests.insert(key, manifest);
    }
    let manifest = *manifests.get(&key).unwrap();
    let PhysicalRedoTargetIdentity::ExtentChunk { chunk, .. } = target.identity() else {
        return Err(PageObservationFailure::InvalidPage(target.identity()));
    };
    let coordinate = target
        .extent_coordinate()
        .ok_or(PageObservationFailure::InvalidTarget(target.identity()))?;
    if coordinate.allocation_epoch() != manifest.record().allocation_epoch()
        || coordinate.record_ordinal() != manifest.record().ordinal()
        || coordinate.logical_bytes() != manifest.logical_bytes()
    {
        return Err(PageObservationFailure::InvalidTarget(target.identity()));
    }
    let capacity = u64::from(manifest.chunk_payload_capacity());
    if chunk > manifest.chunk_count() {
        return Err(PageObservationFailure::InvalidTarget(target.identity()));
    }
    let logical_offset = u64::from(chunk - 1) * capacity;
    let payload = (manifest.logical_bytes() - logical_offset).min(capacity);
    let length = u32::try_from(
        worth_store_physical_format::DURABLE_EXTENT_FRAME_HEADER_BYTES as u64
            + worth_store_physical_format::EXTENT_CHUNK_METADATA_BYTES as u64
            + payload,
    )
    .map_err(|_| PageObservationFailure::InvalidTarget(target.identity()))?;
    let offset = u64::from(chunk - 1) * u64::from(manifest.maximum_frame_bytes());
    if coordinate.logical_offset() != logical_offset
        || target.artifact_offset() != offset
        || target.artifact_length() != length
    {
        return Err(PageObservationFailure::InvalidTarget(target.identity()));
    }
    let extent_artifact = RecordArtifactFile::Extent {
        extent: key.0,
        generation: key.1,
    };
    let frame = required(
        discovery.read_extent_range(key.0, key.1, offset, length, byte_limit),
        Some(target.identity()),
        extent_artifact,
    )?;
    let expected = ExtentChunkCoordinate::new(
        manifest.record(),
        manifest.extent_cell(),
        manifest.logical_bytes(),
        logical_offset,
        chunk,
    )
    .ok_or(PageObservationFailure::InvalidPage(target.identity()))?;
    let (_, found_format) = decode_extent_chunk(&frame, expected)
        .map_err(|_| PageObservationFailure::InvalidPage(target.identity()))?;
    if found_format != format {
        return Err(PageObservationFailure::InvalidPage(target.identity()));
    }
    let page_lsn = decode_data_frame_page_lsn(&frame, DurableFrameKind::Extent)
        .map_err(|_| PageObservationFailure::InvalidPage(target.identity()))?;
    let source_coordinate = RecordFrameCoordinate::new(extent_artifact, offset, length)
        .ok_or(PageObservationFailure::InvalidPage(target.identity()))?;
    let mut routing = Sha256::new();
    routing.update(b"worth.store.recovery.extent-routing.v1");
    routing.update(manifest.encode(format));
    routing.update(placement.record().allocation_epoch());
    routing.update(placement.record().ordinal().to_le_bytes());
    routing.update(placement.extent().get().to_le_bytes());
    routing.update(placement.extent_generation().to_le_bytes());
    Ok(RecoveryPageObservation::materialized(
        target.identity(),
        page_lsn.get(),
        Sha256::digest(&frame).into(),
        source_coordinate,
        routing.finalize().into(),
    ))
}
