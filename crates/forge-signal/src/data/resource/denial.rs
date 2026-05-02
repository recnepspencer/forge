use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct AsyncDenialId(u64);

impl AsyncDenialId {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn get(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompletionDenialClass {
    Stale,
    Superseded,
    Malformed,
    Partial,
    Contradictory,
    Duplicate,
    UnknownRequest,
    Retired,
    RetainedHistoryUnavailable,
    Cancelled,
    Rejected,
    TimedOut,
    Impossible,
}
