use worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanLoopReconstructionSplitConsumptionInput;
use worth_spatial::facade::planar_boolean_events::PlanarBooleanEventLedgerReceipt;

fn reject_loop_reconstruction_from_event_ledger(event_ledger: &PlanarBooleanEventLedgerReceipt) {
    let _ =
        PlanarBooleanLoopReconstructionSplitConsumptionInput::from_event_ledger(event_ledger);
}

fn main() {}
