use super::*;

pub(super) fn decode_targets(
    cursor: &mut Cursor<'_>,
    total: &mut u64,
    maximum: u64,
    distinct: &mut Option<(&mut BTreeSet<PhysicalRedoTargetIdentity>, u64)>,
) -> Result<Box<[PhysicalRedoTarget]>, PhysicalRedoPlanningDenial> {
    let count = cursor.u64()?;
    if count == 0 {
        return Err(PhysicalRedoPlanningDenial::InvalidTarget);
    }
    *total = total
        .checked_add(count)
        .ok_or(PhysicalRedoPlanningDenial::CounterOverflow)?;
    if *total > maximum {
        return Err(PhysicalRedoPlanningDenial::TargetLimit);
    }
    let mut targets = Vec::with_capacity(
        usize::try_from(count).map_err(|_| PhysicalRedoPlanningDenial::TargetLimit)?,
    );
    let mut prior = None;
    for _ in 0..count {
        let encoded = cursor.field()?;
        let resulting_digest = cursor.array()?;
        let target = decode_target(encoded, resulting_digest)?;
        if let Some((identities, maximum_distinct)) = distinct.as_mut() {
            let identity = target.identity();
            if !identities.contains(&identity) && identities.len() as u64 == *maximum_distinct {
                return Err(PhysicalRedoPlanningDenial::DistinctTargetLimit);
            }
            identities.insert(identity);
        }
        let order = target.canonical_order();
        if prior.as_ref().is_some_and(|prior| prior > &order) {
            return Err(PhysicalRedoPlanningDenial::NonCanonicalTargetOrder);
        }
        prior = Some(order);
        targets.push(target);
    }
    Ok(targets.into_boxed_slice())
}

fn decode_target(
    encoded: &[u8],
    resulting_digest: [u8; 32],
) -> Result<PhysicalRedoTarget, PhysicalRedoPlanningDenial> {
    let mut cursor = Cursor::new(encoded);
    let kind = cursor.byte()?;
    let (identity, extent_coordinate) = match kind {
        1 => (decode_inline_identity(&mut cursor)?, None),
        2 => {
            let (identity, coordinate) = decode_extent_identity(&mut cursor)?;
            (identity, Some(coordinate))
        }
        _ => return Err(PhysicalRedoPlanningDenial::InvalidTarget),
    };
    let artifact = cursor.byte()?;
    let (artifact_identity, artifact_generation) = (cursor.u64()?, cursor.u64()?);
    let artifact_offset = cursor.u64()?;
    let artifact_length = cursor.u32()?;
    cursor.require_end()?;
    let artifact = match identity {
        PhysicalRedoTargetIdentity::InlinePage { segment, .. }
            if artifact == 5 && artifact_identity == segment =>
        {
            RecordArtifactFile::Segment {
                segment,
                generation: artifact_generation,
            }
        }
        PhysicalRedoTargetIdentity::ExtentChunk {
            extent, generation, ..
        } if (artifact, artifact_identity, artifact_generation) == (8, extent, generation) => {
            RecordArtifactFile::Extent { extent, generation }
        }
        _ => return Err(PhysicalRedoPlanningDenial::InvalidTarget),
    };
    if artifact_length == 0 {
        return Err(PhysicalRedoPlanningDenial::InvalidTarget);
    }
    Ok(PhysicalRedoTarget {
        identity,
        extent_coordinate,
        artifact,
        artifact_offset,
        artifact_length,
        resulting_digest,
    })
}

fn decode_inline_identity(
    cursor: &mut Cursor<'_>,
) -> Result<PhysicalRedoTargetIdentity, PhysicalRedoPlanningDenial> {
    let segment = cursor.u64()?;
    let page = cursor.u64()?;
    let generation = cursor.u64()?;
    if segment == 0 || page == 0 || generation == 0 {
        return Err(PhysicalRedoPlanningDenial::InvalidTarget);
    }
    Ok(PhysicalRedoTargetIdentity::InlinePage {
        segment,
        page,
        generation,
    })
}

fn decode_extent_identity(
    cursor: &mut Cursor<'_>,
) -> Result<(PhysicalRedoTargetIdentity, PhysicalRedoExtentCoordinate), PhysicalRedoPlanningDenial>
{
    let allocation_epoch = cursor.array()?;
    let record_ordinal = cursor.u64()?;
    let extent = cursor.u64()?;
    let generation = cursor.u64()?;
    let logical_bytes = cursor.u64()?;
    let logical_offset = cursor.u64()?;
    let chunk = cursor.u32()?;
    if allocation_epoch == [0; 16]
        || record_ordinal == 0
        || extent == 0
        || generation == 0
        || logical_bytes == 0
        || logical_offset >= logical_bytes
        || chunk == 0
    {
        return Err(PhysicalRedoPlanningDenial::InvalidTarget);
    }
    Ok((
        PhysicalRedoTargetIdentity::ExtentChunk {
            extent,
            generation,
            chunk,
        },
        PhysicalRedoExtentCoordinate {
            allocation_epoch,
            record_ordinal,
            logical_bytes,
            logical_offset,
        },
    ))
}
