use crate::application::{
    WorthQueryAdmittedConfiguredDomainHandle, WorthQueryAsyncDeclarationClause,
    WorthQueryAsyncDeclarationSupport, WorthQueryAsyncFailurePosture,
    WorthQueryAsyncLoadingPosture, WorthQueryAsyncSourceFamily,
    WorthQueryDeclarationBridgeTruthContext, WorthQueryDeclarationEntryReadinessStatus,
    WorthQueryDeclarationFamilyMarker, WorthQueryDeclarationInput, WorthQueryDomainEntryMarker,
    WorthQueryDomainOperatingContext,
};
use crate::basis_lifecycle::BasisFamily;
use crate::runtime::{WorthQueryRuntimeFacadeFamily, WorthQueryRuntimeFamilySupportStatus};

use super::retained_subject::ReadinessRetainedPosture;

pub(crate) fn async_bridge_readiness_override<
    D: WorthQueryDomainEntryMarker,
    C: WorthQueryDomainOperatingContext<D>,
    I: WorthQueryDeclarationInput<D>,
>(
    handle: &WorthQueryAdmittedConfiguredDomainHandle<D, C>,
    retained_posture: Option<&ReadinessRetainedPosture>,
) -> Option<(WorthQueryDeclarationEntryReadinessStatus, &'static str)> {
    if !async_readiness_applies::<D, I>(retained_posture) {
        return None;
    }

    if let Some(kind) = async_family_readiness_override::<D, I>() {
        return Some(kind);
    }

    if let Some(contract) = I::Family::bridge_continuation_contract() {
        match contract.request().truth_context() {
            WorthQueryDeclarationBridgeTruthContext::Historical => {
                return Some((
                    WorthQueryDeclarationEntryReadinessStatus::InvalidBasis,
                    "async declaration-entry readiness does not currently admit historical bridge truth context",
                ));
            }
            WorthQueryDeclarationBridgeTruthContext::Preview => {
                return Some((
                    WorthQueryDeclarationEntryReadinessStatus::InvalidBasis,
                    "async declaration-entry readiness does not currently admit preview bridge truth context",
                ));
            }
            WorthQueryDeclarationBridgeTruthContext::Current => {}
        }
    }

    if let Some(kind) = async_clause_readiness_denial_kind(retained_posture) {
        return Some(kind);
    }

    async_runtime_readiness(handle)
}

pub(crate) fn async_signal_readiness_override<
    D: WorthQueryDomainEntryMarker,
    C: WorthQueryDomainOperatingContext<D>,
    I: WorthQueryDeclarationInput<D>,
>(
    handle: &WorthQueryAdmittedConfiguredDomainHandle<D, C>,
    retained_posture: Option<&ReadinessRetainedPosture>,
) -> Option<(WorthQueryDeclarationEntryReadinessStatus, &'static str)> {
    if !async_readiness_applies::<D, I>(retained_posture) {
        return None;
    }

    if let Some(kind) = async_family_readiness_override::<D, I>() {
        return Some(kind);
    }

    if let Some(contract) = I::Family::signal_compatibility_contract() {
        if contract
            .required_basis_families()
            .contains(&BasisFamily::HistoricalSnapshot)
        {
            return Some((
                WorthQueryDeclarationEntryReadinessStatus::InvalidBasis,
                "async declaration-entry readiness does not currently admit historical signal basis families",
            ));
        }
        if contract
            .required_basis_families()
            .contains(&BasisFamily::PreviewDerived)
        {
            return Some((
                WorthQueryDeclarationEntryReadinessStatus::InvalidBasis,
                "async declaration-entry readiness does not currently admit preview-derived signal basis families",
            ));
        }
    }

    if let Some(kind) = async_clause_readiness_denial_kind(retained_posture) {
        return Some(kind);
    }

    async_runtime_readiness(handle)
}

fn async_readiness_applies<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>(
    retained_posture: Option<&ReadinessRetainedPosture>,
) -> bool {
    if matches!(
        I::Family::async_declaration_support(),
        WorthQueryAsyncDeclarationSupport::Unsupported
    ) {
        return false;
    }
    retained_posture.is_none_or(|posture| posture.async_declaration_active)
}

fn async_family_readiness_override<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>() -> Option<(WorthQueryDeclarationEntryReadinessStatus, &'static str)> {
    match I::Family::async_declaration_support() {
        WorthQueryAsyncDeclarationSupport::DeferredDebt => Some((
            WorthQueryDeclarationEntryReadinessStatus::Deferred,
            "async declaration-entry readiness remains deferred until this family's async declaration surface is admitted",
        )),
        WorthQueryAsyncDeclarationSupport::CanonicalIdentityOnly
        | WorthQueryAsyncDeclarationSupport::Unsupported => None,
    }
}

fn async_clause_readiness_denial_kind(
    retained_posture: Option<&ReadinessRetainedPosture>,
) -> Option<(WorthQueryDeclarationEntryReadinessStatus, &'static str)> {
    let clauses = retained_posture?.async_resource_clauses.as_slice();
    for clause in clauses {
        match clause {
            WorthQueryAsyncDeclarationClause::ResourceRequest {
                source_family,
                loading_posture,
                failure_posture,
                ..
            } => {
                if *source_family != WorthQueryAsyncSourceFamily::BridgeResource {
                    return Some((
                        WorthQueryDeclarationEntryReadinessStatus::Unsupported,
                        "async declaration-entry readiness only currently admits bridge-resource source families",
                    ));
                }
                if *loading_posture != WorthQueryAsyncLoadingPosture::Blocking {
                    return Some((
                        WorthQueryDeclarationEntryReadinessStatus::Unsupported,
                        "async declaration-entry readiness only currently admits blocking loading posture",
                    ));
                }
                if *failure_posture != WorthQueryAsyncFailurePosture::FailClosed {
                    return Some((
                        WorthQueryDeclarationEntryReadinessStatus::Unsupported,
                        "async declaration-entry readiness only currently admits fail-closed failure posture",
                    ));
                }
            }
            WorthQueryAsyncDeclarationClause::CompletionRequest { .. } => {
                return Some((
                    WorthQueryDeclarationEntryReadinessStatus::Unsupported,
                    "async declaration-entry readiness does not currently admit completion-request lifecycle",
                ));
            }
        }
    }
    None
}

fn async_runtime_readiness<
    D: WorthQueryDomainEntryMarker,
    C: WorthQueryDomainOperatingContext<D>,
>(
    handle: &WorthQueryAdmittedConfiguredDomainHandle<D, C>,
) -> Option<(WorthQueryDeclarationEntryReadinessStatus, &'static str)> {
    match handle
        .support_snapshot()
        .runtime_support_matrix()
        .row_for_family(WorthQueryRuntimeFacadeFamily::AsyncResource)
        .map(|row| row.status())
    {
        Some(WorthQueryRuntimeFamilySupportStatus::DeferredDebt) => Some((
            WorthQueryDeclarationEntryReadinessStatus::Deferred,
            "async declaration-entry readiness remains deferred until the async-resource runtime facade is admitted",
        )),
        Some(WorthQueryRuntimeFamilySupportStatus::Unsupported) | None => Some((
            WorthQueryDeclarationEntryReadinessStatus::Unsupported,
            "async declaration-entry readiness is unavailable because the async-resource runtime facade is unsupported for this operating world",
        )),
        Some(WorthQueryRuntimeFamilySupportStatus::Supported) => None,
    }
}
