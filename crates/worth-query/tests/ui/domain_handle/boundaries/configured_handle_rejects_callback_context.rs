use worth_query::facade::{WorthQueryApplicationFacade, WorthQueryCapabilityFamily, WorthQueryDomainEntryMarker};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryDomainEntry;

impl WorthQueryDomainEntryMarker for GeometryDomainEntry {
    fn domain_key(&self) -> &'static str {
        "example.geometry"
    }

    fn display_name(&self) -> &'static str {
        "GeometryDomainEntry"
    }

    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[WorthQueryCapabilityFamily::QueryComposition]
    }
}

fn main() {
    let query = WorthQueryApplicationFacade::runtime_backed_default();
    let callback = || "policy";
    let _ = query.domain(GeometryDomainEntry).with_operating_context(callback);
}
