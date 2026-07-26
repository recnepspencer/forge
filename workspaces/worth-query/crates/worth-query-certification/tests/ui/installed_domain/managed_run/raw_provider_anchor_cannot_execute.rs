use worth_query_execution::facade::domain_computation::WorthQueryGraphProviderCall;
use worth_query_execution::facade::integration::WorthQueryGraphProviderAnchor;

fn bypass(anchor: &WorthQueryGraphProviderAnchor, call: &WorthQueryGraphProviderCall) {
    let _ = anchor.execute_unmanaged(call);
}

fn main() {}
