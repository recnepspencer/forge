use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryCapabilityFamily,
    ForgeQueryCapabilityStatus, ForgeQueryConfigSectionFamily, ForgeQueryDeclarationAspectFit,
    ForgeQueryDeclarationAuthorityAspectMismatch, ForgeQueryDeclarationCapabilityStatus,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationFamilySupportReport,
    ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
    ForgeQueryLowerAuthorityRouteFamily, ForgeQuerySignalCompatibilityPosture,
};

use super::{
    row::{ForgeQueryDeclarationEntryCrossingRow, ForgeQueryDeclarationEntryCrossingSurface},
    support::{
        ForgeQueryDeclarationEntryReadinessRow, ForgeQueryDeclarationEntryReadinessStatus,
        ReadinessRetainedPosture,
    },
};

pub(crate) fn readiness_row_for_crossing<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    row: ForgeQueryDeclarationEntryCrossingRow,
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    retained_posture: Option<&ReadinessRetainedPosture>,
) -> ForgeQueryDeclarationEntryReadinessRow {
    let family_support = handle.family_support::<I::Family>();
    let envelope_aspect_publication =
        retained_posture.map(|posture| posture.envelope_aspect_publication.clone());
    match row.surface() {
        ForgeQueryDeclarationEntryCrossingSurface::Envelope => {
            envelope_row(row, &family_support, envelope_aspect_publication)
        }
        ForgeQueryDeclarationEntryCrossingSurface::RelationalTruthRouting => {
            let (status, reason) = relational_status::<D, C, I>(handle);
            let relational_summary = retained_posture
                .map(|posture| posture.relational_authority_summary.clone())
                .unwrap_or_else(|| {
                    crate::application::relational_authority_summary_from_coverage(
                        &I::Family::aspect_contract(),
                        I::Family::aspect_coverage(),
                        crate::application::ForgeQueryDeclarationAspectCoverageBasis::DeclaredFamilyCoverage,
                        I::Family::relational_truth_contract().as_ref(),
                    )
                });
            let (status, reason) = reconcile_authority_readiness(
                status,
                reason,
                relational_summary.aspect_mismatch(),
                None,
            );
            ForgeQueryDeclarationEntryReadinessRow::new(
                row,
                status,
                reason,
                envelope_aspect_publication,
                Some(relational_summary),
                None,
                None,
            )
        }
        ForgeQueryDeclarationEntryCrossingSurface::BridgeContinuationRouting => {
            let (status, reason) = bridge_status::<D, C, I>(handle);
            let bridge_summary = retained_posture
                .map(|posture| posture.bridge_authority_summary.clone())
                .unwrap_or_else(|| {
                    crate::application::bridge_authority_summary_from_coverage(
                        &I::Family::aspect_contract(),
                        I::Family::aspect_coverage(),
                        crate::application::ForgeQueryDeclarationAspectCoverageBasis::DeclaredFamilyCoverage,
                        I::Family::bridge_continuation_contract().as_ref(),
                    )
                });
            let (status, reason) = reconcile_authority_readiness(
                status,
                reason,
                bridge_summary.aspect_mismatch(),
                Some(bridge_summary.mapping_fit()),
            );
            ForgeQueryDeclarationEntryReadinessRow::new(
                row,
                status,
                reason,
                envelope_aspect_publication,
                None,
                Some(bridge_summary),
                None,
            )
        }
        ForgeQueryDeclarationEntryCrossingSurface::SignalCompatibility => {
            let (status, reason) = signal_status::<D, C, I>(handle);
            let signal_summary = retained_posture
                .map(|posture| posture.signal_authority_summary.clone())
                .unwrap_or_else(|| {
                    crate::application::signal_authority_summary_from_coverage(
                        &I::Family::aspect_contract(),
                        I::Family::aspect_coverage(),
                        crate::application::ForgeQueryDeclarationAspectCoverageBasis::DeclaredFamilyCoverage,
                        I::Family::signal_compatibility_contract().as_ref(),
                    )
                });
            let (status, reason) = reconcile_authority_readiness(
                status,
                reason,
                signal_summary.aspect_mismatch(),
                None,
            );
            ForgeQueryDeclarationEntryReadinessRow::new(
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

fn envelope_row<D: ForgeQueryDomainEntryMarker, F: ForgeQueryDeclarationFamilyMarker<D>>(
    row: ForgeQueryDeclarationEntryCrossingRow,
    family_support: &ForgeQueryDeclarationFamilySupportReport<D, F>,
    envelope_aspect_publication: Option<crate::application::ForgeQueryDeclarationAspectPublication>,
) -> ForgeQueryDeclarationEntryReadinessRow {
    let status = match family_support.declare_status() {
        ForgeQueryDeclarationCapabilityStatus::Admitted => {
            ForgeQueryDeclarationEntryReadinessStatus::Admitted
        }
        ForgeQueryDeclarationCapabilityStatus::DeferredDebt => {
            ForgeQueryDeclarationEntryReadinessStatus::Deferred
        }
        ForgeQueryDeclarationCapabilityStatus::Unsupported => {
            ForgeQueryDeclarationEntryReadinessStatus::Unsupported
        }
        ForgeQueryDeclarationCapabilityStatus::InvalidContext => {
            ForgeQueryDeclarationEntryReadinessStatus::InvalidBasis
        }
    };
    let reason = family_support
        .row(crate::application::ForgeQueryDeclarationCapabilityVerb::Declare)
        .expect("declare row must exist")
        .reason();
    ForgeQueryDeclarationEntryReadinessRow::new(
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
fn relational_status<D: ForgeQueryDomainEntryMarker, C: ForgeQueryDomainOperatingContext<D>, I: ForgeQueryDeclarationInput<D>>(handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>) -> (ForgeQueryDeclarationEntryReadinessStatus, &'static str) {
    match I::Family::relational_truth_contract() {
        Some(contract) => {
            if !I::Family::route_contract().allowed_route_families().contains(&ForgeQueryLowerAuthorityRouteFamily::Relational) { return (ForgeQueryDeclarationEntryReadinessStatus::Unsupported, "route planning does not currently admit a relational slice for this family"); }
            if !config_sections_enabled(handle, contract.required_config_sections()) { return (ForgeQueryDeclarationEntryReadinessStatus::InvalidBasis, "required relational config sections must be enabled before relational truth routing"); }
            if !capabilities_admitted(handle, contract.required_capability_families()) { return (ForgeQueryDeclarationEntryReadinessStatus::Unsupported, "required relational capability families are not admitted in this operating world"); }
            (ForgeQueryDeclarationEntryReadinessStatus::Admitted, contract.reason())
        }
        None => (ForgeQueryDeclarationEntryReadinessStatus::Unsupported, "family does not expose a relational truth-routing contract"),
    }
}

#[rustfmt::skip]
fn bridge_status<D: ForgeQueryDomainEntryMarker, C: ForgeQueryDomainOperatingContext<D>, I: ForgeQueryDeclarationInput<D>>(handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>) -> (ForgeQueryDeclarationEntryReadinessStatus, &'static str) {
    match I::Family::bridge_continuation_contract() {
        Some(contract) => {
            if !I::Family::route_contract().allowed_route_families().contains(&ForgeQueryLowerAuthorityRouteFamily::Bridge) { return (ForgeQueryDeclarationEntryReadinessStatus::Unsupported, "route planning does not currently admit a bridge slice for this family"); }
            if !config_sections_enabled(handle, contract.required_config_sections()) { return (ForgeQueryDeclarationEntryReadinessStatus::InvalidBasis, "required runtime-bridge config sections must be enabled before bridge continuation routing"); }
            if !capabilities_admitted(handle, contract.required_capability_families()) { return (ForgeQueryDeclarationEntryReadinessStatus::Unsupported, "required bridge continuation capability families are not admitted in this operating world"); }
            (ForgeQueryDeclarationEntryReadinessStatus::Admitted, contract.reason())
        }
        None => (ForgeQueryDeclarationEntryReadinessStatus::Unsupported, "family does not expose a bridge continuation-routing contract"),
    }
}

#[rustfmt::skip]
fn signal_status<D: ForgeQueryDomainEntryMarker, C: ForgeQueryDomainOperatingContext<D>, I: ForgeQueryDeclarationInput<D>>(handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>) -> (ForgeQueryDeclarationEntryReadinessStatus, &'static str) {
    match I::Family::taxonomy().signal_compatibility() {
        ForgeQuerySignalCompatibilityPosture::Compatible => match I::Family::signal_compatibility_contract() {
            Some(contract) => {
                if !config_sections_enabled(handle, contract.required_config_sections()) { return (ForgeQueryDeclarationEntryReadinessStatus::InvalidBasis, "required signal config sections must be enabled before signal compatibility is admitted"); }
                if !capabilities_admitted(handle, contract.required_capability_families()) { return (ForgeQueryDeclarationEntryReadinessStatus::InvalidBasis, "required signal capability families are not admitted for this basis-sensitive continuation"); }
                (ForgeQueryDeclarationEntryReadinessStatus::Admitted, contract.reason())
            }
            None => (ForgeQueryDeclarationEntryReadinessStatus::Unsupported, "family claims signal compatibility but does not expose a signal compatibility contract"),
        },
        ForgeQuerySignalCompatibilityPosture::Deferred => (ForgeQueryDeclarationEntryReadinessStatus::Deferred, "signal compatibility for this family remains explicitly deferred"),
        ForgeQuerySignalCompatibilityPosture::NotCompatible => (ForgeQueryDeclarationEntryReadinessStatus::Unsupported, "family is not structurally signal-compatible"),
    }
}

fn capabilities_admitted<D: ForgeQueryDomainEntryMarker, C: ForgeQueryDomainOperatingContext<D>>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    required: &'static [ForgeQueryCapabilityFamily],
) -> bool {
    required.iter().copied().all(|family| {
        handle.support_snapshot().capability_status(family)
            == Some(ForgeQueryCapabilityStatus::Admitted)
    })
}

fn config_sections_enabled<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    required: &'static [ForgeQueryConfigSectionFamily],
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
    status: ForgeQueryDeclarationEntryReadinessStatus,
    reason: &'static str,
    mismatch: Option<ForgeQueryDeclarationAuthorityAspectMismatch>,
    mapping_fit: Option<ForgeQueryDeclarationAspectFit>,
) -> (ForgeQueryDeclarationEntryReadinessStatus, &'static str) {
    if status != ForgeQueryDeclarationEntryReadinessStatus::Admitted {
        return (status, reason);
    }
    if let Some(mismatch) = mismatch {
        return readiness_from_mismatch(mismatch);
    }
    if let Some(mapping_fit) = mapping_fit {
        return match mapping_fit {
            ForgeQueryDeclarationAspectFit::Exact
            | ForgeQueryDeclarationAspectFit::CompatibleSuperset => (status, reason),
            ForgeQueryDeclarationAspectFit::MissingRequired => (
                ForgeQueryDeclarationEntryReadinessStatus::Unsupported,
                "the retained envelope publication does not map every required bridge continuation aspect",
            ),
            ForgeQueryDeclarationAspectFit::Partial => (
                ForgeQueryDeclarationEntryReadinessStatus::Unsupported,
                "the retained envelope publication only partially maps into the required bridge continuation aspect slice",
            ),
            ForgeQueryDeclarationAspectFit::Conflict => (
                ForgeQueryDeclarationEntryReadinessStatus::Unsupported,
                "the retained envelope publication conflicts with the required bridge continuation aspect mapping",
            ),
        };
    }
    (status, reason)
}

fn readiness_from_mismatch(
    mismatch: ForgeQueryDeclarationAuthorityAspectMismatch,
) -> (ForgeQueryDeclarationEntryReadinessStatus, &'static str) {
    match mismatch {
        ForgeQueryDeclarationAuthorityAspectMismatch::BasisAspectMismatch => (
            ForgeQueryDeclarationEntryReadinessStatus::InvalidBasis,
            mismatch.reason(),
        ),
        ForgeQueryDeclarationAuthorityAspectMismatch::MissingRequiredAspect
        | ForgeQueryDeclarationAuthorityAspectMismatch::AspectConflict
        | ForgeQueryDeclarationAuthorityAspectMismatch::AuthorityAspectGap
        | ForgeQueryDeclarationAuthorityAspectMismatch::AuthorityAspectAmbiguity => (
            ForgeQueryDeclarationEntryReadinessStatus::Unsupported,
            mismatch.reason(),
        ),
    }
}
