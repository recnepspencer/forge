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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authorization_mode_owns_its_exact_decision_fact_family() {
        assert_eq!(
            WorthQueryInstalledApplicationOperationAuthorization::Principal.exact_fact_count(0),
            1
        );
        assert_eq!(
            WorthQueryInstalledApplicationOperationAuthorization::Abilities.exact_fact_count(2),
            3
        );
        assert_eq!(
            WorthQueryInstalledApplicationOperationAuthorization::Capability.exact_fact_count(0),
            2
        );
    }
}
