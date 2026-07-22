pub(super) mod append;
pub(super) mod append_observation;
pub(super) mod batch;
mod catalog_cutover_preflight;
pub(super) mod extent_publication;
mod failure_classification;
pub(super) mod orchestration;
pub(super) mod publication_outcome;
pub(super) mod publication_progression;
pub(super) mod publication_residue;
pub(super) mod segment_publication;
pub(super) mod streaming;

pub(in crate::physical_runtime::record_serving) use failure_classification::{
    classify_catalog_replacement_failure, classify_first_write,
};
pub use orchestration::RecordPublicationStage;
pub(in crate::physical_runtime::record_serving) use orchestration::{
    execute_publication, indeterminate, unpublished_backend, unpublished_candidate_frame_contract,
    unpublished_residency, unpublished_semantic, unpublished_stream, write_candidate_data,
    CandidateDataArtifact, CandidateDataWriteFailure, PublicationPlan,
};
