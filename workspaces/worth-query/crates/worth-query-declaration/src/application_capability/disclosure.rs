use super::ApplicationCapabilityScopeGuard;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ApplicationCapabilityDisclosureRule {
    NotApplicable,
    Permit(Vec<ApplicationCapabilityScopeGuard>),
}

impl ApplicationCapabilityDisclosureRule {
    pub const fn not_applicable() -> Self {
        Self::NotApplicable
    }

    pub fn permit(guards: impl IntoIterator<Item = ApplicationCapabilityScopeGuard>) -> Self {
        let mut guards = guards.into_iter().collect::<Vec<_>>();
        guards.sort();
        guards.dedup();
        Self::Permit(guards)
    }

    pub fn guards(&self) -> Option<&[ApplicationCapabilityScopeGuard]> {
        match self {
            Self::NotApplicable => None,
            Self::Permit(guards) => Some(guards),
        }
    }
}
