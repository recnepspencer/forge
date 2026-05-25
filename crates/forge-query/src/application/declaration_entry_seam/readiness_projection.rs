use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryCapabilityFamily,
    ForgeQueryCapabilityStatus, ForgeQueryConfigSectionFamily,
    ForgeQueryDeclarationBridgeRoutingSupportReport, ForgeQueryDeclarationBridgeRoutingSupportRow,
    ForgeQueryDeclarationBridgeRoutingSupportStatus, ForgeQueryDeclarationCapabilityStatus,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationFamilySupportReport,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationRelationalRoutingSupportReport,
    ForgeQueryDeclarationRelationalRoutingSupportRow,
    ForgeQueryDeclarationRelationalTruthRoutingSupportStatus,
    ForgeQueryDeclarationSignalCompatibilitySupportReport,
    ForgeQueryDeclarationSignalCompatibilitySupportRow,
    ForgeQueryDeclarationSignalCompatibilitySupportStatus,
    ForgeQueryDeclarationSignalExecutionFamily, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext, ForgeQueryLowerAuthorityRouteFamily,
    ForgeQuerySignalCompatibilityPosture,
};
use crate::basis_lifecycle::BasisFamily;

use super::{
    digest::derive_readiness_digest,
    row::{
        crossing_rows_for_family, ForgeQueryDeclarationEntryCrossingRow,
        ForgeQueryDeclarationEntryCrossingSurface,
    },
    support::{ForgeQueryDeclarationEntryReadinessRow, ForgeQueryDeclarationEntryReadinessStatus},
};

pub(crate) fn readiness_row_for_crossing<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    row: ForgeQueryDeclarationEntryCrossingRow,
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
) -> ForgeQueryDeclarationEntryReadinessRow {
    let family_support = handle.family_support::<I::Family>();
    match row.surface() {
        ForgeQueryDeclarationEntryCrossingSurface::Envelope => envelope_row(row, &family_support),
        ForgeQueryDeclarationEntryCrossingSurface::RelationalTruthRouting => {
            let (status, reason) = relational_status::<D, C, I>(handle);
            ForgeQueryDeclarationEntryReadinessRow::new(row, status, reason)
        }
        ForgeQueryDeclarationEntryCrossingSurface::BridgeContinuationRouting => {
            let (status, reason) = bridge_status::<D, C, I>(handle);
            ForgeQueryDeclarationEntryReadinessRow::new(row, status, reason)
        }
        ForgeQueryDeclarationEntryCrossingSurface::SignalCompatibility => {
            let (status, reason) = signal_status::<D, C, I>(handle);
            ForgeQueryDeclarationEntryReadinessRow::new(row, status, reason)
        }
    }
}

fn envelope_row<D: ForgeQueryDomainEntryMarker, F: ForgeQueryDeclarationFamilyMarker<D>>(
    row: ForgeQueryDeclarationEntryCrossingRow,
    family_support: &ForgeQueryDeclarationFamilySupportReport<D, F>,
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
    ForgeQueryDeclarationEntryReadinessRow::new(row, status, reason)
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

pub(crate) fn forge_query_relational_routing_support_from_entry_readiness<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
) -> ForgeQueryDeclarationRelationalRoutingSupportReport<D, I> {
    let mut rows = Vec::new();
    for crossing in crossing_rows_for_family::<D, C, I>(handle)
        .into_iter()
        .filter(|crossing| {
            matches!(
                crossing.surface(),
                ForgeQueryDeclarationEntryCrossingSurface::RelationalTruthRouting
            )
        })
    {
        let row = readiness_row_for_crossing::<D, C, I>(crossing, handle);
        let crossing = row.crossing_row();
        let truth_claim = crossing
            .relational_truth_claim()
            .expect("relational rows must carry truth claim");
        let authority_family = crossing
            .relational_authority_family()
            .expect("relational rows must carry authority family");
        if rows.iter().any(
            |candidate: &ForgeQueryDeclarationRelationalRoutingSupportRow| {
                candidate.truth_claim() == truth_claim
                    && candidate.authority_family() == authority_family
            },
        ) {
            continue;
        }
        rows.push(ForgeQueryDeclarationRelationalRoutingSupportRow::new(
            truth_claim,
            authority_family,
            map_relational_status(row.status()),
            row.reason(),
        ));
    }
    ForgeQueryDeclarationRelationalRoutingSupportReport::new(
        I::Family::semantic_family_key(),
        rows.clone(),
        digest_relational_rows(&rows),
    )
}

pub(crate) fn forge_query_bridge_routing_support_from_entry_readiness<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
) -> ForgeQueryDeclarationBridgeRoutingSupportReport<D, I> {
    let mut rows = Vec::new();
    for crossing in crossing_rows_for_family::<D, C, I>(handle)
        .into_iter()
        .filter(|crossing| {
            matches!(
                crossing.surface(),
                ForgeQueryDeclarationEntryCrossingSurface::BridgeContinuationRouting
            )
        })
    {
        let row = readiness_row_for_crossing::<D, C, I>(crossing, handle);
        let crossing = row.crossing_row();
        let mode = crossing
            .bridge_continuation_mode()
            .expect("bridge rows must carry continuation mode");
        let truth_context = crossing
            .bridge_truth_context()
            .expect("bridge rows must carry truth context");
        let family = crossing
            .bridge_continuation_family()
            .expect("bridge rows must carry continuation family");
        if rows
            .iter()
            .any(|candidate: &ForgeQueryDeclarationBridgeRoutingSupportRow| {
                candidate.continuation_mode() == mode
                    && candidate.truth_context() == truth_context
                    && candidate.family() == family
            })
        {
            continue;
        }
        rows.push(ForgeQueryDeclarationBridgeRoutingSupportRow::new(
            mode,
            truth_context,
            family,
            map_bridge_status(row.status()),
            row.reason(),
        ));
    }
    ForgeQueryDeclarationBridgeRoutingSupportReport::new(
        I::Family::semantic_family_key(),
        rows.clone(),
        digest_bridge_rows(&rows),
    )
}

pub(crate) fn forge_query_signal_compatibility_support_from_entry_readiness<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
) -> ForgeQueryDeclarationSignalCompatibilitySupportReport<D, I> {
    let mut rows = Vec::new();
    for crossing in crossing_rows_for_family::<D, C, I>(handle)
        .into_iter()
        .filter(|crossing| {
            matches!(
                crossing.surface(),
                ForgeQueryDeclarationEntryCrossingSurface::SignalCompatibility
            )
        })
    {
        let row = readiness_row_for_crossing::<D, C, I>(crossing, handle);
        let crossing = row.crossing_row();
        let execution_family = crossing
            .signal_execution_family()
            .unwrap_or(ForgeQueryDeclarationSignalExecutionFamily::RuntimeDerivedExecution);
        let basis_family = crossing
            .basis_families()
            .first()
            .copied()
            .unwrap_or(BasisFamily::CurrentHead);
        if rows.iter().any(
            |candidate: &ForgeQueryDeclarationSignalCompatibilitySupportRow| {
                candidate.execution_family() == execution_family
                    && candidate.basis_family() == basis_family
            },
        ) {
            continue;
        }
        rows.push(ForgeQueryDeclarationSignalCompatibilitySupportRow::new(
            execution_family,
            basis_family,
            map_signal_status(row.status()),
            row.reason(),
        ));
    }
    ForgeQueryDeclarationSignalCompatibilitySupportReport::new(
        I::Family::semantic_family_key(),
        rows.clone(),
        digest_signal_rows(&rows),
    )
}

fn map_relational_status(
    value: ForgeQueryDeclarationEntryReadinessStatus,
) -> ForgeQueryDeclarationRelationalTruthRoutingSupportStatus {
    match value {
        ForgeQueryDeclarationEntryReadinessStatus::Admitted => {
            ForgeQueryDeclarationRelationalTruthRoutingSupportStatus::Admitted
        }
        ForgeQueryDeclarationEntryReadinessStatus::Deferred
        | ForgeQueryDeclarationEntryReadinessStatus::Unsupported => {
            ForgeQueryDeclarationRelationalTruthRoutingSupportStatus::Unsupported
        }
        ForgeQueryDeclarationEntryReadinessStatus::InvalidBasis => {
            ForgeQueryDeclarationRelationalTruthRoutingSupportStatus::InvalidContext
        }
    }
}

fn map_bridge_status(
    value: ForgeQueryDeclarationEntryReadinessStatus,
) -> ForgeQueryDeclarationBridgeRoutingSupportStatus {
    match value {
        ForgeQueryDeclarationEntryReadinessStatus::Admitted => {
            ForgeQueryDeclarationBridgeRoutingSupportStatus::Admitted
        }
        ForgeQueryDeclarationEntryReadinessStatus::Deferred
        | ForgeQueryDeclarationEntryReadinessStatus::Unsupported => {
            ForgeQueryDeclarationBridgeRoutingSupportStatus::Unsupported
        }
        ForgeQueryDeclarationEntryReadinessStatus::InvalidBasis => {
            ForgeQueryDeclarationBridgeRoutingSupportStatus::InvalidContext
        }
    }
}

fn map_signal_status(
    value: ForgeQueryDeclarationEntryReadinessStatus,
) -> ForgeQueryDeclarationSignalCompatibilitySupportStatus {
    match value {
        ForgeQueryDeclarationEntryReadinessStatus::Admitted => {
            ForgeQueryDeclarationSignalCompatibilitySupportStatus::Admitted
        }
        ForgeQueryDeclarationEntryReadinessStatus::Deferred => {
            ForgeQueryDeclarationSignalCompatibilitySupportStatus::Deferred
        }
        ForgeQueryDeclarationEntryReadinessStatus::Unsupported => {
            ForgeQueryDeclarationSignalCompatibilitySupportStatus::Unsupported
        }
        ForgeQueryDeclarationEntryReadinessStatus::InvalidBasis => {
            ForgeQueryDeclarationSignalCompatibilitySupportStatus::InvalidBasis
        }
    }
}

fn digest_relational_rows(rows: &[ForgeQueryDeclarationRelationalRoutingSupportRow]) -> String {
    derive_readiness_digest(
        &rows
            .iter()
            .map(|row| {
                format!(
                    "{}:{}:{}:{}",
                    row.truth_claim().as_str(),
                    row.authority_family().as_str(),
                    row.status().as_str(),
                    row.reason()
                )
            })
            .collect::<Vec<_>>(),
    )
}
fn digest_bridge_rows(rows: &[ForgeQueryDeclarationBridgeRoutingSupportRow]) -> String {
    derive_readiness_digest(
        &rows
            .iter()
            .map(|row| {
                format!(
                    "{}:{}:{}:{}:{}",
                    row.continuation_mode().as_str(),
                    row.truth_context().as_str(),
                    row.family().as_str(),
                    row.status().as_str(),
                    row.reason()
                )
            })
            .collect::<Vec<_>>(),
    )
}
fn digest_signal_rows(rows: &[ForgeQueryDeclarationSignalCompatibilitySupportRow]) -> String {
    derive_readiness_digest(
        &rows
            .iter()
            .map(|row| {
                format!(
                    "{}:{}:{}:{}",
                    row.execution_family().as_str(),
                    row.basis_family().as_str(),
                    row.status().as_str(),
                    row.reason()
                )
            })
            .collect::<Vec<_>>(),
    )
}
