//! Execution evidence issued by the LSM compaction owner.

use worth_store_lsm_authority::LsmMembershipKey;
use worth_store_wal::{
    BlobWalRecordEnvelope, BlobWalRecordIdentity, PublicationDeclaration, PublicationScope,
};

use super::super::BaselineLsmCounterObservation;

pub(super) struct BaselineLsmCompactionExecutionEffects {
    retired_runs: [BaselineLsmRunIdentity; 3],
    counters: BaselineLsmCounterObservation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BaselineLsmCompactionKeyIdentity {
    key: LsmMembershipKey,
}

impl BaselineLsmCompactionKeyIdentity {
    pub const fn tenant_scope(self) -> worth_store_security::StoreTenantScope {
        self.key.tenant_scope()
    }

    pub const fn key_scope(self) -> worth_store_security::StoreKeyScope {
        self.key.key_scope()
    }

    pub fn canonical_key_bytes(&self) -> &[u8] {
        self.key.canonical()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BaselineLsmRunIdentity {
    generation: u64,
    root_record: BlobWalRecordIdentity,
}

impl BaselineLsmRunIdentity {
    pub(super) const fn new_for_executor(root_record: BlobWalRecordIdentity) -> Self {
        Self {
            generation: root_record.sequence(),
            root_record,
        }
    }

    pub const fn generation(self) -> u64 {
        self.generation
    }

    pub const fn root_record(self) -> BlobWalRecordIdentity {
        self.root_record
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaselineLsmCompactionRecordKind {
    Value,
    Tombstone,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BaselineLsmCompactionRecordIdentity {
    key: BaselineLsmCompactionKeyIdentity,
    wal_record: BlobWalRecordIdentity,
    run: BaselineLsmRunIdentity,
    kind: BaselineLsmCompactionRecordKind,
}

impl BaselineLsmCompactionRecordIdentity {
    pub const fn key(self) -> BaselineLsmCompactionKeyIdentity {
        self.key
    }

    pub const fn wal_record(self) -> BlobWalRecordIdentity {
        self.wal_record
    }

    pub const fn run(self) -> BaselineLsmRunIdentity {
        self.run
    }

    pub const fn kind(self) -> BaselineLsmCompactionRecordKind {
        self.kind
    }
}

/// WAL-owner-produced compaction and publication evidence.
///
/// Its fixed-shape input set, scoped canonical key, output identity, manifest
/// publication, replay binding, and counters are minted together by the WAL
/// executor. No downstream crate can construct or partially rebind this receipt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaselineLsmCompactionPublicationReceipt {
    key: BaselineLsmCompactionKeyIdentity,
    input_runs: [BaselineLsmRunIdentity; 3],
    output_run: BaselineLsmRunIdentity,
    output_publication: BlobWalRecordEnvelope,
    tombstone_record: BaselineLsmCompactionRecordIdentity,
    retired_value_record: BaselineLsmCompactionRecordIdentity,
    manifest_publication: PublicationDeclaration,
    replay_binding: [BlobWalRecordIdentity; 3],
    tombstone_blocks_older: bool,
    retired_runs: [BaselineLsmRunIdentity; 3],
    bytes_in: u64,
    bytes_out: u64,
    rewritten_runs: u16,
    counters: BaselineLsmCounterObservation,
}

impl BaselineLsmCompactionPublicationReceipt {
    /// Transition fact emitted by the ordinary WAL compaction executor.
    pub const fn compaction_transition(&self) -> super::super::BaselineLsmCompactionTransition {
        super::super::BaselineLsmCompactionTransition::tombstone_retention_admitted()
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn new(
        key: BaselineLsmCompactionKeyIdentity,
        input_runs: [BaselineLsmRunIdentity; 3],
        output_run: BaselineLsmRunIdentity,
        output_publication: BlobWalRecordEnvelope,
        tombstone_record: BaselineLsmCompactionRecordIdentity,
        retired_value_record: BaselineLsmCompactionRecordIdentity,
        manifest_publication: PublicationDeclaration,
        replay_binding: [BlobWalRecordIdentity; 3],
        bytes_in: u64,
        bytes_out: u64,
        effects: BaselineLsmCompactionExecutionEffects,
    ) -> Self {
        Self {
            key,
            input_runs,
            output_run,
            output_publication,
            tombstone_record,
            retired_value_record,
            manifest_publication,
            replay_binding,
            tombstone_blocks_older: matches!(
                tombstone_record.kind,
                BaselineLsmCompactionRecordKind::Tombstone
            ) && matches!(
                retired_value_record.kind,
                BaselineLsmCompactionRecordKind::Value
            ) && tombstone_record.key == retired_value_record.key
                && tombstone_record.wal_record.sequence()
                    > retired_value_record.wal_record.sequence(),
            retired_runs: effects.retired_runs,
            bytes_in,
            bytes_out,
            rewritten_runs: effects.counters.maintenance_reads(),
            counters: effects.counters,
        }
    }

    pub(super) const fn admitted_key(key: LsmMembershipKey) -> BaselineLsmCompactionKeyIdentity {
        BaselineLsmCompactionKeyIdentity { key }
    }

    pub(super) const fn run(
        generation: u64,
        root_record: BlobWalRecordIdentity,
    ) -> BaselineLsmRunIdentity {
        BaselineLsmRunIdentity {
            generation,
            root_record,
        }
    }

    pub(super) const fn record(
        key: BaselineLsmCompactionKeyIdentity,
        wal_record: BlobWalRecordIdentity,
        run: BaselineLsmRunIdentity,
        kind: BaselineLsmCompactionRecordKind,
    ) -> BaselineLsmCompactionRecordIdentity {
        BaselineLsmCompactionRecordIdentity {
            key,
            wal_record,
            run,
            kind,
        }
    }

    pub const fn key(&self) -> BaselineLsmCompactionKeyIdentity {
        self.key
    }

    pub const fn input_runs(&self) -> &[BaselineLsmRunIdentity; 3] {
        &self.input_runs
    }

    pub const fn output_run(&self) -> BaselineLsmRunIdentity {
        self.output_run
    }

    pub const fn output_publication(&self) -> &BlobWalRecordEnvelope {
        &self.output_publication
    }

    pub const fn tombstone_record(&self) -> BaselineLsmCompactionRecordIdentity {
        self.tombstone_record
    }

    pub const fn retired_value_record(&self) -> BaselineLsmCompactionRecordIdentity {
        self.retired_value_record
    }

    pub const fn manifest_publication(&self) -> &PublicationDeclaration {
        &self.manifest_publication
    }

    pub const fn replay_binding(&self) -> &[BlobWalRecordIdentity; 3] {
        &self.replay_binding
    }

    pub const fn tombstone_older_sequence(&self) -> u64 {
        self.retired_value_record.wal_record.sequence()
    }

    pub const fn tombstone_newer_sequence(&self) -> u64 {
        self.tombstone_record.wal_record.sequence()
    }

    pub const fn tombstone_blocks_older(&self) -> bool {
        self.tombstone_blocks_older
    }

    pub const fn older_precedes_newer_start(&self) -> bool {
        self.input_runs[0].generation < self.input_runs[2].generation
    }

    pub const fn newer_precedence_preserved(&self) -> bool {
        self.tombstone_record.wal_record.sequence()
            > self.retired_value_record.wal_record.sequence()
    }

    pub const fn input_generations(&self) -> [u64; 3] {
        [
            self.input_runs[0].generation,
            self.input_runs[1].generation,
            self.input_runs[2].generation,
        ]
    }

    pub const fn output_generation(&self) -> u64 {
        self.output_run.generation
    }

    pub fn stale_runs_retired(&self) -> bool {
        self.retired_runs[0].root_record == self.input_runs[0].root_record
            && self.retired_runs[1].root_record == self.input_runs[1].root_record
            && self.retired_runs[2].root_record == self.input_runs[2].root_record
    }

    pub const fn bytes_in(&self) -> u64 {
        self.bytes_in
    }

    pub const fn bytes_out(&self) -> u64 {
        self.bytes_out
    }

    pub const fn rewritten_runs(&self) -> u16 {
        self.rewritten_runs
    }

    pub const fn counters(&self) -> BaselineLsmCounterObservation {
        self.counters
    }

    pub fn publication_is_bound(&self) -> bool {
        matches!(
            self.manifest_publication.scope(),
            PublicationScope::Manifest(_)
        ) && self.output_generation() > self.input_runs[2].generation
            && self.output_publication.identity() == self.output_run.root_record
            && self.counters.publications() == 1
            && self.counters.maintenance_reads() == self.rewritten_runs
    }
}

impl BaselineLsmCompactionExecutionEffects {
    pub(super) const fn from_persisted_execution(
        retired_runs: [BaselineLsmRunIdentity; 3],
    ) -> Self {
        Self {
            retired_runs,
            counters: BaselineLsmCounterObservation::compaction(retired_runs.len() as u16),
        }
    }
}
