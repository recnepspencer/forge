#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiDeclaredSelectionIdentityContract {
    StableItemKey,
}

impl UiDeclaredSelectionIdentityContract {
    pub(crate) const fn family(self) -> crate::capability::UiRuntimeServiceFamily {
        crate::capability::UiRuntimeServiceFamily::Selection
    }
}
