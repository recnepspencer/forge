use crate::runtime::{
    WorthQueryManagedLiveLifecycleObservation, WorthQueryRuntimeError, WorthQueryWorkspace,
};

use super::WorthQueryManagedLiveHandle;

impl WorthQueryManagedLiveHandle {
    pub fn observe(
        &self,
        workspace: &mut WorthQueryWorkspace,
    ) -> Result<WorthQueryManagedLiveLifecycleObservation, WorthQueryRuntimeError> {
        workspace.observe_managed_live_view(self.view(), self.workspace_capability())
    }
}
