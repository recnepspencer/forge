use forge_query::facade::{
    ForgeQueryApplicationFacade, ForgeQueryCapabilityFamily, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainEntryProofRoot,
};
use std::marker::PhantomData;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct LocalDomain;

impl ForgeQueryDomainEntryMarker for LocalDomain {
    fn domain_key(&self) -> &'static str {
        "example.local"
    }

    fn display_name(&self) -> &'static str {
        "LocalDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::QueryRead]
    }
}

fn main() {
    let query = ForgeQueryApplicationFacade::runtime_backed_default();
    let _ = ForgeQueryDomainEntryProofRoot::<LocalDomain> {
        domain_key: "example.local",
        display_name: "LocalDomain",
        support_snapshot: query.domain_entry_support_snapshot(),
        marker: PhantomData,
    };
}
