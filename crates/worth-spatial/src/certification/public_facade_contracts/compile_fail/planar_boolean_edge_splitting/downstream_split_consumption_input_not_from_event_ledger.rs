use worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanDownstreamSplitConsumptionInput;
use worth_spatial::facade::planar_boolean_events::PlanarBooleanEventLedgerReceipt;

fn reject_downstream_split_consumption_from_event_ledger(
    event_ledger: &PlanarBooleanEventLedgerReceipt,
) {
    let _ = PlanarBooleanDownstreamSplitConsumptionInput::from_event_ledger(event_ledger);
}

fn main() {}
