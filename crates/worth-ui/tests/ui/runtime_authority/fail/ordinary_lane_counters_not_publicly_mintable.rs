use worth_ui::facade::WorthUiOrdinaryLaneCounters;

fn main() {
    let mut counters = WorthUiOrdinaryLaneCounters::default();
    counters.record_full_plan_scan();
}
