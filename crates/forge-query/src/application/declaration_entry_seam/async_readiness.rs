use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryAsyncDeclarationClause,
    ForgeQueryAsyncDeclarationSupport, ForgeQueryAsyncFailurePosture,
    ForgeQueryAsyncLoadingPosture, ForgeQueryAsyncSourceFamily,
    ForgeQueryDeclarationBridgeTruthContext, ForgeQueryDeclarationEntryReadinessStatus,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext,
};
use crate::basis_lifecycle::BasisFamily;
use crate::runtime::{ForgeQueryRuntimeFacadeFamily, ForgeQueryRuntimeFamilySupportStatus};

use super::retained_subject::ReadinessRetainedPosture;

pub(crate) fn async_bridge_readiness_override<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    retained_posture: Option<&ReadinessRetainedPosture>,
) -> Option<(ForgeQueryDeclarationEntryReadinessStatus, &'static str)> {
    if !async_readiness_applies::<D, I>(retained_posture) {
        return None;
    }

    if let Some(kind) = async_family_readiness_override::<D, I>() {
        return Some(kind);
    }

    if let Some(contract) = I::Family::bridge_continuation_contract() {
        match contract.request().truth_context() {
            ForgeQueryDeclarationBridgeTruthContext::Historical => {
                return Some((
                    ForgeQueryDeclarationEntryReadinessStatus::InvalidBasis,
                    "async declaration-entry readiness does not currently admit historical bridge truth context",
                ));
            }
            ForgeQueryDeclarationBridgeTruthContext::Preview => {
                return Some((
                    ForgeQueryDeclarationEntryReadinessStatus::InvalidBasis,
                    "async declaration-entry readiness does not currently admit preview bridge truth context",
                ));
            }
            ForgeQueryDeclarationBridgeTruthContext::Current => {}
        }
    }

    if let Some(kind) = async_clause_readiness_denial_kind(retained_posture) {
        return Some(kind);
    }

    async_runtime_readiness(handle)
}

pub(crate) fn async_signal_readiness_override<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    retained_posture: Option<&ReadinessRetainedPosture>,
) -> Option<(ForgeQueryDeclarationEntryReadinessStatus, &'static str)> {
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
                ForgeQueryDeclarationEntryReadinessStatus::InvalidBasis,
                "async declaration-entry readiness does not currently admit historical signal basis families",
            ));
        }
        if contract
            .required_basis_families()
            .contains(&BasisFamily::PreviewDerived)
        {
            return Some((
                ForgeQueryDeclarationEntryReadinessStatus::InvalidBasis,
                "async declaration-entry readiness does not currently admit preview-derived signal basis families",
            ));
        }
    }

    if let Some(kind) = async_clause_readiness_denial_kind(retained_posture) {
        return Some(kind);
    }

    async_runtime_readiness(handle)
}

fn async_readiness_applies<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    retained_posture: Option<&ReadinessRetainedPosture>,
) -> bool {
    if matches!(
        I::Family::async_declaration_support(),
        ForgeQueryAsyncDeclarationSupport::Unsupported
    ) {
        return false;
    }
    retained_posture.is_none_or(|posture| posture.async_declaration_active)
}

fn async_family_readiness_override<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>() -> Option<(ForgeQueryDeclarationEntryReadinessStatus, &'static str)> {
    match I::Family::async_declaration_support() {
        ForgeQueryAsyncDeclarationSupport::DeferredDebt => Some((
            ForgeQueryDeclarationEntryReadinessStatus::Deferred,
            "async declaration-entry readiness remains deferred until this family's async declaration surface is admitted",
        )),
        ForgeQueryAsyncDeclarationSupport::CanonicalIdentityOnly
        | ForgeQueryAsyncDeclarationSupport::Unsupported => None,
    }
}

fn async_clause_readiness_denial_kind(
    retained_posture: Option<&ReadinessRetainedPosture>,
) -> Option<(ForgeQueryDeclarationEntryReadinessStatus, &'static str)> {
    let clauses = retained_posture?.async_resource_clauses.as_slice();
    for clause in clauses {
        match clause {
            ForgeQueryAsyncDeclarationClause::ResourceRequest {
                source_family,
                loading_posture,
                failure_posture,
                ..
            } => {
                if *source_family != ForgeQueryAsyncSourceFamily::BridgeResource {
                    return Some((
                        ForgeQueryDeclarationEntryReadinessStatus::Unsupported,
                        "async declaration-entry readiness only currently admits bridge-resource source families",
                    ));
                }
                if *loading_posture != ForgeQueryAsyncLoadingPosture::Blocking {
                    return Some((
                        ForgeQueryDeclarationEntryReadinessStatus::Unsupported,
                        "async declaration-entry readiness only currently admits blocking loading posture",
                    ));
                }
                if *failure_posture != ForgeQueryAsyncFailurePosture::FailClosed {
                    return Some((
                        ForgeQueryDeclarationEntryReadinessStatus::Unsupported,
                        "async declaration-entry readiness only currently admits fail-closed failure posture",
                    ));
                }
            }
            ForgeQueryAsyncDeclarationClause::CompletionRequest { .. } => {
                return Some((
                    ForgeQueryDeclarationEntryReadinessStatus::Unsupported,
                    "async declaration-entry readiness does not currently admit completion-request lifecycle",
                ));
            }
        }
    }
    None
}

fn async_runtime_readiness<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
) -> Option<(ForgeQueryDeclarationEntryReadinessStatus, &'static str)> {
    match handle
        .support_snapshot()
        .runtime_support_matrix()
        .row_for_family(ForgeQueryRuntimeFacadeFamily::AsyncResource)
        .map(|row| row.status())
    {
        Some(ForgeQueryRuntimeFamilySupportStatus::DeferredDebt) => Some((
            ForgeQueryDeclarationEntryReadinessStatus::Deferred,
            "async declaration-entry readiness remains deferred until the async-resource runtime facade is admitted",
        )),
        Some(ForgeQueryRuntimeFamilySupportStatus::Unsupported) | None => Some((
            ForgeQueryDeclarationEntryReadinessStatus::Unsupported,
            "async declaration-entry readiness is unavailable because the async-resource runtime facade is unsupported for this operating world",
        )),
        Some(ForgeQueryRuntimeFamilySupportStatus::Supported) => None,
    }
}
