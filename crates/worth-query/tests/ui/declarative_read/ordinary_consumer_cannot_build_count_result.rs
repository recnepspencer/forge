use worth_query::facade::read::{WorthQueryCountResult, WorthQueryReadReceipt};

fn forge_count(receipt: WorthQueryReadReceipt) -> WorthQueryCountResult {
    WorthQueryCountResult { count: 7, receipt }
}

fn main() {}
