use worth_ui::facade::WorthUiVirtualizedDataCounters;

fn main() {
    let mut counters = WorthUiVirtualizedDataCounters::default();
    counters.record_full_collection_scan();
}
