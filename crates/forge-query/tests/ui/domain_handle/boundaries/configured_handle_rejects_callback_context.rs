use forge_query::facade::{ForgeQueryApplicationFacade, ForgeQueryCapabilityFamily, ForgeQueryDomainEntryMarker};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryDomainEntry;

impl ForgeQueryDomainEntryMarker for GeometryDomainEntry {
    fn domain_key(&self) -> &'static str {
        "example.geometry"
    }

    fn display_name(&self) -> &'static str {
        "GeometryDomainEntry"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::QueryComposition]
    }
}

fn main() {
    let query = ForgeQueryApplicationFacade::runtime_backed_default();
    let callback = || "policy";
    let _ = query.domain(GeometryDomainEntry).with_operating_context(callback);
}
