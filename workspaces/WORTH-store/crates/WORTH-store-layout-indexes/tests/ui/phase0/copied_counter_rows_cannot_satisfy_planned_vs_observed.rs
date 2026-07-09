use worth_foundational::FoundationalPerformanceCounterRow;
use worth_store_layout_indexes::S8PlannedVsObservedCounterReceipt;

fn require_receipt(_: S8PlannedVsObservedCounterReceipt) {}

fn main() {
    let rows: Vec<FoundationalPerformanceCounterRow> = todo!();
    require_receipt(rows);
}
