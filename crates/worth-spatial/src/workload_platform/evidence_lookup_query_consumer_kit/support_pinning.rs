use forge_query::facade::consumer_kit::{
    support_pinning_contract, ForgeQueryPinnedSupportStatus, ForgeQueryPinnedTeachingPosture,
    ForgeQuerySupportPinContract, ForgeQuerySupportPinReport, ForgeQuerySupportPinningError,
    ForgeQuerySupportSnapshot,
};
use forge_query::facade::runtime::ForgeQueryRuntimeFacadeFamily;
use std::collections::BTreeSet;

use crate::workload_platform::evidence_lookup_inventory::EvidenceLookupQuerySurface;
use crate::workload_platform::evidence_lookup_query_surface_matrix::{
    EvidenceLookupQuerySurfaceMatrixCloseout, EvidenceLookupQuerySurfaceMatrixRow,
};

use super::error::{EvidenceLookupQueryConsumerKitError, EvidenceLookupQueryConsumerKitErrorKind};
use super::requirement_row::EvidenceLookupQuerySupportRequirementRow;
use super::row::EvidenceLookupQuerySupportPinRow;

const LOOKUP_SUPPORT_PIN_CONSUMER_NAME: &str = "worth-spatial.evidence-lookup.phase-ten";

pub(crate) fn evidence_lookup_query_support_pinning_contract(
    snapshot: &ForgeQuerySupportSnapshot,
    matrix: &EvidenceLookupQuerySurfaceMatrixCloseout,
) -> Result<ForgeQuerySupportPinContract, ForgeQuerySupportPinningError> {
    let mut builder =
        support_pinning_contract(LOOKUP_SUPPORT_PIN_CONSUMER_NAME).against_snapshot(snapshot)?;

    for family in derived_support_runtime_families(matrix) {
        builder = builder.require_family(family, |row| {
            row.status(ForgeQueryPinnedSupportStatus::Supported)
                .teaching_posture(ForgeQueryPinnedTeachingPosture::OrdinaryRuntimeDx)
                .bind_live_row_digest()
        })?;
    }

    builder.seal()
}

pub(crate) fn derived_support_requirements(
    matrix: &EvidenceLookupQuerySurfaceMatrixCloseout,
) -> Vec<EvidenceLookupQuerySupportRequirementRow> {
    support_pinning_matrix_rows(matrix)
        .filter_map(|row| {
            support_requirement_family_for_row(row).map(|runtime_family| {
                EvidenceLookupQuerySupportRequirementRow::new(
                    runtime_family,
                    row.touchpoint(),
                    row.query_surface(),
                )
            })
        })
        .collect()
}

fn derived_support_runtime_families(
    matrix: &EvidenceLookupQuerySurfaceMatrixCloseout,
) -> Vec<ForgeQueryRuntimeFacadeFamily> {
    let mut seen = BTreeSet::new();
    derived_support_requirements(matrix)
        .into_iter()
        .filter_map(|row| {
            seen.insert(row.runtime_family())
                .then_some(row.runtime_family())
        })
        .collect()
}

pub(crate) fn support_rows_from_snapshot(
    snapshot: &ForgeQuerySupportSnapshot,
    requirements: &[EvidenceLookupQuerySupportRequirementRow],
    report: &ForgeQuerySupportPinReport,
) -> Result<Vec<EvidenceLookupQuerySupportPinRow>, EvidenceLookupQueryConsumerKitError> {
    requirements
        .iter()
        .map(|requirement| {
            let family = requirement.runtime_family();
            let label = family.as_str();
            let snapshot_row = snapshot
                .rows()
                .iter()
                .find(|row| row.facade_family() == Some(label))
                .ok_or_else(|| {
                    EvidenceLookupQueryConsumerKitError::new(
                        EvidenceLookupQueryConsumerKitErrorKind::MissingSupportPinRuntimeFamily,
                        format!("missing live support snapshot row for runtime family `{label}`"),
                    )
                })?;
            Ok(EvidenceLookupQuerySupportPinRow::new(
                family,
                requirement.touchpoint(),
                requirement.query_surface(),
                snapshot_row.surface(),
                snapshot_row.snapshot_row_digest(),
                report.report_digest(),
            ))
        })
        .collect()
}

fn support_pinning_matrix_rows(
    matrix: &EvidenceLookupQuerySurfaceMatrixCloseout,
) -> impl Iterator<Item = &EvidenceLookupQuerySurfaceMatrixRow> {
    matrix
        .rows()
        .iter()
        .filter(|row| row.query_surface() == EvidenceLookupQuerySurface::SupportPinning)
}

fn support_requirement_family_for_row(
    row: &EvidenceLookupQuerySurfaceMatrixRow,
) -> Option<ForgeQueryRuntimeFacadeFamily> {
    (row.query_surface() == EvidenceLookupQuerySurface::SupportPinning)
        .then_some(ForgeQueryRuntimeFacadeFamily::Read)
}
