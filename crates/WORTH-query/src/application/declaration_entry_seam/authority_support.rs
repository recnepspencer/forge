use crate::application::{
    WorthQueryAdmittedConfiguredDomainHandle, WorthQueryDeclarationBridgeRoutingSupportReport,
    WorthQueryDeclarationBridgeRoutingSupportRow, WorthQueryDeclarationBridgeRoutingSupportStatus,
    WorthQueryDeclarationFamilyMarker, WorthQueryDeclarationInput,
    WorthQueryDeclarationRelationalRoutingSupportReport,
    WorthQueryDeclarationRelationalRoutingSupportRow,
    WorthQueryDeclarationRelationalTruthRoutingSupportStatus,
    WorthQueryDeclarationSignalCompatibilitySupportReport,
    WorthQueryDeclarationSignalCompatibilitySupportRow,
    WorthQueryDeclarationSignalCompatibilitySupportStatus,
    WorthQueryDeclarationSignalExecutionFamily, WorthQueryDomainEntryMarker,
    WorthQueryDomainOperatingContext,
};
use crate::basis_lifecycle::BasisFamily;

use super::{
    digest::derive_readiness_digest,
    readiness_projection::readiness_row_for_crossing,
    row::{crossing_rows_for_family, WorthQueryDeclarationEntryCrossingSurface},
    support::WorthQueryDeclarationEntryReadinessStatus,
};

pub(crate) fn worth_query_relational_routing_support_from_entry_readiness<
    D: WorthQueryDomainEntryMarker,
    C: WorthQueryDomainOperatingContext<D>,
    I: WorthQueryDeclarationInput<D>,
>(
    handle: &WorthQueryAdmittedConfiguredDomainHandle<D, C>,
) -> WorthQueryDeclarationRelationalRoutingSupportReport<D, I> {
    let mut rows = Vec::new();
    for crossing in crossing_rows_for_family::<D, C, I>(handle)
        .into_iter()
        .filter(|crossing| {
            matches!(
                crossing.surface(),
                WorthQueryDeclarationEntryCrossingSurface::RelationalTruthRouting
            )
        })
    {
        let row = readiness_row_for_crossing::<D, C, I>(crossing, handle, None);
        let crossing = row.crossing_row();
        let truth_claim = crossing
            .relational_truth_claim()
            .expect("relational rows must carry truth claim");
        let authority_family = crossing
            .relational_authority_family()
            .expect("relational rows must carry authority family");
        if rows.iter().any(
            |candidate: &WorthQueryDeclarationRelationalRoutingSupportRow| {
                candidate.truth_claim() == truth_claim
                    && candidate.authority_family() == authority_family
            },
        ) {
            continue;
        }
        let authority_summary = row
            .relational_authority_summary()
            .expect("relational readiness rows should expose relational authority summary");
        let status = map_relational_status(row.status());
        rows.push(WorthQueryDeclarationRelationalRoutingSupportRow::new(
            truth_claim,
            authority_family,
            authority_summary.aspect_contract().clone(),
            authority_summary.aspect_coverage().clone(),
            authority_summary.aspect_fit(),
            authority_summary.aspect_mismatch(),
            status,
            row.reason(),
        ));
    }
    WorthQueryDeclarationRelationalRoutingSupportReport::new(
        I::Family::semantic_family_key(),
        rows.clone(),
        digest_relational_rows(&rows),
    )
}

pub(crate) fn worth_query_bridge_routing_support_from_entry_readiness<
    D: WorthQueryDomainEntryMarker,
    C: WorthQueryDomainOperatingContext<D>,
    I: WorthQueryDeclarationInput<D>,
>(
    handle: &WorthQueryAdmittedConfiguredDomainHandle<D, C>,
) -> WorthQueryDeclarationBridgeRoutingSupportReport<D, I> {
    let mut rows = Vec::new();
    for crossing in crossing_rows_for_family::<D, C, I>(handle)
        .into_iter()
        .filter(|crossing| {
            matches!(
                crossing.surface(),
                WorthQueryDeclarationEntryCrossingSurface::BridgeContinuationRouting
            )
        })
    {
        let row = readiness_row_for_crossing::<D, C, I>(crossing, handle, None);
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
            .any(|candidate: &WorthQueryDeclarationBridgeRoutingSupportRow| {
                candidate.continuation_mode() == mode
                    && candidate.truth_context() == truth_context
                    && candidate.family() == family
            })
        {
            continue;
        }
        let authority_summary = row
            .bridge_authority_summary()
            .expect("bridge readiness rows should expose bridge authority summary");
        let status = map_bridge_status(row.status());
        rows.push(WorthQueryDeclarationBridgeRoutingSupportRow::new(
            mode,
            truth_context,
            family,
            authority_summary.aspect_contract().clone(),
            authority_summary.aspect_coverage().clone(),
            authority_summary.aspect_fit(),
            authority_summary.aspect_mismatch(),
            authority_summary.mapped_aspects().clone(),
            authority_summary.mapping_fit(),
            status,
            row.reason(),
        ));
    }
    WorthQueryDeclarationBridgeRoutingSupportReport::new(
        I::Family::semantic_family_key(),
        rows.clone(),
        digest_bridge_rows(&rows),
    )
}

pub(crate) fn worth_query_signal_compatibility_support_from_entry_readiness<
    D: WorthQueryDomainEntryMarker,
    C: WorthQueryDomainOperatingContext<D>,
    I: WorthQueryDeclarationInput<D>,
>(
    handle: &WorthQueryAdmittedConfiguredDomainHandle<D, C>,
) -> WorthQueryDeclarationSignalCompatibilitySupportReport<D, I> {
    let mut rows = Vec::new();
    for crossing in crossing_rows_for_family::<D, C, I>(handle)
        .into_iter()
        .filter(|crossing| {
            matches!(
                crossing.surface(),
                WorthQueryDeclarationEntryCrossingSurface::SignalCompatibility
            )
        })
    {
        let row = readiness_row_for_crossing::<D, C, I>(crossing, handle, None);
        let crossing = row.crossing_row();
        let execution_family = crossing
            .signal_execution_family()
            .unwrap_or(WorthQueryDeclarationSignalExecutionFamily::RuntimeDerivedExecution);
        let basis_family = crossing
            .basis_families()
            .first()
            .copied()
            .unwrap_or(BasisFamily::CurrentHead);
        if rows.iter().any(
            |candidate: &WorthQueryDeclarationSignalCompatibilitySupportRow| {
                candidate.execution_family() == execution_family
                    && candidate.basis_family() == basis_family
            },
        ) {
            continue;
        }
        let authority_summary = row
            .signal_authority_summary()
            .expect("signal readiness rows should expose signal authority summary");
        let status = map_signal_status(row.status());
        rows.push(WorthQueryDeclarationSignalCompatibilitySupportRow::new(
            execution_family,
            basis_family,
            authority_summary.dependency_aspects().clone(),
            authority_summary.produced_aspects().clone(),
            authority_summary.aspect_coverage().clone(),
            authority_summary.aspect_fit(),
            authority_summary.aspect_mismatch(),
            status,
            row.reason(),
        ));
    }
    WorthQueryDeclarationSignalCompatibilitySupportReport::new(
        I::Family::semantic_family_key(),
        rows.clone(),
        digest_signal_rows(&rows),
    )
}

fn map_relational_status(
    value: WorthQueryDeclarationEntryReadinessStatus,
) -> WorthQueryDeclarationRelationalTruthRoutingSupportStatus {
    match value {
        WorthQueryDeclarationEntryReadinessStatus::Admitted => {
            WorthQueryDeclarationRelationalTruthRoutingSupportStatus::Admitted
        }
        WorthQueryDeclarationEntryReadinessStatus::Deferred
        | WorthQueryDeclarationEntryReadinessStatus::Unsupported => {
            WorthQueryDeclarationRelationalTruthRoutingSupportStatus::Unsupported
        }
        WorthQueryDeclarationEntryReadinessStatus::InvalidBasis => {
            WorthQueryDeclarationRelationalTruthRoutingSupportStatus::InvalidContext
        }
    }
}

fn map_bridge_status(
    value: WorthQueryDeclarationEntryReadinessStatus,
) -> WorthQueryDeclarationBridgeRoutingSupportStatus {
    match value {
        WorthQueryDeclarationEntryReadinessStatus::Admitted => {
            WorthQueryDeclarationBridgeRoutingSupportStatus::Admitted
        }
        WorthQueryDeclarationEntryReadinessStatus::Deferred
        | WorthQueryDeclarationEntryReadinessStatus::Unsupported => {
            WorthQueryDeclarationBridgeRoutingSupportStatus::Unsupported
        }
        WorthQueryDeclarationEntryReadinessStatus::InvalidBasis => {
            WorthQueryDeclarationBridgeRoutingSupportStatus::InvalidContext
        }
    }
}

fn map_signal_status(
    value: WorthQueryDeclarationEntryReadinessStatus,
) -> WorthQueryDeclarationSignalCompatibilitySupportStatus {
    match value {
        WorthQueryDeclarationEntryReadinessStatus::Admitted => {
            WorthQueryDeclarationSignalCompatibilitySupportStatus::Admitted
        }
        WorthQueryDeclarationEntryReadinessStatus::Deferred => {
            WorthQueryDeclarationSignalCompatibilitySupportStatus::Deferred
        }
        WorthQueryDeclarationEntryReadinessStatus::Unsupported => {
            WorthQueryDeclarationSignalCompatibilitySupportStatus::Unsupported
        }
        WorthQueryDeclarationEntryReadinessStatus::InvalidBasis => {
            WorthQueryDeclarationSignalCompatibilitySupportStatus::InvalidBasis
        }
    }
}

fn digest_relational_rows(rows: &[WorthQueryDeclarationRelationalRoutingSupportRow]) -> String {
    derive_readiness_digest(
        &rows
            .iter()
            .map(|row| {
                format!(
                    "{}:{}:{:?}:{:?}:{}:{}:{}:{}",
                    row.truth_claim().as_str(),
                    row.authority_family().as_str(),
                    row.required_aspect_slice(),
                    row.available_aspect_slice(),
                    format!("{:?}", row.aspect_fit()),
                    format!("{:?}", row.aspect_mismatch()),
                    row.status().as_str(),
                    row.reason()
                )
            })
            .collect::<Vec<_>>(),
    )
}

fn digest_bridge_rows(rows: &[WorthQueryDeclarationBridgeRoutingSupportRow]) -> String {
    derive_readiness_digest(
        &rows
            .iter()
            .map(|row| {
                format!(
                    "{}:{}:{}:{:?}:{:?}:{}:{}:{:?}:{}:{}:{}",
                    row.continuation_mode().as_str(),
                    row.truth_context().as_str(),
                    row.family().as_str(),
                    row.required_aspect_slice(),
                    row.available_aspect_slice(),
                    format!("{:?}", row.aspect_fit()),
                    format!("{:?}", row.aspect_mismatch()),
                    row.mapped_aspect_slice(),
                    format!("{:?}", row.mapping_fit()),
                    row.status().as_str(),
                    row.reason()
                )
            })
            .collect::<Vec<_>>(),
    )
}

fn digest_signal_rows(rows: &[WorthQueryDeclarationSignalCompatibilitySupportRow]) -> String {
    derive_readiness_digest(
        &rows
            .iter()
            .map(|row| {
                format!(
                    "{}:{}:{:?}:{:?}:{:?}:{}:{}:{}:{}",
                    row.execution_family().as_str(),
                    row.basis_family().as_str(),
                    row.required_dependency_aspects(),
                    row.produced_aspects(),
                    row.available_aspect_slice(),
                    format!("{:?}", row.aspect_fit()),
                    format!("{:?}", row.aspect_mismatch()),
                    row.status().as_str(),
                    row.reason()
                )
            })
            .collect::<Vec<_>>(),
    )
}
