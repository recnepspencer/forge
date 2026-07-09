use worth_query::facade::{
    WorthQueryCapabilityFamily, WorthQueryConfigSectionFamily,
    WorthQueryConfiguredDomainHandleDraft, WorthQueryDomainEntryMarker,
    WorthQueryDomainEntrySupportSnapshot, WorthQueryDomainOperatingContext,
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
        &[]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryOperatingContext;

impl WorthQueryDomainOperatingContext<GeometryDomainEntry> for GeometryOperatingContext {
    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[]
    }

    fn required_config_sections(&self) -> &'static [WorthQueryConfigSectionFamily] {
        &[]
    }

    fn context_identity_digest(&self) -> String {
        "geometry".to_string()
    }
}

fn main() {
    let _ = WorthQueryConfiguredDomainHandleDraft::<GeometryDomainEntry, GeometryOperatingContext>::new(
        GeometryDomainEntry,
        GeometryOperatingContext,
        unsafe { std::mem::zeroed::<WorthQueryDomainEntrySupportSnapshot>() },
    );
}
