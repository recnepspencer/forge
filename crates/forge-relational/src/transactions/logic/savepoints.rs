use serde_json::json;

use super::RelationalTransaction;
use crate::diagnostics::data::{
    DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope, RelationalDiagnosticsEntry,
};
use crate::identity::data::{EntityId, PartitionId, RelationId};
use crate::transactions::data::{
    CommitConflict, RecordRef, RollbackOutcome, SavepointId, TransactionIntent, WorkerIntentBatch,
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
        let restored_records = drained
            .into_iter()
            .flat_map(|batch| batch.intents.into_iter())
            .map(|intent| match intent {
                TransactionIntent::CreateEntity(_)
                | TransactionIntent::BulkCreateEntities { .. } => {
                    RecordRef::Entity(EntityId::new(PartitionId::main(), u64::MAX, 0))
                }
                TransactionIntent::UpdateEntity { entity_id, .. } => RecordRef::Entity(entity_id),
                TransactionIntent::ReplaceEntity { entity_id, .. } => RecordRef::Entity(entity_id),
                TransactionIntent::DeleteEntity { entity_id } => RecordRef::Entity(entity_id),
                TransactionIntent::CreateRelation(_)
                | TransactionIntent::BulkCreateRelations { .. } => {
                    RecordRef::Relation(RelationId::new(PartitionId::main(), u64::MAX, 0))
                }
                TransactionIntent::DeleteRelation { relation_id } => {
                    RecordRef::Relation(relation_id)
                }
            })
            .collect();
        self.runtime.push_bounded_diagnostic(
            DiagnosticsScope::Transaction,
            DiagnosticsArtifactKind::Rollback,
            vec![RelationalDiagnosticsEntry {
                code: DiagnosticCode::SavepointRolledBack,
                message: "rolled back to savepoint".to_string(),
                fields: json!({ "savepoint_id": savepoint_id.0 }),
            }],
        );
        Ok(RollbackOutcome {
            transaction_id: self.transaction_id,
            restored_records,
        })
    }
}
