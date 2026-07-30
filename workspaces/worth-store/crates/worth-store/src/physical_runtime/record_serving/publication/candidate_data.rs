use super::super::{
    publication::{
        append_observation::PublicationObservation, extent_publication, segment_publication,
    },
    residency::{
        frame_ports::StoreCandidateFramePublicationSession,
        publication_artifacts::PublicationRecordArtifacts,
    },
    AdmittedPhysicalRecordFormat,
};

use super::{CandidateDataArtifact, CandidateDataWriteFailure};

pub(in crate::physical_runtime::record_serving) fn write_candidate_data(
    artifacts: &PublicationRecordArtifacts<'_>,
    writeback: &super::super::residency::FrameWritebackPort,
    format: AdmittedPhysicalRecordFormat,
    data: &mut CandidateDataArtifact,
    residency: &mut StoreCandidateFramePublicationSession<'_>,
    observation: &mut PublicationObservation,
    work: &mut super::RecordPublicationWorkTrace,
) -> Result<(), CandidateDataWriteFailure> {
    match data {
        CandidateDataArtifact::Segment(plan) => segment_publication::write_segment(
            artifacts,
            writeback,
            format,
            plan,
            residency,
            observation,
            work,
        ),
        CandidateDataArtifact::Extent(plan) => extent_publication::write_extent(
            artifacts,
            writeback,
            format,
            plan,
            residency,
            observation,
            work,
        ),
    }
}
