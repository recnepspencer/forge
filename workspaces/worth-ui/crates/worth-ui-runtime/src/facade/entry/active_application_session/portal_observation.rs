#[cfg(any(test, feature = "certification-support"))]
impl super::WorthUiActiveApplicationSession {
    pub(crate) fn inspect_portal_runtime_for_certification(
        &self,
    ) -> crate::certification_support::UiPortalRuntimeCertificationSnapshot {
        let Some(portal) = self.portal.as_ref() else {
            return crate::certification_support::UiPortalRuntimeCertificationSnapshot::uninstalled(
            );
        };
        crate::certification_support::UiPortalRuntimeCertificationSnapshot::new(
            portal.active_count(),
            portal.posture_count(crate::runtime::portal::UiPortalLifecyclePosture::Open),
            portal.posture_count(crate::runtime::portal::UiPortalLifecyclePosture::Visible),
            portal.posture_count(crate::runtime::portal::UiPortalLifecyclePosture::Closing),
            0,
            portal.admitted_requests(),
            portal.idempotent_requests(),
            portal.revision(),
        )
    }
}
