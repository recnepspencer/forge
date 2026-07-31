#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ApplicationCapabilityDelegationRule {
    Forbidden,
    NarrowAllDimensions,
}

impl ApplicationCapabilityDelegationRule {
    pub const fn forbidden() -> Self {
        Self::Forbidden
    }

    pub const fn narrow_all_dimensions() -> Self {
        Self::NarrowAllDimensions
    }
}
