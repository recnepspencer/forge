use worth_query_host::facade::installed::provider_session::WorthQueryMutationGraphWorkCompletion;
use worth_query_host::facade::primary_graph::WorthQueryApplicationCommitReceipt;

fn cannot_convert_terminal(
    receipt: WorthQueryApplicationCommitReceipt,
) -> WorthQueryMutationGraphWorkCompletion {
    receipt.terminal().into()
}

fn main() {}
