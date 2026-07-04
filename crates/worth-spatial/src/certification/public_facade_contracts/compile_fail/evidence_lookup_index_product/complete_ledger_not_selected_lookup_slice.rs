use worth_spatial::facade::spatial_compiled_product_consumer_cutover::lower_evidence_lookup_index_product;
use worth_spatial::facade::evidence_lookup_plan_selection::EvidenceLookupSelectedPlan;
use worth_spatial::facade::workload_vocabulary::CompleteWorkloadEvidenceLedger;

fn main() {
    let plan: EvidenceLookupSelectedPlan = todo!();
    let ledger: CompleteWorkloadEvidenceLedger = todo!();
    let _ = lower_evidence_lookup_index_product(&plan, &ledger);
}
