/// Sealed insertion point for the accessibility-focus owner planned for Milestone 13.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::runtime) struct UiAccessibilityFocusHook;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::runtime) enum UiAccessibilityFocusHookSupport {
    UnsupportedUntilMilestone13,
}

impl UiAccessibilityFocusHook {
    pub(in crate::runtime) const fn support(self) -> UiAccessibilityFocusHookSupport {
        UiAccessibilityFocusHookSupport::UnsupportedUntilMilestone13
    }
}
