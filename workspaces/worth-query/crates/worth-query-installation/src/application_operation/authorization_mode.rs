#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryInstalledApplicationOperationAuthorization {
    Principal,
    Abilities,
    Capability,
}

impl WorthQueryInstalledApplicationOperationAuthorization {
    pub(crate) const fn exact_fact_count(self, ability_count: usize) -> usize {
        match self {
            Self::Principal => 1,
            Self::Abilities => 1usize.saturating_add(ability_count),
            Self::Capability => 2,
        }
    }

    pub const fn requires_capability(self) -> bool {
        matches!(self, Self::Capability)
    }
}
