#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiDeclaredSelectionIdentityContract {
    StableItemKey,
}

impl UiDeclaredSelectionIdentityContract {
    pub(crate) const fn family(self) -> crate::capability::UiRuntimeServiceFamily {
        crate::capability::UiRuntimeServiceFamily::Selection
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum UiSelectionMode {
    Single,
    Multiple,
    Range,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct UiSelectionPolicy {
    mode: UiSelectionMode,
    preserve_stable_keys: bool,
}

impl UiSelectionPolicy {
    pub const fn single() -> Self {
        Self::new(UiSelectionMode::Single)
    }

    pub const fn multiple() -> Self {
        Self::new(UiSelectionMode::Multiple)
    }

    pub const fn range() -> Self {
        Self::new(UiSelectionMode::Range)
    }

    const fn new(mode: UiSelectionMode) -> Self {
        Self {
            mode,
            preserve_stable_keys: true,
        }
    }

    pub const fn with_stable_key_preservation(mut self, enabled: bool) -> Self {
        self.preserve_stable_keys = enabled;
        self
    }

    pub const fn mode(self) -> UiSelectionMode {
        self.mode
    }

    pub const fn preserves_stable_keys(self) -> bool {
        self.preserve_stable_keys
    }

    pub(crate) const fn digest_basis(self) -> u64 {
        self.mode as u64 | (self.preserve_stable_keys as u64) << 8
    }
}
