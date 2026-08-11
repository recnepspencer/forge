//! Redo denial causes — eight distinct production exit-proof causes (Gate 8.5).

/// Typed redo denial. Each variant is a distinct cause — no shared fallback.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryRedoDenialKind {
    /// Binding or intent is stale against current truth.
    Stale,
    /// Fresh capability/policy admission denied — proved undo is not current authority.
    NewlyUnauthorized,
    /// Intent was copied / not derived from this proved undo path.
    CopiedIntent,
    /// Foreign runtime or principal relative to the proved undo.
    ForeignPrincipal,
    /// Installed operation meaning changed between undo and redo.
    ChangedOperationMeaning,
    /// Equivalent redo already consumed / duplicate.
    DuplicateRedo,
    /// Linear head advanced away from the bound head (lane policy / D7).
    DivergenceInvalidation,
    /// Ordinary compare-and-commit conflicted.
    Conflicted,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryRedoDenial {
    kind: WorthQueryRedoDenialKind,
}

impl WorthQueryRedoDenial {
    pub(crate) const fn new(kind: WorthQueryRedoDenialKind) -> Self {
        Self { kind }
    }

    pub const fn kind(&self) -> WorthQueryRedoDenialKind {
        self.kind
    }

    pub const fn stale() -> Self {
        Self::new(WorthQueryRedoDenialKind::Stale)
    }

    pub const fn newly_unauthorized() -> Self {
        Self::new(WorthQueryRedoDenialKind::NewlyUnauthorized)
    }

    pub const fn copied_intent() -> Self {
        Self::new(WorthQueryRedoDenialKind::CopiedIntent)
    }

    pub const fn foreign_principal() -> Self {
        Self::new(WorthQueryRedoDenialKind::ForeignPrincipal)
    }

    pub const fn changed_operation_meaning() -> Self {
        Self::new(WorthQueryRedoDenialKind::ChangedOperationMeaning)
    }

    pub const fn duplicate_redo() -> Self {
        Self::new(WorthQueryRedoDenialKind::DuplicateRedo)
    }

    pub const fn divergence_invalidation() -> Self {
        Self::new(WorthQueryRedoDenialKind::DivergenceInvalidation)
    }

    pub const fn conflicted() -> Self {
        Self::new(WorthQueryRedoDenialKind::Conflicted)
    }
}
