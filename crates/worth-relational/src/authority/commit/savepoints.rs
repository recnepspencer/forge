use crate::diagnostics::data::{
    DiagnosticCode, DiagnosticsScope, RelationalDiagnosticFields, RelationalDiagnosticValue,
};
use crate::transactions::data::{
    CommitConflict, ConflictClass, RollbackOutcome, RollbackSummary, SavepointId, WorkerIntentBatch,
};
use crate::transactions::RelationalTransaction;

impl<'a> RelationalTransaction<'a> {
    pub fn transaction_id(&self) -> crate::transactions::data::TransactionId {
        self.transaction_id
    }

    pub fn push_batch(&mut self, batch: WorkerIntentBatch) {
        self.batches.push(batch);
    }

    pub fn create_savepoint(&mut self) -> SavepointId {
        assert!(
            self.options.allow_nested_savepoints(),
            "nested savepoints are disabled for this transaction"
        );
        let savepoint_id = self.runtime.services.next_savepoint_id();
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
            let _ = DiagnosticCode::InvalidSavepoint;
            return Err(CommitConflict::new(ConflictClass::InvalidSavepoint {
                savepoint_id,
            }));
        };
        let (_, batch_len) = self.savepoints[index];
        let drained = self.batches.split_off(batch_len);
        self.savepoints.truncate(index);
        let effects = drained
            .into_iter()
            .flat_map(|batch| batch.intents.into_iter())
            .map(|intent| intent.rollback_effect())
            .collect::<Vec<_>>();
        let summary = RollbackSummary::from_effects(&effects);
        self.runtime
            .publication_authority()
            .diagnostic(DiagnosticsScope::Transaction)
            .rollback()
            .emit_entry(
                DiagnosticCode::SavepointRolledBack,
                "rolled back to savepoint",
                savepoint_rollback_fields(savepoint_id),
            );
        Ok(RollbackOutcome {
            transaction_id: self.transaction_id,
            summary,
            effects,
        })
    }
}

fn savepoint_rollback_fields(savepoint_id: SavepointId) -> RelationalDiagnosticFields {
    RelationalDiagnosticValue::object([(
        "savepoint_id",
        RelationalDiagnosticValue::Unsigned(savepoint_id.0),
    )])
    .into()
}
