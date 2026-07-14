use super::super::{
    BaselineLsmCompactionPublicationReceipt, BaselineLsmLookupDisposition,
    BaselineLsmLookupExecution,
};
use super::super::{
    LsmAdvisoryFilterLaw, LsmCompactionOrderingLaw, LsmMemtableWalLaw, LsmRunPublicationLaw,
    LsmStaleRunCleanupLaw, LsmTombstoneLaw, LsmWriteAmplificationLaw,
};
use crate::keyspace::{declare_comparator_law, require_canonical_key_encoding};
use crate::strategy::{StrategyDeclaration, StrategyDenial};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LsmLookupDisposition {
    NewestRun,
    OlderRun,
    NotFound,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LsmInvariantSuite {
    declaration: StrategyDeclaration,
    memtable_wal: LsmMemtableWalLaw,
    run_publication: LsmRunPublicationLaw,
    tombstone: LsmTombstoneLaw,
    compaction: LsmCompactionOrderingLaw,
    advisory_filter: LsmAdvisoryFilterLaw,
    stale_run_cleanup: LsmStaleRunCleanupLaw,
    write_amplification: LsmWriteAmplificationLaw,
}

impl LsmInvariantSuite {
    // Keeping every LSM law explicit makes incomplete suites unrepresentable at the call site.
    #[allow(clippy::too_many_arguments)]
    pub(crate) const fn new(
        declaration: StrategyDeclaration,
        memtable_wal: LsmMemtableWalLaw,
        run_publication: LsmRunPublicationLaw,
        tombstone: LsmTombstoneLaw,
        compaction: LsmCompactionOrderingLaw,
        advisory_filter: LsmAdvisoryFilterLaw,
        stale_run_cleanup: LsmStaleRunCleanupLaw,
        write_amplification: LsmWriteAmplificationLaw,
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

    pub(crate) fn resolve_lookup_from_membership(
        self,
        probe_visible_in_newer_run: bool,
        probe_visible_in_older_run: bool,
        tombstone_blocks_older: bool,
    ) -> Result<LsmLookupDisposition, StrategyDenial> {
        if self
            .run_publication
            .verify_sorted_run_membership(probe_visible_in_newer_run)?
        {
            return Ok(LsmLookupDisposition::NewestRun);
        }
        if self
            .run_publication
            .verify_sorted_run_membership(probe_visible_in_older_run)?
            && !tombstone_blocks_older
        {
            return Ok(LsmLookupDisposition::OlderRun);
        }
        Ok(LsmLookupDisposition::NotFound)
    }

    pub(crate) fn verify_lookup_execution(
        self,
        execution: &BaselineLsmLookupExecution,
    ) -> Result<LsmLookupDisposition, StrategyDenial> {
        let disposition = self.resolve_lookup_from_membership(
            execution.probe_visible_in_newer_run(),
            execution.probe_visible_in_older_run(),
            execution.tombstone_blocks_older(),
        )?;
        if disposition == map_lookup_disposition(execution.disposition()) {
            return Ok(disposition);
        }
        Err(StrategyDenial::SearchPathViolation)
    }

    pub fn verify_owner_mutation_and_compaction(
        self,
        receipt: &BaselineLsmCompactionPublicationReceipt,
    ) -> Result<(), StrategyDenial> {
        self.tombstone.verify_owner_receipt(receipt)?;
        self.compaction.verify_owner_receipt(receipt)?;
        self.run_publication.verify_merge_order_boundary(
            receipt.older_precedes_newer_start(),
            receipt.newer_precedence_preserved(),
        )?;
        self.write_amplification.verify_accounting(
            receipt.bytes_in(),
            receipt.bytes_out(),
            receipt.rewritten_runs(),
        )
    }
}

pub(crate) fn declare_lsm_invariant_suite(
    declaration: StrategyDeclaration,
) -> Result<LsmInvariantSuite, StrategyDenial> {
    let comparator =
        declare_comparator_law(require_canonical_key_encoding(declaration.key_domain()));
    Ok(LsmInvariantSuite::new(
        declaration,
        LsmMemtableWalLaw::baseline(),
        LsmRunPublicationLaw::new(comparator),
        LsmTombstoneLaw::baseline(),
        LsmCompactionOrderingLaw::baseline(),
        LsmAdvisoryFilterLaw::baseline_absent(),
        LsmStaleRunCleanupLaw::baseline(),
        LsmWriteAmplificationLaw::baseline(),
    ))
}

const fn map_lookup_disposition(disposition: BaselineLsmLookupDisposition) -> LsmLookupDisposition {
    match disposition {
        BaselineLsmLookupDisposition::Memtable => LsmLookupDisposition::NewestRun,
        BaselineLsmLookupDisposition::SortedRun => LsmLookupDisposition::OlderRun,
        BaselineLsmLookupDisposition::NotFound => LsmLookupDisposition::NotFound,
    }
}
