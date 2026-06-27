use serde::Serialize;

use super::equivalence_basis::ReplayUndoSemanticGraphEquivalenceBasis;
use super::identity_digest::replay_undo_semantic_graph_identity_digest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UndoScopeIdentityInput {
    equivalence_basis: ReplayUndoSemanticGraphEquivalenceBasis,
}

impl UndoScopeIdentityInput {
    pub fn new(equivalence_basis: ReplayUndoSemanticGraphEquivalenceBasis) -> Self {
        Self { equivalence_basis }
    }

    pub const fn equivalence_basis(&self) -> &ReplayUndoSemanticGraphEquivalenceBasis {
        &self.equivalence_basis
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UndoScopeIdentity {
    digest: String,
    equivalence_basis: ReplayUndoSemanticGraphEquivalenceBasis,
}

impl UndoScopeIdentity {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub const fn equivalence_basis(&self) -> &ReplayUndoSemanticGraphEquivalenceBasis {
        &self.equivalence_basis
    }
}

pub fn admit_undo_scope_identity(input: UndoScopeIdentityInput) -> UndoScopeIdentity {
    let digest = replay_undo_semantic_graph_identity_digest(
        "worth.schema.undo-scope-identity.v1",
        &input.equivalence_basis.digest_parts(),
    );
    UndoScopeIdentity {
        digest,
        equivalence_basis: input.equivalence_basis,
    }
}
