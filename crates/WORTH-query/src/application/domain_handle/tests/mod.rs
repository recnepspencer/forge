mod support;

use super::{
    WorthQueryConfiguredDomainHandleAdmissionError, WorthQueryConfiguredDomainHandleChecked,
    WorthQueryDomainOperatingRequirement,
};
use crate::application::{
    WorthQueryApplicationFacade, WorthQueryCapabilityFamily, WorthQueryConfig,
    WorthQueryConfigSectionFamily, WorthQueryQueryConfig, WorthQueryRelationalConfig,
    WorthQueryRuntimeBridgeConfig, WorthQuerySignalConfig,
};
use support::{
    AsyncRequirementContext, DeferredStoreContext, DisabledSignalContext, GeometryDomainEntry,
    GeometryOperatingContext, MissingSectionContext, TemporalRequirementContext,
};

#[test]
fn equivalent_operating_contexts_yield_identical_handle_digests() {
    let facade = WorthQueryApplicationFacade::runtime_backed_default();
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
    let facade = WorthQueryApplicationFacade::runtime_backed_default();
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
    let facade = WorthQueryApplicationFacade::runtime_backed_default();
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
        WorthQueryConfiguredDomainHandleChecked::Admitted(handle) => {
            assert_eq!(
                ordinary.handle_identity_digest(),
                handle.handle_identity_digest()
            );
            assert_eq!(ordinary.support_snapshot(), handle.support_snapshot());
            assert_eq!(
                ordinary.required_capability_families(),
                handle.required_capability_families()
            );
            assert_eq!(
                ordinary.required_operating_requirements(),
                handle.required_operating_requirements()
            );
        }
        other => panic!("expected admitted configured handle, got {other:?}"),
    }
}

#[test]
fn disabled_required_sections_deny_before_declaration_authoring() {
    let facade = WorthQueryApplicationFacade::new(
        WorthQueryConfig::runtime_backed_default().with_signal(WorthQuerySignalConfig::disabled()),
    )
    .expect("signal-disabled config remains valid");
    let checked = facade
        .domain_checked(GeometryDomainEntry)
        .with_operating_context(DisabledSignalContext);

    match checked {
        WorthQueryConfiguredDomainHandleChecked::InvalidContext(denial) => {
            assert_eq!(
                denial.blocking_config_sections(),
                &[WorthQueryConfigSectionFamily::Signal]
            );
        }
        other => panic!("expected invalid-context denial, got {other:?}"),
    }
}

#[test]
fn deferred_and_unsupported_capabilities_deny_before_declaration_authoring() {
    let deferred = WorthQueryApplicationFacade::runtime_backed_default()
        .domain_checked(GeometryDomainEntry)
        .with_operating_context(DeferredStoreContext);
    match deferred {
        WorthQueryConfiguredDomainHandleChecked::Deferred(denial) => {
            assert_eq!(
                denial.blocking_capability_families(),
                &[WorthQueryCapabilityFamily::DurableArtifacts]
            );
        }
        other => panic!("expected deferred denial, got {other:?}"),
    }

    let unsupported = WorthQueryApplicationFacade::new(
        WorthQueryConfig::runtime_backed_default()
            .with_relational(WorthQueryRelationalConfig::disabled())
            .with_runtime_bridge(WorthQueryRuntimeBridgeConfig::disabled())
            .with_query(WorthQueryQueryConfig::enabled()),
    )
    .expect("config remains valid");
    let checked = unsupported
        .domain_checked(GeometryDomainEntry)
        .with_operating_context(GeometryOperatingContext::with_permissive_invariants());
    match checked {
        WorthQueryConfiguredDomainHandleChecked::Unsupported(denial) => {
            assert!(denial
                .blocking_capability_families()
                .contains(&WorthQueryCapabilityFamily::HistoricalEvaluation));
        }
        other => panic!("expected unsupported denial, got {other:?}"),
    }
}

#[test]
fn invalid_context_rejects_missing_required_section_mapping() {
    let facade = WorthQueryApplicationFacade::runtime_backed_default();
    let validation = facade
        .domain(GeometryDomainEntry)
        .with_operating_context(MissingSectionContext)
        .validate();

    match validation {
        Err(denial) => {
            assert_eq!(
                denial.blocking_config_sections(),
                &[WorthQueryConfigSectionFamily::Relational]
            );
        }
        Ok(_) => panic!("expected invalid context denial"),
    }
}

#[test]
fn proof_lane_can_validate_configured_handle_without_losing_identity() {
    let facade = WorthQueryApplicationFacade::runtime_backed_default();
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
    assert_eq!(
        ordinary.required_operating_requirements(),
        proof.required_operating_requirements()
    );
}

#[test]
fn admit_returns_typed_denials_for_non_admitted_handles() {
    let facade = WorthQueryApplicationFacade::runtime_backed_default();
    let validation = facade
        .domain(GeometryDomainEntry)
        .with_operating_context(DeferredStoreContext)
        .validate()
        .expect("store-backed context still validates structurally");

    match validation.admit() {
        Err(WorthQueryConfiguredDomainHandleAdmissionError::Deferred(denial)) => {
            assert_eq!(
                denial.blocking_capability_families(),
                &[WorthQueryCapabilityFamily::DurableArtifacts]
            );
        }
        other => panic!("expected deferred admission error, got {other:?}"),
    }
}

#[test]
fn temporal_and_async_operating_requirements_admit_when_runtime_backed_support_is_closed() {
    let facade = WorthQueryApplicationFacade::runtime_backed_default();

    match facade
        .domain_checked(GeometryDomainEntry)
        .with_operating_context(TemporalRequirementContext)
    {
        WorthQueryConfiguredDomainHandleChecked::Admitted(handle) => {
            assert_eq!(
                handle.required_operating_requirements(),
                &[WorthQueryDomainOperatingRequirement::TemporalQuery]
            );
        }
        other => panic!("expected admitted temporal operating handle, got {other:?}"),
    }

    match facade
        .domain_checked(GeometryDomainEntry)
        .with_operating_context(AsyncRequirementContext)
    {
        WorthQueryConfiguredDomainHandleChecked::Admitted(handle) => {
            assert_eq!(
                handle.required_operating_requirements(),
                &[WorthQueryDomainOperatingRequirement::AsyncResourceQuery]
            );
        }
        other => panic!("expected admitted async operating handle, got {other:?}"),
    }
}
