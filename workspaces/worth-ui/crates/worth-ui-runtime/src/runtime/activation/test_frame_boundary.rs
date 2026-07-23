impl crate::runtime::WorthUiRuntime {
    pub(crate) fn safe_frame_boundary(&self) -> crate::runtime::WorthUiFrameBoundary {
        crate::runtime::WorthUiFrameBoundary::safe_to_activate(
            self.frame_epoch(),
            self.host_plan_binding.session_identity(),
        )
    }

    pub(crate) fn traversal_frame_boundary_for_test(&self) -> crate::runtime::WorthUiFrameBoundary {
        crate::runtime::WorthUiFrameBoundary::traversal_in_progress_for_test(
            self.frame_epoch(),
            self.host_plan_binding.session_identity(),
        )
    }

    pub(crate) fn safe_frame_boundary_for_epoch_for_test(
        &self,
        frame_epoch: crate::runtime::WorthUiRuntimeFrameEpoch,
    ) -> crate::runtime::WorthUiFrameBoundary {
        crate::runtime::WorthUiFrameBoundary::safe_to_activate(
            frame_epoch,
            self.host_plan_binding.session_identity(),
        )
    }
}
