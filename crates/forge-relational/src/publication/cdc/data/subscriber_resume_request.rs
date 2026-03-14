use crate::publication::cdc::data::SubscriberCheckpoint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubscriberResumeRequest {
    checkpoint: Option<SubscriberCheckpoint>,
    max_commits: usize,
}

impl SubscriberResumeRequest {
    pub fn from_head(max_commits: usize) -> Self {
        Self {
            checkpoint: None,
            max_commits,
        }
    }

    pub fn resume_after(checkpoint: SubscriberCheckpoint, max_commits: usize) -> Self {
        Self {
            checkpoint: Some(checkpoint),
            max_commits,
        }
    }

    pub fn checkpoint(&self) -> Option<&SubscriberCheckpoint> {
        self.checkpoint.as_ref()
    }

    pub fn max_commits(&self) -> usize {
        self.max_commits
    }
}

impl Default for SubscriberResumeRequest {
    fn default() -> Self {
        Self::from_head(usize::MAX)
    }
}
