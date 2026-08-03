#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UiIdentityLifecycleDecision {
    Unaffected,
    Preserve,
    Create,
    Retire,
    Rebind,
    Move,
    Remount,
}

impl UiIdentityLifecycleDecision {
    pub const fn preserves_instance(self) -> bool {
        matches!(
            self,
            Self::Unaffected | Self::Preserve | Self::Rebind | Self::Move
        )
    }

    pub const fn preserves_incarnation(self) -> bool {
        self.preserves_instance()
    }
}
