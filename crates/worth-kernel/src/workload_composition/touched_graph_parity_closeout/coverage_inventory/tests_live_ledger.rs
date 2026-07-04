use super::current::current_cross_family_coverage_inventory;
use super::claim_derivation::derive_residue_rows;
use super::ledger_row::ArchitectureClaimLedgerRowKind;
use super::live_ledger::{
    current_live_coverage_ledger, derive_live_coverage_ledger, LiveCoverageLedgerError,
};
use crate::workload_composition::planner_owned_routing::{
    current_worth_touched_graph_conflict_public_closeout,
    current_worth_touched_graph_conflict_selected_route_packet,
};

fn run_stack_heavy_planner_owned_routing_test(test: impl FnOnce() + Send + 'static) {
    let result = std::thread::Builder::new()
        .stack_size(256 * 1024 * 1024)
        .spawn(test)
        .expect("planner-owned routing test should spawn on a larger stack")
        .join();
    if let Err(panic_payload) = result {
        std::panic::resume_unwind(panic_payload);
    }
}

#[test]
fn live_coverage_ledger_matches_current_family_surfaces() {
    run_stack_heavy_planner_owned_routing_test(|| {
        let inventory =
            current_cross_family_coverage_inventory().expect("cross-family coverage inventory");
        let ledger = current_live_coverage_ledger().expect("live coverage ledger");

        let covered_rows = ledger
            .rows()
            .iter()
            .filter(|row| row.claim_kind() == ArchitectureClaimLedgerRowKind::Covered)
            .collect::<Vec<_>>();

        assert_eq!(covered_rows.len(), inventory.rows().len());
        for inventory_row in inventory.rows() {
            let matched = covered_rows.iter().find(|row| {
                row.family_kind() == inventory_row.family_kind()
                    && row.surface_name() == inventory_row.current_surface()
                    && row.surface_path() == inventory_row.source_path()
            });
            let row =
                matched.expect("covered ledger row should match the live contributor surface");
            assert!(!row.selected_route_packet_digest().is_empty());
            assert!(!row.seed_digest().is_empty());
            assert!(!row.architecture_claim_digest().is_empty());
            assert!(
                row.residue_or_firewall_digest().is_none(),
                "covered rows must not carry residue or firewall digests"
            );
        }
    });
}

#[test]
fn architecture_claim_rows_fail_on_stale_seed_or_report_inputs() {
    run_stack_heavy_planner_owned_routing_test(|| {
        let inventory =
            current_cross_family_coverage_inventory().expect("cross-family coverage inventory");
        let closeout = current_worth_touched_graph_conflict_public_closeout()
            .expect("current public closeout");
        let packet = current_worth_touched_graph_conflict_selected_route_packet()
            .expect("current selected-route packet");

        let stale_selected_route = derive_live_coverage_ledger(
            &inventory,
            &closeout,
            "stale-selected-route-packet-digest",
            closeout.milestone_fifteen_seed().seed_digest(),
            closeout.architecture_alignment_report().report_digest(),
        );
        assert_eq!(
            stale_selected_route,
            Err(LiveCoverageLedgerError::MismatchedSelectedRoutePacketDigest)
        );

        let stale_seed = derive_live_coverage_ledger(
            &inventory,
            &closeout,
            packet.packet_digest(),
            "stale-seed-digest",
            closeout.architecture_alignment_report().report_digest(),
        );
        assert_eq!(
            stale_seed,
            Err(LiveCoverageLedgerError::MismatchedSeedDigest)
        );

        let stale_report = derive_live_coverage_ledger(
            &inventory,
            &closeout,
            packet.packet_digest(),
            closeout.milestone_fifteen_seed().seed_digest(),
            "stale-architecture-alignment-report-digest",
        );
        assert_eq!(
            stale_report,
            Err(LiveCoverageLedgerError::MismatchedArchitectureAlignmentDigest)
        );
    });
}

#[test]
fn closeout_and_residue_counts_derive_from_live_ledger() {
    run_stack_heavy_planner_owned_routing_test(|| {
        let ledger = current_live_coverage_ledger().expect("live coverage ledger");

        assert_eq!(
            ledger.covered_count(),
            ledger
                .rows()
                .iter()
                .filter(|row| row.claim_kind() == ArchitectureClaimLedgerRowKind::Covered)
                .count()
        );
        assert_eq!(
            ledger.capped_residue_count(),
            ledger
                .rows()
                .iter()
                .filter(|row| row.claim_kind() == ArchitectureClaimLedgerRowKind::CappedResidue)
                .count()
        );
        assert_eq!(
            ledger.query_gap_count(),
            ledger
                .rows()
                .iter()
                .filter(|row| row.claim_kind() == ArchitectureClaimLedgerRowKind::QueryGap)
                .count()
        );
        assert_eq!(
            ledger.blocked_outside_roadmap_count(),
            ledger
                .rows()
                .iter()
                .filter(
                    |row| row.claim_kind() == ArchitectureClaimLedgerRowKind::BlockedOutsideRoadmap
                )
                .count()
        );
        assert_eq!(
            ledger.residue_count(),
            ledger.rows().len() - ledger.covered_count()
        );
        assert!(!ledger.closeout_architecture_claim_digest().is_empty());
        assert!(!ledger.ledger_digest().is_empty());
        assert_ne!(
            ledger.closeout_architecture_claim_digest(),
            ledger.ledger_digest()
        );
        assert!(ledger.rows().iter().all(|row| {
            row.claim_kind() == ArchitectureClaimLedgerRowKind::Covered
                || row.residue_or_firewall_digest().is_some()
        }));
    });
}

#[test]
fn residue_rows_lower_directly_from_architecture_alignment_report_groups() {
    run_stack_heavy_planner_owned_routing_test(|| {
        let ledger = current_live_coverage_ledger().expect("live coverage ledger");
        let closeout = current_worth_touched_graph_conflict_public_closeout()
            .expect("current public closeout");
        let report = closeout.architecture_alignment_report();

        for row in report.capped_residue_rows() {
            assert!(ledger.rows().iter().any(|ledger_row| {
                ledger_row.claim_kind() == ArchitectureClaimLedgerRowKind::CappedResidue
                    && ledger_row.family_kind() == row.family_kind()
                    && ledger_row.surface_name() == row.surface_name()
                    && ledger_row.surface_path() == row.source_path()
            }));
        }
        for row in report.query_gap_support_rows() {
            assert!(ledger.rows().iter().any(|ledger_row| {
                ledger_row.claim_kind() == ArchitectureClaimLedgerRowKind::QueryGap
                    && ledger_row.family_kind() == row.family_kind()
                    && ledger_row.surface_name() == row.surface_name()
                    && ledger_row.surface_path() == row.source_path()
            }));
        }
        for row in report.ordinary_second_ontology_blockers() {
            assert!(ledger.rows().iter().any(|ledger_row| {
                ledger_row.claim_kind() == ArchitectureClaimLedgerRowKind::BlockedOutsideRoadmap
                    && ledger_row.family_kind() == row.family_kind()
                    && ledger_row.surface_name() == row.surface_name()
                    && ledger_row.surface_path() == row.source_path()
            }));
        }
    });
}

#[test]
fn residue_collapse_is_exact_and_mechanically_derived() {
    run_stack_heavy_planner_owned_routing_test(|| {
        let closeout = current_worth_touched_graph_conflict_public_closeout()
            .expect("current public closeout");
        let residue_rows = derive_residue_rows(
            &closeout,
            closeout.proof_chain().selected_route_packet_digest(),
            closeout.milestone_fifteen_seed().seed_digest(),
            closeout.architecture_alignment_report().report_digest(),
        )
        .into_iter()
        .filter(|row| row.claim_kind() != ArchitectureClaimLedgerRowKind::Covered)
        .collect::<Vec<_>>();

        for row in &residue_rows {
            assert!(!row.owner().is_empty());
            assert!(!row.surface_name().is_empty());
            assert!(!row.surface_path().is_empty());
            assert!(row.residue_or_firewall_digest().is_some());
        }

        for row in residue_rows
            .iter()
            .filter(|row| row.claim_kind() == ArchitectureClaimLedgerRowKind::QueryGap)
        {
            assert!(row.query_gap_kind().is_some());
            assert!(row.mechanically_unreachable_from_ordinary_path());
        }

        for row in residue_rows.iter().filter(|row| {
            row.claim_kind() == ArchitectureClaimLedgerRowKind::BlockedOutsideRoadmap
        }) {
            assert!(row.mechanically_unreachable_from_ordinary_path());
        }
    });
}
