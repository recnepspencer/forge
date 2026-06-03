/// Standard-path request for routing one committed truth change.
///
/// Callers must carry the typed committed truth identity across this boundary:
///
/// ```rust
/// use forge_runtime_bridge::facade::{BridgeRouteRequest, TruthCommitIdentity};
///
/// let request = BridgeRouteRequest::for_commit(TruthCommitIdentity::new("commit:steel-main"));
/// assert_eq!(request.commit_identity().as_str(), "commit:steel-main");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeRouteRequest {
    committed_patch: crate::adapter::RelationalCommittedPatchRequest,
}

impl BridgeRouteRequest {
    /// Builds a route request from one committed truth identity.
    pub fn for_commit(commit_identity: crate::input::envelope::TruthCommitIdentity) -> Self {
        Self {
            committed_patch: crate::adapter::RelationalCommittedPatchRequest::new(commit_identity),
        }
    }

    /// Returns the authoritative truth commit identity carried by this request.
    pub fn commit_identity(&self) -> &crate::input::envelope::TruthCommitIdentity {
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

impl From<crate::input::envelope::TruthCommitIdentity> for BridgeRouteRequest {
    fn from(value: crate::input::envelope::TruthCommitIdentity) -> Self {
        Self::for_commit(value)
    }
}
