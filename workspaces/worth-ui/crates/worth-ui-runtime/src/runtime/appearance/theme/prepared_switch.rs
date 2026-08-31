#[derive(Debug, Eq, PartialEq)]
pub(crate) struct UiPreparedThemeSwitch {
    pub(super) reservation: u64,
    pub(super) predecessor_generation: u64,
    pub(super) successor: super::UiActiveThemeBinding,
    pub(super) origin: super::UiThemeSwitchOrigin,
    pub(super) owner_affinity: u64,
}

impl UiPreparedThemeSwitch {
    pub(crate) const fn reservation(&self) -> u64 {
        self.reservation
    }

    pub(crate) const fn predecessor_generation(&self) -> u64 {
        self.predecessor_generation
    }

    pub(crate) const fn successor(&self) -> &super::UiActiveThemeBinding {
        &self.successor
    }

    pub(crate) const fn origin(&self) -> &super::UiThemeSwitchOrigin {
        &self.origin
    }
}
