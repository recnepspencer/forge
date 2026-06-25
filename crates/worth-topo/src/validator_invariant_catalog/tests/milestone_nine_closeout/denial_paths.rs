use crate::validator_invariant_catalog::{
    WorthTopologyLegalityCatalogError, WorthTopologyMilestoneNineCloseout,
    WorthTopologyMilestoneNineCloseoutDenialKind, WorthTopologyMilestoneNineDeletionLedgerReport,
    WorthTopologyMilestoneNineDeletionLedgerRow, WorthTopologyMilestoneNineResidueAuditReport,
    WorthTopologyMilestoneNineResidueAuditRow, WorthTopologyMilestoneNineResidueStatus,
    WorthTopologyMilestoneNineSourceFirewallReport,
};

use super::fixtures::{alternate_operator_cutover_closeout, operator_cutover_closeout};

#[test]
fn mismatched_phase_eight_seed_is_denied() {
    let cutover = operator_cutover_closeout();
    let other_cutover = alternate_operator_cutover_closeout();

    assert_ne!(
        cutover.phase_eight_seed().seed_digest(),
        other_cutover.phase_eight_seed().seed_digest()
    );

    let error = WorthTopologyMilestoneNineCloseout::from_operator_cutover(
        other_cutover.phase_eight_seed(),
        &cutover,
    )
    .expect_err("closeout should reject a stale Phase 8 seed");

    assert_eq!(
        denial_kind(error),
        WorthTopologyMilestoneNineCloseoutDenialKind::PhaseEightSeedMismatch
    );
}

#[test]
fn empty_selected_obligation_rows_are_denied() {
    let cutover = operator_cutover_closeout().without_selected_rows_for_tests();

    let error = WorthTopologyMilestoneNineCloseout::from_operator_cutover(
        cutover.phase_eight_seed(),
        &cutover,
    )
    .expect_err("closeout should reject empty selected obligation proof");

    assert_eq!(
        denial_kind(error),
        WorthTopologyMilestoneNineCloseoutDenialKind::EmptySelectedObligationProof
    );
}

#[test]
fn missing_execution_backed_adoption_proof_is_denied() {
    let cutover = operator_cutover_closeout().without_execution_proof_for_tests();

    let error = WorthTopologyMilestoneNineCloseout::from_operator_cutover(
        cutover.phase_eight_seed(),
        &cutover,
    )
    .expect_err("closeout should reject missing execution proof digest");

    assert_eq!(
        denial_kind(error),
        WorthTopologyMilestoneNineCloseoutDenialKind::MissingExecutionBackedAdoptionProof
    );
}

#[test]
fn source_firewall_violation_is_denied() {
    let cutover = operator_cutover_closeout();
    let deletion_ledger =
        WorthTopologyMilestoneNineDeletionLedgerReport::from_operator_cutover(&cutover);
    let residue_audit =
        WorthTopologyMilestoneNineResidueAuditReport::from_cutover_and_deletion_ledger(
            &cutover,
            &deletion_ledger,
        );
    let source_firewall =
        WorthTopologyMilestoneNineSourceFirewallReport::from_source_pairs_with_deletion_ledger(
            [("new_operator_surface.rs", "DERIVED_TOPOLOGY_RULE_SPECS")],
            &deletion_ledger,
        );

    let error = WorthTopologyMilestoneNineCloseout::from_operator_cutover_with_reports_for_tests(
        cutover.phase_eight_seed(),
        &cutover,
        deletion_ledger,
        residue_audit,
        source_firewall,
    )
    .expect_err("closeout should reject source firewall violations");

    assert_eq!(
        denial_kind(error),
        WorthTopologyMilestoneNineCloseoutDenialKind::SourceFirewallViolation
    );
}

#[test]
fn uncapped_residue_is_denied() {
    let cutover = operator_cutover_closeout();
    let deletion_ledger =
        WorthTopologyMilestoneNineDeletionLedgerReport::from_operator_cutover(&cutover);
    let residue_audit = WorthTopologyMilestoneNineResidueAuditReport::from_rows([
        WorthTopologyMilestoneNineResidueAuditRow::new(
            "uncapped_old_authority.rs",
            "uncapped-residue",
            None,
            WorthTopologyMilestoneNineResidueStatus::UncappedAuthority,
        ),
    ]);
    let source_firewall =
        WorthTopologyMilestoneNineSourceFirewallReport::current_with_deletion_ledger(
            &deletion_ledger,
        );

    let error = WorthTopologyMilestoneNineCloseout::from_operator_cutover_with_reports_for_tests(
        cutover.phase_eight_seed(),
        &cutover,
        deletion_ledger,
        residue_audit,
        source_firewall,
    )
    .expect_err("closeout should reject uncapped old authority residue");

    assert_eq!(
        denial_kind(error),
        WorthTopologyMilestoneNineCloseoutDenialKind::UncappedOldAuthority
    );
}

#[test]
fn stale_residue_without_deletion_ledger_is_denied() {
    let cutover = operator_cutover_closeout();
    let deletion_ledger = WorthTopologyMilestoneNineDeletionLedgerReport::from_rows(Vec::<
        WorthTopologyMilestoneNineDeletionLedgerRow,
    >::new());
    let residue_audit =
        WorthTopologyMilestoneNineResidueAuditReport::from_cutover_and_deletion_ledger(
            &cutover,
            &deletion_ledger,
        );
    let source_firewall =
        WorthTopologyMilestoneNineSourceFirewallReport::from_source_pairs_with_deletion_ledger(
            Vec::<(&str, &str)>::new(),
            &deletion_ledger,
        );

    let error = WorthTopologyMilestoneNineCloseout::from_operator_cutover_with_reports_for_tests(
        cutover.phase_eight_seed(),
        &cutover,
        deletion_ledger,
        residue_audit,
        source_firewall,
    )
    .expect_err("closeout should reject residue not backed by deletion ledger rows");

    assert_eq!(
        denial_kind(error),
        WorthTopologyMilestoneNineCloseoutDenialKind::StaleResidueWithoutDeletionLedger
    );
}

#[test]
fn selection_only_rows_are_denied() {
    let cutover = operator_cutover_closeout();
    let selection_only_rows = cutover
        .selected_obligation_closeout_rows()
        .iter()
        .map(|row| row.with_query_execution_status_for_tests("selected"))
        .collect::<Vec<_>>();
    let cutover = cutover.with_selected_rows_for_tests(selection_only_rows);

    let error = WorthTopologyMilestoneNineCloseout::from_operator_cutover(
        cutover.phase_eight_seed(),
        &cutover,
    )
    .expect_err("closeout should reject selection-only rows");

    assert_eq!(
        denial_kind(error),
        WorthTopologyMilestoneNineCloseoutDenialKind::SelectionOnlyProof
    );
}

fn denial_kind(
    error: WorthTopologyLegalityCatalogError,
) -> WorthTopologyMilestoneNineCloseoutDenialKind {
    match error {
        WorthTopologyLegalityCatalogError::MilestoneNineCloseout(denial) => denial.kind(),
        other => panic!("expected Milestone 9 closeout denial, got {other:?}"),
    }
}
