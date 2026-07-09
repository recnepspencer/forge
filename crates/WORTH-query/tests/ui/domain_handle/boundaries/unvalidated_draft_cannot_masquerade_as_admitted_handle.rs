use worth_query::facade::{
    WorthQueryAdmittedConfiguredDomainHandle, WorthQueryApplicationFacade, WorthQueryCapabilityFamily,
    WorthQueryConfigSectionFamily, WorthQueryDomainEntryMarker, WorthQueryDomainOperatingContext,
};

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryOperatingContext;

impl WorthQueryDomainOperatingContext<GeometryDomainEntry> for GeometryOperatingContext {
    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[]
    }

    fn required_config_sections(&self) -> &'static [WorthQueryConfigSectionFamily] {
        &[WorthQueryConfigSectionFamily::Query]
    }

    fn context_identity_digest(&self) -> String {
        "geometry".to_string()
    }
}

fn require_admitted(
    _handle: WorthQueryAdmittedConfiguredDomainHandle<GeometryDomainEntry, GeometryOperatingContext>,
) {
}

fn main() {
    let query = WorthQueryApplicationFacade::runtime_backed_default();
    let draft = query
        .domain(GeometryDomainEntry)
        .with_operating_context(GeometryOperatingContext);
    require_admitted(draft);
}
