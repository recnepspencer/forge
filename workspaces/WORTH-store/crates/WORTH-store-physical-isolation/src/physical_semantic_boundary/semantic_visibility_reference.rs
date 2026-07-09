#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SemanticVisibilityReferenceKind {
    Transaction,
    Branch,
    Snapshot,
    Projection,
    CurrentBasis,
    Commit,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SemanticVisibilityReference {
    kind: SemanticVisibilityReferenceKind,
    runtime: String,
    semantic_id: String,
}

impl SemanticVisibilityReference {
    pub fn transaction(runtime: impl Into<String>, transaction_id: impl Into<String>) -> Self {
        Self::new(
            SemanticVisibilityReferenceKind::Transaction,
            runtime,
            transaction_id,
        )
    }

    pub fn branch(runtime: impl Into<String>, branch_id: impl Into<String>) -> Self {
        Self::new(SemanticVisibilityReferenceKind::Branch, runtime, branch_id)
    }

    pub fn relational_snapshot(runtime: impl Into<String>, snapshot_id: impl Into<String>) -> Self {
        Self::new(
            SemanticVisibilityReferenceKind::Snapshot,
            runtime,
            snapshot_id,
        )
    }

    pub fn projection(runtime: impl Into<String>, projection_id: impl Into<String>) -> Self {
        Self::new(
            SemanticVisibilityReferenceKind::Projection,
            runtime,
            projection_id,
        )
    }

    pub fn current_basis(runtime: impl Into<String>, basis_id: impl Into<String>) -> Self {
        Self::new(
            SemanticVisibilityReferenceKind::CurrentBasis,
            runtime,
            basis_id,
        )
    }

    pub fn commit(runtime: impl Into<String>, commit_id: impl Into<String>) -> Self {
        Self::new(SemanticVisibilityReferenceKind::Commit, runtime, commit_id)
    }

    pub fn new(
        kind: SemanticVisibilityReferenceKind,
        runtime: impl Into<String>,
        semantic_id: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            runtime: runtime.into(),
            semantic_id: semantic_id.into(),
        }
    }

    pub const fn kind(&self) -> SemanticVisibilityReferenceKind {
        self.kind
    }

    pub fn runtime(&self) -> &str {
        &self.runtime
    }

    pub fn semantic_id(&self) -> &str {
        &self.semantic_id
    }

    pub const fn is_store_physical_stability_authority(&self) -> bool {
        false
    }
}
