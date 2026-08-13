use worth_query_host::facade::installed::provider_session::{
    WorthQueryClosedProviderSessionDisposition, WorthQueryCommittedProviderSession,
};

fn main() {
    let construct = |disposition: WorthQueryClosedProviderSessionDisposition| {
        WorthQueryCommittedProviderSession { disposition }
    };
    let _ = construct;
}
