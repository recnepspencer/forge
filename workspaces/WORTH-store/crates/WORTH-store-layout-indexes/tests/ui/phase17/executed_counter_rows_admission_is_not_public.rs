use worth_store_layout_indexes::access_lowering;
use worth_store_physical_format::{
    PageRecordCounterSnapshot, PhysicalOperationCounterSnapshot,
};

fn main() {
    let ready: worth_store_layout_indexes::S8ExecutionReadyAccessReceipt = unimplemented!();
    let rows = [PhysicalOperationCounterSnapshot::from_page_record_locate(
        PageRecordCounterSnapshot::for_locate_attempt().with_slot_lookup(),
    )];
    let _ = access_lowering().admit_executed_counter_rows(&ready, &rows);
}
