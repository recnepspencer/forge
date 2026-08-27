#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiDeclaredFocusOwnershipContract {
    SemanticKeyboardFocus,
}

impl UiDeclaredFocusOwnershipContract {
    pub(crate) const fn family(self) -> crate::capability::UiRuntimeServiceFamily {
        crate::capability::UiRuntimeServiceFamily::Focus
    }
}
