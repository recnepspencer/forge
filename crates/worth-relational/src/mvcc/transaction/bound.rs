use crate::transactions::data::{MergedCommitPlan, TransactionId, WorkerIntentBatch};

use super::{
    DetachedRelationalTransactionOverlay, RelationalTransactionFootprint,
    RelationalTransactionIntent,
};

/// Move-only detached transaction bound to one owner-admitted exact basis.
/// It contains no reference to `RelationalRuntime`.
#[derive(Debug)]
pub struct BranchBoundRelationalTransaction {
    pub(crate) basis: crate::branch::AdmittedRelationalBranchBasis,
    pub(crate) mutation_authority: crate::branch::RelationalBranchMutationAuthority,
    pub(crate) transaction_id: TransactionId,
    pub(crate) intent: RelationalTransactionIntent,
    pub(crate) merge_parent_bases: Vec<crate::branch::AdmittedRelationalBranchBasis>,
    pub(crate) schema_authority_input: Option<crate::schema::SchemaContinuityAuthorityInput>,
    pub(crate) schema_authority: std::sync::Arc<crate::branch::RelationalBranchRootSchemaAuthority>,
    pub(crate) overlay: DetachedRelationalTransactionOverlay,
    pub(crate) footprint: RelationalTransactionFootprint,
    pub(crate) savepoints: Vec<super::RelationalTransactionSavepoint>,
    pub(crate) next_savepoint_ordinal: u64,
    pub(crate) last_merged_plan: Option<MergedCommitPlan>,
    pub(crate) client_key_symbol_policy: crate::symbols::data::ClientKeySymbolPolicy,
}

impl BranchBoundRelationalTransaction {
    pub fn transaction_id(&self) -> TransactionId {
        self.transaction_id
    }

    pub fn basis(&self) -> &crate::branch::AdmittedRelationalBranchBasis {
        &self.basis
    }

    pub fn footprint(&self) -> &RelationalTransactionFootprint {
        &self.footprint
    }

    pub fn push_batch(&mut self, batch: WorkerIntentBatch) {
        self.overlay.stage(batch, &mut self.footprint);
        self.last_merged_plan = None;
    }

    pub(crate) fn batches(&self) -> &[WorkerIntentBatch] {
        self.overlay.batches()
    }

    pub fn commit(
        self,
        runtime: &mut crate::runtime::RelationalRuntime,
    ) -> Result<
        crate::transactions::data::CommitResult,
        crate::transactions::data::TransactionCommitError,
    > {
        runtime.commit_branch_transaction(self)
    }

    pub fn validate(
        self,
        runtime: &mut crate::runtime::RelationalRuntime,
    ) -> Result<
        crate::mvcc::ValidatedRelationalProposal,
        crate::transactions::data::TransactionCommitError,
    > {
        runtime.validate_branch_transaction(self)
    }
}
