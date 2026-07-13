use forge_store_layout_indexes::PageLookupRequest;

fn forge<'a>() -> PageLookupRequest<'a> {
    PageLookupRequest {
        catalog: panic!(),
        security: panic!(),
        segment: panic!(),
        page: panic!(),
        probe_slot: panic!(),
        kind: panic!(),
        budget: panic!(),
        source: panic!(),
    }
}

fn main() {
    let _ = forge();
}
