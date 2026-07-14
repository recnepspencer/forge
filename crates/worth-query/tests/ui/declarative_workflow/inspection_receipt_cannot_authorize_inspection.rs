use worth_query::facade::inspection::{declare, WorthQueryInspectionReceipt};

fn cannot_inspect_receipt(receipt: &WorthQueryInspectionReceipt) {
    let _declaration = declare(receipt);
}

fn main() {}
