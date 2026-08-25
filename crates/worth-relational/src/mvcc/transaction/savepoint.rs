use crate::transactions::data::{
    CommitConflict, ConflictClass, RollbackOutcome, RollbackSummary, SavepointId,
};

#[derive(Clone, Debug)]
pub(crate) struct RelationalTransactionSavepoint {
    id: SavepointId,
    retained_batch_count: usize,
    footprint: super::RelationalTransactionFootprint,
}

impl RelationalTransactionSavepoint {
    pub(crate) fn new(
        id: SavepointId,
        retained_batch_count: usize,
        footprint: super::RelationalTransactionFootprint,
    ) -> Self {
        Self {
            id,
            retained_batch_count,
            footprint,
        }
    }

    pub(crate) const fn id(&self) -> SavepointId {
        self.id
    }

    pub(crate) const fn retained_batch_count(&self) -> usize {
        self.retained_batch_count
    }

    pub(crate) fn footprint(&self) -> &super::RelationalTransactionFootprint {
        &self.footprint
    }
}

impl super::BranchBoundRelationalTransaction {
    pub fn create_savepoint(&mut self) -> SavepointId {
        assert!(
            self.intent.allow_nested_savepoints(),
            "nested savepoints are disabled for this transaction"
        );
        let savepoint_id = SavepointId(self.next_savepoint_ordinal);
        self.next_savepoint_ordinal = self
            .next_savepoint_ordinal
            .checked_add(1)
            .expect("transaction-local savepoint identity exhausted");
        self.savepoints.push(RelationalTransactionSavepoint::new(
            savepoint_id,
            self.batches().len(),
            self.footprint.clone(),
        ));
        savepoint_id
    }

    pub fn rollback_to_savepoint(
        &mut self,
        savepoint_id: SavepointId,
    ) -> Result<RollbackOutcome, CommitConflict> {
        let Some(index) = self
            .savepoints
            .iter()
            .position(|candidate| candidate.id() == savepoint_id)
        else {
            return Err(CommitConflict::new(ConflictClass::InvalidSavepoint {
                savepoint_id,
            }));
        };
        let batch_len = self.savepoints[index].retained_batch_count();
        let restored_footprint = self.savepoints[index].footprint().clone();
        let drained = self
            .overlay
            .truncate_batches(batch_len, &mut self.footprint, &self.basis);
        self.footprint = restored_footprint;
        self.last_merged_plan = None;
        self.savepoints.truncate(index);
        let effects = drained
            .into_iter()
            .flat_map(|batch| batch.intents.into_iter())
            .map(|intent| intent.rollback_effect())
            .collect::<Vec<_>>();
        let summary = RollbackSummary::from_effects(&effects);
        Ok(RollbackOutcome {
            transaction_id: self.transaction_id,
            summary,
            effects,
        })
    }
}
