/// WUI-owned capability that lends one Query workspace only to the binding
/// owner responsible for operation-native progression.
///
/// Runtime code can carry this request without importing or naming Query's
/// operating world. The workspace never enters retained runtime state.
pub struct WorthUiOperationLiveRefreshRequest<'workspace> {
    reference: crate::WorthUiInstalledQueryBindingReference,
    workspace: &'workspace mut worth_query::facade::runtime::WorthQueryWorkspace,
}

impl<'workspace> WorthUiOperationLiveRefreshRequest<'workspace> {
    pub fn new(
        reference: &crate::WorthUiInstalledQueryBindingReference,
        workspace: &'workspace mut worth_query::facade::runtime::WorthQueryWorkspace,
    ) -> Self {
        Self {
            reference: reference.clone(),
            workspace,
        }
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        crate::WorthUiInstalledQueryBindingReference,
        &'workspace mut worth_query::facade::runtime::WorthQueryWorkspace,
    ) {
        (self.reference, self.workspace)
    }
}
