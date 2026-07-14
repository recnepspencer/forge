use worth_query::facade::inspection::{inspect, WorthQueryInspectionReceipt};

fn cannot_inspect_receipt(receipt: &WorthQueryInspectionReceipt) {
    let _declaration = inspect(receipt);
}

fn main() {}
