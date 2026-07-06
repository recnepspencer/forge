use super::super::types::BlobStreamingResumeAdmission;
use crate::{BlobStreamingIngestDenial, BlobStreamingIngestRequest};

pub(crate) fn verify_resume_request_matches(
    resume_admission: &BlobStreamingResumeAdmission,
    request: &BlobStreamingIngestRequest,
) -> Result<(), BlobStreamingIngestDenial> {
    if request.security_metadata() == resume_admission.security_metadata()
        && request.rule() == resume_admission.chunking_rule()
        && request.declared_total_bytes() == resume_admission.declared_total_bytes()
    {
        Ok(())
    } else {
        Err(BlobStreamingIngestDenial::ResumeSessionRequestMismatch)
    }
}