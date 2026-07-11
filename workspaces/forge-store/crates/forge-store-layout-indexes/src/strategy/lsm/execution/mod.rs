#[path = "../compaction/execution.rs"]
mod baseline_lsm_compaction_execution;
#[path = "../compaction/transition.rs"]
mod baseline_lsm_compaction_transition;
#[path = "request.rs"]
mod baseline_lsm_execution_request;
#[path = "witness.rs"]
mod baseline_lsm_execution_witness;

pub use baseline_lsm_compaction_execution::{
    BaselineLsmCompactionKeyIdentity, BaselineLsmCompactionPublicationReceipt,
    BaselineLsmCompactionRecordIdentity, BaselineLsmCompactionRecordKind, BaselineLsmRunIdentity,
};
pub use baseline_lsm_compaction_transition::BaselineLsmCompactionTransition;
#[cfg(test)]
pub(crate) use baseline_lsm_execution_request::baseline_lsm_manifest_membership_digest;
pub(crate) use baseline_lsm_execution_request::BaselineLsmExecutionRequest;
pub use baseline_lsm_execution_request::{
    baseline_lsm_manifest_artifact_bytes, baseline_lsm_output_artifact_bytes,
    baseline_lsm_record_artifact_bytes,
};
pub use baseline_lsm_execution_request::{
    BaselineLsmAdmittedKey, BaselineLsmAdmittedRecord, BaselineLsmCompactionPlan,
    BaselineLsmDurableInputs, BaselineLsmExecutionIntent, BaselineLsmMembershipObservation,
    BaselineLsmPhysicalPublicationBinding, BaselineLsmWalIndexSession,
};

use forge_store_wal::{
    BlobWalRecordEnvelope, BlobWalRecordIdentity, BlobWalRecordKind, DurablePublicationDeclaration,
    DurablePublicationScope,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BaselineLsmCounterObservation {
    point_lookups: u16,
    range_lookups: u16,
    wal_replays: u16,
    publications: u16,
    maintenance_reads: u16,
}

impl BaselineLsmCounterObservation {
    const fn new(
        point_lookups: u16,
        range_lookups: u16,
        wal_replays: u16,
        publications: u16,
        maintenance_reads: u16,
    ) -> Self {
        Self {
            point_lookups,
            range_lookups,
            wal_replays,
            publications,
            maintenance_reads,
        }
    }

    pub const fn point_lookups(self) -> u16 {
        self.point_lookups
    }

    pub const fn range_lookups(self) -> u16 {
        self.range_lookups
    }

    pub const fn wal_replays(self) -> u16 {
        self.wal_replays
    }

    pub const fn publications(self) -> u16 {
        self.publications
    }

    pub const fn maintenance_reads(self) -> u16 {
        self.maintenance_reads
    }

    pub(super) fn record_maintenance_read(&mut self) {
        self.maintenance_reads = self.maintenance_reads.saturating_add(1);
    }

    pub(super) fn record_publication(&mut self) {
        self.publications = self.publications.saturating_add(1);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaselineLsmLookupDisposition {
    Memtable,
    SortedRun,
    NotFound,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BaselineLsmLookupExecution {
    probe_sequence: u64,
    disposition: BaselineLsmLookupDisposition,
    memtable_record: BlobWalRecordIdentity,
    sorted_run_record: BlobWalRecordIdentity,
    probe_visible_in_newer_run: bool,
    probe_visible_in_older_run: bool,
    tombstone_blocks_older: bool,
    counters: BaselineLsmCounterObservation,
}

impl BaselineLsmLookupExecution {
    const fn new(
        probe_sequence: u64,
        disposition: BaselineLsmLookupDisposition,
        memtable_record: BlobWalRecordIdentity,
        sorted_run_record: BlobWalRecordIdentity,
        probe_visible_in_newer_run: bool,
        probe_visible_in_older_run: bool,
        tombstone_blocks_older: bool,
        counters: BaselineLsmCounterObservation,
    ) -> Self {
        Self {
            probe_sequence,
            disposition,
            memtable_record,
            sorted_run_record,
            probe_visible_in_newer_run,
            probe_visible_in_older_run,
            tombstone_blocks_older,
            counters,
        }
    }

    pub const fn probe_sequence(self) -> u64 {
        self.probe_sequence
    }

    pub const fn disposition(self) -> BaselineLsmLookupDisposition {
        self.disposition
    }

    pub const fn memtable_record(self) -> BlobWalRecordIdentity {
        self.memtable_record
    }

    pub const fn sorted_run_record(self) -> BlobWalRecordIdentity {
        self.sorted_run_record
    }

    pub const fn probe_visible_in_newer_run(self) -> bool {
        self.probe_visible_in_newer_run
    }

    pub const fn probe_visible_in_older_run(self) -> bool {
        self.probe_visible_in_older_run
    }

    pub const fn tombstone_blocks_older(self) -> bool {
        self.tombstone_blocks_older
    }

    pub const fn counters(self) -> BaselineLsmCounterObservation {
        self.counters
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaselineLsmManifestPublicationExecution {
    wal_publication: BlobWalRecordEnvelope,
    manifest_publication: DurablePublicationDeclaration,
    published_run_count: u16,
    stale_runs_removed: bool,
    advisory_filter_present: bool,
    counters: BaselineLsmCounterObservation,
}

impl BaselineLsmManifestPublicationExecution {
    fn new(
        wal_publication: BlobWalRecordEnvelope,
        manifest_publication: DurablePublicationDeclaration,
        published_run_count: u16,
        stale_runs_removed: bool,
        advisory_filter_present: bool,
        counters: BaselineLsmCounterObservation,
    ) -> Self {
        Self {
            wal_publication,
            manifest_publication,
            published_run_count,
            stale_runs_removed,
            advisory_filter_present,
            counters,
        }
    }

    pub const fn wal_publication(&self) -> &BlobWalRecordEnvelope {
        &self.wal_publication
    }

    pub const fn manifest_publication(&self) -> &DurablePublicationDeclaration {
        &self.manifest_publication
    }

    pub fn manifest_sequence_advanced(&self) -> bool {
        matches!(
            self.manifest_publication.scope(),
            DurablePublicationScope::Manifest(scope) if scope.covered_lsn_end() > scope.covered_lsn_start()
        )
    }

    pub const fn published_run_count(&self) -> u16 {
        self.published_run_count
    }

    pub const fn stale_runs_removed(&self) -> bool {
        self.stale_runs_removed
    }

    pub const fn advisory_filter_present(&self) -> bool {
        self.advisory_filter_present
    }

    pub const fn counters(&self) -> BaselineLsmCounterObservation {
        self.counters
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BaselineLsmReplayExecution {
    replay_tail: [BlobWalRecordKind; 3],
    replayable_count: u16,
    stale_run_count: u16,
    cleanup_batch_count: u16,
    remaining_run_count: u16,
    counters: BaselineLsmCounterObservation,
}

impl BaselineLsmReplayExecution {
    const fn new(
        replay_tail: [BlobWalRecordKind; 3],
        replayable_count: u16,
        stale_run_count: u16,
        cleanup_batch_count: u16,
        remaining_run_count: u16,
        counters: BaselineLsmCounterObservation,
    ) -> Self {
        Self {
            replay_tail,
            replayable_count,
            stale_run_count,
            cleanup_batch_count,
            remaining_run_count,
            counters,
        }
    }

    pub const fn replay_tail(self) -> [BlobWalRecordKind; 3] {
        self.replay_tail
    }

    pub const fn replayable_count(self) -> u16 {
        self.replayable_count
    }

    pub const fn replay_monotonic(self) -> bool {
        self.replayable_count > 0
    }

    pub const fn stale_run_count(self) -> u16 {
        self.stale_run_count
    }

    pub const fn cleanup_batch_count(self) -> u16 {
        self.cleanup_batch_count
    }

    pub const fn remaining_run_count(self) -> u16 {
        self.remaining_run_count
    }

    pub const fn counters(self) -> BaselineLsmCounterObservation {
        self.counters
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaselineLsmExecutionWitness {
    pub(crate) memtable_records: [BlobWalRecordIdentity; 1],
    pub(crate) sorted_run_records: [BlobWalRecordIdentity; 2],
    pub(crate) wal_publication: BlobWalRecordEnvelope,
    pub(crate) manifest_publication: DurablePublicationDeclaration,
    pub(crate) replay_tail: [BlobWalRecordEnvelope; 3],
    pub(crate) compaction: BaselineLsmCompactionPublicationReceipt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaselineLsmExecutionAdmissionDenial {
    CanonicalKeyRequired,
    MemtableDoesNotFollowSortedRuns,
    SortedRunsNotCanonical,
    ReplayTailNotCanonical,
    ReplayBindingMismatch,
    TombstoneRecordRequired,
    ValueRecordRequired,
    OutputGenerationOverflow,
    OutputPublicationMismatch,
    ManifestPublicationRequired,
    ManifestDoesNotCoverCompaction,
    ManifestMembershipMismatch,
    PersistedMembershipAmbiguous,
    PersistedMembershipIncomplete,
    PersistedMembershipStale,
    PersistedIndexIo,
    PhysicalTargetEpochRequired,
    DurableRecordBindingMismatch,
    RecordKeyScopeMismatch,
    PhysicalPublicationBindingMismatch,
}
