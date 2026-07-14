use crate::runtime::{
    WorthQueryLiveReadResult, WorthQueryLiveView, WorthQueryManagedLiveWorkspaceCapability,
    WorthQueryNativeRow, WorthQueryRuntimeError, WorthQueryWorkspace,
};
use std::sync::Arc;

use super::WorthQueryManagedLiveDelivery;

#[derive(Debug)]
#[must_use = "managed live resources remain active until the handle is explicitly closed"]
pub struct WorthQueryManagedLiveHandle {
    view: Option<WorthQueryLiveView<WorthQueryNativeRow>>,
    workspace_capability: Arc<WorthQueryManagedLiveWorkspaceCapability>,
}

impl WorthQueryManagedLiveHandle {
    pub fn name(&self) -> &str {
        self.view().name()
    }

    pub fn read(
        &self,
        workspace: &mut WorthQueryWorkspace,
    ) -> Result<WorthQueryLiveReadResult, WorthQueryRuntimeError> {
        workspace.read_managed_live_view(self.view(), &self.workspace_capability)
    }

    pub fn drain(
        &self,
        workspace: &mut WorthQueryWorkspace,
    ) -> Result<WorthQueryManagedLiveDelivery, WorthQueryRuntimeError> {
        workspace
            .drain_managed_live_view(self.view(), &self.workspace_capability)
            .map(WorthQueryManagedLiveDelivery::from_runtime)
    }

    pub(crate) fn view(&self) -> &WorthQueryLiveView<WorthQueryNativeRow> {
        self.view
            .as_ref()
            .expect("active managed live handle must retain its resource view")
    }

    pub(crate) fn workspace_capability(&self) -> &Arc<WorthQueryManagedLiveWorkspaceCapability> {
        &self.workspace_capability
    }

    pub(crate) fn new(
        view: WorthQueryLiveView<WorthQueryNativeRow>,
        workspace_capability: Arc<WorthQueryManagedLiveWorkspaceCapability>,
    ) -> Self {
        Self {
            view: Some(view),
            workspace_capability,
        }
    }

    pub(crate) fn into_resource_parts(
        mut self,
    ) -> (
        WorthQueryLiveView<WorthQueryNativeRow>,
        Arc<WorthQueryManagedLiveWorkspaceCapability>,
    ) {
        let view = self
            .view
            .take()
            .expect("transferred managed live handle must retain its resource view");
        (view, Arc::clone(&self.workspace_capability))
    }

    pub(crate) fn disarm(&mut self) {
        self.view = None;
    }
}

impl Drop for WorthQueryManagedLiveHandle {
    fn drop(&mut self) {
        if let Some(view) = self.view.take() {
            self.workspace_capability.abandon(view);
        }
    }
}
