use worth_query::facade::foundation::{WorthQueryApplicationFacade, WorthQueryCapabilityFamily, WorthQueryDomainEntryMarker, WorthQueryDomainEntryProofRoot};
use std::marker::PhantomData;

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
    let _ = WorthQueryDomainEntryProofRoot::<LocalDomain> {
        domain_key: "example.local",
        display_name: "LocalDomain",
        support_snapshot: query.domain_entry_support_snapshot(),
        marker: PhantomData,
    };
}
