#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiDeclaredFocusOwnershipContract {
    SemanticKeyboardFocus,
}

impl UiDeclaredFocusOwnershipContract {
    pub(crate) const fn family(self) -> crate::capability::UiRuntimeServiceFamily {
        crate::capability::UiRuntimeServiceFamily::Focus
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum UiFocusScopePolicy {
    Workbench,
    Portal,
    Composite,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct UiFocusPolicy {
    scope: UiFocusScopePolicy,
    restore_on_scope_close: bool,
    reveal_focused_target: bool,
}

impl UiFocusPolicy {
    pub const fn workbench() -> Self {
        Self::new(UiFocusScopePolicy::Workbench)
    }

    pub const fn portal_scope() -> Self {
        Self::new(UiFocusScopePolicy::Portal)
    }

    pub const fn composite_scope() -> Self {
        Self::new(UiFocusScopePolicy::Composite)
    }

    const fn new(scope: UiFocusScopePolicy) -> Self {
        Self {
            scope,
            restore_on_scope_close: true,
            reveal_focused_target: true,
        }
    }

    pub const fn with_scope_restoration(mut self, enabled: bool) -> Self {
        self.restore_on_scope_close = enabled;
        self
    }

    pub const fn with_focus_reveal(mut self, enabled: bool) -> Self {
        self.reveal_focused_target = enabled;
        self
    }

    pub const fn scope(self) -> UiFocusScopePolicy {
        self.scope
    }

    pub const fn restores_on_scope_close(self) -> bool {
        self.restore_on_scope_close
    }

    pub const fn reveals_focused_target(self) -> bool {
        self.reveal_focused_target
    }

    pub(crate) const fn digest_basis(self) -> u64 {
        self.scope as u64
            | (self.restore_on_scope_close as u64) << 8
            | (self.reveal_focused_target as u64) << 9
    }
}
