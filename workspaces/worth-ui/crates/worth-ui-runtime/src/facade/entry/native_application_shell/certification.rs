impl super::WorthUiNativeApplicationShell {
    pub fn inspect_motion_presentation_for_certification(
        &self,
    ) -> crate::certification_support::UiMotionPresentationCertificationSnapshot {
        self.session.inspect_motion_presentation_for_certification()
    }

    pub fn admit_reduced_motion_tick_for_certification(&mut self, tick: u64) -> Result<bool, ()> {
        self.admit_native_motion_tick(
            tick,
            worth_ui_host_native::UiNativeReducedMotionPosture::Reduce,
        )
        .map(|disposition| {
            matches!(
                disposition,
                super::motion_sampling::UiNativeMotionTickDisposition::Inactive
            )
        })
    }

    pub fn inspect_portal_runtime_for_certification(
        &self,
    ) -> crate::certification_support::UiPortalRuntimeCertificationSnapshot {
        self.session.inspect_portal_runtime_for_certification()
    }

    pub fn inspect_focus_runtime_for_certification(
        &self,
    ) -> crate::certification_support::UiFocusRuntimeCertificationSnapshot {
        self.session.inspect_focus_runtime_for_certification()
    }

    pub fn inspect_service_proposals_for_certification(
        &self,
    ) -> crate::certification_support::UiServiceProposalCertificationSnapshot {
        self.session.inspect_service_proposals_for_certification()
    }
}
