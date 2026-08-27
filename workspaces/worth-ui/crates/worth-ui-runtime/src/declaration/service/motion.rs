#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiDeclaredMotionPolicyContract {
    ReducedMotionAware,
}

impl UiDeclaredMotionPolicyContract {
    pub(crate) const fn family(self) -> crate::capability::UiRuntimeServiceFamily {
        crate::capability::UiRuntimeServiceFamily::Motion
    }
}
