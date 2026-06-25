use crate::graph_read_access_plan_adoption::{
    current_worth_graph_read_access_plan_adoption_closeout,
    WorthGraphReadAccessPlanAdoptionCloseoutErrorKind,
};

use super::phase_chain_fixture::production_phase_eight_seed;

#[test]
fn closeout_rejects_empty_receipt_proof() {
    let seed = production_phase_eight_seed().with_empty_receipt_accounting_for_tests();

    let error = current_worth_graph_read_access_plan_adoption_closeout(&seed)
        .expect_err("Phase 8 closeout must reject missing receipt/posture proof");

    assert_eq!(
        error.kind(),
        WorthGraphReadAccessPlanAdoptionCloseoutErrorKind::MissingReceiptOrPostureProof
    );
}

#[test]
fn closeout_rejects_pending_admitted_plan_without_receipt_or_visible_posture() {
    let seed = production_phase_eight_seed().with_only_pending_admitted_receipts_for_tests();

    let error = current_worth_graph_read_access_plan_adoption_closeout(&seed)
        .expect_err("pending admitted plans alone cannot satisfy Phase 8 closeout");

    assert_eq!(
        error.kind(),
        WorthGraphReadAccessPlanAdoptionCloseoutErrorKind::MissingReceiptOrPostureProof
    );
}

#[test]
fn closeout_rejects_empty_counter_proof() {
    let seed = production_phase_eight_seed().with_empty_counter_accounting_for_tests();

    let error = current_worth_graph_read_access_plan_adoption_closeout(&seed)
        .expect_err("Phase 8 closeout must reject missing counter proof");

    assert_eq!(
        error.kind(),
        WorthGraphReadAccessPlanAdoptionCloseoutErrorKind::MissingCounterProof
    );
}

#[test]
fn closeout_rejects_empty_batch_proof() {
    let seed = production_phase_eight_seed().with_empty_batch_accounting_for_tests();

    let error = current_worth_graph_read_access_plan_adoption_closeout(&seed)
        .expect_err("Phase 8 closeout must reject missing batch accounting proof");

    assert_eq!(
        error.kind(),
        WorthGraphReadAccessPlanAdoptionCloseoutErrorKind::MissingBatchAccountingProof
    );
}

#[test]
fn closeout_rejects_lost_batch_receipt_association() {
    let seed = production_phase_eight_seed().with_lost_batch_receipt_association_for_tests();

    let error = current_worth_graph_read_access_plan_adoption_closeout(&seed)
        .expect_err("Phase 8 closeout must reject lost per-read receipt association");

    assert_eq!(
        error.kind(),
        WorthGraphReadAccessPlanAdoptionCloseoutErrorKind::BatchCounterReceiptAssociationLost
    );
}

#[test]
fn closeout_rejects_caller_owned_graph_work() {
    let seed = production_phase_eight_seed().with_caller_owned_graph_work_for_tests();

    let error = current_worth_graph_read_access_plan_adoption_closeout(&seed)
        .expect_err("Phase 8 closeout must reject caller-owned graph work");

    assert_eq!(
        error.kind(),
        WorthGraphReadAccessPlanAdoptionCloseoutErrorKind::CallerOwnedGraphWorkDetected
    );
}

#[test]
fn closeout_rejects_unresolved_deletion_proof() {
    let seed = production_phase_eight_seed().with_unresolved_deletion_for_tests();

    let error = current_worth_graph_read_access_plan_adoption_closeout(&seed)
        .expect_err("Phase 8 closeout must reject unresolved deletion proof");

    assert_eq!(
        error.kind(),
        WorthGraphReadAccessPlanAdoptionCloseoutErrorKind::UnresolvedDeletionProof
    );
}

#[test]
fn closeout_rejects_uncapped_residue() {
    let seed = production_phase_eight_seed().with_uncapped_residue_for_tests();

    let error = current_worth_graph_read_access_plan_adoption_closeout(&seed)
        .expect_err("Phase 8 closeout must reject uncapped residue");

    assert_eq!(
        error.kind(),
        WorthGraphReadAccessPlanAdoptionCloseoutErrorKind::UncappedResidue
    );
}

#[test]
fn closeout_rejects_source_firewall_violations() {
    let seed = production_phase_eight_seed().with_source_firewall_violation_for_tests();

    let error = current_worth_graph_read_access_plan_adoption_closeout(&seed)
        .expect_err("Phase 8 closeout must reject source firewall violations");

    assert_eq!(
        error.kind(),
        WorthGraphReadAccessPlanAdoptionCloseoutErrorKind::SourceFirewallViolation
    );
}
