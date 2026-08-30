#[cfg(any(test, feature = "certification-support"))]
impl super::WorthUiActiveApplicationSession {
    pub(crate) fn inspect_runtime_service_installation_for_certification(
        &self,
    ) -> crate::certification_support::UiRuntimeServiceInstallationCertificationSnapshot {
        crate::certification_support::UiRuntimeServiceInstallationCertificationSnapshot::new(
            self.portal.is_installed(),
            self.focus.is_installed(),
            self.motion.is_installed(),
            self.command_routing.is_installed(),
            self.scroll.is_installed(),
            self.selection.is_installed(),
        )
    }
}
