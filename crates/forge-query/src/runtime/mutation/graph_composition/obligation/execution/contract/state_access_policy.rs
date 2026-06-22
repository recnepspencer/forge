#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryGraphObligationStateAccessPolicy {
    SelectionOnly,
    DeclaredBudgetOnly,
    BoundedStateLoad,
}

impl ForgeQueryGraphObligationStateAccessPolicy {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SelectionOnly => "selection-only",
            Self::DeclaredBudgetOnly => "declared-budget-only",
            Self::BoundedStateLoad => "bounded-state-load",
        }
    }
}
