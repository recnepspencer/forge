#[cfg(any(test, feature = "certification-support"))]
impl super::WorthUiActiveApplicationSession {
    pub(crate) fn inspect_focus_runtime_for_certification(
        &self,
    ) -> crate::certification_support::UiFocusRuntimeCertificationSnapshot {
        let Some(focus) = self.focus.as_ref() else {
            return crate::certification_support::UiFocusRuntimeCertificationSnapshot::uninstalled(
            );
        };
        let (current, active_descendant, participants, pending, revision) =
            focus.inspect_for_certification();
        crate::certification_support::UiFocusRuntimeCertificationSnapshot::new(
            current,
            active_descendant,
            participants,
            pending,
            revision,
        )
    }
}
