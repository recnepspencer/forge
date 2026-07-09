use serde::{Deserialize, Serialize};

use super::request::{
    ResourceAttemptId, ResourceBranchEpoch, ResourceGeneration, ResourceRequestHandle,
    ResourceRequestId,
};

/// Runtime-owned proof that a resource request was admitted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdmittedResourceRequest {
    handle: ResourceRequestHandle,
    attempt: ResourceAttemptId,
}

impl AdmittedResourceRequest {
    #[allow(dead_code)]
    pub(crate) fn new(
        request_id: ResourceRequestId,
        generation: ResourceGeneration,
        branch_epoch: ResourceBranchEpoch,
        attempt: ResourceAttemptId,
    ) -> Self {
        Self {
            handle: ResourceRequestHandle::new(request_id, generation, branch_epoch),
            attempt,
        }
    }

    pub fn handle(self) -> ResourceRequestHandle {
        self.handle
    }

    pub fn attempt(self) -> ResourceAttemptId {
        self.attempt
    }
}
