use crate::failure::StoreError;
use crate::wal::{DurableMutationId, DurablePublicationPhase, WalRecord};

use super::{StateBackedStoreBackend, StatePersistence};

impl<P: StatePersistence> StateBackedStoreBackend<P> {
    pub fn admit_durable_mutation(
        &mut self,
        runtime_session_id: &str,
        operation_name: &str,
    ) -> Result<DurableMutationId, StoreError> {
        let durable_mutation_id = DurableMutationId(self.state.next_durable_mutation_id);
        let record = WalRecord::durable_mutation_intent(
            self.state.next_wal_sequence,
            durable_mutation_id,
            runtime_session_id,
            operation_name,
        )?;
        self.state.next_durable_mutation_id += 1;
        if let Err(error) = self.append_wal_record_committed(record) {
            self.state.next_durable_mutation_id = durable_mutation_id.0;
            return Err(error);
        }
        self.counters.record_durable_mutation_admit();
        self.counters.record_wal_append();
        Ok(durable_mutation_id)
    }

    pub fn record_hosted_runtime_commit_result(
        &mut self,
        runtime_session_id: &str,
        durable_mutation_id: DurableMutationId,
        envelope: forge_relational::facade::replay::CanonicalCommitEnvelope,
    ) -> Result<(), StoreError> {
        let record = WalRecord::hosted_runtime_commit_result(
            self.state.next_wal_sequence,
            durable_mutation_id,
            runtime_session_id,
            envelope,
        )?;
        self.append_wal_record_committed(record)?;
        self.counters.record_wal_append();
        Ok(())
    }

    pub fn record_publication_phase(
        &mut self,
        runtime_session_id: &str,
        durable_mutation_id: DurableMutationId,
        phase: DurablePublicationPhase,
        commit_id: Option<forge_relational::facade::history::CommitId>,
    ) -> Result<(), StoreError> {
        let record = WalRecord::durable_publication_progress(
            self.state.next_wal_sequence,
            durable_mutation_id,
            runtime_session_id,
            phase,
            commit_id,
        )?;
        self.append_wal_record_committed(record)?;
        self.counters.record_wal_append();
        Ok(())
    }

    pub fn record_bulk_checkpoint_publication_intent(
        &mut self,
        runtime_session_id: &str,
        durable_mutation_id: DurableMutationId,
        checkpoint_sequence: Option<u64>,
    ) -> Result<(), StoreError> {
        let record = WalRecord::bulk_checkpoint_publication_intent(
            self.state.next_wal_sequence,
            durable_mutation_id,
            runtime_session_id,
            checkpoint_sequence,
        )?;
        self.append_wal_record_committed(record)?;
        self.counters.record_wal_append();
        Ok(())
    }
}
