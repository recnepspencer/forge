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
    format: AdmittedPhysicalRecordFormat,
    data: &mut CandidateDataArtifact,
    residency: &mut StoreCandidateFramePublicationSession<'_>,
    observation: &mut PublicationObservation,
    work: &mut super::RecordPublicationWorkTrace,
) -> Result<(), CandidateDataWriteFailure> {
    match data {
        CandidateDataArtifact::Segment(plan) => segment_publication::write_segment(
            artifacts,
            format,
            plan,
            residency,
            observation,
            work,
        ),
        CandidateDataArtifact::Extent(plan) => {
            extent_publication::write_extent(artifacts, format, plan, residency, observation, work)
        }
    }
}
