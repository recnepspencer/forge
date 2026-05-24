use forge_query::facade::{
    ForgeQueryApplicationFacade, ForgeQueryCapabilityFamily, ForgeQueryDomainEntryMarker,
};

const ENTRY_CAPABILITIES: &[ForgeQueryCapabilityFamily] =
    &[ForgeQueryCapabilityFamily::DurableArtifacts];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ExampleDeferredDomain;

impl ForgeQueryDomainEntryMarker for ExampleDeferredDomain {
    fn domain_key(&self) -> &'static str {
        "example.deferred"
    }

    fn display_name(&self) -> &'static str {
        "ExampleDeferredDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        ENTRY_CAPABILITIES
    }
}

fn main() {
    let query = ForgeQueryApplicationFacade::runtime_backed_default();
    let checked = query.domain_checked(ExampleDeferredDomain);

    let _ = checked.support_snapshot();
}
