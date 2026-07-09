use worth_store_layout_indexes::{access_lowering, S8ExecutionReadyAccessReceipt};

fn attempt(ready: S8ExecutionReadyAccessReceipt) {
    let _ = access_lowering().execute_ready(ready, ready.selected().planned_counter_envelope().lookup());
}

fn main() {}
