use crate::application::{
    WorthQueryAdmittedConfiguredDomainHandle, WorthQueryDeclarationBridgeTruthContext,
    WorthQueryDeclarationEntryReadinessStatus, WorthQueryDeclarationFamilyMarker,
    WorthQueryDeclarationInput, WorthQueryDomainEntryMarker, WorthQueryDomainOperatingContext,
    WorthQueryTemporalDeclarationSupport,
};
use crate::basis_lifecycle::BasisFamily;
use crate::runtime::{WorthQueryRuntimeFacadeFamily, WorthQueryRuntimeFamilySupportStatus};

use super::retained_subject::ReadinessRetainedPosture;

pub(crate) fn temporal_bridge_readiness_override<
    D: WorthQueryDomainEntryMarker,
    C: WorthQueryDomainOperatingContext<D>,
    I: WorthQueryDeclarationInput<D>,
>(
    handle: &WorthQueryAdmittedConfiguredDomainHandle<D, C>,
    retained_posture: Option<&ReadinessRetainedPosture>,
) -> Option<(WorthQueryDeclarationEntryReadinessStatus, &'static str)> {
    if !temporal_readiness_applies::<D, I>(retained_posture) {
        return None;
    }

    if let Some(contract) = I::Family::bridge_continuation_contract() {
        match contract.request().truth_context() {
            WorthQueryDeclarationBridgeTruthContext::Historical => {
                return Some((
                    WorthQueryDeclarationEntryReadinessStatus::InvalidBasis,
                    "temporal declaration-entry readiness does not currently admit historical bridge truth context",
                ));
            }
            WorthQueryDeclarationBridgeTruthContext::Preview => {
                return Some((
                    WorthQueryDeclarationEntryReadinessStatus::InvalidBasis,
                    "temporal declaration-entry readiness does not currently admit preview bridge truth context",
                ));
            }
            WorthQueryDeclarationBridgeTruthContext::Current => {}
        }
    }

    temporal_runtime_readiness(handle)
}

pub(crate) fn temporal_signal_readiness_override<
    D: WorthQueryDomainEntryMarker,
    C: WorthQueryDomainOperatingContext<D>,
    I: WorthQueryDeclarationInput<D>,
>(
    handle: &WorthQueryAdmittedConfiguredDomainHandle<D, C>,
    retained_posture: Option<&ReadinessRetainedPosture>,
) -> Option<(WorthQueryDeclarationEntryReadinessStatus, &'static str)> {
    if !temporal_readiness_applies::<D, I>(retained_posture) {
        return None;
    }

    if let Some(contract) = I::Family::signal_compatibility_contract() {
        if contract
            .required_basis_families()
            .contains(&BasisFamily::HistoricalSnapshot)
        {
            return Some((
                WorthQueryDeclarationEntryReadinessStatus::InvalidBasis,
                "temporal declaration-entry readiness does not currently admit historical signal basis families",
            ));
        }
        if contract
            .required_basis_families()
            .contains(&BasisFamily::PreviewDerived)
        {
            return Some((
                WorthQueryDeclarationEntryReadinessStatus::InvalidBasis,
                "temporal declaration-entry readiness does not currently admit preview-derived signal basis families",
            ));
        }
    }

    temporal_runtime_readiness(handle)
}

fn temporal_family_enabled<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>(
) -> bool {
    I::Family::temporal_declaration_support() != WorthQueryTemporalDeclarationSupport::Unsupported
}

fn temporal_readiness_applies<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>(
    retained_posture: Option<&ReadinessRetainedPosture>,
) -> bool {
    if !temporal_family_enabled::<D, I>() {
        return false;
    }
    retained_posture.is_none_or(|posture| posture.temporal_declaration_active)
}

fn temporal_runtime_readiness<
    D: WorthQueryDomainEntryMarker,
    C: WorthQueryDomainOperatingContext<D>,
>(
    handle: &WorthQueryAdmittedConfiguredDomainHandle<D, C>,
) -> Option<(WorthQueryDeclarationEntryReadinessStatus, &'static str)> {
    match handle
        .support_snapshot()
        .runtime_support_matrix()
        .row_for_family(WorthQueryRuntimeFacadeFamily::Temporal)
        .map(|row| row.status())
    {
        Some(WorthQueryRuntimeFamilySupportStatus::DeferredDebt) => Some((
            WorthQueryDeclarationEntryReadinessStatus::Deferred,
            "temporal declaration-entry readiness remains deferred until the temporal runtime facade is admitted",
        )),
        Some(WorthQueryRuntimeFamilySupportStatus::Unsupported) | None => Some((
            WorthQueryDeclarationEntryReadinessStatus::Unsupported,
            "temporal declaration-entry readiness is unavailable because the temporal runtime facade is unsupported for this operating world",
        )),
        Some(WorthQueryRuntimeFamilySupportStatus::Supported) => None,
    }
}
