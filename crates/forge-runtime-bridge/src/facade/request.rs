use std::sync::Arc;

/// Standard-path request for routing one committed truth change.
///
/// Most callers can construct this implicitly from a commit identity string:
///
/// ```rust
/// use forge_runtime_bridge::facade::BridgeRouteRequest;
///
/// let request = BridgeRouteRequest::for_commit("commit:steel-main");
/// assert_eq!(request.commit_identity(), "commit:steel-main");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeRouteRequest {
    committed_patch: crate::adapter::RelationalCommittedPatchRequest,
}

impl BridgeRouteRequest {
    /// Builds a route request from one committed truth identity.
    pub fn for_commit(commit_identity: impl Into<Arc<str>>) -> Self {
        Self {
            committed_patch: crate::adapter::RelationalCommittedPatchRequest::new(commit_identity),
        }
    }

    /// Returns the authoritative truth commit identity carried by this request.
    pub fn commit_identity(&self) -> &str {
        self.committed_patch.commit_identity()
    }

    pub(crate) fn into_committed_patch_request(
        self,
    ) -> crate::adapter::RelationalCommittedPatchRequest {
        self.committed_patch
    }
}

impl From<crate::adapter::RelationalCommittedPatchRequest> for BridgeRouteRequest {
    fn from(value: crate::adapter::RelationalCommittedPatchRequest) -> Self {
        Self {
            committed_patch: value,
        }
    }
}

impl From<String> for BridgeRouteRequest {
    fn from(value: String) -> Self {
        Self::for_commit(value)
    }
}

impl From<&str> for BridgeRouteRequest {
    fn from(value: &str) -> Self {
        Self::for_commit(value)
    }
}

impl From<Arc<str>> for BridgeRouteRequest {
    fn from(value: Arc<str>) -> Self {
        Self::for_commit(value)
    }
}
