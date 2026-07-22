use std::collections::BTreeMap;

use worth_store_physical_format::{
    CurrentPhysicalRecordPlacement, DurableExtentManifest, DurableExtentRecordPlacement,
    PersistedRecordIdentity, PhysicalGeneration, PhysicalGenerationAuthority, RecordArtifactFile,
    DURABLE_EXTENT_FRAME_HEADER_BYTES, EXTENT_CHUNK_METADATA_BYTES,
};

use super::super::{
    planning::batch_placement::ExtentInput, publication::extent_publication::ExtentDataPlan,
    publication::CandidateDataArtifact, AdmittedPhysicalRecordFormat, RecordAllocationFrontier,
    RecordAppendDenial, RecordAppendError,
};

pub(in crate::physical_runtime::record_serving) fn lower_extents(
    format: AdmittedPhysicalRecordFormat,
    frontier: &mut RecordAllocationFrontier,
    extents: Vec<ExtentInput>,
    data: &mut Vec<CandidateDataArtifact>,
    manifests: &mut Vec<(RecordArtifactFile, Vec<u8>)>,
    placements: &mut BTreeMap<PersistedRecordIdentity, CurrentPhysicalRecordPlacement>,
) -> Result<(), RecordAppendError> {
    for extent in extents {
        lower_extent(format, frontier, extent, data, manifests, placements)?;
    }
    Ok(())
}

fn lower_extent(
    format: AdmittedPhysicalRecordFormat,
    frontier: &mut RecordAllocationFrontier,
    extent: ExtentInput,
    data: &mut Vec<CandidateDataArtifact>,
    manifests: &mut Vec<(RecordArtifactFile, Vec<u8>)>,
    placements: &mut BTreeMap<PersistedRecordIdentity, CurrentPhysicalRecordPlacement>,
) -> Result<(), RecordAppendError> {
    let extent_id = frontier.allocate_extent().ok_or(RecordAppendError::Denied(
        RecordAppendDenial::PhysicalIdentityExhausted,
    ))?;
    let maximum_frame_bytes = format.declaration().page_size().bytes();
    let chunk_payload_capacity = maximum_frame_bytes
        .checked_sub((DURABLE_EXTENT_FRAME_HEADER_BYTES + EXTENT_CHUNK_METADATA_BYTES) as u32)
        .ok_or(RecordAppendError::Denied(
            RecordAppendDenial::RecordTooLarge,
        ))?;
    let chunk_count = u32::try_from(extent.length.div_ceil(u64::from(chunk_payload_capacity)))
        .map_err(|_| RecordAppendError::Denied(RecordAppendDenial::RecordTooLarge))?;
    let extent_generation = PhysicalGenerationAuthority::for_canonical_physical_format()
        .record_extent_cell(extent_id)
        .with_extent_generation(
            PhysicalGeneration::from_raw(1).expect("initial extent generation is nonzero"),
        );
    let manifest = DurableExtentManifest::new(
        format.declaration(),
        extent.record,
        extent_generation,
        extent.length,
        maximum_frame_bytes,
        chunk_count,
    )
    .ok_or(RecordAppendError::Denied(
        RecordAppendDenial::RecordTooLarge,
    ))?;
    let artifact = RecordArtifactFile::Extent {
        extent: extent_id.get(),
        generation: extent_generation.generation().get(),
    };
    manifests.push((
        RecordArtifactFile::ExtentManifest {
            extent: extent_id.get(),
            generation: extent_generation.generation().get(),
        },
        manifest.encode(format.declaration()),
    ));
    data.push(CandidateDataArtifact::Extent(ExtentDataPlan {
        artifact,
        manifest,
        source: extent.source,
    }));
    placements.insert(
        extent.record,
        CurrentPhysicalRecordPlacement::Extent(
            DurableExtentRecordPlacement::new(extent.record, extent_generation, extent.length)
                .ok_or(RecordAppendError::Denied(
                    RecordAppendDenial::RecordTooLarge,
                ))?,
        ),
    );
    Ok(())
}
