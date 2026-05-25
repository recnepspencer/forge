use forge_query::facade::{
    ForgeQueryApplicationFacade, ForgeQueryCapabilityFamily, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainEntryRoot,
};

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
    let _ = ForgeQueryDomainEntryRoot::<LocalDomain> {
        marker: LocalDomain,
        support_snapshot: query.domain_entry_support_snapshot(),
    };
}
