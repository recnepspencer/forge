use super::*;

pub(super) fn validate_extent_closure(
    placement: worth_store_physical_format::DurableExtentRecordPlacement,
    projection: &PersistedPhysicalRecoveryProjection,
    format: PhysicalRecordFormatDeclaration,
) -> Result<(), PhysicalRedoPlanningDenial> {
    let manifests = projection
        .manifests()
        .iter()
        .filter(|manifest| {
            manifest.artifact()
                == worth_store_physical_format::RecordArtifactFile::ExtentManifest {
                    extent: placement.extent().get(),
                    generation: placement.extent_generation(),
                }
        })
        .collect::<Vec<_>>();
    if manifests.len() != 1 {
        return Err(PhysicalRedoPlanningDenial::InvalidRecoveryProjection);
    }
    let (manifest, decoded_format) = DurableExtentManifest::decode(manifests[0].bytes())
        .map_err(|_| PhysicalRedoPlanningDenial::InvalidRecoveryProjection)?;
    if decoded_format != format
        || manifest.record() != placement.record()
        || manifest.extent_cell() != placement.extent_cell()
        || manifest.logical_bytes() != placement.payload_bytes()
    {
        return Err(PhysicalRedoPlanningDenial::InvalidRecoveryProjection);
    }
    let mut chunks = projection
        .frames()
        .iter()
        .filter_map(|frame| match frame.subject() {
            PersistedPhysicalDataFrameSubject::ExtentChunk(coordinate)
                if coordinate.record() == placement.record()
                    && coordinate.extent_cell() == placement.extent_cell() =>
            {
                Some((coordinate, frame))
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    chunks.sort_by_key(|(coordinate, _)| coordinate.ordinal());
    if chunks.len() != manifest.chunk_count() as usize {
        return Err(PhysicalRedoPlanningDenial::InvalidRecoveryProjection);
    }
    let mut logical_offset = 0_u64;
    for (ordinal, (coordinate, frame)) in (1_u32..).zip(chunks) {
        let (bytes, found_format) = decode_extent_chunk(frame.bytes(), coordinate)
            .map_err(|_| PhysicalRedoPlanningDenial::InvalidRecoveryProjection)?;
        if found_format != format
            || coordinate.ordinal() != ordinal
            || coordinate.logical_offset() != logical_offset
            || coordinate.logical_bytes() != manifest.logical_bytes()
            || frame.bytes().len() > manifest.maximum_frame_bytes() as usize
        {
            return Err(PhysicalRedoPlanningDenial::InvalidRecoveryProjection);
        }
        logical_offset = logical_offset
            .checked_add(bytes.len() as u64)
            .ok_or(PhysicalRedoPlanningDenial::CounterOverflow)?;
    }
    if logical_offset != manifest.logical_bytes() {
        return Err(PhysicalRedoPlanningDenial::InvalidRecoveryProjection);
    }
    Ok(())
}

pub(super) fn projected_record_bytes(
    identity: worth_store_physical_format::PersistedRecordIdentity,
    placement: CurrentPhysicalRecordPlacement,
    projection: &PersistedPhysicalRecoveryProjection,
    format: PhysicalRecordFormatDeclaration,
) -> Result<Vec<u8>, PhysicalRedoPlanningDenial> {
    match placement {
        CurrentPhysicalRecordPlacement::Inline(value) => {
            let frames = projection
                .frames()
                .iter()
                .filter(|frame| {
                    frame.subject()
                        == PersistedPhysicalDataFrameSubject::InlinePage(value.page_cell())
                })
                .collect::<Vec<_>>();
            if frames.len() != 1 {
                return Err(PhysicalRedoPlanningDenial::InvalidRecoveryProjection);
            }
            let (range, decoded_format) = decode_inline_record(
                frames[0].bytes(),
                identity,
                value.page_cell(),
                value.slot_cell(),
            )
            .map_err(|_| PhysicalRedoPlanningDenial::InvalidRecoveryProjection)?;
            if decoded_format != format {
                return Err(PhysicalRedoPlanningDenial::InvalidRecoveryProjection);
            }
            Ok(frames[0].bytes()[range.range()].to_vec())
        }
        CurrentPhysicalRecordPlacement::Extent(value) => {
            let mut chunks = projection
                .frames()
                .iter()
                .filter_map(|frame| match frame.subject() {
                    PersistedPhysicalDataFrameSubject::ExtentChunk(coordinate)
                        if coordinate.record() == identity
                            && coordinate.extent_cell() == value.extent_cell() =>
                    {
                        Some((coordinate, frame))
                    }
                    _ => None,
                })
                .collect::<Vec<_>>();
            chunks.sort_by_key(|(coordinate, _)| coordinate.ordinal());
            let mut payload = Vec::new();
            for (expected_ordinal, (coordinate, frame)) in (1_u32..).zip(chunks) {
                if coordinate.ordinal() != expected_ordinal
                    || coordinate.logical_offset() != payload.len() as u64
                    || coordinate.logical_bytes() != value.payload_bytes()
                {
                    return Err(PhysicalRedoPlanningDenial::InvalidRecoveryProjection);
                }
                let (chunk, decoded_format) = decode_extent_chunk(frame.bytes(), coordinate)
                    .map_err(|_| PhysicalRedoPlanningDenial::InvalidRecoveryProjection)?;
                if decoded_format != format {
                    return Err(PhysicalRedoPlanningDenial::InvalidRecoveryProjection);
                }
                payload.extend_from_slice(chunk);
            }
            if payload.len() as u64 != value.payload_bytes() {
                return Err(PhysicalRedoPlanningDenial::InvalidRecoveryProjection);
            }
            Ok(payload)
        }
    }
}
