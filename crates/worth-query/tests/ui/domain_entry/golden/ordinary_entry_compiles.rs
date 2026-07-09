use worth_query::facade::{
    WorthQueryApplicationFacade, WorthQueryCapabilityFamily, WorthQueryDomainEntryMarker,
};

const ENTRY_CAPABILITIES: &[WorthQueryCapabilityFamily] = &[WorthQueryCapabilityFamily::QueryRead];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ExampleReadDomain;

impl WorthQueryDomainEntryMarker for ExampleReadDomain {
    fn domain_key(&self) -> &'static str {
        "example.read"
    }

    fn display_name(&self) -> &'static str {
        "ExampleReadDomain"
    }

    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        ENTRY_CAPABILITIES
    }
}

fn main() {
    let query = WorthQueryApplicationFacade::runtime_backed_default();
    let root = query.domain(ExampleReadDomain);

    let _ = root.domain_key();
    let _ = root.display_name();
}
