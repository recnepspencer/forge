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

    pub(crate) fn acquire_visual_snapshot(
        &self,
        frame: worth_ui_host_contract::UiMountedFrameIdentity,
        binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
    ) -> Result<
        crate::mounting::UiMountedVisualCaptureBasis,
        crate::mounting::UiMountedVisualRetentionDenial,
    > {
        self.retention.acquire_visual_snapshot(frame, binding)
    }

    pub(crate) fn visual_snapshot_relation(
        &self,
        frame: worth_ui_host_contract::UiMountedFrameIdentity,
    ) -> Option<worth_ui_inspection::UiVisualSnapshotRelation> {
        self.retention.visual_snapshot_relation(frame)
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn acquire_visual_overlay_for_certification(
        &self,
        frame: worth_ui_host_contract::UiMountedFrameIdentity,
        binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
    ) -> Result<
        crate::mounting::UiMountedVisualOverlayLease,
        crate::mounting::UiMountedVisualRetentionDenial,
    > {
        self.retention
            .acquire_visual_overlay_for_certification(frame, binding)
    }
}
