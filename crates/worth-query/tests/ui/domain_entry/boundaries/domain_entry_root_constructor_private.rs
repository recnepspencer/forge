use worth_query::facade::{
    WorthQueryApplicationFacade, WorthQueryCapabilityFamily, WorthQueryDomainEntryMarker,
    WorthQueryDomainEntryRoot,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct LocalDomain;

impl WorthQueryDomainEntryMarker for LocalDomain {
    fn domain_key(&self) -> &'static str {
        "example.local"
    }

    fn display_name(&self) -> &'static str {
        "LocalDomain"
    }

    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[WorthQueryCapabilityFamily::QueryRead]
    }
}

fn main() {
    let query = WorthQueryApplicationFacade::runtime_backed_default();
    let _ = WorthQueryDomainEntryRoot::<LocalDomain> {
        marker: LocalDomain,
        support_snapshot: query.domain_entry_support_snapshot(),
    };
}
