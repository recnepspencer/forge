use worth_query_host::facade::publication::domain_computation::WorthQueryApplicationCommitPublicationReceipt;

fn inspect_ambiguous_release(receipt: &WorthQueryApplicationCommitPublicationReceipt) {
    let _ = receipt.inspect().attempt_resources_released();
}

fn main() {}
