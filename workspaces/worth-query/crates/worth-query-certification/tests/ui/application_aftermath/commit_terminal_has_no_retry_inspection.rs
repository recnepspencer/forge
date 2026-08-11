use worth_query_host::facade::primary_graph::WorthQueryApplicationCommitReceipt;

fn cannot_extract_retry_inspection(receipt: WorthQueryApplicationCommitReceipt) {
    let _ = receipt.terminal().retry_inspection();
}

fn main() {}
