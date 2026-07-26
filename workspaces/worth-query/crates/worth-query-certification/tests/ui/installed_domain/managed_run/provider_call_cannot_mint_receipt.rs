use worth_query_execution::facade::domain_computation::{
    WorthQueryGraphProviderCall, WorthQueryProviderWorkReport,
};

fn mint_receipt(call: &WorthQueryGraphProviderCall) {
    let fabricated = WorthQueryProviderWorkReport::new(99, 99, 99, 99);
    let _ = call.completed("forged-provider-receipt", fabricated);
}

fn main() {}
