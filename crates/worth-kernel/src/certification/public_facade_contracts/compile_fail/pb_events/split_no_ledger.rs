use worth_kernel::workload_composition::WorthWorkload;

fn main() {
    let workload: WorthWorkload = todo!();
    let raw_events = Vec::new();
    let _ = workload.require_boolean_event_ledger(&raw_events);
}
