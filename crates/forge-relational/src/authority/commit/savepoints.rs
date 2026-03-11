use serde_json::json;

use crate::transactions::logic::RelationalTransaction;
use crate::diagnostics::data::{
    DiagnosticCode, DiagnosticsScope,
};
use crate::transactions::data::{
    CommitConflict, RollbackOutcome, SavepointId, WorkerIntentBatch,
};

impl<'a> RelationalTransaction<'a> {
    pub fn transaction_id(&self) -> crate::transactions::data::TransactionId {
        self.transaction_id
    }

    pub fn push_batch(&mut self, batch: WorkerIntentBatch) {
        self.batches.push(batch);
    }

    pub fn create_savepoint(&mut self) -> SavepointId {
        assert!(
            self.options.allow_nested_savepoints,
            "nested savepoints are disabled for this transaction"
        );
        let savepoint_id = SavepointId(self.runtime.sequence.next_savepoint_id);
        self.runtime.sequence.next_savepoint_id += 1;
        self.savepoints.push((savepoint_id, self.batches.len()));
        savepoint_id
    }

    pub fn rollback_to_savepoint(
        &mut self,
        savepoint_id: SavepointId,
    ) -> Result<RollbackOutcome, CommitConflict> {
        let Some(index) = self
            .savepoints
            .iter()
            .position(|(candidate, _)| *candidate == savepoint_id)
        else {
            return Err(CommitConflict {
                code: DiagnosticCode::InvalidSavepoint,
                detail: format!("savepoint {:?} does not exist", savepoint_id),
            });
        };
        let (_, batch_len) = self.savepoints[index];
        let drained = self.batches.split_off(batch_len);
        self.savepoints.truncate(index);
        let effects = drained
            .into_iter()
            .flat_map(|batch| batch.intents.into_iter())
            .map(|intent| intent.rollback_effect())
            .collect();
        self.runtime
            .diagnostic(DiagnosticsScope::Transaction)
            .rollback()
            .emit_entry(
                DiagnosticCode::SavepointRolledBack,
                "rolled back to savepoint",
                json!({ "savepoint_id": savepoint_id.0 }),
            );
        Ok(RollbackOutcome {
            transaction_id: self.transaction_id,
            effects,
        })
    }
}
