use forge_store_layout_indexes::WalLookupRequest;

fn forge<'a>() -> WalLookupRequest<'a> {
    WalLookupRequest {
        catalog: panic!(),
        security: panic!(),
        record_family: panic!(),
        record_identity: panic!(),
        probe_sequence: 1,
        budget: panic!(),
        source: panic!(),
    }
}

fn main() {
    let _ = forge();
}
