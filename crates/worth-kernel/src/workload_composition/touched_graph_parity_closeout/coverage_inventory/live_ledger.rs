use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::current::{current_cross_family_coverage_inventory, CrossFamilyCoverageInventory};
use super::ledger_row::{ArchitectureClaimLedgerRow, ArchitectureClaimLedgerRowKind};
use crate::workload_composition::planner_owned_routing::{
    current_worth_touched_graph_conflict_public_closeout,
    current_worth_touched_graph_conflict_selected_route_packet,
    WorthTouchedGraphConflictPublicCloseout,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveCoverageLedger {
    rows: Vec<ArchitectureClaimLedgerRow>,
    closeout_architecture_claim_digest: String,
    ledger_digest: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LiveCoverageLedgerError {
    CurrentCoverageInventoryUnavailable,
    CurrentPublicCloseoutUnavailable,
    CurrentSelectedRoutePacketUnavailable,
    MismatchedSelectedRoutePacketDigest,
    MismatchedSeedDigest,
    MismatchedArchitectureAlignmentDigest,
    OrdinaryPathBlockersStillReachable,
    MismatchedArchitectureAlignmentRows,
}

pub fn current_live_coverage_ledger() -> Result<LiveCoverageLedger, LiveCoverageLedgerError> {
    let inventory = current_cross_family_coverage_inventory()
        .map_err(|_| LiveCoverageLedgerError::CurrentCoverageInventoryUnavailable)?;
    let closeout = current_worth_touched_graph_conflict_public_closeout()
        .map_err(|_| LiveCoverageLedgerError::CurrentPublicCloseoutUnavailable)?;
    let selected_route_packet = current_worth_touched_graph_conflict_selected_route_packet()
        .map_err(|_| LiveCoverageLedgerError::CurrentSelectedRoutePacketUnavailable)?;

    live_coverage_ledger_from_authorities(&inventory, &closeout, &selected_route_packet)
}

pub(crate) fn live_coverage_ledger_from_authorities(
    inventory: &CrossFamilyCoverageInventory,
    closeout: &WorthTouchedGraphConflictPublicCloseout,
    selected_route_packet: &crate::workload_composition::WorthTouchedGraphConflictSelectedRoutePacket,
) -> Result<LiveCoverageLedger, LiveCoverageLedgerError> {
    derive_live_coverage_ledger(
        inventory,
        closeout,
        selected_route_packet.packet_digest(),
        closeout.milestone_fifteen_seed().seed_digest(),
        closeout.architecture_alignment_report().report_digest(),
    )
}

pub(crate) fn derive_live_coverage_ledger(
    inventory: &CrossFamilyCoverageInventory,
    closeout: &WorthTouchedGraphConflictPublicCloseout,
    selected_route_packet_digest: &str,
    seed_digest: &str,
    architecture_alignment_report_digest: &str,
) -> Result<LiveCoverageLedger, LiveCoverageLedgerError> {
    if selected_route_packet_digest != closeout.proof_chain().selected_route_packet_digest() {
        return Err(LiveCoverageLedgerError::MismatchedSelectedRoutePacketDigest);
    }
    if seed_digest != closeout.milestone_fifteen_seed().seed_digest() {
        return Err(LiveCoverageLedgerError::MismatchedSeedDigest);
    }
    if architecture_alignment_report_digest
        != closeout.architecture_alignment_report().report_digest()
    {
        return Err(LiveCoverageLedgerError::MismatchedArchitectureAlignmentDigest);
    }
    if closeout
        .architecture_alignment_report()
        .ordinary_second_ontology_blockers()
        .iter()
        .any(|row| !row.mechanically_unreachable_from_ordinary_path())
    {
        return Err(LiveCoverageLedgerError::OrdinaryPathBlockersStillReachable);
    }

    let mut rows =
        crate::workload_composition::touched_graph_parity_closeout::coverage_inventory::claim_derivation::derive_covered_rows(
            inventory,
            selected_route_packet_digest,
            seed_digest,
            architecture_alignment_report_digest,
        );
    rows.extend(
        crate::workload_composition::touched_graph_parity_closeout::coverage_inventory::claim_derivation::derive_residue_rows(
            closeout,
            selected_route_packet_digest,
            seed_digest,
            architecture_alignment_report_digest,
        ),
    );
    Ok(live_coverage_ledger_from_rows(rows))
}

fn live_coverage_ledger_from_rows(mut rows: Vec<ArchitectureClaimLedgerRow>) -> LiveCoverageLedger {
    rows.sort_by(
        |left: &ArchitectureClaimLedgerRow, right: &ArchitectureClaimLedgerRow| {
            left.surface_path()
                .cmp(right.surface_path())
                .then(left.surface_name().cmp(right.surface_name()))
                .then(left.claim_kind().as_str().cmp(right.claim_kind().as_str()))
        },
    );

    let closeout_architecture_claim_digest = truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &rows
            .iter()
            .map(|row: &ArchitectureClaimLedgerRow| {
                format!(
                    "{}:{}:{}",
                    row.claim_kind().as_str(),
                    row.family_kind().as_str(),
                    row.architecture_claim_digest()
                )
            })
            .chain(std::iter::once(
                "worth-kernel:touched-graph-parity-closeout-architecture-claim-digest:v1"
                    .to_string(),
            ))
            .collect::<Vec<_>>(),
    );

    let ledger_digest = truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &rows
            .iter()
            .map(|row: &ArchitectureClaimLedgerRow| {
                format!(
                    "{}:{}:{}:{}:{}:{}:{}:{}:{}:{}",
                    row.family_kind().as_str(),
                    row.claim_kind().as_str(),
                    row.owner(),
                    row.surface_name(),
                    row.surface_path(),
                    row.selected_route_packet_digest(),
                    row.seed_digest(),
                    row.architecture_claim_digest(),
                    row.query_gap_kind().map_or("none", |kind| kind.as_str()),
                    row.mechanically_unreachable_from_ordinary_path()
                )
            })
            .chain(rows.iter().filter_map(|row: &ArchitectureClaimLedgerRow| {
                row.residue_or_firewall_digest()
                    .map(|digest| format!("residue-or-firewall:{digest}"))
            }))
            .chain(std::iter::once(
                "worth-kernel:touched-graph-parity-live-coverage-ledger:v1".to_string(),
            ))
            .collect::<Vec<_>>(),
    );

    LiveCoverageLedger {
        rows,
        closeout_architecture_claim_digest,
        ledger_digest,
    }
}

impl LiveCoverageLedger {
    pub fn rows(&self) -> &[ArchitectureClaimLedgerRow] {
        &self.rows
    }

    pub fn ledger_digest(&self) -> &str {
        &self.ledger_digest
    }

    pub fn closeout_architecture_claim_digest(&self) -> &str {
        &self.closeout_architecture_claim_digest
    }

    pub fn covered_count(&self) -> usize {
        self.rows
            .iter()
            .filter(|row| row.claim_kind() == ArchitectureClaimLedgerRowKind::Covered)
            .count()
    }

    pub fn capped_residue_count(&self) -> usize {
        self.rows
            .iter()
            .filter(|row| row.claim_kind() == ArchitectureClaimLedgerRowKind::CappedResidue)
            .count()
    }

    pub fn query_gap_count(&self) -> usize {
        self.rows
            .iter()
            .filter(|row| row.claim_kind() == ArchitectureClaimLedgerRowKind::QueryGap)
            .count()
    }

    pub fn blocked_outside_roadmap_count(&self) -> usize {
        self.rows
            .iter()
            .filter(|row| row.claim_kind() == ArchitectureClaimLedgerRowKind::BlockedOutsideRoadmap)
            .count()
    }

    pub fn residue_count(&self) -> usize {
        self.capped_residue_count() + self.query_gap_count() + self.blocked_outside_roadmap_count()
    }
}
