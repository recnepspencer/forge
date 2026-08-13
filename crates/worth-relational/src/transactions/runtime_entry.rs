//! Runtime-owned entry into a new Relational transaction.

use super::data::TransactionOptions;
use super::RelationalTransaction;
use crate::runtime::RelationalRuntime;

impl RelationalRuntime {
    pub fn begin_transaction<'a>(
        &'a mut self,
        options: TransactionOptions,
    ) -> RelationalTransaction<'a> {
        let transaction_id = self.services.next_transaction_id();
        RelationalTransaction {
            runtime: self,
            transaction_id,
            options,
            batches: Vec::new(),
            savepoints: Vec::new(),
            last_merged_plan: None,
        }
    }
}
