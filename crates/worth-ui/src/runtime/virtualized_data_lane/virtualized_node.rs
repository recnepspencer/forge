use crate::runtime::{WorthUiQueryPatchPosture, WorthUiRuntimeHandle};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiVirtualizedDataNode {
    runtime_handle: WorthUiRuntimeHandle,
    query_patch_posture: WorthUiQueryPatchPosture,
}

impl WorthUiVirtualizedDataNode {
    pub(crate) fn new(
        runtime_handle: WorthUiRuntimeHandle,
        query_patch_posture: WorthUiQueryPatchPosture,
    ) -> Self {
        Self {
            runtime_handle,
            query_patch_posture,
        }
    }

    pub fn runtime_handle(&self) -> WorthUiRuntimeHandle {
        self.runtime_handle
    }

    pub fn plan_index(&self) -> u32 {
        self.runtime_handle.plan_index()
    }

    pub fn query_patch_posture(&self) -> &WorthUiQueryPatchPosture {
        &self.query_patch_posture
    }
}
