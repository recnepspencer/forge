use worth_store_physical_format::{
    prepare_extent_chunk_reusing, DurableExtentManifest, ExtentChunkCoordinate, RecordArtifactFile,
    RecordFrameCoordinate, DURABLE_EXTENT_FRAME_HEADER_BYTES, EXTENT_CHUNK_METADATA_BYTES,
};

use super::super::publication::append_observation::PublicationObservation;
use super::super::publication::{classify_first_write, CandidateDataWriteFailure};
use super::super::residency::publication_artifacts::{
    classify_candidate_write, PublicationRecordArtifacts,
};
use super::super::{
    AdmittedPhysicalRecordFormat, RecordAppendDenial, RecordStreamFailure, RecordStreamFailureKind,
    RecordWriteSource,
};

pub(in crate::physical_runtime::record_serving) struct ExtentDataPlan {
    pub(in crate::physical_runtime::record_serving) artifact: RecordArtifactFile,
    pub(in crate::physical_runtime::record_serving) manifest: DurableExtentManifest,
    pub(in crate::physical_runtime::record_serving) source: Box<dyn RecordWriteSource>,
}

pub(in crate::physical_runtime::record_serving) fn write_extent(
    artifacts: &PublicationRecordArtifacts<'_>,
    format: AdmittedPhysicalRecordFormat,
    plan: &mut ExtentDataPlan,
    residency: &mut super::super::residency::frame_ports::StoreCandidateFramePublicationSession,
    observation: &mut PublicationObservation,
) -> Result<(), CandidateDataWriteFailure> {
    let mut writer = artifacts
        .create_new_file(plan.artifact)
        .map_err(classify_first_write)?;
    let transfer = plan.manifest.chunk_payload_capacity() as usize;
    let mut completed = 0_u64;
    let mut scratch = Vec::new();
    for ordinal in 1..=plan.manifest.chunk_count() {
        let expected =
            usize::try_from((plan.manifest.logical_bytes() - completed).min(transfer as u64))
                .expect("extent frames are bounded by the admitted page size");
        let coordinate = ExtentChunkCoordinate::new(
            plan.manifest.record(),
            plan.manifest.extent_cell(),
            plan.manifest.logical_bytes(),
            completed,
            ordinal,
        )
        .ok_or(CandidateDataWriteFailure::Semantic(
            RecordAppendDenial::PublishedLayoutDamaged,
        ))?;
        let mut frame =
            prepare_extent_chunk_reusing(format.declaration(), coordinate, expected, scratch)
                .map_err(|_| {
                    CandidateDataWriteFailure::Semantic(RecordAppendDenial::PublishedLayoutDamaged)
                })?;
        observation.observe_scratch(
            DURABLE_EXTENT_FRAME_HEADER_BYTES + EXTENT_CHUNK_METADATA_BYTES + expected,
        );
        let buffer = frame.payload_mut();
        let mut filled = 0;
        while filled < expected {
            let count = plan
                .source
                .read_next(&mut buffer[filled..expected])
                .map_err(|_| producer_failure(completed + filled as u64))?;
            if count == 0 {
                return Err(stream_failure(
                    RecordStreamFailureKind::SourceEndedEarly,
                    completed + filled as u64,
                ));
            }
            if count > expected - filled {
                return Err(stream_failure(
                    RecordStreamFailureKind::InvalidTransferCount,
                    completed + filled as u64,
                ));
            }
            observation.observe_copy(count);
            filled += count;
        }
        let sealed = frame.seal();
        let offset = writer.completed_bytes();
        let candidate_coordinate =
            super::super::residency::frame_ports::CandidateFrameCoordinate::new(
                plan.artifact,
                offset,
            );
        let physical_coordinate = RecordFrameCoordinate::new(
            plan.artifact,
            offset,
            u32::try_from(sealed.len()).expect("extent frames are u32-bounded"),
        )
        .expect("extent frames are nonempty and offset-bounded");
        let frame = residency
            .write_frame(
                super::super::residency::frame_ports::CandidateFrame::new(
                    super::super::residency::frame_ports::CandidateFrameRole::ExtentChunk,
                    candidate_coordinate,
                    sealed,
                ),
                &mut |bytes| {
                    classify_candidate_write(writer.write_exact_chunk(physical_coordinate, bytes))
                },
            )
            .map_err(CandidateDataWriteFailure::from_frame_write)?;
        observation.observe_transfer(frame.frame_bytes() as usize);
        scratch = frame.into_reusable_bytes().unwrap_or_default();
        completed += expected as u64;
    }
    reject_trailing_source_bytes(plan, completed)
}

fn reject_trailing_source_bytes(
    plan: &mut ExtentDataPlan,
    completed: u64,
) -> Result<(), CandidateDataWriteFailure> {
    let mut extra = [0_u8; 1];
    let extra_count = plan
        .source
        .read_next(&mut extra)
        .map_err(|_| producer_failure(completed))?;
    if extra_count != 0 {
        return Err(stream_failure(
            RecordStreamFailureKind::SourceExceededDeclaredLength,
            completed,
        ));
    }
    Ok(())
}

fn producer_failure(completed: u64) -> CandidateDataWriteFailure {
    stream_failure(RecordStreamFailureKind::ProducerRejected, completed)
}

fn stream_failure(kind: RecordStreamFailureKind, completed: u64) -> CandidateDataWriteFailure {
    CandidateDataWriteFailure::Stream(RecordStreamFailure::after_media_write(kind, completed))
}
