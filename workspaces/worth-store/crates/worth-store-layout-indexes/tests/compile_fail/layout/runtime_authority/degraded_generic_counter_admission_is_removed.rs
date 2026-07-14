use worth_store_layout_indexes::{degraded_scan_runtime, DegradedScanReady};

fn submit_counters(ready: &DegradedScanReady) {
    let _ = degraded_scan_runtime().admit_counters(ready, panic!());
}
