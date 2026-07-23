use worth_store_physical_format::{
    append_inline_records_owned, InlineRecordAppend, PageGenerationCell, PersistedRecordIdentity,
    RecordArtifactFile, RecordFrameCoordinate, SlotGenerationCell,
};

use super::super::publication::append_observation::PublicationObservation;
use super::super::publication::{classify_first_write, CandidateDataWriteFailure};
use super::super::residency::publication_artifacts::{
    classify_candidate_write, PublicationRecordArtifacts,
};
use super::super::{AdmittedPhysicalRecordFormat, RecordAppendDenial};

pub(in crate::physical_runtime::record_serving) struct SegmentDataPlan {
    pub(in crate::physical_runtime::record_serving) artifact: RecordArtifactFile,
    pub(in crate::physical_runtime::record_serving) pages: Vec<PageDataPlan>,
}

pub(in crate::physical_runtime::record_serving) struct PageDataPlan {
    pub(in crate::physical_runtime::record_serving) page: PageGenerationCell,
    pub(in crate::physical_runtime::record_serving) existing_frame: Option<Vec<u8>>,
    pub(in crate::physical_runtime::record_serving) records:
        Vec<(PersistedRecordIdentity, SlotGenerationCell, Vec<u8>)>,
}

pub(in crate::physical_runtime::record_serving) fn write_segment(
    artifacts: &PublicationRecordArtifacts<'_>,
    format: AdmittedPhysicalRecordFormat,
    plan: &mut SegmentDataPlan,
    residency: &mut super::super::residency::frame_ports::StoreCandidateFramePublicationSession,
    observation: &mut PublicationObservation,
) -> Result<(), CandidateDataWriteFailure> {
    let mut writer = artifacts
        .create_new_file(plan.artifact)
        .map_err(classify_first_write)?;
    for page in &mut plan.pages {
        let records = page
            .records
            .iter()
            .map(|(record, slot, bytes)| InlineRecordAppend::new(*record, *slot, bytes.as_slice()))
            .collect::<Vec<_>>();
        let (candidate, _) = append_inline_records_owned(
            format.declaration(),
            page.page,
            page.existing_frame.take(),
            &records,
        )
        .map_err(|_| {
            CandidateDataWriteFailure::Semantic(RecordAppendDenial::PublishedLayoutDamaged)
        })?;
        observation.observe_scratch(candidate.len());
        for (_, _, bytes) in &page.records {
            observation.observe_copy(bytes.len());
        }
        let offset = writer.completed_bytes();
        let coordinate = super::super::residency::frame_ports::CandidateFrameCoordinate::new(
            plan.artifact,
            offset,
        );
        let physical_coordinate = RecordFrameCoordinate::new(
            plan.artifact,
            offset,
            u32::try_from(candidate.len()).expect("page frames are u32-bounded"),
        )
        .expect("page frames are nonempty and offset-bounded");
        let resident = residency
            .write_frame(
                super::super::residency::frame_ports::CandidateFrame::new(
                    super::super::residency::frame_ports::CandidateFrameRole::InlinePage,
                    coordinate,
                    candidate,
                ),
                &mut |bytes| {
                    classify_candidate_write(writer.write_exact_chunk(physical_coordinate, bytes))
                },
            )
            .map_err(CandidateDataWriteFailure::from_frame_write)?;
        observation.observe_transfer(resident.frame_bytes() as usize);
    }
    Ok(())
}
