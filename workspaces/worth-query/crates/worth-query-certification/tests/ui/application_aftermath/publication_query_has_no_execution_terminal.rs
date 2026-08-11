use worth_query_host::facade::publication::domain_computation::WorthQueryApplicationQueryPublicationReceipt;

fn read_execution_terminal(receipt: &WorthQueryApplicationQueryPublicationReceipt) {
    let _ = receipt.read_completion();
}

fn main() {}
