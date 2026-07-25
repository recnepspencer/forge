use super::WorthUiActiveApplicationSession;

impl WorthUiActiveApplicationSession {
    pub fn inspect_mounted_frame(
        &self,
        request: crate::inspection::mounted_frame::UiMountedInspectionRequest,
    ) -> crate::inspection::mounted_frame::UiMountedInspectionReceipt {
        crate::inspection::mounted_frame::inspect(&self.mounted_retention, request)
    }

    pub fn mounted_retention_report(
        &self,
    ) -> crate::inspection::mounted_frame::UiMountedRetentionReport {
        crate::inspection::mounted_frame::retention_report(
            &self.mounted_retention,
            &self.host_observations,
        )
    }
}
