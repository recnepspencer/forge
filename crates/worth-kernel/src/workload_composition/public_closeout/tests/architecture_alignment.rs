use std::collections::BTreeSet;

use super::*;

#[test]
fn architecture_alignment_report_proves_parallel_cutover_law() {
    let closeout = current_worth_touched_graph_conflict_public_closeout()
        .expect("current public closeout should publish from real proof products");
    let deletion_closeout = current_worth_touched_graph_conflict_deletion_closeout()
        .expect("current deletion closeout");
    let report = closeout.architecture_alignment_report();

    let expected_all = deletion_closeout
        .deletion_ledger()
        .rows()
        .iter()
        .map(deletion_row_key)
        .collect::<BTreeSet<_>>();
    let expected_deleted = deletion_closeout
        .deletion_ledger()
        .rows()
        .iter()
        .filter(|row| {
            row.disposition()
                == crate::workload_composition::WorthTouchedGraphConflictDeletionDisposition::DeletedAuthority
        })
        .map(deletion_row_key)
        .collect::<BTreeSet<_>>();
    let expected_capped = deletion_closeout
        .deletion_ledger()
        .rows()
        .iter()
        .filter(|row| {
            row.disposition()
                == crate::workload_composition::WorthTouchedGraphConflictDeletionDisposition::CappedResidue
        })
        .map(deletion_row_key)
        .collect::<BTreeSet<_>>();
    let expected_fenced = deletion_closeout
        .deletion_ledger()
        .rows()
        .iter()
        .filter(|row| {
            row.disposition()
                == crate::workload_composition::WorthTouchedGraphConflictDeletionDisposition::CertificationOnlyFence
        })
        .map(deletion_row_key)
        .collect::<BTreeSet<_>>();
    let expected_residue = closeout
        .residue_chain()
        .rows()
        .iter()
        .filter(|row| {
            row.disposition() == WorthTouchedGraphConflictResidueDisposition::ExplicitResidue
                && row.boundary_posture()
                    != WorthTouchedGraphConflictResidueBoundaryPosture::CoveredOrdinaryConsumerDependency
        })
        .map(residue_row_key)
        .collect::<BTreeSet<_>>();
    let expected_query_gaps = closeout
        .residue_chain()
        .rows()
        .iter()
        .filter(|row| {
            row.disposition() == WorthTouchedGraphConflictResidueDisposition::QueryGap
                || row.boundary_posture()
                    == WorthTouchedGraphConflictResidueBoundaryPosture::QueryGapSupportGap
        })
        .map(residue_row_key)
        .collect::<BTreeSet<_>>();

    assert!(report.ordinary_second_ontology_blockers().is_empty());
    assert_eq!(
        report
            .displaced_legacy_authority_rows()
            .iter()
            .map(deletion_alignment_key)
            .collect::<BTreeSet<_>>(),
        expected_all
    );
    assert_eq!(
        report
            .deleted_authority_rows()
            .iter()
            .map(deletion_alignment_key)
            .collect::<BTreeSet<_>>(),
        expected_deleted
    );
    assert_eq!(
        report
            .capped_deletion_rows()
            .iter()
            .map(deletion_alignment_key)
            .collect::<BTreeSet<_>>(),
        expected_capped
    );
    assert_eq!(
        report
            .certification_only_fence_rows()
            .iter()
            .map(deletion_alignment_key)
            .collect::<BTreeSet<_>>(),
        expected_fenced
    );
    assert_eq!(
        report
            .capped_residue_rows()
            .iter()
            .map(architecture_row_key)
            .collect::<BTreeSet<_>>(),
        expected_residue
    );
    assert_eq!(
        report
            .query_gap_support_rows()
            .iter()
            .map(architecture_row_key)
            .collect::<BTreeSet<_>>(),
        expected_query_gaps
    );
    assert!(report
        .query_gap_support_rows()
        .iter()
        .all(|row| row.query_gap_kind().is_some()
            && row.mechanically_unreachable_from_ordinary_path()));
    assert!(!report
        .topology_compiled_product_identity_digest()
        .is_empty());
    assert!(!report
        .topology_equivalence_policy_identity_digest()
        .is_empty());
    assert!(!report.spatial_compiled_product_identity_digest().is_empty());
    assert!(!report
        .spatial_equivalence_policy_identity_digest()
        .is_empty());
    assert!(
        report.reuse_decision_identity_digest().is_some()
            || report.rebuild_denial_identity_digest().is_some()
    );
    assert!(report.milestone_fifteen_ready());
    assert!(!report.report_digest().is_empty());
}

#[test]
fn architecture_alignment_report_surfaces_second_ontology_blockers() {
    let deletion_closeout = current_worth_touched_graph_conflict_deletion_closeout()
        .expect("current deletion closeout");
    let hostile_inventory = hostile_inventory_with_open_ordinary_dependency();
    let hostile_cutover = ordinary_consumer_cutover_from_inventory_for_tests(&hostile_inventory)
        .expect("hostile cutover should lower from the real inventory path");
    let hostile_residue_chain =
        WorthTouchedGraphConflictResidueChain::from_cutover_rows(hostile_cutover.rows());
    let selected_route_packet = current_worth_touched_graph_conflict_selected_route_packet()
        .expect("current selected-route packet should remain available for architecture alignment");

    let report = build_architecture_alignment_report(
        &deletion_closeout,
        &hostile_residue_chain,
        &selected_route_packet,
    )
    .expect("architecture report should lower from typed residue and deletion authority");

    let expected_blockers = hostile_residue_chain
        .rows()
        .iter()
        .filter(|row| {
            row.boundary_posture()
                == WorthTouchedGraphConflictResidueBoundaryPosture::CoveredOrdinaryConsumerDependency
        })
        .map(residue_row_key)
        .collect::<BTreeSet<_>>();

    assert_eq!(
        report
            .ordinary_second_ontology_blockers()
            .iter()
            .map(architecture_row_key)
            .collect::<BTreeSet<_>>(),
        expected_blockers
    );
    assert!(report
        .ordinary_second_ontology_blockers()
        .iter()
        .all(|row| !row.mechanically_unreachable_from_ordinary_path()));
    assert!(!report.milestone_fifteen_ready());
}

fn deletion_row_key(
    row: &crate::workload_composition::WorthTouchedGraphConflictDeletionLedgerRow,
) -> (String, String, String, String) {
    (
        row.source_path().to_string(),
        row.surface_name().to_string(),
        row.blocker().to_string(),
        row.removal_trigger().to_string(),
    )
}

fn deletion_alignment_key(
    row: &crate::workload_composition::public_closeout::WorthTouchedGraphConflictDeletionAlignmentRow,
) -> (String, String, String, String) {
    (
        row.source_path().to_string(),
        row.surface_name().to_string(),
        row.blocker().to_string(),
        row.removal_trigger().to_string(),
    )
}

fn residue_row_key(row: &WorthTouchedGraphConflictResidueRow) -> (String, String, String, String) {
    (
        row.surface_name().to_string(),
        row.owner().to_string(),
        row.blocker().to_string(),
        row.removal_trigger().to_string(),
    )
}

fn architecture_row_key(
    row: &crate::workload_composition::public_closeout::WorthTouchedGraphConflictArchitectureAlignmentReportRow,
) -> (String, String, String, String) {
    (
        row.surface_name().to_string(),
        row.owner().to_string(),
        row.blocker().to_string(),
        row.removal_trigger().to_string(),
    )
}
