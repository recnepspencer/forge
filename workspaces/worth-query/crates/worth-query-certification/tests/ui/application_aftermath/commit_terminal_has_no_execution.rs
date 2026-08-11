use worth_query_host::facade::primary_graph::WorthQueryApplicationCommitReceipt;

fn cannot_extract_execution(receipt: WorthQueryApplicationCommitReceipt) {
    let _ = receipt.terminal().execution();
}

fn main() {}
