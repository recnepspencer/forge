use super::super::types::BlobStreamingResumeAdmission;
use crate::BlobStreamingIngest;

pub(crate) fn bind_resume_session(
    ingest: BlobStreamingIngest,
    resume_admission: BlobStreamingResumeAdmission,
) -> BlobStreamingIngest {
    ingest.bind_resume_admission(resume_admission)
}