use worth_store_physical_format::{
    append_inline_records_owned, InlineRecordAppend, PageGenerationCell, PersistedRecordIdentity,
    RecordArtifactFile, RecordFrameCoordinate, SlotGenerationCell,
};

use super::super::publication::append_observation::PublicationObservation;
use super::super::publication::CandidateDataWriteFailure;
use super::super::residency::publication_artifacts::PublicationRecordArtifacts;
use super::super::{AdmittedPhysicalRecordFormat, RecordAppendDenial};
use super::RecordPublicationStage;

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
    residency: &mut super::super::residency::frame_ports::StoreCandidateFramePublicationSession<'_>,
    observation: &mut PublicationObservation,
    work: &mut super::RecordPublicationWorkTrace,
) -> Result<(), CandidateDataWriteFailure> {
    let mut stage = artifacts.at(RecordPublicationStage::CandidateDataWrite, work);
    let mut completed_bytes = 0_u64;
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
        let offset = completed_bytes;
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
        let frame = super::super::residency::frame_ports::CandidateFrame::new(
            super::super::residency::frame_ports::CandidateFrameRole::InlinePage,
            coordinate,
            candidate,
        );
        let resident = if offset == 0 {
            stage.write_new_candidate(residency, frame, plan.artifact)
        } else {
            stage.append_candidate(residency, frame, physical_coordinate)
        }
        .map_err(CandidateDataWriteFailure::from_frame_write)?;
        observation.observe_transfer(resident.frame_bytes() as usize);
        completed_bytes = completed_bytes.saturating_add(resident.frame_bytes());
    }
    Ok(())
}
