use worth_store_blob_chunks::{BlobResumeReplayOutcome, BlobResumeUnfinishedState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S7BlobResumeCrashPoint {
    AfterChunkWrite,
    AfterSessionCheckpoint,
    AfterChunkTreeNodeWrite,
    BeforeRootPublication,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum S7BlobResumeExpectedOutcome {
    ResumesRootPublication {
        session_digest: String,
        chunk_tree_root_digest: String,
        logical_content_digest: String,
    },
    DeniesWithLocalizedUnfinishedState(BlobResumeUnfinishedState),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S7BlobResumeRecoveryScenario {
    crash_point: S7BlobResumeCrashPoint,
    expected: S7BlobResumeExpectedOutcome,
}

impl S7BlobResumeRecoveryScenario {
    pub fn from_replay_outcome(
        crash_point: S7BlobResumeCrashPoint,
        outcome: BlobResumeReplayOutcome,
    ) -> Self {
        let expected = match outcome {
            BlobResumeReplayOutcome::RootPublicationReady(ready) => {
                S7BlobResumeExpectedOutcome::ResumesRootPublication {
                    session_digest: ready.session_digest().to_owned(),
                    chunk_tree_root_digest: ready.chunk_tree_root_digest().to_owned(),
                    logical_content_digest: ready.logical_content_digest().to_owned(),
                }
            }
            BlobResumeReplayOutcome::Unfinished { state, .. } => {
                S7BlobResumeExpectedOutcome::DeniesWithLocalizedUnfinishedState(state)
            }
        };
        Self {
            crash_point,
            expected,
        }
    }

    pub const fn crash_point(&self) -> S7BlobResumeCrashPoint {
        self.crash_point
    }

    pub const fn expected(&self) -> &S7BlobResumeExpectedOutcome {
        &self.expected
    }
}
