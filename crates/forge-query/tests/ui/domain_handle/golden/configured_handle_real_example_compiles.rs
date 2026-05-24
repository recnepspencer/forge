use forge_query::facade::{
    ForgeQueryApplicationFacade, ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily,
    ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AccessClass {
    CollaborativeEditor,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum InvariantRegime {
    Conservative,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AssumptionRegime {
    TightTolerance,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryDomainEntry;

impl ForgeQueryDomainEntryMarker for GeometryDomainEntry {
    fn domain_key(&self) -> &'static str {
        "worth.geometry"
    }

    fn display_name(&self) -> &'static str {
        "GeometryDomainEntry"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[
            ForgeQueryCapabilityFamily::QueryComposition,
            ForgeQueryCapabilityFamily::QueryContext,
            ForgeQueryCapabilityFamily::IdentityEvolution,
        ]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryOperatingContext {
    access_class: AccessClass,
    invariant_regime: InvariantRegime,
    assumption_regime: AssumptionRegime,
}

impl GeometryOperatingContext {
    fn collaborative() -> Self {
        Self {
            access_class: AccessClass::CollaborativeEditor,
            invariant_regime: InvariantRegime::Conservative,
            assumption_regime: AssumptionRegime::TightTolerance,
        }
    }
}

impl ForgeQueryDomainOperatingContext<GeometryDomainEntry> for GeometryOperatingContext {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[
            ForgeQueryCapabilityFamily::PreviewSession,
            ForgeQueryCapabilityFamily::HistoricalEvaluation,
        ]
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        &[
            ForgeQueryConfigSectionFamily::Query,
            ForgeQueryConfigSectionFamily::RuntimeBridge,
            ForgeQueryConfigSectionFamily::Relational,
        ]
    }

    fn context_identity_digest(&self) -> String {
        format!(
            "access:{:?}|invariant:{:?}|assumption:{:?}",
            self.access_class, self.invariant_regime, self.assumption_regime
        )
    }
}

fn main() {
    let query = ForgeQueryApplicationFacade::runtime_backed_default();
    let handle = query
        .domain(GeometryDomainEntry)
        .with_operating_context(GeometryOperatingContext::collaborative())
        .validate()
        .unwrap()
        .admit()
        .unwrap();

    let _ = handle.domain_key();
    let _ = handle.required_capability_families();
    let _ = handle.required_config_sections();
    let _ = handle.support_snapshot().snapshot_digest();
}
