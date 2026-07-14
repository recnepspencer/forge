use worth_store_layout_indexes::{degraded_scan_runtime, DegradedScanReady};

fn execute_without_physical_owner(ready: DegradedScanReady) {
    let _ = degraded_scan_runtime().execute(ready, panic!());
}
