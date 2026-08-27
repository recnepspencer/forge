#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiDeclaredCommandRoutingContract {
    TypedInvocation,
}

impl UiDeclaredCommandRoutingContract {
    pub(crate) const fn family(self) -> crate::capability::UiRuntimeServiceFamily {
        crate::capability::UiRuntimeServiceFamily::CommandRouting
    }
}
