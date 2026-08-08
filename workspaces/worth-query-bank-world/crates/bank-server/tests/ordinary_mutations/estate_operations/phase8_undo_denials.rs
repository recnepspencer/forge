//! R8.39 irreversible undo denials through production admission.

use worth_query_host::facade::provisional_aftermath::{
    deny_irreversible_undo_attempt, WorthQueryUndoDenialKind, WorthQueryUndoDerivedRequest,
};

use super::disburse_estate::fixture::disbursement_world;
use super::phase8_undo_denial_support::{
    commit_disbursement, graph_snapshot, install_irreversible,
};
use crate::support::request_scope;

#[test]
fn released_estate_has_no_recovery_handle_lane_and_denies_without_writes() {
    let fixture = disbursement_world("undo-deny-released", 500);
    let _ = commit_disbursement(&fixture, 31);
    // Irreversible contracts have no handle mint or transition lane. Their
    // typed denial is classified directly from the installed contract.
    let irreversible = install_irreversible(&fixture.world.runtime);
    let before = graph_snapshot(&fixture);
    let denied = deny_irreversible_undo_attempt(&irreversible)
        .expect_err("irreversible contract has no undo lane");
    assert_eq!(denied.kind(), WorthQueryUndoDenialKind::ReleasedEstate);
    assert_eq!(
        graph_snapshot(&fixture),
        before,
        "undo denial must not mutate journals or activity"
    );
}

#[test]
fn compensatable_disbursement_undo_admits_as_positive_twin() {
    let fixture = disbursement_world("undo-deny-positive-twin", 500);
    let (specialist, receipt, _) = commit_disbursement(&fixture, 41);
    let handle = fixture
        .world
        .runtime
        .open_commit_recovery(&receipt)
        .expect("mint");
    let before = graph_snapshot(&fixture);
    let admission = fixture
        .world
        .runtime
        .admit_undo_disbursement_recovery(handle, &specialist, &request_scope())
        .expect("compensatable undo admits");
    assert_eq!(
        admission.derived_request(),
        WorthQueryUndoDerivedRequest::Compensation
    );
    assert_eq!(graph_snapshot(&fixture), before);
}
