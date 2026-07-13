use super::super::super::support::*;
use super::fixtures::local_identity_read;
use crate::ordinary::read::{current, declare, WorthQueryReadOutcome};
use crate::runtime::{
    admit_graph_read_access_authority, WorthQueryGraphReadAccessAuthorityRequest,
    WorthQueryReadBuilder, WorthQueryReadFamily,
};

#[test]
fn ordinary_read_matches_internal_phase_chain_result_and_receipt_identity() {
    let declaration = declare(local_identity_read).expect("ordinary declaration should build");
    let declaration_identity = declaration.identity().as_str().to_string();
    let mut workspace = read_runtime()
        .workspace("ordinary-read-parity")
        .expect("ordinary workspace should open");
    let ordinary = declaration
        .using(current())
        .run(&mut workspace)
        .into_result()
        .expect("ordinary read should execute")
        .into_result();
    let oracle_read_graph = local_identity_read(WorthQueryReadBuilder::new())
        .expect("internal oracle declaration should build");
    let oracle_family = WorthQueryReadFamily::new_kernel_only("declared_read", oracle_read_graph);
    let oracle_authority = admit_graph_read_access_authority(
        WorthQueryGraphReadAccessAuthorityRequest::current_head(),
    )
    .expect("internal oracle current authority should admit");
    let oracle = workspace
        .read_family_intent_in_graph_read_authority(&oracle_family, &oracle_authority)
        .execute()
        .expect("internal phase-chain oracle should execute");

    assert_eq!(declaration_identity, ordinary.receipt().read_graph_digest());
    assert_eq!(
        ordinary
            .receipt()
            .graph_read_access_plan()
            .expect("ordinary execution must carry its admitted plan")
            .digest(),
        oracle
            .receipt()
            .graph_read_access_plan()
            .expect("internal execution must carry its admitted plan")
            .digest()
    );
    assert_eq!(
        ordinary.receipt().graph_read_access_complexity_counters(),
        oracle.receipt().graph_read_access_complexity_counters()
    );
    assert_eq!(ordinary.receipt(), oracle.receipt());
    assert_eq!(ordinary, oracle);
}

#[test]
fn ordinary_read_exposes_success_without_phase_artifacts() {
    let declaration = declare(local_identity_read).expect("ordinary declaration should build");
    let mut workspace = read_runtime()
        .workspace("ordinary-read-outcome")
        .expect("ordinary workspace should open");

    match declaration.using(current()).run(&mut workspace) {
        WorthQueryReadOutcome::Completed(completion) => {
            assert!(!completion.result().receipt().query_digest().is_empty());
            assert_eq!(
                completion
                    .context_receipt()
                    .counters()
                    .graph_authority_admitted_count(),
                1
            );
        }
        WorthQueryReadOutcome::Stopped(stop) => {
            panic!("ordinary read unexpectedly stopped: {:?}", stop.source())
        }
    }
}
