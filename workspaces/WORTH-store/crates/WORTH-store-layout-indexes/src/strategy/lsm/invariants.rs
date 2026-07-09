use super::{
    S8LsmAdvisoryFilterLaw, S8LsmCompactionOrderingLaw, S8LsmMemtableWalLaw,
    S8LsmRunPublicationLaw, S8LsmStaleRunCleanupLaw, S8LsmTombstoneLaw, S8LsmWriteAmplificationLaw,
};
use crate::key_domain::{declare_comparator_law, require_canonical_key_encoding};
use crate::strategy::{S8StrategyDeclaration, S8StrategyDenial};
use worth_store_wal::layout_access::baseline_lsm_counter_observation::BaselineLsmLookupDisposition;
use worth_store_wal::layout_access::baseline_lsm_invariant_proof::{
    prove_baseline_lsm_invariants, BaselineLsmLookupInvariantProof,
};
use worth_store_wal::BlobWalRecordKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8LsmLookupDisposition {
    NewestRun,
    OlderRun,
    NotFound,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8LsmInvariantSuite {
    declaration: S8StrategyDeclaration,
    memtable_wal: S8LsmMemtableWalLaw,
    run_publication: S8LsmRunPublicationLaw,
    tombstone: S8LsmTombstoneLaw,
    compaction: S8LsmCompactionOrderingLaw,
    advisory_filter: S8LsmAdvisoryFilterLaw,
    stale_run_cleanup: S8LsmStaleRunCleanupLaw,
    write_amplification: S8LsmWriteAmplificationLaw,
}

impl S8LsmInvariantSuite {
    pub(crate) const fn new(
        declaration: S8StrategyDeclaration,
        memtable_wal: S8LsmMemtableWalLaw,
        run_publication: S8LsmRunPublicationLaw,
        tombstone: S8LsmTombstoneLaw,
        compaction: S8LsmCompactionOrderingLaw,
        advisory_filter: S8LsmAdvisoryFilterLaw,
        stale_run_cleanup: S8LsmStaleRunCleanupLaw,
        write_amplification: S8LsmWriteAmplificationLaw,
    ) -> Self {
        Self {
            declaration,
            memtable_wal,
            run_publication,
            tombstone,
            compaction,
            advisory_filter,
            stale_run_cleanup,
            write_amplification,
        }
    }

    pub(crate) const fn memtable_wal_law(self) -> S8LsmMemtableWalLaw {
        self.memtable_wal
    }

    pub(crate) const fn run_publication_law(self) -> S8LsmRunPublicationLaw {
        self.run_publication
    }

    pub(crate) const fn tombstone_law(self) -> S8LsmTombstoneLaw {
        self.tombstone
    }

    pub(crate) const fn compaction_ordering_law(self) -> S8LsmCompactionOrderingLaw {
        self.compaction
    }

    pub(crate) const fn advisory_filter_law(self) -> S8LsmAdvisoryFilterLaw {
        self.advisory_filter
    }

    pub(crate) const fn stale_run_cleanup_law(self) -> S8LsmStaleRunCleanupLaw {
        self.stale_run_cleanup
    }

    pub(crate) const fn write_amplification_law(self) -> S8LsmWriteAmplificationLaw {
        self.write_amplification
    }

    pub(crate) fn resolve_lookup_from_membership(
        self,
        probe_visible_in_newer_run: bool,
        probe_visible_in_older_run: bool,
        tombstone_blocks_older: bool,
    ) -> Result<S8LsmLookupDisposition, S8StrategyDenial> {
        if self
            .run_publication
            .verify_sorted_run_membership(probe_visible_in_newer_run)?
        {
            return Ok(S8LsmLookupDisposition::NewestRun);
        }
        if self
            .run_publication
            .verify_sorted_run_membership(probe_visible_in_older_run)?
            && !tombstone_blocks_older
        {
            return Ok(S8LsmLookupDisposition::OlderRun);
        }
        Ok(S8LsmLookupDisposition::NotFound)
    }

    pub(crate) fn verify_lookup_proof(
        self,
        proof: BaselineLsmLookupInvariantProof,
    ) -> Result<S8LsmLookupDisposition, S8StrategyDenial> {
        let disposition = self.resolve_lookup_from_membership(
            proof.probe_visible_in_newer_run(),
            proof.probe_visible_in_older_run(),
            proof.tombstone_blocks_older(),
        )?;

        self.memtable_wal
            .verify_memtable_visibility(proof.memtable_sequence(), proof.probe_sequence())?;
        if disposition == map_lookup_disposition(proof.disposition()) {
            return Ok(disposition);
        }
        Err(S8StrategyDenial::SearchPathViolation)
    }

    pub fn verify_baseline_lookup(self) -> Result<S8LsmLookupDisposition, S8StrategyDenial> {
        self.verify_lookup_proof(prove_baseline_lsm_invariants().lookup())
    }

    pub fn verify_baseline_publication(self) -> Result<(), S8StrategyDenial> {
        let proof = prove_baseline_lsm_invariants().publication();
        self.run_publication.verify_manifest_publication_progress(
            proof.manifest_sequence_advanced(),
            proof.published_run_count(),
        )?;
        self.run_publication.verify_manifest_update_progress(
            proof.manifest_sequence_advanced(),
            proof.stale_runs_removed(),
        )?;
        self.advisory_filter
            .verify_filter_posture(proof.advisory_filter_present())
    }

    pub fn verify_baseline_recovery(self) -> Result<(), S8StrategyDenial> {
        let proof = prove_baseline_lsm_invariants().recovery();
        self.memtable_wal
            .verify_recovery_replay_progress(proof.replay_monotonic())?;
        if proof.replay_tail()[1] != BlobWalRecordKind::GenerationPublication {
            return Err(S8StrategyDenial::RecoveryReplayViolation);
        }
        self.stale_run_cleanup.verify_cleanup(
            proof.stale_run_count(),
            proof.cleanup_batch_count(),
            proof.remaining_run_count(),
        )
    }

    pub fn verify_baseline_mutation_and_compaction(self) -> Result<(), S8StrategyDenial> {
        let proof = prove_baseline_lsm_invariants().compaction();
        self.tombstone.verify_shadowing(
            proof.tombstone_newer_sequence(),
            proof.tombstone_older_sequence(),
            proof.tombstone_blocks_older(),
        )?;
        self.run_publication.verify_merge_order_boundary(
            proof.older_precedes_newer_start(),
            proof.newer_precedence_preserved(),
        )?;
        self.compaction.verify_compaction(
            &proof.input_generations(),
            proof.output_generation(),
            proof.stale_runs_retired(),
        )?;
        self.write_amplification.verify_accounting(
            proof.bytes_in(),
            proof.bytes_out(),
            proof.rewritten_runs(),
        )
    }
}

pub(crate) fn declare_lsm_invariant_suite(
    declaration: S8StrategyDeclaration,
) -> Result<S8LsmInvariantSuite, S8StrategyDenial> {
    let comparator =
        declare_comparator_law(require_canonical_key_encoding(declaration.key_domain()));
    Ok(S8LsmInvariantSuite::new(
        declaration,
        S8LsmMemtableWalLaw::baseline(),
        S8LsmRunPublicationLaw::new(comparator),
        S8LsmTombstoneLaw::baseline(),
        S8LsmCompactionOrderingLaw::baseline(),
        S8LsmAdvisoryFilterLaw::baseline_absent(),
        S8LsmStaleRunCleanupLaw::baseline(),
        S8LsmWriteAmplificationLaw::baseline(),
    ))
}

const fn map_lookup_disposition(
    disposition: BaselineLsmLookupDisposition,
) -> S8LsmLookupDisposition {
    match disposition {
        BaselineLsmLookupDisposition::Memtable => S8LsmLookupDisposition::NewestRun,
        BaselineLsmLookupDisposition::SortedRun => S8LsmLookupDisposition::OlderRun,
        BaselineLsmLookupDisposition::NotFound => S8LsmLookupDisposition::NotFound,
    }
}
