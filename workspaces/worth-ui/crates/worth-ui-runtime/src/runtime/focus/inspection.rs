#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiFocusInspectionSnapshot {
    current: Option<super::UiSemanticKeyboardFocus>,
    active_descendant: Option<super::UiActiveDescendant>,
    accessibility_focus: super::UiAccessibilityFocusHookSupport,
    window_focus: super::UiWindowFocus,
    modality: super::UiFocusVisibleModality,
    focus_visible: bool,
    scope_count: usize,
    participant_count: usize,
    revision: u64,
}

impl UiFocusInspectionSnapshot {
    pub(super) const fn new(
        current: Option<super::UiSemanticKeyboardFocus>,
        active_descendant: Option<super::UiActiveDescendant>,
        accessibility_focus: super::UiAccessibilityFocusHookSupport,
        window_focus: super::UiWindowFocus,
        modality: super::UiFocusVisibleModality,
        focus_visible: bool,
        scope_count: usize,
        participant_count: usize,
        revision: u64,
    ) -> Self {
        Self {
            current,
            active_descendant,
            accessibility_focus,
            window_focus,
            modality,
            focus_visible,
            scope_count,
            participant_count,
            revision,
        }
    }

    pub(crate) const fn current(self) -> Option<super::UiSemanticKeyboardFocus> {
        self.current
    }
    pub(in crate::runtime) const fn active_descendant(self) -> Option<super::UiActiveDescendant> {
        self.active_descendant
    }
    pub(in crate::runtime) const fn accessibility_focus(
        self,
    ) -> super::UiAccessibilityFocusHookSupport {
        self.accessibility_focus
    }
    pub(in crate::runtime) const fn window_focus(self) -> super::UiWindowFocus {
        self.window_focus
    }
    pub(in crate::runtime) const fn modality(self) -> super::UiFocusVisibleModality {
        self.modality
    }
    pub(in crate::runtime) const fn focus_visible(self) -> bool {
        self.focus_visible
    }
    pub(in crate::runtime) const fn scope_count(self) -> usize {
        self.scope_count
    }
    pub(in crate::runtime) const fn participant_count(self) -> usize {
        self.participant_count
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
