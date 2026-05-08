use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerHostBoundaryCausality {
    pub transaction_sequence: u64,
    pub generation: u64,
    pub ordering_basis: String,
}

impl WorkerHostBoundaryCausality {
    pub(in crate::runtime::worker_host) fn new(transaction_sequence: u64) -> Self {
        Self {
            transaction_sequence,
            generation: 0,
            ordering_basis: "transactionSequenceThenGeneration".to_owned(),
        }
    }
}
