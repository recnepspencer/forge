use super::WorthUiMountedSessionState;

impl WorthUiMountedSessionState {
    pub(crate) fn inspect_frame(
        &self,
        request: crate::inspection::mounted_frame::UiMountedInspectionRequest,
    ) -> crate::inspection::mounted_frame::UiMountedInspectionReceipt {
        crate::inspection::mounted_frame::inspect(&self.retention, request)
    }

    pub(crate) fn retention_snapshot(&self) -> crate::mounting::UiMountedFrameRetentionSnapshot {
        self.retention.retention_snapshot()
    }
}
