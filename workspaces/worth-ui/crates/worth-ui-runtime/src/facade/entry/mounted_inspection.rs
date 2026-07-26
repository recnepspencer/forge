use super::WorthUiActiveApplicationSession;

impl WorthUiActiveApplicationSession {
    pub fn inspect_mounted_frame(
        &self,
        request: crate::inspection::mounted_frame::UiMountedInspectionRequest,
    ) -> crate::inspection::mounted_frame::UiMountedInspectionReceipt {
        self.mounted.inspect_frame(request)
    }

    pub fn mounted_retention_report(
        &self,
    ) -> crate::inspection::mounted_frame::UiMountedRetentionReport {
        crate::inspection::mounted_frame::retention_report(
            self.mounted.retention_snapshot(),
            self.host_exchange.observation_retention_snapshot(),
        )
    }
}
