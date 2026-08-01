use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlatformPulseIntentWatcherShutdownEvidence {
    worker_joined: bool,
    pending_input_count: u64,
}

impl PlatformPulseIntentWatcherShutdownEvidence {
    pub fn new(worker_joined: bool, pending_input_count: u64) -> Self {
        Self {
            worker_joined,
            pending_input_count,
        }
    }

    pub fn worker_joined(self) -> bool {
        self.worker_joined
    }

    pub fn pending_input_count(self) -> u64 {
        self.pending_input_count
    }
}
