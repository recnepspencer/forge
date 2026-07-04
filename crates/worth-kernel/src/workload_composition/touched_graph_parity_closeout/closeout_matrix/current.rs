use std::collections::{BTreeMap, BTreeSet};

use schema::facade::platform::authority::touched_graph_parity_closeout::{
    TouchedGraphParityFamilyKind, TouchedGraphParityReadinessInput,
};

use super::matrix::WorthTouchedGraphCrossFamilyCloseoutMatrix;
use super::row::WorthTouchedGraphCrossFamilyCloseoutMatrixRow;
use super::validation::{
    validate_closeout_matrix, CloseoutMatrixAuthority, WorthTouchedGraphCrossFamilyCloseoutMatrixError,
    WorthTouchedGraphCrossFamilyCloseoutMatrixErrorKind,
};
use crate::workload_composition::planner_owned_routing::current_worth_touched_graph_conflict_public_closeout;
use crate::workload_composition::public_closeout::WorthTouchedGraphConflictDeletionAlignmentRow;
use crate::workload_composition::touched_graph_parity_closeout::{
    current_conflict_family_parity_claim, current_live_coverage_ledger,
    current_public_projection_parity_claim,
    current_replay_undo_family_parity_claim, current_representative_selected_route_parity_path,
    current_reuse_family_parity_claim, current_spatial_family_parity_claim,
    current_topology_family_declare_once_parity_claim, ArchitectureClaimLedgerRowKind,
    LiveCoverageLedger,
};

pub fn current_worth_touched_graph_cross_family_closeout_matrix(
) -> Result<WorthTouchedGraphCrossFamilyCloseoutMatrix, WorthTouchedGraphCrossFamilyCloseoutMatrixError>
{
    let live_ledger = current_live_coverage_ledger().map_err(|_| {
        WorthTouchedGraphCrossFamilyCloseoutMatrixError::new(
            WorthTouchedGraphCrossFamilyCloseoutMatrixErrorKind::CurrentLiveCoverageLedgerUnavailable,
            "cross-family closeout matrix requires the current live coverage ledger",
        )
    })?;
    let representative_path = current_representative_selected_route_parity_path().map_err(
        |error| {
            WorthTouchedGraphCrossFamilyCloseoutMatrixError::new(
                WorthTouchedGraphCrossFamilyCloseoutMatrixErrorKind::CurrentRepresentativePathUnavailable,
                error.detail(),
            )
        },
    )?;
    let readiness = current_readiness_input().map_err(|error| {
        WorthTouchedGraphCrossFamilyCloseoutMatrixError::new(error.kind(), error.detail())
    })?;
    let closeout = current_worth_touched_graph_conflict_public_closeout().map_err(|error| {
        WorthTouchedGraphCrossFamilyCloseoutMatrixError::new(
            WorthTouchedGraphCrossFamilyCloseoutMatrixErrorKind::CurrentPublicCloseoutUnavailable,
            error.detail(),
        )
    })?;
    closeout_matrix_from_authorities(&live_ledger, &readiness, &representative_path, &closeout)
}

pub(crate) fn closeout_matrix_from_authorities(
    live_ledger: &LiveCoverageLedger,
    readiness: &TouchedGraphParityReadinessInput,
    representative_path: &crate::workload_composition::RepresentativeSelectedRouteParityPath,
    closeout: &crate::workload_composition::planner_owned_routing::WorthTouchedGraphConflictPublicCloseout,
) -> Result<WorthTouchedGraphCrossFamilyCloseoutMatrix, WorthTouchedGraphCrossFamilyCloseoutMatrixError>
{
    let authority = current_matrix_authority(
        &live_ledger,
        &readiness,
        &representative_path,
        closeout.architecture_alignment_report().deleted_authority_rows(),
    )?;
    let representative_path_coverage = representative_path.covered_family_kinds();
    let readiness_coverage = readiness
        .representative_family_coverage()
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    let rows = TouchedGraphParityFamilyKind::ALL
        .into_iter()
        .map(|family_kind| {
            WorthTouchedGraphCrossFamilyCloseoutMatrixRow::new(
                family_kind,
                authority.covered_counts.get(&family_kind).copied().unwrap_or(0),
                representative_path_coverage.contains(&family_kind)
                    && !representative_path.selected_route_identity_digest().is_empty(),
                authority.family_parity_coverage.contains(&family_kind),
                authority.public_proof_parity_passed,
                authority.diagnostic_parity_passed,
                readiness_coverage.contains(&family_kind)
                    && readiness.architecture_claim_digest()
                        == live_ledger.closeout_architecture_claim_digest(),
                authority.deleted_counts.get(&family_kind).copied().unwrap_or(0),
                authority
                    .capped_residue_counts
                    .get(&family_kind)
                    .copied()
                    .unwrap_or(0),
                authority.query_gap_counts.get(&family_kind).copied().unwrap_or(0),
                authority.blocked_counts.get(&family_kind).copied().unwrap_or(0),
            )
        })
        .collect::<Vec<_>>();
    let matrix = WorthTouchedGraphCrossFamilyCloseoutMatrix::new(
        rows,
        live_ledger.closeout_architecture_claim_digest(),
    );
    validate_closeout_matrix(&matrix, &authority)?;
    Ok(matrix)
}

pub(crate) fn current_matrix_authority(
    live_ledger: &LiveCoverageLedger,
    readiness: &TouchedGraphParityReadinessInput,
    representative_path: &crate::workload_composition::RepresentativeSelectedRouteParityPath,
    deleted_rows: &[WorthTouchedGraphConflictDeletionAlignmentRow],
) -> Result<CloseoutMatrixAuthority, WorthTouchedGraphCrossFamilyCloseoutMatrixError> {
    let mut family_parity_coverage = BTreeSet::new();
    let topology = current_topology_family_declare_once_parity_claim().map_err(|error| {
        WorthTouchedGraphCrossFamilyCloseoutMatrixError::new(
            WorthTouchedGraphCrossFamilyCloseoutMatrixErrorKind::CurrentTopologyParityUnavailable,
            error.detail(),
        )
    })?;
    family_parity_coverage.extend(topology.rows().iter().map(|row| row.family_kind()));
    let spatial = current_spatial_family_parity_claim().map_err(|error| {
        WorthTouchedGraphCrossFamilyCloseoutMatrixError::new(
            WorthTouchedGraphCrossFamilyCloseoutMatrixErrorKind::CurrentSpatialParityUnavailable,
            error.detail(),
        )
    })?;
    family_parity_coverage.extend(spatial.rows().iter().map(|row| row.family_kind()));
    let replay_undo = current_replay_undo_family_parity_claim().map_err(|error| {
        WorthTouchedGraphCrossFamilyCloseoutMatrixError::new(
            WorthTouchedGraphCrossFamilyCloseoutMatrixErrorKind::CurrentReplayUndoParityUnavailable,
            error.detail(),
        )
    })?;
    family_parity_coverage.extend(replay_undo.rows().iter().map(|row| row.family_kind()));
    let conflict = current_conflict_family_parity_claim().map_err(|error| {
        WorthTouchedGraphCrossFamilyCloseoutMatrixError::new(
            WorthTouchedGraphCrossFamilyCloseoutMatrixErrorKind::CurrentConflictParityUnavailable,
            error.detail(),
        )
    })?;
    family_parity_coverage.extend(conflict.rows().iter().map(|row| row.family_kind()));
    let reuse = current_reuse_family_parity_claim().map_err(|error| {
        WorthTouchedGraphCrossFamilyCloseoutMatrixError::new(
            WorthTouchedGraphCrossFamilyCloseoutMatrixErrorKind::CurrentReuseParityUnavailable,
            error.detail(),
        )
    })?;
    family_parity_coverage.extend(reuse.rows().iter().map(|row| row.family_kind()));
    let public_projection = current_public_projection_parity_claim().map_err(|error| {
        WorthTouchedGraphCrossFamilyCloseoutMatrixError::new(
            WorthTouchedGraphCrossFamilyCloseoutMatrixErrorKind::CurrentPublicProjectionParityUnavailable,
            error.detail(),
        )
    })?;
    family_parity_coverage.extend(public_projection.rows().iter().map(|row| match row.kind() {
        crate::workload_composition::PublicProjectionContributorRowKind::PublicProof => {
            TouchedGraphParityFamilyKind::PublicProof
        }
        crate::workload_composition::PublicProjectionContributorRowKind::DerivedDiagnostics => {
            TouchedGraphParityFamilyKind::DerivedDiagnostics
        }
    }));

    let mut covered_counts = BTreeMap::new();
    let mut capped_residue_counts = BTreeMap::new();
    let mut query_gap_counts = BTreeMap::new();
    let mut blocked_counts = BTreeMap::new();
    let readiness_coverage = readiness
        .representative_family_coverage()
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    for row in live_ledger.rows() {
        let target = match row.claim_kind() {
            ArchitectureClaimLedgerRowKind::Covered => &mut covered_counts,
            ArchitectureClaimLedgerRowKind::CappedResidue => &mut capped_residue_counts,
            ArchitectureClaimLedgerRowKind::QueryGap => &mut query_gap_counts,
            ArchitectureClaimLedgerRowKind::BlockedOutsideRoadmap => &mut blocked_counts,
        };
        *target.entry(row.family_kind()).or_insert(0) += 1;
    }

    Ok(CloseoutMatrixAuthority {
        representative_path_coverage: representative_path.covered_family_kinds(),
        family_parity_coverage,
        readiness_coverage,
        public_proof_parity_passed: public_projection.rows().iter().any(|row| {
            matches!(
                row.kind(),
                crate::workload_composition::PublicProjectionContributorRowKind::PublicProof
            )
        }),
        diagnostic_parity_passed: public_projection.rows().iter().any(|row| {
            matches!(
                row.kind(),
                crate::workload_composition::PublicProjectionContributorRowKind::DerivedDiagnostics
            )
        }),
        covered_counts,
        deleted_counts: deleted_counts(deleted_rows)?,
        capped_residue_counts,
        query_gap_counts,
        blocked_counts,
        closeout_architecture_claim_digest: live_ledger.closeout_architecture_claim_digest().to_string(),
    })
}

fn current_readiness_input(
) -> Result<TouchedGraphParityReadinessInput, WorthTouchedGraphCrossFamilyCloseoutMatrixError> {
    crate::workload_composition::current_touched_graph_readiness_handoff().map_err(|error| {
        WorthTouchedGraphCrossFamilyCloseoutMatrixError::new(
            WorthTouchedGraphCrossFamilyCloseoutMatrixErrorKind::CurrentReadinessHandoffUnavailable,
            error.detail(),
        )
    })
}

fn deleted_counts(
    rows: &[WorthTouchedGraphConflictDeletionAlignmentRow],
) -> Result<BTreeMap<TouchedGraphParityFamilyKind, usize>, WorthTouchedGraphCrossFamilyCloseoutMatrixError>
{
    let mut counts = BTreeMap::new();
    for row in rows {
        *counts.entry(row.family_kind()).or_insert(0) += 1;
    }
    Ok(counts)
}
