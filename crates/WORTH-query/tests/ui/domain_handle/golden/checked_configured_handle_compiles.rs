use worth_query::facade::{
    WorthQueryApplicationFacade, WorthQueryCapabilityFamily, WorthQueryConfigSectionFamily,
    WorthQueryConfiguredDomainHandleChecked, WorthQueryDomainEntryMarker,
    WorthQueryDomainOperatingContext,
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
        &[
            WorthQueryCapabilityFamily::QueryComposition,
            WorthQueryCapabilityFamily::QueryContext,
        ]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryOperatingContext;

impl WorthQueryDomainOperatingContext<GeometryDomainEntry> for GeometryOperatingContext {
    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[WorthQueryCapabilityFamily::HistoricalEvaluation]
    }

    fn required_config_sections(&self) -> &'static [WorthQueryConfigSectionFamily] {
        &[
            WorthQueryConfigSectionFamily::Query,
            WorthQueryConfigSectionFamily::Relational,
        ]
    }

    fn context_identity_digest(&self) -> String {
        "access:review|invariant:strict|assumption:tight".to_string()
    }
}

fn main() {
    let query = WorthQueryApplicationFacade::runtime_backed_default();

    match query
        .domain_checked(GeometryDomainEntry)
        .with_operating_context(GeometryOperatingContext)
    {
        WorthQueryConfiguredDomainHandleChecked::Admitted(handle) => {
            let _ = handle.handle_identity_digest();
        }
        WorthQueryConfiguredDomainHandleChecked::Deferred(denial) => {
            let _ = denial.blocking_capability_families();
        }
        WorthQueryConfiguredDomainHandleChecked::Unsupported(denial) => {
            let _ = denial.blocking_capability_families();
        }
        WorthQueryConfiguredDomainHandleChecked::InvalidContext(denial) => {
            let _ = denial.blocking_config_sections();
        }
    }
}
