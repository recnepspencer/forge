use worth_query_execution::facade::primary_graph::{
    WorthQueryApplicationCommitReceipt, WorthQueryPrimaryGraphApplicationRuntime,
};
use worth_query_host::facade::domain::ApplicationSchema;

fn mint_from_carried_commit_truth<Schema: ApplicationSchema>(
    runtime: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    receipt: &WorthQueryApplicationCommitReceipt,
) {
    let _ = runtime.mint_recovery_handle(receipt);
}

fn main() {}
