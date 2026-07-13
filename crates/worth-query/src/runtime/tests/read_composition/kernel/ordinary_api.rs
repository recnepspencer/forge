use super::super::support::*;
use crate::authoring::{AspectFieldSelector, AuthoredResultShapeField};
use crate::ordinary::read::{declare, WorthQueryReadOutcome};

#[test]
fn ordinary_read_matches_internal_phase_chain_result_and_receipt_identity() {
    let declaration = declare(local_identity_read).expect("ordinary declaration should build");
    let declaration_identity = declaration.identity().as_str().to_string();
    let mut ordinary_workspace = read_runtime()
        .workspace("ordinary-read-parity")
        .expect("ordinary workspace should open");
    let ordinary = declaration
        .run(&mut ordinary_workspace)
        .into_result()
        .expect("ordinary read should execute");

    let mut oracle_workspace = read_runtime()
        .workspace("internal-read-parity")
        .expect("oracle workspace should open");
    let oracle = oracle_workspace
        .compose_read(local_identity_read)
        .expect("internal phase-chain oracle should execute");

    assert_eq!(declaration_identity, ordinary.receipt().read_graph_digest());
    assert_eq!(ordinary.rows(), oracle.rows());
    assert_eq!(
        ordinary.receipt().query_digest(),
        oracle.receipt().query_digest()
    );
    assert_eq!(
        ordinary.receipt().read_graph_digest(),
        oracle.receipt().read_graph_digest()
    );
    assert_eq!(ordinary.receipt().breadth(), oracle.receipt().breadth());
}

#[test]
fn ordinary_read_exposes_success_without_phase_artifacts() {
    let declaration = declare(local_identity_read).expect("ordinary declaration should build");
    let mut workspace = read_runtime()
        .workspace("ordinary-read-outcome")
        .expect("ordinary workspace should open");

    match declaration.run(&mut workspace) {
        WorthQueryReadOutcome::Completed(result) => {
            assert!(!result.receipt().query_digest().is_empty());
        }
        WorthQueryReadOutcome::Stopped(stop) => {
            panic!("ordinary read unexpectedly stopped: {:?}", stop.source())
        }
    }
}

fn local_identity_read(
    read: crate::runtime::WorthQueryReadBuilder,
) -> Result<crate::runtime::WorthQueryReadGraph, crate::runtime::WorthQueryReadDenial> {
    read.local_detail(
        "user",
        manager_schema(),
        |query| {
            query.project(
                AspectFieldSelector::new("identity", "id")
                    .expect("identity projection should build"),
            )
        },
        |shape| {
            shape.field(
                AuthoredResultShapeField::new("identity", "id", "id")
                    .expect("identity result field should build"),
            )
        },
    )
}
