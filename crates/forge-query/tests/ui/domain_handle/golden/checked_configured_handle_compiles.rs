use forge_query::facade::{
    ForgeQueryApplicationFacade, ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily,
    ForgeQueryConfiguredDomainHandleChecked, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext,
};

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
        &[
            ForgeQueryCapabilityFamily::QueryComposition,
            ForgeQueryCapabilityFamily::QueryContext,
        ]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryOperatingContext;

impl ForgeQueryDomainOperatingContext<GeometryDomainEntry> for GeometryOperatingContext {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::HistoricalEvaluation]
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        &[
            ForgeQueryConfigSectionFamily::Query,
            ForgeQueryConfigSectionFamily::Relational,
        ]
    }

    fn context_identity_digest(&self) -> String {
        "access:review|invariant:strict|assumption:tight".to_string()
    }
}

fn main() {
    let query = ForgeQueryApplicationFacade::runtime_backed_default();

    match query
        .domain_checked(GeometryDomainEntry)
        .with_operating_context(GeometryOperatingContext)
    {
        ForgeQueryConfiguredDomainHandleChecked::Admitted(handle) => {
            let _ = handle.handle_identity_digest();
        }
        ForgeQueryConfiguredDomainHandleChecked::Deferred(denial) => {
            let _ = denial.blocking_capability_families();
        }
        ForgeQueryConfiguredDomainHandleChecked::Unsupported(denial) => {
            let _ = denial.blocking_capability_families();
        }
        ForgeQueryConfiguredDomainHandleChecked::InvalidContext(denial) => {
            let _ = denial.blocking_config_sections();
        }
    }
}
