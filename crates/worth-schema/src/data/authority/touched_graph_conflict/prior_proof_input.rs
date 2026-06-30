use serde::Serialize;

use crate::data::authority::replay_undo_semantic_graph::{
    ReplayScopeIdentity, ReplayUndoSemanticGraphPriorProofIdentity,
    ReplayUndoTransactionScopeClaim, UndoScopeIdentity,
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum ConflictPriorProofIdentity {
    ExecutionReceipt(ReplayUndoSemanticGraphPriorProofIdentity),
    ReplayScope(ReplayScopeIdentity),
    UndoScope(UndoScopeIdentity),
    TransactionScope(ReplayUndoTransactionScopeClaim),
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct ConflictPriorProofInput {
    identities: Vec<ConflictPriorProofIdentity>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConflictTransactionProofInput {
    claim: ReplayUndoTransactionScopeClaim,
}

impl ConflictPriorProofInput {
    pub fn none() -> Self {
        Self::default()
    }

    pub fn from_identities(identities: Vec<ConflictPriorProofIdentity>) -> Self {
        Self { identities }
    }

    pub fn identities(&self) -> &[ConflictPriorProofIdentity] {
        &self.identities
    }

    pub fn digest_parts(&self) -> Vec<String> {
        let mut parts = self
            .identities
            .iter()
            .map(ConflictPriorProofIdentity::canonical_part)
            .collect::<Vec<_>>();
        parts.sort();
        parts
    }
}

impl ConflictTransactionProofInput {
    pub fn new(claim: ReplayUndoTransactionScopeClaim) -> Self {
        Self { claim }
    }

    pub const fn claim(&self) -> &ReplayUndoTransactionScopeClaim {
        &self.claim
    }
}

impl ConflictPriorProofIdentity {
    pub fn canonical_part(&self) -> String {
        match self {
            Self::ExecutionReceipt(identity) => format!("execution:{}", identity.digest_part()),
            Self::ReplayScope(identity) => format!("replay-scope:{}", identity.digest()),
            Self::UndoScope(identity) => format!("undo-scope:{}", identity.digest()),
            Self::TransactionScope(claim) => format!(
                "transaction:{:?}:{}",
                claim.kind(),
                claim.scope_identity_digest()
            ),
        }
    }

    pub fn is_replay_undo_or_execution(&self) -> bool {
        matches!(
            self,
            Self::ExecutionReceipt(_) | Self::ReplayScope(_) | Self::UndoScope(_)
        )
    }

    pub fn is_transaction_scope(&self) -> bool {
        matches!(self, Self::TransactionScope(_))
    }
}

impl From<ReplayUndoSemanticGraphPriorProofIdentity> for ConflictPriorProofIdentity {
    fn from(value: ReplayUndoSemanticGraphPriorProofIdentity) -> Self {
        Self::ExecutionReceipt(value)
    }
}

impl From<ReplayScopeIdentity> for ConflictPriorProofIdentity {
    fn from(value: ReplayScopeIdentity) -> Self {
        Self::ReplayScope(value)
    }
}

impl From<UndoScopeIdentity> for ConflictPriorProofIdentity {
    fn from(value: UndoScopeIdentity) -> Self {
        Self::UndoScope(value)
    }
}

impl From<ReplayUndoTransactionScopeClaim> for ConflictPriorProofIdentity {
    fn from(value: ReplayUndoTransactionScopeClaim) -> Self {
        Self::TransactionScope(value)
    }
}
