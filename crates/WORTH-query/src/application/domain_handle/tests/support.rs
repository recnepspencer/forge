use crate::application::{
    WorthQueryCapabilityFamily, WorthQueryConfigSectionFamily, WorthQueryDomainEntryMarker,
};

use super::super::{WorthQueryDomainOperatingContext, WorthQueryDomainOperatingRequirement};

pub const ENTRY_CAPABILITIES: &[WorthQueryCapabilityFamily] = &[
    WorthQueryCapabilityFamily::QueryComposition,
    WorthQueryCapabilityFamily::QueryContext,
];

const COLLABORATIVE_CAPABILITIES: &[WorthQueryCapabilityFamily] = &[
    WorthQueryCapabilityFamily::PreviewSession,
    WorthQueryCapabilityFamily::HistoricalEvaluation,
];

const COLLABORATIVE_SECTIONS: &[WorthQueryConfigSectionFamily] = &[
    WorthQueryConfigSectionFamily::Query,
    WorthQueryConfigSectionFamily::RuntimeBridge,
    WorthQueryConfigSectionFamily::Relational,
];

const STORE_SECTIONS: &[WorthQueryConfigSectionFamily] = &[
    WorthQueryConfigSectionFamily::Query,
    WorthQueryConfigSectionFamily::Store,
];

const SIGNAL_SECTIONS: &[WorthQueryConfigSectionFamily] = &[
    WorthQueryConfigSectionFamily::Query,
    WorthQueryConfigSectionFamily::Signal,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GeometryDomainEntry;

impl WorthQueryDomainEntryMarker for GeometryDomainEntry {
    fn domain_key(&self) -> &'static str {
        "test.geometry"
    }

    fn display_name(&self) -> &'static str {
        "GeometryDomainEntry"
    }

    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        ENTRY_CAPABILITIES
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AccessClass {
    Collaborative,
    Restricted,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum InvariantRegime {
    Conservative,
    Permissive,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AssumptionRegime {
    TightTolerance,
    BroadTolerance,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GeometryOperatingContext {
    access_class: AccessClass,
    invariant_regime: InvariantRegime,
    assumption_regime: AssumptionRegime,
}

impl GeometryOperatingContext {
    pub fn collaborative() -> Self {
        Self {
            access_class: AccessClass::Collaborative,
            invariant_regime: InvariantRegime::Conservative,
            assumption_regime: AssumptionRegime::TightTolerance,
        }
    }

    pub fn collaborative_reordered() -> Self {
        Self::collaborative()
    }

    pub fn restricted() -> Self {
        Self {
            access_class: AccessClass::Restricted,
            invariant_regime: InvariantRegime::Conservative,
            assumption_regime: AssumptionRegime::TightTolerance,
        }
    }

    pub fn with_permissive_invariants() -> Self {
        Self {
            access_class: AccessClass::Collaborative,
            invariant_regime: InvariantRegime::Permissive,
            assumption_regime: AssumptionRegime::BroadTolerance,
        }
    }
}

impl WorthQueryDomainOperatingContext<GeometryDomainEntry> for GeometryOperatingContext {
    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        COLLABORATIVE_CAPABILITIES
    }

    fn required_config_sections(&self) -> &'static [WorthQueryConfigSectionFamily] {
        COLLABORATIVE_SECTIONS
    }

    fn context_identity_digest(&self) -> String {
        format!(
            "access:{:?}|invariant:{:?}|assumption:{:?}",
            self.access_class, self.invariant_regime, self.assumption_regime
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DeferredStoreContext;

impl WorthQueryDomainOperatingContext<GeometryDomainEntry> for DeferredStoreContext {
    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[WorthQueryCapabilityFamily::DurableArtifacts]
    }

    fn required_config_sections(&self) -> &'static [WorthQueryConfigSectionFamily] {
        STORE_SECTIONS
    }

    fn context_identity_digest(&self) -> String {
        "store-context".to_string()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DisabledSignalContext;

impl WorthQueryDomainOperatingContext<GeometryDomainEntry> for DisabledSignalContext {
    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[]
    }

    fn required_config_sections(&self) -> &'static [WorthQueryConfigSectionFamily] {
        SIGNAL_SECTIONS
    }

    fn context_identity_digest(&self) -> String {
        "signal-context".to_string()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MissingSectionContext;

impl WorthQueryDomainOperatingContext<GeometryDomainEntry> for MissingSectionContext {
    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[WorthQueryCapabilityFamily::HistoricalEvaluation]
    }

    fn required_config_sections(&self) -> &'static [WorthQueryConfigSectionFamily] {
        &[WorthQueryConfigSectionFamily::Query]
    }

    fn context_identity_digest(&self) -> String {
        "missing-relational-section".to_string()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TemporalRequirementContext;

impl WorthQueryDomainOperatingContext<GeometryDomainEntry> for TemporalRequirementContext {
    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[]
    }

    fn required_config_sections(&self) -> &'static [WorthQueryConfigSectionFamily] {
        &[WorthQueryConfigSectionFamily::Query]
    }

    fn required_operating_requirements(&self) -> &'static [WorthQueryDomainOperatingRequirement] {
        &[WorthQueryDomainOperatingRequirement::TemporalQuery]
    }

    fn context_identity_digest(&self) -> String {
        "temporal-requirement-context".to_string()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AsyncRequirementContext;

impl WorthQueryDomainOperatingContext<GeometryDomainEntry> for AsyncRequirementContext {
    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[]
    }

    fn required_config_sections(&self) -> &'static [WorthQueryConfigSectionFamily] {
        &[WorthQueryConfigSectionFamily::Query]
    }

    fn required_operating_requirements(&self) -> &'static [WorthQueryDomainOperatingRequirement] {
        &[WorthQueryDomainOperatingRequirement::AsyncResourceQuery]
    }

    fn context_identity_digest(&self) -> String {
        "async-requirement-context".to_string()
    }
}
