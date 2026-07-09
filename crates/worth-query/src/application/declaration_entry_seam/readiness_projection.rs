use crate::application::{
    WorthQueryAdmittedConfiguredDomainHandle, WorthQueryCapabilityFamily,
    WorthQueryCapabilityStatus, WorthQueryConfigSectionFamily, WorthQueryDeclarationAspectFit,
    WorthQueryDeclarationAuthorityAspectMismatch, WorthQueryDeclarationCapabilityStatus,
    WorthQueryDeclarationFamilyMarker, WorthQueryDeclarationFamilySupportReport,
    WorthQueryDeclarationInput, WorthQueryDomainEntryMarker, WorthQueryDomainOperatingContext,
    WorthQueryLowerAuthorityRouteFamily, WorthQuerySignalCompatibilityPosture,
};

use super::{
    async_readiness::{async_bridge_readiness_override, async_signal_readiness_override},
    retained_subject::ReadinessRetainedPosture,
    row::{WorthQueryDeclarationEntryCrossingRow, WorthQueryDeclarationEntryCrossingSurface},
    support::{WorthQueryDeclarationEntryReadinessRow, WorthQueryDeclarationEntryReadinessStatus},
    temporal_readiness::{temporal_bridge_readiness_override, temporal_signal_readiness_override},
};

pub(crate) fn readiness_row_for_crossing<
    D: WorthQueryDomainEntryMarker,
    C: WorthQueryDomainOperatingContext<D>,
    I: WorthQueryDeclarationInput<D>,
>(
    row: WorthQueryDeclarationEntryCrossingRow,
    handle: &WorthQueryAdmittedConfiguredDomainHandle<D, C>,
    retained_posture: Option<&ReadinessRetainedPosture>,
) -> WorthQueryDeclarationEntryReadinessRow {
    let family_support = handle.family_support::<I::Family>();
    let envelope_aspect_publication =
        retained_posture.map(|posture| posture.envelope_aspect_publication.clone());
    match row.surface() {
        WorthQueryDeclarationEntryCrossingSurface::Envelope => {
            envelope_row(row, &family_support, envelope_aspect_publication)
        }
        WorthQueryDeclarationEntryCrossingSurface::RelationalTruthRouting => {
            let (status, reason) = relational_status::<D, C, I>(handle);
            let relational_summary = retained_posture
                .map(|posture| posture.relational_authority_summary.clone())
                .unwrap_or_else(|| {
                    crate::application::relational_authority_summary_from_coverage(
                        &I::Family::aspect_contract(),
                        I::Family::aspect_coverage(),
                        crate::application::WorthQueryDeclarationAspectCoverageBasis::DeclaredFamilyCoverage,
                        I::Family::relational_truth_contract().as_ref(),
                    )
                });
            let (status, reason) = reconcile_authority_readiness(
                status,
                reason,
                relational_summary.aspect_mismatch(),
                None,
            );
            WorthQueryDeclarationEntryReadinessRow::new(
                row,
                status,
                reason,
                envelope_aspect_publication,
                Some(relational_summary),
                None,
                None,
            )
        }
        WorthQueryDeclarationEntryCrossingSurface::BridgeContinuationRouting => {
            let (status, reason) = bridge_status::<D, C, I>(handle);
            let (status, reason) =
                temporal_bridge_readiness_override::<D, C, I>(handle, retained_posture)
                    .unwrap_or((status, reason));
            let (status, reason) =
                async_bridge_readiness_override::<D, C, I>(handle, retained_posture)
                    .unwrap_or((status, reason));
            let bridge_summary = retained_posture
                .map(|posture| posture.bridge_authority_summary.clone())
                .unwrap_or_else(|| {
                    crate::application::bridge_authority_summary_from_coverage(
                        &I::Family::aspect_contract(),
                        I::Family::aspect_coverage(),
                        crate::application::WorthQueryDeclarationAspectCoverageBasis::DeclaredFamilyCoverage,
                        I::Family::bridge_continuation_contract().as_ref(),
                    )
                });
            let (status, reason) = reconcile_authority_readiness(
                status,
                reason,
                bridge_summary.aspect_mismatch(),
                Some(bridge_summary.mapping_fit()),
            );
            WorthQueryDeclarationEntryReadinessRow::new(
                row,
                status,
                reason,
                envelope_aspect_publication,
                None,
                Some(bridge_summary),
                None,
            )
        }
        WorthQueryDeclarationEntryCrossingSurface::SignalCompatibility => {
            let (status, reason) = signal_status::<D, C, I>(handle);
            let (status, reason) =
                temporal_signal_readiness_override::<D, C, I>(handle, retained_posture)
                    .unwrap_or((status, reason));
            let (status, reason) =
                async_signal_readiness_override::<D, C, I>(handle, retained_posture)
                    .unwrap_or((status, reason));
            let signal_summary = retained_posture
                .map(|posture| posture.signal_authority_summary.clone())
                .unwrap_or_else(|| {
                    crate::application::signal_authority_summary_from_coverage(
                        &I::Family::aspect_contract(),
                        I::Family::aspect_coverage(),
                        crate::application::WorthQueryDeclarationAspectCoverageBasis::DeclaredFamilyCoverage,
                        I::Family::signal_compatibility_contract().as_ref(),
                    )
                });
            let (status, reason) = reconcile_authority_readiness(
                status,
                reason,
                signal_summary.aspect_mismatch(),
                None,
            );
            WorthQueryDeclarationEntryReadinessRow::new(
                row,
                status,
                reason,
                envelope_aspect_publication,
                None,
                None,
                Some(signal_summary),
            )
        }
    }
}

fn envelope_row<D: WorthQueryDomainEntryMarker, F: WorthQueryDeclarationFamilyMarker<D>>(
    row: WorthQueryDeclarationEntryCrossingRow,
    family_support: &WorthQueryDeclarationFamilySupportReport<D, F>,
    envelope_aspect_publication: Option<crate::application::WorthQueryDeclarationAspectPublication>,
) -> WorthQueryDeclarationEntryReadinessRow {
    let status = match family_support.declare_status() {
        WorthQueryDeclarationCapabilityStatus::Admitted => {
            WorthQueryDeclarationEntryReadinessStatus::Admitted
        }
        WorthQueryDeclarationCapabilityStatus::DeferredDebt => {
            WorthQueryDeclarationEntryReadinessStatus::Deferred
        }
        WorthQueryDeclarationCapabilityStatus::Unsupported => {
            WorthQueryDeclarationEntryReadinessStatus::Unsupported
        }
        WorthQueryDeclarationCapabilityStatus::InvalidContext => {
            WorthQueryDeclarationEntryReadinessStatus::InvalidBasis
        }
    };
    let reason = family_support
        .row(crate::application::WorthQueryDeclarationCapabilityVerb::Declare)
        .expect("declare row must exist")
        .reason();
    WorthQueryDeclarationEntryReadinessRow::new(
        row,
        status,
        reason,
        envelope_aspect_publication,
        None,
        None,
        None,
    )
}

#[rustfmt::skip]
fn relational_status<D: WorthQueryDomainEntryMarker, C: WorthQueryDomainOperatingContext<D>, I: WorthQueryDeclarationInput<D>>(handle: &WorthQueryAdmittedConfiguredDomainHandle<D, C>) -> (WorthQueryDeclarationEntryReadinessStatus, &'static str) {
    match I::Family::relational_truth_contract() {
        Some(contract) => {
            if !I::Family::route_contract().allowed_route_families().contains(&WorthQueryLowerAuthorityRouteFamily::Relational) { return (WorthQueryDeclarationEntryReadinessStatus::Unsupported, "route planning does not currently admit a relational slice for this family"); }
            if !config_sections_enabled(handle, contract.required_config_sections()) { return (WorthQueryDeclarationEntryReadinessStatus::InvalidBasis, "required relational config sections must be enabled before relational truth routing"); }
            if !capabilities_admitted(handle, contract.required_capability_families()) { return (WorthQueryDeclarationEntryReadinessStatus::Unsupported, "required relational capability families are not admitted in this operating world"); }
            (WorthQueryDeclarationEntryReadinessStatus::Admitted, contract.reason())
        }
        None => (WorthQueryDeclarationEntryReadinessStatus::Unsupported, "family does not expose a relational truth-routing contract"),
    }
}

#[rustfmt::skip]
fn bridge_status<D: WorthQueryDomainEntryMarker, C: WorthQueryDomainOperatingContext<D>, I: WorthQueryDeclarationInput<D>>(handle: &WorthQueryAdmittedConfiguredDomainHandle<D, C>) -> (WorthQueryDeclarationEntryReadinessStatus, &'static str) {
    match I::Family::bridge_continuation_contract() {
        Some(contract) => {
            if !I::Family::route_contract().allowed_route_families().contains(&WorthQueryLowerAuthorityRouteFamily::Bridge) { return (WorthQueryDeclarationEntryReadinessStatus::Unsupported, "route planning does not currently admit a bridge slice for this family"); }
            if !config_sections_enabled(handle, contract.required_config_sections()) { return (WorthQueryDeclarationEntryReadinessStatus::InvalidBasis, "required runtime-bridge config sections must be enabled before bridge continuation routing"); }
            if !capabilities_admitted(handle, contract.required_capability_families()) { return (WorthQueryDeclarationEntryReadinessStatus::Unsupported, "required bridge continuation capability families are not admitted in this operating world"); }
            (WorthQueryDeclarationEntryReadinessStatus::Admitted, contract.reason())
        }
        None => (WorthQueryDeclarationEntryReadinessStatus::Unsupported, "family does not expose a bridge continuation-routing contract"),
    }
}

#[rustfmt::skip]
fn signal_status<D: WorthQueryDomainEntryMarker, C: WorthQueryDomainOperatingContext<D>, I: WorthQueryDeclarationInput<D>>(handle: &WorthQueryAdmittedConfiguredDomainHandle<D, C>) -> (WorthQueryDeclarationEntryReadinessStatus, &'static str) {
    match I::Family::taxonomy().signal_compatibility() {
        WorthQuerySignalCompatibilityPosture::Compatible => match I::Family::signal_compatibility_contract() {
            Some(contract) => {
                if !config_sections_enabled(handle, contract.required_config_sections()) { return (WorthQueryDeclarationEntryReadinessStatus::InvalidBasis, "required signal config sections must be enabled before signal compatibility is admitted"); }
                if !capabilities_admitted(handle, contract.required_capability_families()) { return (WorthQueryDeclarationEntryReadinessStatus::InvalidBasis, "required signal capability families are not admitted for this basis-sensitive continuation"); }
                (WorthQueryDeclarationEntryReadinessStatus::Admitted, contract.reason())
            }
            None => (WorthQueryDeclarationEntryReadinessStatus::Unsupported, "family claims signal compatibility but does not expose a signal compatibility contract"),
        },
        WorthQuerySignalCompatibilityPosture::Deferred => (WorthQueryDeclarationEntryReadinessStatus::Deferred, "signal compatibility for this family remains explicitly deferred"),
        WorthQuerySignalCompatibilityPosture::NotCompatible => (WorthQueryDeclarationEntryReadinessStatus::Unsupported, "family is not structurally signal-compatible"),
    }
}

fn capabilities_admitted<D: WorthQueryDomainEntryMarker, C: WorthQueryDomainOperatingContext<D>>(
    handle: &WorthQueryAdmittedConfiguredDomainHandle<D, C>,
    required: &'static [WorthQueryCapabilityFamily],
) -> bool {
    required.iter().copied().all(|family| {
        handle.support_snapshot().capability_status(family)
            == Some(WorthQueryCapabilityStatus::Admitted)
    })
}

fn config_sections_enabled<
    D: WorthQueryDomainEntryMarker,
    C: WorthQueryDomainOperatingContext<D>,
>(
    handle: &WorthQueryAdmittedConfiguredDomainHandle<D, C>,
    required: &'static [WorthQueryConfigSectionFamily],
) -> bool {
    required.iter().copied().all(|section| {
        handle
            .support_snapshot()
            .section_postures()
            .iter()
            .find(|posture| posture.section() == section)
            .is_some_and(|posture| posture.enabled())
    })
}

fn reconcile_authority_readiness(
    status: WorthQueryDeclarationEntryReadinessStatus,
    reason: &'static str,
    mismatch: Option<WorthQueryDeclarationAuthorityAspectMismatch>,
    mapping_fit: Option<WorthQueryDeclarationAspectFit>,
) -> (WorthQueryDeclarationEntryReadinessStatus, &'static str) {
    if status != WorthQueryDeclarationEntryReadinessStatus::Admitted {
        return (status, reason);
    }
    if let Some(mismatch) = mismatch {
        return readiness_from_mismatch(mismatch);
    }
    if let Some(mapping_fit) = mapping_fit {
        return match mapping_fit {
            WorthQueryDeclarationAspectFit::Exact
            | WorthQueryDeclarationAspectFit::CompatibleSuperset => (status, reason),
            WorthQueryDeclarationAspectFit::MissingRequired => (
                WorthQueryDeclarationEntryReadinessStatus::Unsupported,
                "the retained envelope publication does not map every required bridge continuation aspect",
            ),
            WorthQueryDeclarationAspectFit::Partial => (
                WorthQueryDeclarationEntryReadinessStatus::Unsupported,
                "the retained envelope publication only partially maps into the required bridge continuation aspect slice",
            ),
            WorthQueryDeclarationAspectFit::Conflict => (
                WorthQueryDeclarationEntryReadinessStatus::Unsupported,
                "the retained envelope publication conflicts with the required bridge continuation aspect mapping",
            ),
        };
    }
    (status, reason)
}

fn readiness_from_mismatch(
    mismatch: WorthQueryDeclarationAuthorityAspectMismatch,
) -> (WorthQueryDeclarationEntryReadinessStatus, &'static str) {
    match mismatch {
        WorthQueryDeclarationAuthorityAspectMismatch::BasisAspectMismatch => (
            WorthQueryDeclarationEntryReadinessStatus::InvalidBasis,
            mismatch.reason(),
        ),
        WorthQueryDeclarationAuthorityAspectMismatch::MissingRequiredAspect
        | WorthQueryDeclarationAuthorityAspectMismatch::AspectConflict
        | WorthQueryDeclarationAuthorityAspectMismatch::AuthorityAspectGap
        | WorthQueryDeclarationAuthorityAspectMismatch::AuthorityAspectAmbiguity => (
            WorthQueryDeclarationEntryReadinessStatus::Unsupported,
            mismatch.reason(),
        ),
    }
}
