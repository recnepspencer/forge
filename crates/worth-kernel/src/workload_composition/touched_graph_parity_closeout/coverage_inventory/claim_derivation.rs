use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::current::CrossFamilyCoverageInventory;
use super::ledger_row::{ArchitectureClaimLedgerRow, ArchitectureClaimLedgerRowKind};
use crate::workload_composition::planner_owned_routing::{
    WorthTouchedGraphConflictArchitectureAlignmentReportRow,
    WorthTouchedGraphConflictPublicCloseout,
};

pub(crate) fn derive_covered_rows(
    inventory: &CrossFamilyCoverageInventory,
    selected_route_packet_digest: &str,
    seed_digest: &str,
    architecture_alignment_report_digest: &str,
) -> Vec<ArchitectureClaimLedgerRow> {
    inventory
        .rows()
        .iter()
        .map(|row| {
            let architecture_claim_digest = truth_digest_parts(
                TruthDigestScope::ArtifactIdentity,
                &[
                    format!("claim-kind:{}", ArchitectureClaimLedgerRowKind::Covered.as_str()),
                    format!("family-kind:{}", row.family_kind().as_str()),
                    format!("owner:{}", row.current_owner_crate()),
                    format!("surface-name:{}", row.current_surface()),
                    format!("surface-path:{}", row.source_path()),
                    format!("selected-route-packet-digest:{selected_route_packet_digest}"),
                    format!("seed-digest:{seed_digest}"),
                    format!("architecture-alignment-report-digest:{architecture_alignment_report_digest}"),
                    format!(
                        "selected-identities:{}",
                        row.selected_identity_fields_consumed().join(",")
                    ),
                ],
            );
            ArchitectureClaimLedgerRow::new(
                row.family_kind(),
                row.current_owner_crate(),
                row.current_surface(),
                row.source_path(),
                selected_route_packet_digest,
                seed_digest,
                architecture_claim_digest,
                None,
                ArchitectureClaimLedgerRowKind::Covered,
                None,
                false,
            )
        })
        .collect()
}

pub(crate) fn derive_residue_rows(
    closeout: &WorthTouchedGraphConflictPublicCloseout,
    selected_route_packet_digest: &str,
    seed_digest: &str,
    architecture_alignment_report_digest: &str,
) -> Vec<ArchitectureClaimLedgerRow> {
    let residue_digest = closeout.residue_chain().residue_digest().to_string();
    let source_firewall_digest = closeout.source_firewall_digest().to_string();
    let report = closeout.architecture_alignment_report();
    let mut rows = derive_report_group_rows(
        report.capped_residue_rows(),
        ArchitectureClaimLedgerRowKind::CappedResidue,
        selected_route_packet_digest,
        seed_digest,
        architecture_alignment_report_digest,
        &residue_digest,
    );
    rows.extend(derive_report_group_rows(
        report.query_gap_support_rows(),
        ArchitectureClaimLedgerRowKind::QueryGap,
        selected_route_packet_digest,
        seed_digest,
        architecture_alignment_report_digest,
        &residue_digest,
    ));
    rows.extend(derive_report_group_rows(
        report.ordinary_second_ontology_blockers(),
        ArchitectureClaimLedgerRowKind::BlockedOutsideRoadmap,
        selected_route_packet_digest,
        seed_digest,
        architecture_alignment_report_digest,
        &source_firewall_digest,
    ));
    rows
}

fn derive_report_group_rows(
    rows: &[WorthTouchedGraphConflictArchitectureAlignmentReportRow],
    claim_kind: ArchitectureClaimLedgerRowKind,
    selected_route_packet_digest: &str,
    seed_digest: &str,
    architecture_alignment_report_digest: &str,
    residue_or_firewall_basis_digest: &str,
) -> Vec<ArchitectureClaimLedgerRow> {
    rows.iter()
        .map(|row| {
            derive_report_row(
                row,
                claim_kind,
                selected_route_packet_digest,
                seed_digest,
                architecture_alignment_report_digest,
                residue_or_firewall_basis_digest,
            )
        })
        .collect()
}

fn derive_report_row(
    row: &WorthTouchedGraphConflictArchitectureAlignmentReportRow,
    claim_kind: ArchitectureClaimLedgerRowKind,
    selected_route_packet_digest: &str,
    seed_digest: &str,
    architecture_alignment_report_digest: &str,
    residue_or_firewall_basis_digest: &str,
) -> ArchitectureClaimLedgerRow {
    let residue_or_firewall_digest = truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            format!("kind:{}", claim_kind.as_str()),
            format!("basis-digest:{residue_or_firewall_basis_digest}"),
            format!("family-kind:{}", row.family_kind().as_str()),
            format!("surface:{}", row.surface_name()),
            format!("source:{}", row.source_path()),
            format!("owner:{}", row.owner()),
            format!(
                "query-gap-kind:{}",
                row.query_gap_kind().map_or("none", |kind| kind.as_str())
            ),
            format!(
                "ordinary-path-unreachable:{}",
                row.mechanically_unreachable_from_ordinary_path()
            ),
        ],
    );
    let architecture_claim_digest = truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            format!("claim-kind:{}", claim_kind.as_str()),
            format!("family-kind:{}", row.family_kind().as_str()),
            format!("owner:{}", row.owner()),
            format!("surface-name:{}", row.surface_name()),
            format!("surface-path:{}", row.source_path()),
            format!("selected-route-packet-digest:{selected_route_packet_digest}"),
            format!("seed-digest:{seed_digest}"),
            format!("architecture-alignment-report-digest:{architecture_alignment_report_digest}"),
            format!("blocker:{}", row.blocker()),
            format!("removal-trigger:{}", row.removal_trigger()),
            format!(
                "query-gap-kind:{}",
                row.query_gap_kind().map_or("none", |kind| kind.as_str())
            ),
            format!(
                "ordinary-path-unreachable:{}",
                row.mechanically_unreachable_from_ordinary_path()
            ),
        ],
    );

    ArchitectureClaimLedgerRow::new(
        row.family_kind(),
        row.owner(),
        row.surface_name(),
        row.source_path(),
        selected_route_packet_digest,
        seed_digest,
        architecture_claim_digest,
        Some(residue_or_firewall_digest),
        claim_kind,
        row.query_gap_kind(),
        row.mechanically_unreachable_from_ordinary_path(),
    )
}
