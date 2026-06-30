use serde::Serialize;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize)]
pub enum ReplayUndoTransactionScopeKind {
    Replay,
    Undo,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ReplayUndoTransactionScopeClaim {
    kind: ReplayUndoTransactionScopeKind,
    scope_identity_digest: String,
}

impl ReplayUndoTransactionScopeClaim {
    pub fn new(
        kind: ReplayUndoTransactionScopeKind,
        scope_identity_digest: impl Into<String>,
    ) -> Self {
        let scope_identity_digest = scope_identity_digest.into();
        assert!(
            !scope_identity_digest.trim().is_empty(),
            "transaction scope claim requires a non-empty scope identity digest"
        );
        Self {
            kind,
            scope_identity_digest,
        }
    }

    pub const fn kind(&self) -> ReplayUndoTransactionScopeKind {
        self.kind
    }

    pub fn scope_identity_digest(&self) -> &str {
        &self.scope_identity_digest
    }
}
