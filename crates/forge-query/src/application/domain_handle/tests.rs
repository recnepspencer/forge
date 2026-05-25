use super::{
    ForgeQueryConfiguredDomainHandleAdmissionError, ForgeQueryConfiguredDomainHandleChecked,
    ForgeQueryDomainOperatingContext,
};
use crate::application::{
    ForgeQueryApplicationFacade, ForgeQueryCapabilityFamily, ForgeQueryConfig,
    ForgeQueryConfigSectionFamily, ForgeQueryDomainEntryMarker, ForgeQueryQueryConfig,
    ForgeQueryRelationalConfig, ForgeQueryRuntimeBridgeConfig, ForgeQuerySignalConfig,
};

const ENTRY_CAPABILITIES: &[ForgeQueryCapabilityFamily] = &[
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
struct GeometryDomainEntry;

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
struct GeometryOperatingContext {
    access_class: AccessClass,
    invariant_regime: InvariantRegime,
    assumption_regime: AssumptionRegime,
}

impl GeometryOperatingContext {
    fn collaborative() -> Self {
        Self {
            access_class: AccessClass::Collaborative,
            invariant_regime: InvariantRegime::Conservative,
            assumption_regime: AssumptionRegime::TightTolerance,
        }
    }

    fn collaborative_reordered() -> Self {
        Self::collaborative()
    }

    fn restricted() -> Self {
        Self {
            access_class: AccessClass::Restricted,
            invariant_regime: InvariantRegime::Conservative,
            assumption_regime: AssumptionRegime::TightTolerance,
        }
    }

    fn with_permissive_invariants() -> Self {
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
struct DeferredStoreContext;

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
struct DisabledSignalContext;

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
struct MissingSectionContext;

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

#[test]
fn equivalent_operating_contexts_yield_identical_handle_digests() {
    let facade = ForgeQueryApplicationFacade::runtime_backed_default();
    let left = facade
        .domain(GeometryDomainEntry)
        .with_operating_context(GeometryOperatingContext::collaborative())
        .validate()
        .expect("collaborative context should validate");
    let right = facade
        .domain(GeometryDomainEntry)
        .with_operating_context(GeometryOperatingContext::collaborative_reordered())
        .validate()
        .expect("equivalent context should validate");

    assert_eq!(
        left.handle_identity_digest(),
        right.handle_identity_digest()
    );
    assert_eq!(
        left.required_capability_families(),
        right.required_capability_families()
    );
    assert_eq!(
        left.required_config_sections(),
        right.required_config_sections()
    );
}

#[test]
fn distinct_stable_regimes_yield_distinct_handle_digests() {
    let facade = ForgeQueryApplicationFacade::runtime_backed_default();
    let collaborative = facade
        .domain(GeometryDomainEntry)
        .with_operating_context(GeometryOperatingContext::collaborative())
        .validate()
        .expect("collaborative context should validate");
    let restricted = facade
        .domain(GeometryDomainEntry)
        .with_operating_context(GeometryOperatingContext::restricted())
        .validate()
        .expect("restricted context should validate");

    assert_ne!(
        collaborative.handle_identity_digest(),
        restricted.handle_identity_digest()
    );
    assert_ne!(
        collaborative.operating_context_identity_digest(),
        restricted.operating_context_identity_digest()
    );
}

#[test]
fn ordinary_and_checked_handles_preserve_identical_identity_and_support() {
    let facade = ForgeQueryApplicationFacade::runtime_backed_default();
    let ordinary = facade
        .domain(GeometryDomainEntry)
        .with_operating_context(GeometryOperatingContext::collaborative())
        .validate()
        .expect("context should validate")
        .admit()
        .expect("context should admit");
    let checked = facade
        .domain_checked(GeometryDomainEntry)
        .with_operating_context(GeometryOperatingContext::collaborative());

    match checked {
        ForgeQueryConfiguredDomainHandleChecked::Admitted(handle) => {
            assert_eq!(
                ordinary.handle_identity_digest(),
                handle.handle_identity_digest()
            );
            assert_eq!(ordinary.support_snapshot(), handle.support_snapshot());
            assert_eq!(
                ordinary.required_capability_families(),
                handle.required_capability_families()
            );
        }
        other => panic!("expected admitted configured handle, got {other:?}"),
    }
}

#[test]
fn disabled_required_sections_deny_before_declaration_authoring() {
    let facade = ForgeQueryApplicationFacade::new(
        ForgeQueryConfig::runtime_backed_default().with_signal(ForgeQuerySignalConfig::disabled()),
    )
    .expect("signal-disabled config remains valid");
    let checked = facade
        .domain_checked(GeometryDomainEntry)
        .with_operating_context(DisabledSignalContext);

    match checked {
        ForgeQueryConfiguredDomainHandleChecked::InvalidContext(denial) => {
            assert_eq!(
                denial.blocking_config_sections(),
                &[ForgeQueryConfigSectionFamily::Signal]
            );
        }
        other => panic!("expected invalid-context denial, got {other:?}"),
    }
}

#[test]
fn deferred_and_unsupported_capabilities_deny_before_declaration_authoring() {
    let deferred = ForgeQueryApplicationFacade::runtime_backed_default()
        .domain_checked(GeometryDomainEntry)
        .with_operating_context(DeferredStoreContext);
    match deferred {
        ForgeQueryConfiguredDomainHandleChecked::Deferred(denial) => {
            assert_eq!(
                denial.blocking_capability_families(),
                &[ForgeQueryCapabilityFamily::DurableArtifacts]
            );
        }
        other => panic!("expected deferred denial, got {other:?}"),
    }

    let unsupported = ForgeQueryApplicationFacade::new(
        ForgeQueryConfig::runtime_backed_default()
            .with_relational(ForgeQueryRelationalConfig::disabled())
            .with_runtime_bridge(ForgeQueryRuntimeBridgeConfig::disabled())
            .with_query(ForgeQueryQueryConfig::enabled()),
    )
    .expect("config remains valid");
    let checked = unsupported
        .domain_checked(GeometryDomainEntry)
        .with_operating_context(GeometryOperatingContext::with_permissive_invariants());
    match checked {
        ForgeQueryConfiguredDomainHandleChecked::Unsupported(denial) => {
            assert!(denial
                .blocking_capability_families()
                .contains(&ForgeQueryCapabilityFamily::HistoricalEvaluation));
        }
        other => panic!("expected unsupported denial, got {other:?}"),
    }
}

#[test]
fn invalid_context_rejects_missing_required_section_mapping() {
    let facade = ForgeQueryApplicationFacade::runtime_backed_default();
    let validation = facade
        .domain(GeometryDomainEntry)
        .with_operating_context(MissingSectionContext)
        .validate();

    match validation {
        Err(denial) => {
            assert_eq!(
                denial.blocking_config_sections(),
                &[ForgeQueryConfigSectionFamily::Relational]
            );
        }
        Ok(_) => panic!("expected invalid context denial"),
    }
}

#[test]
fn proof_lane_can_validate_configured_handle_without_losing_identity() {
    let facade = ForgeQueryApplicationFacade::runtime_backed_default();
    let ordinary = facade
        .domain(GeometryDomainEntry)
        .with_operating_context(GeometryOperatingContext::collaborative())
        .validate()
        .expect("ordinary path should validate");
    let proof = facade
        .domain_proof_root(GeometryDomainEntry)
        .with_operating_context(GeometryOperatingContext::collaborative())
        .validate()
        .expect("proof path should validate");

    assert_eq!(
        ordinary.handle_identity_digest(),
        proof.handle_identity_digest()
    );
}

#[test]
fn admit_returns_typed_denials_for_non_admitted_handles() {
    let facade = ForgeQueryApplicationFacade::runtime_backed_default();
    let validation = facade
        .domain(GeometryDomainEntry)
        .with_operating_context(DeferredStoreContext)
        .validate()
        .expect("store-backed context still validates structurally");

    match validation.admit() {
        Err(ForgeQueryConfiguredDomainHandleAdmissionError::Deferred(denial)) => {
            assert_eq!(
                denial.blocking_capability_families(),
                &[ForgeQueryCapabilityFamily::DurableArtifacts]
            );
        }
        other => panic!("expected deferred admission error, got {other:?}"),
    }
}
