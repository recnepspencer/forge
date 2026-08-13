use super::*;

mod cursor;
use cursor::*;

impl PersistedPhysicalRecoveryProjection {
    pub fn encode(&self) -> Vec<u8> {
        let mut target = Vec::new();
        field(&mut target, DOMAIN);
        target.extend_from_slice(&self.source_root_generation.to_le_bytes());
        field(&mut target, &self.root_state.encode());
        write_sequence(&mut target, &self.record_identities, |target, record| {
            write_record(target, *record)
        });
        write_sequence(&mut target, &self.frames, write_frame);
        write_sequence(&mut target, &self.placements, write_placement);
        write_sequence(&mut target, &self.segment_updates, write_segment_update);
        write_sequence(&mut target, &self.manifests, write_manifest);
        target
    }

    pub fn decode(
        bytes: &[u8],
        limits: PhysicalRecoveryProjectionDecodeLimits,
    ) -> Result<Self, PhysicalRecoveryProjectionDenial> {
        let mut cursor = Cursor::new(bytes);
        if cursor.field()? != DOMAIN {
            return Err(PhysicalRecoveryProjectionDenial::Malformed);
        }
        let source_root_generation = cursor.u64()?;
        let root_state =
            PersistedPhysicalRecoveryRootState::decode(cursor.field()?, limits.inline_allocations)
                .ok_or(PhysicalRecoveryProjectionDenial::Malformed)?;
        let record_identities = read_sequence(&mut cursor, limits.record_identities, |bytes| {
            let mut cursor = Cursor::new(bytes);
            let record = read_record(&mut cursor)?;
            cursor.end()?;
            Ok(record)
        })?;
        let frames = read_sequence(&mut cursor, limits.frames, read_frame)?;
        let mut remaining_entries = limits.total_entries;
        let placements = read_bounded_sequence(
            &mut cursor,
            limits.placements,
            &mut remaining_entries,
            read_placement,
        )?;
        let segment_updates = read_bounded_sequence(
            &mut cursor,
            limits.segment_updates,
            &mut remaining_entries,
            read_segment_update,
        )?;
        let manifests = read_bounded_sequence(
            &mut cursor,
            limits.manifests,
            &mut remaining_entries,
            read_manifest,
        )?;
        cursor.end()?;
        Self::new(
            source_root_generation,
            root_state,
            record_identities,
            frames,
            placements,
            segment_updates,
            manifests,
        )
        .ok_or(PhysicalRecoveryProjectionDenial::Malformed)
    }
}

fn write_sequence<T>(target: &mut Vec<u8>, values: &[T], write: fn(&mut Vec<u8>, &T)) {
    target.extend_from_slice(&(values.len() as u64).to_le_bytes());
    for value in values {
        let mut encoded = Vec::new();
        write(&mut encoded, value);
        field(target, &encoded);
    }
}

fn read_sequence<T>(
    cursor: &mut Cursor<'_>,
    maximum: u64,
    read: fn(&[u8]) -> Result<T, PhysicalRecoveryProjectionDenial>,
) -> Result<Vec<T>, PhysicalRecoveryProjectionDenial> {
    let count = cursor.u64()?;
    if count > maximum {
        return Err(PhysicalRecoveryProjectionDenial::EntryLimit);
    }
    let mut values = Vec::with_capacity(count as usize);
    for _ in 0..count {
        values.push(read(cursor.field()?)?);
    }
    Ok(values)
}

fn read_bounded_sequence<T>(
    cursor: &mut Cursor<'_>,
    maximum: u64,
    remaining_total: &mut u64,
    read: fn(&[u8]) -> Result<T, PhysicalRecoveryProjectionDenial>,
) -> Result<Vec<T>, PhysicalRecoveryProjectionDenial> {
    let count = cursor.u64()?;
    if count > maximum || count > *remaining_total {
        return Err(PhysicalRecoveryProjectionDenial::EntryLimit);
    }
    *remaining_total -= count;
    let mut values = Vec::with_capacity(count as usize);
    for _ in 0..count {
        values.push(read(cursor.field()?)?);
    }
    Ok(values)
}

fn write_frame(target: &mut Vec<u8>, frame: &PersistedPhysicalRecoveryFrame) {
    write_subject_coordinate(target, frame.subject, frame.coordinate);
    field(target, frame.bytes());
}

fn read_frame(
    bytes: &[u8],
) -> Result<PersistedPhysicalRecoveryFrame, PhysicalRecoveryProjectionDenial> {
    let mut cursor = Cursor::new(bytes);
    let (subject, coordinate) = read_subject_coordinate(&mut cursor)?;
    let payload = cursor.field()?;
    cursor.end()?;
    PersistedPhysicalRecoveryFrame::new(subject, coordinate, payload)
        .ok_or(PhysicalRecoveryProjectionDenial::InvalidFrame)
}

fn write_placement(target: &mut Vec<u8>, placement: &CurrentPhysicalRecordPlacement) {
    match placement {
        CurrentPhysicalRecordPlacement::Inline(value) => {
            target.push(1);
            write_record(target, value.record());
            target.extend_from_slice(&value.segment().get().to_le_bytes());
            target.extend_from_slice(&value.segment_generation().to_le_bytes());
            target.extend_from_slice(&value.page().get().to_le_bytes());
            target.extend_from_slice(&value.page_generation().to_le_bytes());
            target.extend_from_slice(&value.slot().get().to_le_bytes());
            target.extend_from_slice(&value.slot_generation().to_le_bytes());
            target.extend_from_slice(&value.segment_page_capacity().to_le_bytes());
            target.extend_from_slice(&value.payload_bytes().to_le_bytes());
        }
        CurrentPhysicalRecordPlacement::Extent(value) => {
            target.push(2);
            write_record(target, value.record());
            target.extend_from_slice(&value.extent().get().to_le_bytes());
            target.extend_from_slice(&value.extent_generation().to_le_bytes());
            target.extend_from_slice(&value.payload_bytes().to_le_bytes());
        }
    }
}

fn read_placement(
    bytes: &[u8],
) -> Result<CurrentPhysicalRecordPlacement, PhysicalRecoveryProjectionDenial> {
    let mut cursor = Cursor::new(bytes);
    let kind = cursor.byte()?;
    let record = read_record(&mut cursor)?;
    let authority = PhysicalGenerationAuthority::for_canonical_physical_format();
    let placement = match kind {
        1 => {
            let segment = PhysicalSegmentId::from_raw(cursor.u64()?)
                .map_err(|_| PhysicalRecoveryProjectionDenial::InvalidPlacement)?;
            let segment_generation = generation(cursor.u64()?)?;
            let page = PhysicalPageId::from_raw(cursor.u64()?)
                .map_err(|_| PhysicalRecoveryProjectionDenial::InvalidPlacement)?;
            let page_generation = generation(cursor.u64()?)?;
            let slot = PhysicalRecordSlot::from_raw(cursor.u16()?)
                .map_err(|_| PhysicalRecoveryProjectionDenial::InvalidPlacement)?;
            let slot_generation = generation(cursor.u64()?)?;
            let capacity = cursor.u32()?;
            let payload = cursor.u64()?;
            CurrentPhysicalRecordPlacement::Inline(
                DurableInlineRecordPlacement::new(
                    record,
                    authority
                        .segment_cell(segment)
                        .with_segment_generation(segment_generation),
                    authority
                        .page_cell(segment, page)
                        .with_page_generation(page_generation),
                    authority
                        .slot_cell(segment, page, slot)
                        .with_slot_generation(slot_generation),
                    capacity,
                    payload,
                )
                .ok_or(PhysicalRecoveryProjectionDenial::InvalidPlacement)?,
            )
        }
        2 => {
            let extent = PhysicalExtentId::from_raw(cursor.u64()?)
                .map_err(|_| PhysicalRecoveryProjectionDenial::InvalidPlacement)?;
            let extent_generation = generation(cursor.u64()?)?;
            let payload = cursor.u64()?;
            CurrentPhysicalRecordPlacement::Extent(
                DurableExtentRecordPlacement::new(
                    record,
                    authority
                        .record_extent_cell(extent)
                        .with_extent_generation(extent_generation),
                    payload,
                )
                .ok_or(PhysicalRecoveryProjectionDenial::InvalidPlacement)?,
            )
        }
        _ => return Err(PhysicalRecoveryProjectionDenial::InvalidPlacement),
    };
    cursor.end()?;
    Ok(placement)
}

fn write_segment_update(target: &mut Vec<u8>, update: &RecordSegmentPageManifestEntry) {
    target.extend_from_slice(&update.page_cell().segment_id().get().to_le_bytes());
    target.extend_from_slice(&update.page().get().to_le_bytes());
    target.extend_from_slice(&update.page_generation().to_le_bytes());
    target.extend_from_slice(&update.data_generation().to_le_bytes());
    target.extend_from_slice(&update.data_page_count().to_le_bytes());
    target.extend_from_slice(&update.frame_index().to_le_bytes());
}

fn read_segment_update(
    bytes: &[u8],
) -> Result<RecordSegmentPageManifestEntry, PhysicalRecoveryProjectionDenial> {
    let mut cursor = Cursor::new(bytes);
    let authority = PhysicalGenerationAuthority::for_canonical_physical_format();
    let segment = PhysicalSegmentId::from_raw(cursor.u64()?)
        .map_err(|_| PhysicalRecoveryProjectionDenial::InvalidSegmentUpdate)?;
    let page = PhysicalPageId::from_raw(cursor.u64()?)
        .map_err(|_| PhysicalRecoveryProjectionDenial::InvalidSegmentUpdate)?;
    let page_generation = generation(cursor.u64()?)?;
    let data_generation = generation(cursor.u64()?)?;
    let count = cursor.u32()?;
    let index = cursor.u32()?;
    cursor.end()?;
    RecordSegmentPageManifestEntry::new(
        authority
            .page_cell(segment, page)
            .with_page_generation(page_generation),
        authority
            .segment_cell(segment)
            .with_segment_generation(data_generation),
        count,
        index,
    )
    .ok_or(PhysicalRecoveryProjectionDenial::InvalidSegmentUpdate)
}

fn write_manifest(target: &mut Vec<u8>, manifest: &PersistedPhysicalRecoveryManifest) {
    let RecordArtifactFile::ExtentManifest { extent, generation } = manifest.artifact else {
        unreachable!()
    };
    target.extend_from_slice(&extent.to_le_bytes());
    target.extend_from_slice(&generation.to_le_bytes());
    field(target, manifest.bytes());
}

fn read_manifest(
    bytes: &[u8],
) -> Result<PersistedPhysicalRecoveryManifest, PhysicalRecoveryProjectionDenial> {
    let mut cursor = Cursor::new(bytes);
    let artifact = RecordArtifactFile::ExtentManifest {
        extent: cursor.u64()?,
        generation: cursor.u64()?,
    };
    let payload = cursor.field()?;
    cursor.end()?;
    PersistedPhysicalRecoveryManifest::new(artifact, payload)
        .ok_or(PhysicalRecoveryProjectionDenial::InvalidManifest)
}

fn write_subject_coordinate(
    target: &mut Vec<u8>,
    subject: PersistedPhysicalDataFrameSubject,
    coordinate: RecordFrameCoordinate,
) {
    match subject {
        PersistedPhysicalDataFrameSubject::InlinePage(page) => {
            target.push(1);
            target.extend_from_slice(&page.segment_id().get().to_le_bytes());
            target.extend_from_slice(&page.page_id().get().to_le_bytes());
            target.extend_from_slice(&page.generation().get().to_le_bytes());
            let RecordArtifactFile::Segment {
                segment,
                generation,
            } = coordinate.artifact()
            else {
                unreachable!("an admitted inline projection frame owns a segment coordinate")
            };
            debug_assert_eq!(segment, page.segment_id().get());
            target.extend_from_slice(&generation.to_le_bytes());
        }
        PersistedPhysicalDataFrameSubject::ExtentChunk(chunk) => {
            target.push(2);
            write_record(target, chunk.record());
            target.extend_from_slice(&chunk.extent_cell().extent_id().get().to_le_bytes());
            target.extend_from_slice(&chunk.extent_cell().generation().get().to_le_bytes());
            target.extend_from_slice(&chunk.logical_bytes().to_le_bytes());
            target.extend_from_slice(&chunk.logical_offset().to_le_bytes());
            target.extend_from_slice(&chunk.ordinal().to_le_bytes());
        }
    }
    target.extend_from_slice(&coordinate.offset().to_le_bytes());
    target.extend_from_slice(&coordinate.length().to_le_bytes());
}

fn read_subject_coordinate(
    cursor: &mut Cursor<'_>,
) -> Result<
    (PersistedPhysicalDataFrameSubject, RecordFrameCoordinate),
    PhysicalRecoveryProjectionDenial,
> {
    let kind = cursor.byte()?;
    let authority = PhysicalGenerationAuthority::for_canonical_physical_format();
    let (subject, artifact) = match kind {
        1 => {
            let segment = PhysicalSegmentId::from_raw(cursor.u64()?)
                .map_err(|_| PhysicalRecoveryProjectionDenial::InvalidFrame)?;
            let page = PhysicalPageId::from_raw(cursor.u64()?)
                .map_err(|_| PhysicalRecoveryProjectionDenial::InvalidFrame)?;
            let page_generation = generation(cursor.u64()?)?;
            let artifact_generation = generation(cursor.u64()?)?;
            (
                PersistedPhysicalDataFrameSubject::InlinePage(
                    authority
                        .page_cell(segment, page)
                        .with_page_generation(page_generation),
                ),
                RecordArtifactFile::Segment {
                    segment: segment.get(),
                    generation: artifact_generation.get(),
                },
            )
        }
        2 => {
            let record = read_record(cursor)?;
            let extent = PhysicalExtentId::from_raw(cursor.u64()?)
                .map_err(|_| PhysicalRecoveryProjectionDenial::InvalidFrame)?;
            let generation = generation(cursor.u64()?)?;
            let logical_bytes = cursor.u64()?;
            let logical_offset = cursor.u64()?;
            let ordinal = cursor.u32()?;
            let cell = authority
                .record_extent_cell(extent)
                .with_extent_generation(generation);
            let chunk =
                ExtentChunkCoordinate::new(record, cell, logical_bytes, logical_offset, ordinal)
                    .ok_or(PhysicalRecoveryProjectionDenial::InvalidFrame)?;
            (
                PersistedPhysicalDataFrameSubject::ExtentChunk(chunk),
                RecordArtifactFile::Extent {
                    extent: extent.get(),
                    generation: generation.get(),
                },
            )
        }
        _ => return Err(PhysicalRecoveryProjectionDenial::InvalidFrame),
    };
    let coordinate = RecordFrameCoordinate::new(artifact, cursor.u64()?, cursor.u32()?)
        .ok_or(PhysicalRecoveryProjectionDenial::InvalidFrame)?;
    Ok((subject, coordinate))
}
