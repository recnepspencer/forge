use std::num::NonZeroU32;

/// Installed upper bound for one capability delegation chain.
///
/// This is contract meaning, not a caller-selected execution limit. Query
/// rejects a chain before authority when its observed depth exceeds this
/// bound. The platform ceiling keeps every installed ordinary admission
/// finitely reviewable and resource-bounded.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationCapabilityDelegationDepth(NonZeroU32);

impl ApplicationCapabilityDelegationDepth {
    pub const PLATFORM_MAXIMUM: u32 = 64;

    pub const fn new(maximum: u32) -> Option<Self> {
        match NonZeroU32::new(maximum) {
            Some(maximum) if maximum.get() <= Self::PLATFORM_MAXIMUM => Some(Self(maximum)),
            Some(_) | None => None,
        }
    }

    pub const fn maximum(self) -> u32 {
        self.0.get()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ApplicationCapabilityDelegationRule {
    Forbidden,
    NarrowAllDimensions {
        maximum_depth: ApplicationCapabilityDelegationDepth,
    },
}

impl ApplicationCapabilityDelegationRule {
    pub const fn forbidden() -> Self {
        Self::Forbidden
    }

    pub const fn narrow_all_dimensions(
        maximum_depth: ApplicationCapabilityDelegationDepth,
    ) -> Self {
        Self::NarrowAllDimensions { maximum_depth }
    }

    pub const fn maximum_depth(self) -> Option<ApplicationCapabilityDelegationDepth> {
        match self {
            Self::Forbidden => None,
            Self::NarrowAllDimensions { maximum_depth } => Some(maximum_depth),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn installed_depth_is_nonzero_and_platform_bounded() {
        assert_eq!(ApplicationCapabilityDelegationDepth::new(0), None);
        assert_eq!(
            ApplicationCapabilityDelegationDepth::new(
                ApplicationCapabilityDelegationDepth::PLATFORM_MAXIMUM + 1,
            ),
            None
        );
        assert_eq!(
            ApplicationCapabilityDelegationDepth::new(8)
                .unwrap()
                .maximum(),
            8
        );
    }
}
