use worth_query::facade::{
    WorthQueryApplicationFacade, WorthQueryCapabilityFamily, WorthQueryConfigSectionFamily,
    WorthQueryDomainEntryMarker, WorthQueryDomainOperatingContext,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryDomainEntry;

impl WorthQueryDomainEntryMarker for GeometryDomainEntry {
    fn domain_key(&self) -> &'static str {
        "example.geometry.world-basis"
    }

    fn display_name(&self) -> &'static str {
        "GeometryWorldBasisDomain"
    }

    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[WorthQueryCapabilityFamily::QueryComposition]
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
        "geometry.world-basis".to_string()
    }
}

fn main() {
    let checked = match WorthQueryApplicationFacade::runtime_backed_default()
        .domain(GeometryDomainEntry)
        .with_operating_context(GeometryOperatingContext)
        .validate()
    {
        Ok(value) => value,
        Err(_) => return,
    };
    let admitted = match checked.admit() {
        Ok(value) => value,
        Err(_) => return,
    };
    let basis = admitted.retained_world_basis();

    let _ = basis.domain_key();
    let _ = basis.display_name();
    let _ = basis.operating_context_identity_digest();
    let _ = basis.handle_identity_for_reporting();
    let _ = basis.support_snapshot_digest();
    let _ = basis.basis_lifecycle_support_for_reporting();
}
