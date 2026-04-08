use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeRouteRequest {
    committed_patch: crate::adapter::RelationalCommittedPatchRequest,
}

impl BridgeRouteRequest {
    pub fn for_commit(commit_identity: impl Into<Arc<str>>) -> Self {
        Self {
            committed_patch: crate::adapter::RelationalCommittedPatchRequest::new(commit_identity),
        }
    }

    pub fn commit_identity(&self) -> &str {
        self.committed_patch.commit_identity()
    }

    pub(crate) fn into_committed_patch_request(
        self,
    ) -> crate::adapter::RelationalCommittedPatchRequest {
        self.committed_patch
    }
}
