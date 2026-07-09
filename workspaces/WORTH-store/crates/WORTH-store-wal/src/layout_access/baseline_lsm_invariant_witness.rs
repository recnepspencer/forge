use super::baseline_lsm_counter_observation::{
    execute_baseline_lsm_transcript, BaselineLsmLookupDisposition,
};
use crate::BlobWalRecordKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BaselineLsmLookupCaseInvariantWitness {
    probe_sequence: u64,
    memtable_sequence: u64,
    probe_visible_in_newer_run: bool,
    probe_visible_in_older_run: bool,
    disposition: BaselineLsmLookupDisposition,
    tombstone_blocks_older: bool,
}

impl BaselineLsmLookupCaseInvariantWitness {
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
pub struct BaselineLsmLookupInvariantWitness {
    newest_run: BaselineLsmLookupCaseInvariantWitness,
    older_run: BaselineLsmLookupCaseInvariantWitness,
    tombstone_blocked: BaselineLsmLookupCaseInvariantWitness,
}

impl BaselineLsmLookupInvariantWitness {
    pub const fn newest_run(self) -> BaselineLsmLookupCaseInvariantWitness {
        self.newest_run
    }

    pub const fn older_run(self) -> BaselineLsmLookupCaseInvariantWitness {
        self.older_run
    }

    pub const fn tombstone_blocked(self) -> BaselineLsmLookupCaseInvariantWitness {
        self.tombstone_blocked
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BaselineLsmPublicationInvariantWitness {
    manifest_sequence_advanced: bool,
    published_run_count: u16,
    stale_runs_removed: bool,
    advisory_filter_present: bool,
}

impl BaselineLsmPublicationInvariantWitness {
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
pub struct BaselineLsmRecoveryInvariantWitness {
    replay_monotonic: bool,
    replay_tail: [BlobWalRecordKind; 3],
    stale_run_count: u16,
    cleanup_batch_count: u16,
    remaining_run_count: u16,
}

impl BaselineLsmRecoveryInvariantWitness {
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
pub struct BaselineLsmCompactionInvariantWitness {
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

impl BaselineLsmCompactionInvariantWitness {
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
pub struct BaselineLsmInvariantWitness {
    lookup: BaselineLsmLookupInvariantWitness,
    publication: BaselineLsmPublicationInvariantWitness,
    recovery: BaselineLsmRecoveryInvariantWitness,
    compaction: BaselineLsmCompactionInvariantWitness,
}

impl BaselineLsmInvariantWitness {
    pub const fn lookup(self) -> BaselineLsmLookupInvariantWitness {
        self.lookup
    }
    pub const fn publication(self) -> BaselineLsmPublicationInvariantWitness {
        self.publication
    }
    pub const fn recovery(self) -> BaselineLsmRecoveryInvariantWitness {
        self.recovery
    }
    pub const fn compaction(self) -> BaselineLsmCompactionInvariantWitness {
        self.compaction
    }
}

pub fn collect_baseline_lsm_invariant_witness() -> BaselineLsmInvariantWitness {
    let transcript = execute_baseline_lsm_transcript();
    let newest_run = transcript.newest_lookup();
    let older_run = transcript.older_lookup();
    let tombstone_blocked = transcript.tombstone_blocked_lookup();
    let publication = transcript.publication();
    let recovery = transcript.recovery();
    let compaction = transcript.compaction();

    BaselineLsmInvariantWitness {
        lookup: BaselineLsmLookupInvariantWitness {
            newest_run: BaselineLsmLookupCaseInvariantWitness {
                probe_sequence: newest_run.probe_sequence(),
                memtable_sequence: newest_run.memtable_record().sequence(),
                probe_visible_in_newer_run: newest_run.probe_visible_in_newer_run(),
                probe_visible_in_older_run: newest_run.probe_visible_in_older_run(),
                disposition: newest_run.disposition(),
                tombstone_blocks_older: newest_run.tombstone_blocks_older(),
            },
            older_run: BaselineLsmLookupCaseInvariantWitness {
                probe_sequence: older_run.probe_sequence(),
                memtable_sequence: older_run.memtable_record().sequence(),
                probe_visible_in_newer_run: older_run.probe_visible_in_newer_run(),
                probe_visible_in_older_run: older_run.probe_visible_in_older_run(),
                disposition: older_run.disposition(),
                tombstone_blocks_older: older_run.tombstone_blocks_older(),
            },
            tombstone_blocked: BaselineLsmLookupCaseInvariantWitness {
                probe_sequence: tombstone_blocked.probe_sequence(),
                memtable_sequence: tombstone_blocked.memtable_record().sequence(),
                probe_visible_in_newer_run: tombstone_blocked.probe_visible_in_newer_run(),
                probe_visible_in_older_run: tombstone_blocked.probe_visible_in_older_run(),
                disposition: tombstone_blocked.disposition(),
                tombstone_blocks_older: tombstone_blocked.tombstone_blocks_older(),
            },
        },
        publication: BaselineLsmPublicationInvariantWitness {
            manifest_sequence_advanced: publication.manifest_sequence_advanced(),
            published_run_count: publication.published_run_count(),
            stale_runs_removed: publication.stale_runs_removed(),
            advisory_filter_present: publication.advisory_filter_present(),
        },
        recovery: BaselineLsmRecoveryInvariantWitness {
            replay_monotonic: recovery.replay_monotonic(),
            replay_tail: recovery.replay_tail(),
            stale_run_count: recovery.stale_run_count(),
            cleanup_batch_count: recovery.cleanup_batch_count(),
            remaining_run_count: recovery.remaining_run_count(),
        },
        compaction: BaselineLsmCompactionInvariantWitness {
            tombstone_older_sequence: compaction.tombstone_older_sequence(),
            tombstone_newer_sequence: compaction.tombstone_newer_sequence(),
            tombstone_blocks_older: compaction.tombstone_blocks_older(),
            older_precedes_newer_start: compaction.older_precedes_newer_start(),
            newer_precedence_preserved: compaction.newer_precedence_preserved(),
            input_generations: compaction.input_generations(),
            output_generation: compaction.output_generation(),
            stale_runs_retired: compaction.stale_runs_retired(),
            bytes_in: compaction.bytes_in(),
            bytes_out: compaction.bytes_out(),
            rewritten_runs: compaction.rewritten_runs(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{collect_baseline_lsm_invariant_witness, BaselineLsmLookupDisposition};
    use crate::BlobWalRecordKind;

    #[test]
    fn baseline_lsm_invariant_witness_carries_execution_owned_facts() {
        let witness = collect_baseline_lsm_invariant_witness();
        assert_eq!(
            witness.lookup().newest_run().disposition(),
            BaselineLsmLookupDisposition::Memtable
        );
        assert_eq!(
            witness.lookup().newest_run().probe_sequence(),
            witness.lookup().newest_run().memtable_sequence()
        );
        assert_eq!(
            witness.lookup().older_run().disposition(),
            BaselineLsmLookupDisposition::SortedRun
        );
        assert_eq!(
            witness.lookup().tombstone_blocked().disposition(),
            BaselineLsmLookupDisposition::NotFound
        );
        assert!(witness
            .lookup()
            .tombstone_blocked()
            .tombstone_blocks_older());
        assert_eq!(
            witness.recovery().replay_tail()[1],
            BlobWalRecordKind::GenerationPublication
        );
        assert!(witness.compaction().stale_runs_retired());
    }
}
