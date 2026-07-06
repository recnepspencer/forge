use forge_store_budgets::{AllocationEnvelopeSet, CounterEvidenceStrength};
use forge_store_buffer_pool::AllocationReceipt;

use crate::{
    BlobChunkSecurityMetadataWitness, BlobChunkingRuleAdmission, BlobResumeSessionAdmitted,
    BlobStreamingChunkWriter, BlobStreamingIngest, BlobStreamingIngestDenial,
    BlobStreamingIngestRequest, BlobStreamingPressureAdmission, BlobStreamingSourceFrame,
    BlobStreamingWindow,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobStreamingResumePosture {
    frontier_chunk_tree_root_digest: String,
    total_frontier_bytes: u64,
    resume_session_digest: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobStreamingResumeAdmission {
    session_digest: String,
    authority_digest: String,
    security_metadata: BlobChunkSecurityMetadataWitness,
    chunking_rule: BlobChunkingRuleAdmission,
    declared_total_bytes: u64,
}

impl BlobStreamingResumePosture {
    pub(crate) fn from_frontier(frontier: &crate::BlobStreamingContentFrontier) -> Self {
        Self {
            frontier_chunk_tree_root_digest: frontier
                .chunk_tree_root()
                .digest()
                .as_str()
                .to_owned(),
            total_frontier_bytes: frontier.proof_frontier().total_bytes(),
            resume_session_digest: None,
        }
    }

    pub(crate) fn with_resume_session(mut self, session_digest: &str) -> Self {
        self.resume_session_digest = Some(session_digest.to_owned());
        self
    }

    pub fn frontier_chunk_tree_root_digest(&self) -> &str {
        &self.frontier_chunk_tree_root_digest
    }

    pub const fn total_frontier_bytes(&self) -> u64 {
        self.total_frontier_bytes
    }

    pub fn resume_session_digest(&self) -> Option<&str> {
        self.resume_session_digest.as_deref()
    }
}

impl BlobStreamingResumeAdmission {
    pub fn from_admitted_resume_session(session: &BlobResumeSessionAdmitted) -> Self {
        Self {
            session_digest: session.export_session_id().as_str().to_owned(),
            authority_digest: session.authority_digest().to_owned(),
            security_metadata: session.security_metadata(),
            chunking_rule: session.chunking_rule().clone(),
            declared_total_bytes: session.declared_total_bytes(),
        }
    }

    pub fn session_digest(&self) -> &str {
        &self.session_digest
    }

    pub fn authority_digest(&self) -> &str {
        &self.authority_digest
    }

    fn admits_request(&self, request: &BlobStreamingIngestRequest) -> bool {
        request.security_metadata() == self.security_metadata
            && request.rule() == &self.chunking_rule
            && request.declared_total_bytes() == self.declared_total_bytes
    }
}

pub fn run_resumable_streaming_ingest<W>(
    request: BlobStreamingIngestRequest,
    resume_admission: BlobStreamingResumeAdmission,
    window: BlobStreamingWindow,
    allocation: AllocationReceipt,
    envelopes: AllocationEnvelopeSet,
    pressure: BlobStreamingPressureAdmission,
    source_frames: impl IntoIterator<Item = BlobStreamingSourceFrame>,
    writer: &mut W,
    counter_strength: CounterEvidenceStrength,
) -> Result<BlobStreamingIngest, BlobStreamingIngestDenial>
where
    W: BlobStreamingChunkWriter,
{
    if !resume_admission.admits_request(&request) {
        return Err(BlobStreamingIngestDenial::ResumeSessionRequestMismatch);
    }
    BlobStreamingIngest::run_bounded(
        request,
        window,
        allocation,
        envelopes,
        pressure,
        source_frames,
        writer,
        counter_strength,
    )
    .map(|ingest| ingest.bind_resume_admission(resume_admission))
}
