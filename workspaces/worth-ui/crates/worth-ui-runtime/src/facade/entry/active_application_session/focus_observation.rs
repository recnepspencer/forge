#[cfg(any(test, feature = "certification-support"))]
impl super::WorthUiActiveApplicationSession {
    pub(crate) fn inspect_focus_runtime_for_certification(
        &self,
    ) -> crate::certification_support::UiFocusRuntimeCertificationSnapshot {
        let (current, active_descendant, participants, pending, revision) =
            self.focus.inspect_for_certification();
        crate::certification_support::UiFocusRuntimeCertificationSnapshot::new(
            current,
            active_descendant,
            participants,
            pending,
            revision,
        )
    }
}
