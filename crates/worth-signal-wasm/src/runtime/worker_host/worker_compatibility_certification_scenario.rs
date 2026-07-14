use serde::{Deserialize, Serialize};

use crate::recipe::model::TransactionOp;

use super::WorkerPortableGraphPublication;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerCompatibilityCertificationScenario {
    pub publication: WorkerPortableGraphPublication,
    pub transaction_ops: Vec<TransactionOp>,
    pub feature_transaction_ops: Vec<TransactionOp>,
    pub main_transaction_ops: Vec<TransactionOp>,
    pub observed_signal_id: String,
    pub async_signal_id: String,
    pub async_payload_contract_id: u64,
    pub async_payload_byte_len: u64,
    pub independent_region_recipe_ids: Vec<String>,
}
