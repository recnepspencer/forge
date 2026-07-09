use worth_query::facade::{
    WorthQueryApplicationFacade, WorthQueryCapabilityFamily, WorthQueryDomainEntryMarker,
};

const ENTRY_CAPABILITIES: &[WorthQueryCapabilityFamily] =
    &[WorthQueryCapabilityFamily::DurableArtifacts];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ExampleDeferredDomain;

impl WorthQueryDomainEntryMarker for ExampleDeferredDomain {
    fn domain_key(&self) -> &'static str {
        "example.deferred"
    }

    fn display_name(&self) -> &'static str {
        "ExampleDeferredDomain"
    }

    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        ENTRY_CAPABILITIES
    }
}

fn main() {
    let query = WorthQueryApplicationFacade::runtime_backed_default();
    let checked = query.domain_checked(ExampleDeferredDomain);

    let _ = checked.support_snapshot();
}
