#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MosaicParentGrowthBehavior {
    DoesNotForceParent,
    MayGrowParent,
    GrowUntilConstrained,
    MissingForDiagnostics,
}

impl MosaicParentGrowthBehavior {
    pub fn does_not_force_parent() -> Self {
        Self::DoesNotForceParent
    }

    pub fn may_grow_parent() -> Self {
        Self::MayGrowParent
    }

    pub fn grow_until_constrained() -> Self {
        Self::GrowUntilConstrained
    }

    pub fn missing_for_diagnostics() -> Self {
        Self::MissingForDiagnostics
    }

    pub(crate) fn is_missing(&self) -> bool {
        matches!(self, Self::MissingForDiagnostics)
    }

    pub(crate) fn digest_basis(&self) -> &'static str {
        match self {
            Self::DoesNotForceParent => "does_not_force_parent",
            Self::MayGrowParent => "may_grow_parent",
            Self::GrowUntilConstrained => "grow_until_constrained",
            Self::MissingForDiagnostics => "missing",
        }
    }
}
