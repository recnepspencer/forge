use worth_query::facade::read::{WorthQueryReadReceipt, WorthQueryReadResult};

fn forge_success(receipt: WorthQueryReadReceipt) -> WorthQueryReadResult {
    WorthQueryReadResult {
        rows: Vec::new(),
        receipt,
    }
}

fn main() {}
