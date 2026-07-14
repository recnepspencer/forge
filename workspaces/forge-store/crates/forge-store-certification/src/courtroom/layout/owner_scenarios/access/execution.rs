use super::super::LayoutOwnerObservationLedger;

pub(in crate::courtroom::layout::owner_scenarios) struct AccessScenarioEvidence {
    pub(in crate::courtroom::layout::owner_scenarios) performance:
        forge_store_layout_indexes::LayoutAccessPerformanceReceipt,
    pub(in crate::courtroom::layout::owner_scenarios) btree:
        super::super::durable_observation::BTreeDurableObservationSource,
}

pub(in crate::courtroom::layout::owner_scenarios) fn execute(
    ledger: &mut LayoutOwnerObservationLedger,
) -> AccessScenarioEvidence {
    let evidence = super::btree::execute_lookup(ledger);
    super::btree::execute_replay(ledger);
    super::degraded_scan::execute(ledger);
    super::lsm::execute(ledger);
    super::imported_blob::execute(ledger);
    evidence
}
