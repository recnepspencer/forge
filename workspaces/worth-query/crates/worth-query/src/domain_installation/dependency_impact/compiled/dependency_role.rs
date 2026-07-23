#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WorthQuerySemanticDependencyRole {
    OperationalIdentity,
    SelectionOrMembership,
    Ordering,
    ProjectedValue,
    Grouping,
    WindowBoundary,
    SupportAndLifecycle,
    ConditionalEligibilityOrSemanticCleanliness,
    InstalledDomainInvariant,
    AdvisoryOnlyContext,
}

impl WorthQuerySemanticDependencyRole {
    pub(crate) const COUNT: usize = 10;

    pub(crate) const fn canonical_ordinal(self) -> usize {
        self as usize
    }

    pub(crate) const fn canonical_name(self) -> &'static str {
        match self {
            Self::OperationalIdentity => "operational-identity",
            Self::SelectionOrMembership => "selection-or-membership",
            Self::Ordering => "ordering",
            Self::ProjectedValue => "projected-value",
            Self::Grouping => "grouping",
            Self::WindowBoundary => "window-boundary",
            Self::SupportAndLifecycle => "support-and-lifecycle",
            Self::ConditionalEligibilityOrSemanticCleanliness => {
                "conditional-eligibility-or-semantic-cleanliness"
            }
            Self::InstalledDomainInvariant => "installed-domain-invariant",
            Self::AdvisoryOnlyContext => "advisory-only-context",
        }
    }
}
