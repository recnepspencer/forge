#[cfg(any(test, feature = "certification-support"))]
impl super::WorthUiActiveApplicationSession {
    pub(crate) fn inspect_portal_runtime_for_certification(
        &self,
    ) -> crate::certification_support::UiPortalRuntimeCertificationSnapshot {
        crate::certification_support::UiPortalRuntimeCertificationSnapshot::new(
            self.portal.active_count(),
            self.portal
                .posture_count(crate::runtime::portal::UiPortalLifecyclePosture::Open),
            self.portal
                .posture_count(crate::runtime::portal::UiPortalLifecyclePosture::Visible),
            self.portal
                .posture_count(crate::runtime::portal::UiPortalLifecyclePosture::Closing),
            0,
            self.portal.admitted_requests(),
            self.portal.idempotent_requests(),
            self.portal.revision(),
        )
    }
}
