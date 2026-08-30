#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiFocusInspectionSnapshot {
    current: Option<super::UiSemanticKeyboardFocus>,
    #[cfg(test)]
    active_descendant: Option<super::UiActiveDescendant>,
    #[cfg(test)]
    accessibility_focus: super::UiAccessibilityFocusHookSupport,
    #[cfg(test)]
    focus_visible: bool,
    revision: u64,
}

impl UiFocusInspectionSnapshot {
    #[cfg(not(test))]
    pub(super) const fn new(
        current: Option<super::UiSemanticKeyboardFocus>,
        revision: u64,
    ) -> Self {
        Self { current, revision }
    }

    #[cfg(test)]
    pub(super) const fn for_test(
        current: Option<super::UiSemanticKeyboardFocus>,
        active_descendant: Option<super::UiActiveDescendant>,
        accessibility_focus: super::UiAccessibilityFocusHookSupport,
        focus_visible: bool,
        revision: u64,
    ) -> Self {
        Self {
            current,
            active_descendant,
            accessibility_focus,
            focus_visible,
            revision,
        }
    }

    pub(crate) const fn current(self) -> Option<super::UiSemanticKeyboardFocus> {
        self.current
    }
    #[cfg(test)]
    pub(in crate::runtime) const fn active_descendant(self) -> Option<super::UiActiveDescendant> {
        self.active_descendant
    }
    #[cfg(test)]
    pub(in crate::runtime) const fn accessibility_focus(
        self,
    ) -> super::UiAccessibilityFocusHookSupport {
        self.accessibility_focus
    }
    #[cfg(test)]
    pub(crate) const fn focus_visible(self) -> bool {
        self.focus_visible
    }
    pub(crate) const fn revision(self) -> u64 {
        self.revision
    }
}

impl super::UiFocusRuntimeState {
    pub(crate) const fn last_transition(&self) -> Option<super::UiFocusTransitionReceipt> {
        self.last_transition
    }

    pub(crate) const fn last_restoration_failure(&self) -> Option<super::UiFocusTransitionReceipt> {
        self.last_restoration_failure
    }

    pub(crate) fn resource_counts(&self) -> (usize, usize, usize) {
        (
            self.participant_index.len(),
            self.pending_portal.len(),
            self.portal_restorations.len(),
        )
    }
}
