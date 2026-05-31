use forge_relational::facade::history::BranchId;
use forge_relational::facade::runtime::RelationalRuntime;
use forge_relational::facade::transactions::{
    CommitResult, MutationIntent, TransactionCommitError, TransactionOptions, WorkerIntentBatch,
};

#[derive(Debug)]
pub enum TopologyMutationSetCommitError {
    Commit(TransactionCommitError),
}

impl From<TransactionCommitError> for TopologyMutationSetCommitError {
    fn from(value: TransactionCommitError) -> Self {
        Self::Commit(value)
    }
}

impl std::fmt::Display for TopologyMutationSetCommitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Commit(error) => write!(f, "{error:?}"),
        }
    }
}

impl std::error::Error for TopologyMutationSetCommitError {}

pub(crate) fn commit_topology_mutation_set_internal(
    runtime: &mut RelationalRuntime,
    transaction_label: &'static str,
    mutations: impl IntoIterator<Item = MutationIntent>,
) -> Result<CommitResult, TransactionCommitError> {
    let mutation_transaction = mutations.into_iter().fold(
        WorkerIntentBatch::new(transaction_label),
        |mutation_transaction, mutation| mutation_transaction.push(mutation),
    );
    let mut tx = runtime.begin_transaction(TransactionOptions::default());
    tx.push_batch(mutation_transaction);
    tx.commit()
}

pub(crate) fn commit_topology_mutation_set_on_branch_internal(
    runtime: &mut RelationalRuntime,
    branch_id: &BranchId,
    transaction_label: &'static str,
    mutations: impl IntoIterator<Item = MutationIntent>,
) -> Result<CommitResult, TransactionCommitError> {
    let mutation_transaction = mutations.into_iter().fold(
        WorkerIntentBatch::new(transaction_label),
        |mutation_transaction, mutation| mutation_transaction.push(mutation),
    );
    let mut tx = runtime.begin_transaction(TransactionOptions {
        target_branch: Some(branch_id.clone()),
        ..TransactionOptions::default()
    });
    tx.push_batch(mutation_transaction);
    tx.commit()
}

pub fn commit_topology_mutation_set(
    runtime: &mut RelationalRuntime,
    transaction_label: &'static str,
    mutations: impl IntoIterator<Item = MutationIntent>,
) -> Result<CommitResult, TopologyMutationSetCommitError> {
    commit_topology_mutation_set_internal(runtime, transaction_label, mutations).map_err(Into::into)
}

pub fn commit_topology_mutation_set_on_branch(
    runtime: &mut RelationalRuntime,
    branch_id: BranchId,
    transaction_label: &'static str,
    mutations: impl IntoIterator<Item = MutationIntent>,
) -> Result<CommitResult, TopologyMutationSetCommitError> {
    commit_topology_mutation_set_on_branch_internal(
        runtime,
        &branch_id,
        transaction_label,
        mutations,
    )
    .map_err(Into::into)
}
