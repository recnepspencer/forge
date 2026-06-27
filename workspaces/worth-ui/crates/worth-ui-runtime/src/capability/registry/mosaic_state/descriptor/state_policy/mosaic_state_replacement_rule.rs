#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MosaicStateReplacementRule {
    PreserveWhenOwnerMatches,
    DiscardWhenOwnerChanges,
    RemapWhenRuntimeSuppliesAlias,
    MissingForDiagnostics,
}

impl MosaicStateReplacementRule {
    pub fn preserve_when_owner_matches() -> Self {
        Self::PreserveWhenOwnerMatches
    }

    pub fn discard_when_owner_changes() -> Self {
        Self::DiscardWhenOwnerChanges
    }

    pub fn remap_when_runtime_supplies_alias() -> Self {
        Self::RemapWhenRuntimeSuppliesAlias
    }

    pub fn missing_for_diagnostics() -> Self {
        Self::MissingForDiagnostics
    }

    pub(crate) fn is_missing(&self) -> bool {
        matches!(self, Self::MissingForDiagnostics)
    }

    pub(crate) fn digest_basis(&self) -> &'static str {
        match self {
            Self::PreserveWhenOwnerMatches => "preserve_when_owner_matches",
            Self::DiscardWhenOwnerChanges => "discard_when_owner_changes",
            Self::RemapWhenRuntimeSuppliesAlias => "remap_when_runtime_supplies_alias",
            Self::MissingForDiagnostics => "missing",
        }
    }
}
