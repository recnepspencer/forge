use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationBridgeTruthContext,
    ForgeQueryDeclarationEntryReadinessStatus, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
    ForgeQueryTemporalDeclarationSupport,
};
use crate::basis_lifecycle::BasisFamily;
use crate::runtime::{ForgeQueryRuntimeFacadeFamily, ForgeQueryRuntimeFamilySupportStatus};

use super::retained_subject::ReadinessRetainedPosture;

pub(crate) fn temporal_bridge_readiness_override<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    retained_posture: Option<&ReadinessRetainedPosture>,
) -> Option<(ForgeQueryDeclarationEntryReadinessStatus, &'static str)> {
    if !temporal_readiness_applies::<D, I>(retained_posture) {
        return None;
    }

    if let Some(contract) = I::Family::bridge_continuation_contract() {
        match contract.request().truth_context() {
            ForgeQueryDeclarationBridgeTruthContext::Historical => {
                return Some((
                    ForgeQueryDeclarationEntryReadinessStatus::InvalidBasis,
                    "temporal declaration-entry readiness does not currently admit historical bridge truth context",
                ));
            }
            ForgeQueryDeclarationBridgeTruthContext::Preview => {
                return Some((
                    ForgeQueryDeclarationEntryReadinessStatus::InvalidBasis,
                    "temporal declaration-entry readiness does not currently admit preview bridge truth context",
                ));
            }
            ForgeQueryDeclarationBridgeTruthContext::Current => {}
        }
    }

    temporal_runtime_readiness(handle)
}

pub(crate) fn temporal_signal_readiness_override<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    retained_posture: Option<&ReadinessRetainedPosture>,
) -> Option<(ForgeQueryDeclarationEntryReadinessStatus, &'static str)> {
    if !temporal_readiness_applies::<D, I>(retained_posture) {
        return None;
    }

    if let Some(contract) = I::Family::signal_compatibility_contract() {
        if contract
            .required_basis_families()
            .contains(&BasisFamily::HistoricalSnapshot)
        {
            return Some((
                ForgeQueryDeclarationEntryReadinessStatus::InvalidBasis,
                "temporal declaration-entry readiness does not currently admit historical signal basis families",
            ));
        }
        if contract
            .required_basis_families()
            .contains(&BasisFamily::PreviewDerived)
        {
            return Some((
                ForgeQueryDeclarationEntryReadinessStatus::InvalidBasis,
                "temporal declaration-entry readiness does not currently admit preview-derived signal basis families",
            ));
        }
    }

    temporal_runtime_readiness(handle)
}

fn temporal_family_enabled<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
) -> bool {
    I::Family::temporal_declaration_support() != ForgeQueryTemporalDeclarationSupport::Unsupported
}

fn temporal_readiness_applies<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    retained_posture: Option<&ReadinessRetainedPosture>,
) -> bool {
    if !temporal_family_enabled::<D, I>() {
        return false;
    }
    retained_posture.is_none_or(|posture| posture.temporal_declaration_active)
}

fn temporal_runtime_readiness<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
) -> Option<(ForgeQueryDeclarationEntryReadinessStatus, &'static str)> {
    match handle
        .support_snapshot()
        .runtime_support_matrix()
        .row_for_family(ForgeQueryRuntimeFacadeFamily::Temporal)
        .map(|row| row.status())
    {
        Some(ForgeQueryRuntimeFamilySupportStatus::DeferredDebt) => Some((
            ForgeQueryDeclarationEntryReadinessStatus::Deferred,
            "temporal declaration-entry readiness remains deferred until the temporal runtime facade is admitted",
        )),
        Some(ForgeQueryRuntimeFamilySupportStatus::Unsupported) | None => Some((
            ForgeQueryDeclarationEntryReadinessStatus::Unsupported,
            "temporal declaration-entry readiness is unavailable because the temporal runtime facade is unsupported for this operating world",
        )),
        Some(ForgeQueryRuntimeFamilySupportStatus::Supported) => None,
    }
}
