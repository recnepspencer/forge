use super::baseline_lsm_compaction_execution::BaselineLsmCompactionExecution;
use super::baseline_lsm_counter_support::{
    seeded_compaction_state, seeded_manifest_publication, seeded_memtable_records,
    seeded_replay_tail, seeded_sorted_run_records, seeded_wal_publication,
};
use super::{
    BaselineLsmCounterObservation, BaselineLsmExecutionWitness, BaselineLsmLookupDisposition,
    BaselineLsmLookupExecution, BaselineLsmManifestPublicationExecution,
    BaselineLsmReplayExecution,
};
use crate::{
    record_kind_admits_recovery_replay, BlobWalRecordEnvelope, BlobWalRecordIdentity,
    DurablePublicationDeclaration, DurablePublicationScope,
};

impl BaselineLsmExecutionWitness {
    pub fn seeded() -> Self {
        Self {
            memtable_records: seeded_memtable_records(),
            sorted_run_records: seeded_sorted_run_records(),
            wal_publication: seeded_wal_publication(),
            manifest_publication: seeded_manifest_publication(),
            replay_tail: seeded_replay_tail(),
        }
    }

    pub fn execute_lookup_latest_visible_record(
        &self,
        probe_sequence: u64,
    ) -> BaselineLsmLookupExecution {
        let memtable_record = self.memtable_records[0];
        let sorted_run_record = self.sorted_run_records[0];
        let disposition = self.lookup_disposition_for(probe_sequence);
        let tombstone_blocks_older = disposition == BaselineLsmLookupDisposition::NotFound
            && probe_sequence == sorted_run_record.sequence();
        BaselineLsmLookupExecution::new(
            probe_sequence,
            disposition,
            memtable_record,
            sorted_run_record,
            disposition == BaselineLsmLookupDisposition::Memtable,
            disposition == BaselineLsmLookupDisposition::SortedRun,
            tombstone_blocks_older,
            BaselineLsmCounterObservation::new(1, 1, 0, 0, 0),
        )
    }

    pub fn execute_lookup_older_visible_record(&self) -> BaselineLsmLookupExecution {
        let older_visible = self.sorted_run_records[1].sequence();
        let sorted_run_record = self.sorted_run_records[1];
        BaselineLsmLookupExecution::new(
            older_visible,
            BaselineLsmLookupDisposition::SortedRun,
            self.memtable_records[0],
            sorted_run_record,
            false,
            true,
            false,
            BaselineLsmCounterObservation::new(1, 1, 0, 0, 0),
        )
    }

    pub fn execute_lookup_tombstone_blocked_record(&self) -> BaselineLsmLookupExecution {
        let blocked_older = self.sorted_run_records[0].sequence();
        BaselineLsmLookupExecution::new(
            blocked_older,
            BaselineLsmLookupDisposition::NotFound,
            self.memtable_records[0],
            self.sorted_run_records[0],
            false,
            true,
            true,
            BaselineLsmCounterObservation::new(1, 1, 0, 0, 0),
        )
    }

    pub fn execute_manifest_publication(&self) -> BaselineLsmManifestPublicationExecution {
        assert!(matches!(
            self.wal_publication.durable_publication().scope(),
            DurablePublicationScope::WalFrame(_)
        ));
        assert!(matches!(
            self.manifest_publication.scope(),
            DurablePublicationScope::Manifest(_)
        ));
        BaselineLsmManifestPublicationExecution::new(
            self.wal_publication.clone(),
            self.manifest_publication.clone(),
            self.sorted_run_records.len() as u16,
            self.sorted_run_records.len() > 1,
            false,
            BaselineLsmCounterObservation::new(0, 0, 0, 2, self.sorted_run_records.len() as u16),
        )
    }

    pub fn execute_replay_wal_tail(&self) -> BaselineLsmReplayExecution {
        let replay_tail = self
            .replay_tail
            .clone()
            .map(|record| record.identity().kind());
        let replayable = self
            .replay_tail
            .iter()
            .filter(|record| record_kind_admits_recovery_replay(record.identity().kind()))
            .count() as u16;
        BaselineLsmReplayExecution::new(
            replay_tail,
            replayable,
            (self.sorted_run_records.len() - 1) as u16,
            1,
            self.sorted_run_records.len() as u16,
            BaselineLsmCounterObservation::new(0, 0, replayable, 0, 1),
        )
    }

    pub fn execute_compaction_ordering(&self) -> BaselineLsmCompactionExecution {
        let state = seeded_compaction_state();
        BaselineLsmCompactionExecution::new(
            state.older_generation,
            state.newer_generation,
            true,
            state.older_generation < state.newer_generation,
            state.newer_generation > state.middle_generation,
            [
                state.older_generation,
                state.middle_generation,
                state.newer_generation,
            ],
            state.output_generation,
            state.stale_runs_retired,
            state.bytes_in,
            state.bytes_out,
            state.rewritten_runs,
        )
    }

    pub fn lookup_disposition_for(&self, probe_sequence: u64) -> BaselineLsmLookupDisposition {
        if self
            .memtable_records
            .iter()
            .any(|record| record.sequence() == probe_sequence)
        {
            return BaselineLsmLookupDisposition::Memtable;
        }
        if probe_sequence == self.sorted_run_records[1].sequence() {
            return BaselineLsmLookupDisposition::SortedRun;
        }
        BaselineLsmLookupDisposition::NotFound
    }

    pub const fn memtable_records(&self) -> &[BlobWalRecordIdentity; 1] {
        &self.memtable_records
    }

    pub const fn sorted_run_records(&self) -> &[BlobWalRecordIdentity; 2] {
        &self.sorted_run_records
    }

    pub const fn wal_publication(&self) -> &BlobWalRecordEnvelope {
        &self.wal_publication
    }

    pub const fn manifest_publication(&self) -> &DurablePublicationDeclaration {
        &self.manifest_publication
    }

    pub const fn replay_tail(&self) -> &[BlobWalRecordEnvelope; 3] {
        &self.replay_tail
    }
}
