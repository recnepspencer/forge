#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiDeclaredScrollOwnershipContract {
    RuntimeOwnedOffset,
}

impl UiDeclaredScrollOwnershipContract {
    pub(crate) const fn family(self) -> crate::capability::UiRuntimeServiceFamily {
        crate::capability::UiRuntimeServiceFamily::Scroll
    }
}
