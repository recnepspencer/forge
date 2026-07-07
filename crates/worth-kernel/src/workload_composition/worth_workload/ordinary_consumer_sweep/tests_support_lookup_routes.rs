use crate::workload_composition::CompletedBooleanSplitHandoff;
use worth_spatial::facade::planar_boolean_events::PlanarBooleanEventLedgerLookupExecutionPacket;

pub(crate) fn lookup_packet(
    completed_split_handoff: &CompletedBooleanSplitHandoff,
) -> &PlanarBooleanEventLedgerLookupExecutionPacket {
    completed_split_handoff
        .test_event_ledger_lookup_packet()
        .expect("event-ledger lookup packet")
}

pub(crate) fn run_stack_heavy_lookup_test(test: impl FnOnce() + Send + 'static) {
    let result = std::thread::Builder::new()
        .stack_size(32 * 1024 * 1024)
        .spawn(test)
        .expect("lookup-consumed test should spawn on a larger stack")
        .join();
    if let Err(panic_payload) = result {
        std::panic::resume_unwind(panic_payload);
    }
}
