#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobStreamingResumePosture {
    frontier_chunk_tree_root_digest: String,
    total_frontier_bytes: u64,
    resume_session_digest: Option<String>,
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