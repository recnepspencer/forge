use crate::source::WorthUiArtifactHandle;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiIdentityMatchEdge {
    active_handle: WorthUiArtifactHandle,
    candidate_handle: WorthUiArtifactHandle,
    identity_basis: String,
}

impl WorthUiIdentityMatchEdge {
    pub(crate) fn new(
        active_handle: WorthUiArtifactHandle,
        candidate_handle: WorthUiArtifactHandle,
        identity_basis: String,
    ) -> Self {
        Self {
            active_handle,
            candidate_handle,
            identity_basis,
        }
    }

    pub fn identity_basis(&self) -> &str {
        &self.identity_basis
    }

    pub fn moved_between_handles(&self) -> bool {
        self.active_handle != self.candidate_handle
    }
}
