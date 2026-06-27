use schema::facade::platform::authority::replay_undo_semantic_graph::{
    ReplayUndoTransactionScopeClaim, ReplayUndoTransactionScopeKind,
};
use worth_spatial::facade::replay_undo_semantic_graph::{
    SpatialReplayScopeProduct, SpatialUndoScopeProduct,
};

#[derive(Clone, Copy)]
pub enum ReplayUndoTransactionMutationClaimSource<'a> {
    ReplayScope(&'a SpatialReplayScopeProduct<'a>),
    UndoScope(&'a SpatialUndoScopeProduct<'a>),
}

pub fn lower_replay_undo_transaction_mutation_claims(
    sources: &[ReplayUndoTransactionMutationClaimSource<'_>],
) -> Vec<ReplayUndoTransactionScopeClaim> {
    sources
        .iter()
        .map(|source| match source {
            ReplayUndoTransactionMutationClaimSource::ReplayScope(scope_product) => {
                ReplayUndoTransactionScopeClaim::new(
                    ReplayUndoTransactionScopeKind::Replay,
                    scope_product.scope_identity().digest(),
                )
            }
            ReplayUndoTransactionMutationClaimSource::UndoScope(scope_product) => {
                ReplayUndoTransactionScopeClaim::new(
                    ReplayUndoTransactionScopeKind::Undo,
                    scope_product.scope_identity().digest(),
                )
            }
        })
        .collect()
}
