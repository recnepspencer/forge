use crate::source::WorthUiArtifactHandle;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiMovedNodeIdentity {
    active_handle: WorthUiArtifactHandle,
    candidate_handle: WorthUiArtifactHandle,
}

impl WorthUiMovedNodeIdentity {
    pub(crate) fn new(
        active_handle: WorthUiArtifactHandle,
        candidate_handle: WorthUiArtifactHandle,
        _identity_basis: String,
    ) -> Self {
        Self {
            active_handle,
            candidate_handle,
        }
    }

    #[cfg(test)]
    pub fn crossed_module_boundary(&self) -> bool {
        self.active_handle.module_id() != self.candidate_handle.module_id()
    }
}
