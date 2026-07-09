use super::baseline_lsm_counter_observation::BaselineLsmLookupDisposition;
use super::baseline_lsm_invariant_witness::collect_baseline_lsm_invariant_witness;
use crate::BlobWalRecordKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BaselineLsmLookupInvariantProof {
    probe_sequence: u64,
    memtable_sequence: u64,
    probe_visible_in_newer_run: bool,
    probe_visible_in_older_run: bool,
    disposition: BaselineLsmLookupDisposition,
    tombstone_blocks_older: bool,
}

impl BaselineLsmLookupInvariantProof {
    pub const fn probe_sequence(self) -> u64 {
        self.probe_sequence
    }

    pub const fn memtable_sequence(self) -> u64 {
        self.memtable_sequence
    }

    pub const fn probe_visible_in_newer_run(self) -> bool {
        self.probe_visible_in_newer_run
    }

    pub const fn probe_visible_in_older_run(self) -> bool {
        self.probe_visible_in_older_run
    }

    pub const fn disposition(self) -> BaselineLsmLookupDisposition {
        self.disposition
    }

    pub const fn tombstone_blocks_older(self) -> bool {
        self.tombstone_blocks_older
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BaselineLsmPublicationInvariantProof {
    manifest_sequence_advanced: bool,
    published_run_count: u16,
    stale_runs_removed: bool,
    advisory_filter_present: bool,
}

impl BaselineLsmPublicationInvariantProof {
    pub const fn manifest_sequence_advanced(self) -> bool {
        self.manifest_sequence_advanced
    }

    pub const fn published_run_count(self) -> u16 {
        self.published_run_count
    }

    pub const fn stale_runs_removed(self) -> bool {
        self.stale_runs_removed
    }

    pub const fn advisory_filter_present(self) -> bool {
        self.advisory_filter_present
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BaselineLsmRecoveryInvariantProof {
    replay_monotonic: bool,
    replay_tail: [BlobWalRecordKind; 3],
    stale_run_count: u16,
    cleanup_batch_count: u16,
    remaining_run_count: u16,
}

impl BaselineLsmRecoveryInvariantProof {
    pub const fn replay_monotonic(self) -> bool {
        self.replay_monotonic
    }

    pub const fn replay_tail(self) -> [BlobWalRecordKind; 3] {
        self.replay_tail
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BaselineLsmCompactionInvariantProof {
    tombstone_older_sequence: u64,
    tombstone_newer_sequence: u64,
    tombstone_blocks_older: bool,
    older_precedes_newer_start: bool,
    newer_precedence_preserved: bool,
    input_generations: [u64; 3],
    output_generation: u64,
    stale_runs_retired: bool,
    bytes_in: u64,
    bytes_out: u64,
    rewritten_runs: u16,
}

impl BaselineLsmCompactionInvariantProof {
    pub const fn tombstone_older_sequence(self) -> u64 {
        self.tombstone_older_sequence
    }

    pub const fn tombstone_newer_sequence(self) -> u64 {
        self.tombstone_newer_sequence
    }

    pub const fn tombstone_blocks_older(self) -> bool {
        self.tombstone_blocks_older
    }

    pub const fn older_precedes_newer_start(self) -> bool {
        self.older_precedes_newer_start
    }

    pub const fn newer_precedence_preserved(self) -> bool {
        self.newer_precedence_preserved
    }

    pub const fn input_generations(self) -> [u64; 3] {
        self.input_generations
    }

    pub const fn output_generation(self) -> u64 {
        self.output_generation
    }

    pub const fn stale_runs_retired(self) -> bool {
        self.stale_runs_retired
    }

    pub const fn bytes_in(self) -> u64 {
        self.bytes_in
    }

    pub const fn bytes_out(self) -> u64 {
        self.bytes_out
    }

    pub const fn rewritten_runs(self) -> u16 {
        self.rewritten_runs
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BaselineLsmInvariantProof {
    lookup: BaselineLsmLookupInvariantProof,
    publication: BaselineLsmPublicationInvariantProof,
    recovery: BaselineLsmRecoveryInvariantProof,
    compaction: BaselineLsmCompactionInvariantProof,
}

impl BaselineLsmInvariantProof {
    pub const fn lookup(self) -> BaselineLsmLookupInvariantProof {
        self.lookup
    }

    pub const fn publication(self) -> BaselineLsmPublicationInvariantProof {
        self.publication
    }

    pub const fn recovery(self) -> BaselineLsmRecoveryInvariantProof {
        self.recovery
    }

    pub const fn compaction(self) -> BaselineLsmCompactionInvariantProof {
        self.compaction
    }
}

fn lookup_case_proof(
    case: super::baseline_lsm_invariant_witness::BaselineLsmLookupCaseInvariantWitness,
) -> BaselineLsmLookupInvariantProof {
    BaselineLsmLookupInvariantProof {
        probe_sequence: case.probe_sequence(),
        memtable_sequence: case.memtable_sequence(),
        probe_visible_in_newer_run: case.probe_visible_in_newer_run(),
        probe_visible_in_older_run: case.probe_visible_in_older_run(),
        disposition: case.disposition(),
        tombstone_blocks_older: case.tombstone_blocks_older(),
    }
}

pub fn prove_baseline_lsm_invariants() -> BaselineLsmInvariantProof {
    let witness = collect_baseline_lsm_invariant_witness();

    BaselineLsmInvariantProof {
        lookup: lookup_case_proof(witness.lookup().newest_run()),
        publication: BaselineLsmPublicationInvariantProof {
            manifest_sequence_advanced: witness.publication().manifest_sequence_advanced(),
            published_run_count: witness.publication().published_run_count(),
            stale_runs_removed: witness.publication().stale_runs_removed(),
            advisory_filter_present: witness.publication().advisory_filter_present(),
        },
        recovery: BaselineLsmRecoveryInvariantProof {
            replay_monotonic: witness.recovery().replay_monotonic(),
            replay_tail: witness.recovery().replay_tail(),
            stale_run_count: witness.recovery().stale_run_count(),
            cleanup_batch_count: witness.recovery().cleanup_batch_count(),
            remaining_run_count: witness.recovery().remaining_run_count(),
        },
        compaction: BaselineLsmCompactionInvariantProof {
            tombstone_older_sequence: witness.compaction().tombstone_older_sequence(),
            tombstone_newer_sequence: witness.compaction().tombstone_newer_sequence(),
            tombstone_blocks_older: witness.compaction().tombstone_blocks_older(),
            older_precedes_newer_start: witness.compaction().older_precedes_newer_start(),
            newer_precedence_preserved: witness.compaction().newer_precedence_preserved(),
            input_generations: witness.compaction().input_generations(),
            output_generation: witness.compaction().output_generation(),
            stale_runs_retired: witness.compaction().stale_runs_retired(),
            bytes_in: witness.compaction().bytes_in(),
            bytes_out: witness.compaction().bytes_out(),
            rewritten_runs: witness.compaction().rewritten_runs(),
        },
    }
}

pub fn prove_baseline_lsm_older_run_lookup() -> BaselineLsmLookupInvariantProof {
    lookup_case_proof(
        collect_baseline_lsm_invariant_witness()
            .lookup()
            .older_run(),
    )
}

pub fn prove_baseline_lsm_tombstone_blocked_lookup() -> BaselineLsmLookupInvariantProof {
    lookup_case_proof(
        collect_baseline_lsm_invariant_witness()
            .lookup()
            .tombstone_blocked(),
    )
}

#[cfg(test)]
mod tests {
    use super::{
        prove_baseline_lsm_invariants, prove_baseline_lsm_older_run_lookup,
        prove_baseline_lsm_tombstone_blocked_lookup, BaselineLsmLookupDisposition,
    };
    use crate::layout_access::baseline_lsm_invariant_witness::collect_baseline_lsm_invariant_witness;
    use crate::BlobWalRecordKind;

    #[test]
    fn baseline_lsm_invariant_proof_is_wal_owned() {
        let proof = prove_baseline_lsm_invariants();
        let witness = collect_baseline_lsm_invariant_witness();

        assert_eq!(
            proof.lookup().disposition(),
            BaselineLsmLookupDisposition::Memtable
        );
        assert_eq!(
            proof.lookup().probe_sequence(),
            proof.lookup().memtable_sequence()
        );
        assert!(proof.lookup().probe_visible_in_newer_run());
        assert!(!proof.lookup().probe_visible_in_older_run());
        assert_eq!(
            proof.recovery().replay_tail()[1],
            BlobWalRecordKind::GenerationPublication
        );
        assert_eq!(
            proof.publication().published_run_count(),
            witness.publication().published_run_count()
        );
        assert_eq!(
            proof.compaction().input_generations(),
            witness.compaction().input_generations()
        );
        assert!(proof.compaction().stale_runs_retired());
        assert!(proof.compaction().bytes_out() >= proof.compaction().bytes_in());
    }

    #[test]
    fn baseline_lsm_lookup_proofs_cover_older_run_and_tombstone_denial() {
        let older = prove_baseline_lsm_older_run_lookup();
        let denied = prove_baseline_lsm_tombstone_blocked_lookup();

        assert_eq!(older.disposition(), BaselineLsmLookupDisposition::SortedRun);
        assert!(older.probe_visible_in_older_run());
        assert!(!older.tombstone_blocks_older());
        assert_eq!(denied.disposition(), BaselineLsmLookupDisposition::NotFound);
        assert!(denied.probe_visible_in_older_run());
        assert!(denied.tombstone_blocks_older());
    }
}
