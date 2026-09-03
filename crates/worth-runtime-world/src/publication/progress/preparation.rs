use worth_relational::facade::mvcc::PreparedRelationalCommitCandidate;

use super::{RelationalAttemptProgress, RelationalAttemptProgressPosture};

/// Non-sendable preparation custody is kept outside the post-effect progress
/// row. A candidate can cross into owner publication, but it can never be
/// installed in the cross-thread recovery catalog.
#[derive(Debug)]
pub(crate) struct PreparedRelationalAttemptProgress {
    candidate: PreparedRelationalCommitCandidate,
}

impl PreparedRelationalAttemptProgress {
    pub(crate) fn new(candidate: PreparedRelationalCommitCandidate) -> Self {
        Self { candidate }
    }

    pub(crate) fn posture(&self) -> RelationalAttemptProgressPosture {
        RelationalAttemptProgressPosture::Prepared
    }

    pub(crate) fn into_candidate(self) -> PreparedRelationalCommitCandidate {
        self.candidate
    }
}

impl RelationalAttemptProgress {
    pub(crate) fn prepared(
        candidate: PreparedRelationalCommitCandidate,
    ) -> PreparedRelationalAttemptProgress {
        PreparedRelationalAttemptProgress::new(candidate)
    }
}
