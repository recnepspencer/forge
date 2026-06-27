use std::path::PathBuf;

use forge_query::facade::consumer_kit::{
    ForgeQueryBoundaryAuditSourceSet, ForgeQuerySupportSnapshot,
};

use crate::workload_platform::evidence_lookup_inventory::EvidenceLookupQuerySurface;
use crate::workload_platform::evidence_lookup_query_surface_matrix::EvidenceLookupQuerySurfaceMatrixCloseout;

use super::super::boundary_audit::audit_evidence_lookup_query_hard_prohibitions_for_sources;
use super::super::counters::EvidenceLookupQueryConsumerKitCounters;
use super::super::error::{
    EvidenceLookupQueryConsumerKitError, EvidenceLookupQueryConsumerKitErrorKind,
};
use super::super::evidence_report::EvidenceLookupQueryConsumerKitEvidence;
use super::super::residue_audit::{
    assert_clean_consumer_residue, audit_evidence_lookup_query_consumer_residue_for_roots,
    residue_rows_from_report,
};
use super::super::source_set::{
    evidence_lookup_query_consumer_kit_boundary_sources,
    evidence_lookup_query_consumer_kit_residue_roots,
};
use super::super::support_pinning::{
    derived_support_requirements, evidence_lookup_query_support_pinning_contract,
    support_rows_from_snapshot,
};
use super::super::support_snapshot::project_evidence_lookup_query_support_snapshot;
use super::bindings::binding_rows_from_matrix;
use super::digest::closeout_digest;
use super::model::EvidenceLookupQueryConsumerKitCloseout;

pub(super) fn project_current_closeout(
    matrix: EvidenceLookupQuerySurfaceMatrixCloseout,
) -> Result<EvidenceLookupQueryConsumerKitCloseout, EvidenceLookupQueryConsumerKitError> {
    let support_snapshot =
        project_evidence_lookup_query_support_snapshot(&matrix).map_err(|error| {
            EvidenceLookupQueryConsumerKitError::new(
                EvidenceLookupQueryConsumerKitErrorKind::SupportPinning,
                format!("{error:?}"),
            )
        })?;
    evaluate_consumer_kit_closeout_from_parts(
        matrix,
        support_snapshot,
        evidence_lookup_query_consumer_kit_boundary_sources(),
        evidence_lookup_query_consumer_kit_residue_roots(),
    )
}

pub(crate) fn evaluate_consumer_kit_closeout_from_parts(
    matrix: EvidenceLookupQuerySurfaceMatrixCloseout,
    support_snapshot: ForgeQuerySupportSnapshot,
    boundary_sources: ForgeQueryBoundaryAuditSourceSet,
    residue_roots: Vec<PathBuf>,
) -> Result<EvidenceLookupQueryConsumerKitCloseout, EvidenceLookupQueryConsumerKitError> {
    let support_requirement_rows = derived_support_requirements(&matrix);
    let contract = evidence_lookup_query_support_pinning_contract(&support_snapshot, &matrix)
        .map_err(|error| {
            EvidenceLookupQueryConsumerKitError::new(
                EvidenceLookupQueryConsumerKitErrorKind::SupportPinning,
                format!("{error:?}"),
            )
        })?;
    let support_pin_report = contract
        .evaluate_snapshot(&support_snapshot)
        .map_err(|error| {
            EvidenceLookupQueryConsumerKitError::new(
                EvidenceLookupQueryConsumerKitErrorKind::SupportPinning,
                format!("{error:?}"),
            )
        })?;
    support_pin_report.assert_satisfied().map_err(|error| {
        EvidenceLookupQueryConsumerKitError::new(
            EvidenceLookupQueryConsumerKitErrorKind::SupportPinning,
            format!("{error:?}"),
        )
    })?;
    let boundary_audit = audit_evidence_lookup_query_hard_prohibitions_for_sources(
        boundary_sources,
    )
    .map_err(|error| {
        EvidenceLookupQueryConsumerKitError::new(
            EvidenceLookupQueryConsumerKitErrorKind::BoundaryAudit,
            format!("{error:?}"),
        )
    })?;
    boundary_audit.try_assert_clean().map_err(|failure| {
        EvidenceLookupQueryConsumerKitError::new(
            EvidenceLookupQueryConsumerKitErrorKind::BoundaryAudit,
            format!("{failure:?}"),
        )
    })?;
    let residue_report = audit_evidence_lookup_query_consumer_residue_for_roots(residue_roots)
        .map_err(|error| {
            EvidenceLookupQueryConsumerKitError::new(
                EvidenceLookupQueryConsumerKitErrorKind::ResidueAudit,
                format!("{error:?}"),
            )
        })?;
    assert_clean_consumer_residue(&residue_report)?;

    let evidence = EvidenceLookupQueryConsumerKitEvidence::declare(
        matrix.matrix_digest(),
        support_snapshot.snapshot_digest(),
        support_pin_report.report_digest(),
        boundary_audit
            .coverage_identity()
            .terminal_projection_for_reporting(),
        boundary_audit
            .report_identity()
            .terminal_projection_for_reporting(),
        residue_report
            .report_identity()
            .terminal_projection_for_reporting(),
    )?;
    let binding_rows = binding_rows_from_matrix(&matrix, support_pin_report.report_digest());
    if binding_rows.is_empty() {
        return Err(EvidenceLookupQueryConsumerKitError::new(
            EvidenceLookupQueryConsumerKitErrorKind::EmptyCloseout,
            "lookup query consumer kit closeout requires at least one query-owned matrix row",
        ));
    }
    let support_rows = support_rows_from_snapshot(
        &support_snapshot,
        &support_requirement_rows,
        &support_pin_report,
    )?;
    let query_residue_rows = residue_rows_from_report(&residue_report)?;
    let counters = EvidenceLookupQueryConsumerKitCounters::new(
        binding_rows.len(),
        binding_rows
            .iter()
            .filter(|row| row.query_surface() == EvidenceLookupQuerySurface::SupportPinning)
            .count(),
        support_rows.len(),
        query_residue_rows.len(),
        boundary_audit.findings().len(),
    );
    let closeout_digest = closeout_digest(
        &binding_rows,
        &support_rows,
        &query_residue_rows,
        &counters,
        matrix.matrix_digest(),
        support_snapshot.snapshot_digest(),
        contract.contract_digest(),
        support_pin_report.report_digest(),
        evidence.report_identity(),
        evidence.digest_participation_identity(),
        boundary_audit
            .coverage_identity()
            .terminal_projection_for_reporting(),
        boundary_audit
            .report_identity()
            .terminal_projection_for_reporting(),
        residue_report
            .report_identity()
            .terminal_projection_for_reporting(),
        residue_report.source_inventory_digest(),
    );

    Ok(EvidenceLookupQueryConsumerKitCloseout {
        query_surface_matrix_digest: matrix.matrix_digest().to_string(),
        support_snapshot_digest: support_snapshot.snapshot_digest().to_string(),
        support_pin_contract_digest: contract.contract_digest().to_string(),
        support_pin_report_digest: support_pin_report.report_digest().to_string(),
        evidence_report_identity: evidence.report_identity().to_string(),
        evidence_digest_participation_identity: evidence
            .digest_participation_identity()
            .to_string(),
        boundary_audit_coverage_identity: boundary_audit
            .coverage_identity()
            .terminal_projection_for_reporting()
            .to_string(),
        boundary_audit_report_identity: boundary_audit
            .report_identity()
            .terminal_projection_for_reporting()
            .to_string(),
        consumer_residue_report_identity: residue_report
            .report_identity()
            .terminal_projection_for_reporting()
            .to_string(),
        consumer_residue_source_inventory_digest: residue_report
            .source_inventory_digest()
            .to_string(),
        binding_rows,
        support_requirement_rows,
        support_rows,
        query_residue_rows,
        counters,
        closeout_digest,
    })
}
