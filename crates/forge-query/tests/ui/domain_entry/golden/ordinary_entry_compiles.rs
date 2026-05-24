use forge_query::facade::{
    ForgeQueryApplicationFacade, ForgeQueryCapabilityFamily, ForgeQueryDomainEntryMarker,
};

const ENTRY_CAPABILITIES: &[ForgeQueryCapabilityFamily] = &[ForgeQueryCapabilityFamily::QueryRead];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ExampleReadDomain;

impl ForgeQueryDomainEntryMarker for ExampleReadDomain {
    fn domain_key(&self) -> &'static str {
        "example.read"
    }

    fn display_name(&self) -> &'static str {
        "ExampleReadDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        ENTRY_CAPABILITIES
    }
}

fn main() {
    let query = ForgeQueryApplicationFacade::runtime_backed_default();
    let root = query.domain(ExampleReadDomain);

    let _ = root.domain_key();
    let _ = root.display_name();
}
