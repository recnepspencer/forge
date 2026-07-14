use worth_store_blob_chunks::{BlobResumeReplayOutcome, BlobResumeUnfinishedState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobResumeCrashPoint {
    AfterChunkWrite,
    AfterSessionCheckpoint,
    AfterChunkTreeNodeWrite,
    BeforeRootPublication,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlobResumeExpectedOutcome {
    ResumesRootPublication {
        session_digest: String,
        chunk_tree_root_digest: String,
        logical_content_digest: String,
    },
    DeniesWithLocalizedUnfinishedState(BlobResumeUnfinishedState),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobResumeRecoveryScenario {
    crash_point: BlobResumeCrashPoint,
    expected: BlobResumeExpectedOutcome,
}

impl BlobResumeRecoveryScenario {
    pub fn from_replay_outcome(
        crash_point: BlobResumeCrashPoint,
        outcome: BlobResumeReplayOutcome,
    ) -> Self {
        let expected = match outcome {
            BlobResumeReplayOutcome::RootPublicationReady(ready) => {
                BlobResumeExpectedOutcome::ResumesRootPublication {
                    session_digest: ready.session_digest().to_owned(),
                    chunk_tree_root_digest: ready.chunk_tree_root_digest().to_owned(),
                    logical_content_digest: ready.logical_content_digest().to_owned(),
                }
            }
            BlobResumeReplayOutcome::Unfinished { state, .. } => {
                BlobResumeExpectedOutcome::DeniesWithLocalizedUnfinishedState(state)
            }
        };
        Self {
            crash_point,
            expected,
        }
    }

    pub const fn crash_point(&self) -> BlobResumeCrashPoint {
        self.crash_point
    }

    pub const fn expected(&self) -> &BlobResumeExpectedOutcome {
        &self.expected
    }
}
