use forge_store_layout_indexes::BaselineBTreeExactCounterWitness;

struct FoundationalPerformanceCounterRow;

fn require_receipt(_: BaselineBTreeExactCounterWitness) {}

fn main() {
    let rows: Vec<FoundationalPerformanceCounterRow> = Vec::new();
    require_receipt(rows);
}
