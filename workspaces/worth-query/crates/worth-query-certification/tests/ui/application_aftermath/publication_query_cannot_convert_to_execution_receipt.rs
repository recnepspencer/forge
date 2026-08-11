use worth_query_host::facade::primary_graph::WorthQueryApplicationQueryAccessReceipt;
use worth_query_host::facade::publication::domain_computation::WorthQueryApplicationQueryPublicationReceipt;

fn extract_execution_receipt(
    receipt: WorthQueryApplicationQueryPublicationReceipt,
) -> WorthQueryApplicationQueryAccessReceipt {
    receipt.into()
}

fn main() {}
