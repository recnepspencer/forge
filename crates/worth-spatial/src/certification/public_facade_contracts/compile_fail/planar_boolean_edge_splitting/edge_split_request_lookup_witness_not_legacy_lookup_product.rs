use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanCandidateIndexConsumptionGate, PlanarBooleanEdgeSplitRequestInput,
};
use worth_spatial::facade::planar_boolean_events::PlanarBooleanEventLedgerReceipt;
use worth_spatial::facade::workload_vocabulary::WorkloadEvidenceBooleanReceiptLookupProduct;

fn main() {
    let event_ledger: PlanarBooleanEventLedgerReceipt =
        panic!("receipt construction is irrelevant to the type boundary");
    let gate: PlanarBooleanCandidateIndexConsumptionGate =
        panic!("gate construction is irrelevant to the type boundary");
    let legacy_lookup: WorkloadEvidenceBooleanReceiptLookupProduct =
        panic!("legacy lookup product must not satisfy the migrated witness lane");

    let _ = PlanarBooleanEdgeSplitRequestInput::new(
        &event_ledger,
        &gate,
        &legacy_lookup,
        None,
    );
}
