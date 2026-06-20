use forge_query::facade::{
    ForgeQueryApplicationFacade, ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily,
    ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryDomainEntry;

impl ForgeQueryDomainEntryMarker for GeometryDomainEntry {
    fn domain_key(&self) -> &'static str {
        "example.geometry.world-basis"
    }

    fn display_name(&self) -> &'static str {
        "GeometryWorldBasisDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::QueryComposition]
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
        "geometry.world-basis".to_string()
    }
}

fn main() {
    let checked = match ForgeQueryApplicationFacade::runtime_backed_default()
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
