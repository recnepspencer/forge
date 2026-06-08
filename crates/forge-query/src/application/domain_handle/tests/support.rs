use crate::application::{
    ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily, ForgeQueryDomainEntryMarker,
};

use super::super::{ForgeQueryDomainOperatingContext, ForgeQueryDomainOperatingRequirement};

pub const ENTRY_CAPABILITIES: &[ForgeQueryCapabilityFamily] = &[
    ForgeQueryCapabilityFamily::QueryComposition,
    ForgeQueryCapabilityFamily::QueryContext,
];

const COLLABORATIVE_CAPABILITIES: &[ForgeQueryCapabilityFamily] = &[
    ForgeQueryCapabilityFamily::PreviewSession,
    ForgeQueryCapabilityFamily::HistoricalEvaluation,
];

const COLLABORATIVE_SECTIONS: &[ForgeQueryConfigSectionFamily] = &[
    ForgeQueryConfigSectionFamily::Query,
    ForgeQueryConfigSectionFamily::RuntimeBridge,
    ForgeQueryConfigSectionFamily::Relational,
];

const STORE_SECTIONS: &[ForgeQueryConfigSectionFamily] = &[
    ForgeQueryConfigSectionFamily::Query,
    ForgeQueryConfigSectionFamily::Store,
];

const SIGNAL_SECTIONS: &[ForgeQueryConfigSectionFamily] = &[
    ForgeQueryConfigSectionFamily::Query,
    ForgeQueryConfigSectionFamily::Signal,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GeometryDomainEntry;

impl ForgeQueryDomainEntryMarker for GeometryDomainEntry {
    fn domain_key(&self) -> &'static str {
        "test.geometry"
    }

    fn display_name(&self) -> &'static str {
        "GeometryDomainEntry"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
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

impl ForgeQueryDomainOperatingContext<GeometryDomainEntry> for GeometryOperatingContext {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        COLLABORATIVE_CAPABILITIES
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
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

impl ForgeQueryDomainOperatingContext<GeometryDomainEntry> for DeferredStoreContext {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::DurableArtifacts]
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        STORE_SECTIONS
    }

    fn context_identity_digest(&self) -> String {
        "store-context".to_string()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DisabledSignalContext;

impl ForgeQueryDomainOperatingContext<GeometryDomainEntry> for DisabledSignalContext {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[]
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        SIGNAL_SECTIONS
    }

    fn context_identity_digest(&self) -> String {
        "signal-context".to_string()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MissingSectionContext;

impl ForgeQueryDomainOperatingContext<GeometryDomainEntry> for MissingSectionContext {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::HistoricalEvaluation]
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        &[ForgeQueryConfigSectionFamily::Query]
    }

    fn context_identity_digest(&self) -> String {
        "missing-relational-section".to_string()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TemporalRequirementContext;

impl ForgeQueryDomainOperatingContext<GeometryDomainEntry> for TemporalRequirementContext {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[]
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        &[ForgeQueryConfigSectionFamily::Query]
    }

    fn required_operating_requirements(&self) -> &'static [ForgeQueryDomainOperatingRequirement] {
        &[ForgeQueryDomainOperatingRequirement::TemporalQuery]
    }

    fn context_identity_digest(&self) -> String {
        "temporal-requirement-context".to_string()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AsyncRequirementContext;

impl ForgeQueryDomainOperatingContext<GeometryDomainEntry> for AsyncRequirementContext {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[]
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        &[ForgeQueryConfigSectionFamily::Query]
    }

    fn required_operating_requirements(&self) -> &'static [ForgeQueryDomainOperatingRequirement] {
        &[ForgeQueryDomainOperatingRequirement::AsyncResourceQuery]
    }

    fn context_identity_digest(&self) -> String {
        "async-requirement-context".to_string()
    }
}
