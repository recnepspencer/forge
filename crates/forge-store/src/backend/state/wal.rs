use crate::backend::records::StoreState;
use crate::failure::{StoreError, StoreErrorKind};
use crate::wal::{DurableMutationId, WalRecord};

impl StoreState {
    pub fn allocate_durable_mutation_id(&mut self) -> DurableMutationId {
        let id = DurableMutationId(self.next_durable_mutation_id);
        self.next_durable_mutation_id += 1;
        id
    }

    pub fn append_wal_record(&mut self, record: WalRecord) -> Result<(), StoreError> {
        if record.wal_sequence != self.next_wal_sequence {
            return Err(StoreError::new(
                StoreErrorKind::DurablePublicationStateGap,
                format!(
                    "wal record sequence {} does not match expected next sequence {}",
                    record.wal_sequence, self.next_wal_sequence
                ),
            ));
        }
        self.wal_records.insert(record.wal_sequence, record);
        self.next_wal_sequence += 1;
        Ok(())
    }

    pub fn wal_records_for_mutation(
        &self,
        durable_mutation_id: DurableMutationId,
    ) -> Vec<&WalRecord> {
        self.wal_records
            .values()
            .filter(|record| record.durable_mutation_id == durable_mutation_id)
            .collect()
    }

    pub fn verify_wal_record_family(&self) -> Result<(), StoreError> {
        let mut expected_sequence = 1_u64;
        for (sequence, record) in &self.wal_records {
            if *sequence != expected_sequence {
                return Err(StoreError::new(
                    StoreErrorKind::DurablePublicationStateGap,
                    format!(
                        "wal sequence gap detected: expected {}, found {}",
                        expected_sequence, sequence
                    ),
                ));
            }
            record.validate_integrity()?;
            expected_sequence += 1;
        }
        if self.next_wal_sequence != expected_sequence {
            return Err(StoreError::new(
                StoreErrorKind::DurablePublicationStateGap,
                format!(
                    "next wal sequence {} does not match expected {}",
                    self.next_wal_sequence, expected_sequence
                ),
            ));
        }
        Ok(())
    }
}
