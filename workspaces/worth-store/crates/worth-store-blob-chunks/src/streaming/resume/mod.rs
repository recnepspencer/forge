//! Resume proof grammar: verify_request_match → resume_bounded_ingest → bind_resume_session.
mod transitions;
mod types;
mod verification;

#[cfg(test)]
mod equivalence_tests;
#[cfg(test)]
mod resume_tests;

pub use transitions::resume_bounded_ingest::run_resumable_streaming_ingest;
pub use types::{BlobStreamingResumeAdmission, BlobStreamingResumePosture};
