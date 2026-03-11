use crate::transactions::data::{
    MergedCommitPlan, SavepointId, TransactionOptions, WorkerIntentBatch,
};

use crate::logic::runtime::RelationalRuntime;

#[derive(Debug)]
pub struct RelationalTransaction<'a> {
    pub(crate) runtime: &'a mut RelationalRuntime,
    pub(crate) transaction_id: crate::transactions::data::TransactionId,
    pub(crate) options: TransactionOptions,
    pub(crate) batches: Vec<WorkerIntentBatch>,
    pub(crate) savepoints: Vec<(SavepointId, usize)>,
    pub(crate) last_merged_plan: Option<MergedCommitPlan>,
}
