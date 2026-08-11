use worth_query_host::facade::installed::domain_computation::{
    WorthQueryDirectRunCleanupReceipt, WorthQueryMutationGraphWorkCompletion,
};

fn inspect(receipt: &WorthQueryDirectRunCleanupReceipt) {
    let inspection = receipt.inspection();
    let _ = inspection.bridge();
    let _ = inspection.relational();
    let _ = inspection.attempt();
}

fn inspect_completion(completion: &WorthQueryMutationGraphWorkCompletion) {
    let _ = completion.cleanup();
}

fn main() {}
