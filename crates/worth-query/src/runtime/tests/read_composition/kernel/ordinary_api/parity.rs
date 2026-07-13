use super::super::super::support::*;
use super::fixtures::{
    anchored_manager_graph_read, current_bounded_manager_relationship_context,
    current_manager_relationship_context, local_identity_collection_read, local_identity_read,
    local_manager_relationship_read,
};
use crate::ordinary::read::{
    admit_read_context_declaration, current, declare, WorthQueryReadOutcome,
};
use crate::runtime::{
    WorthQueryReadBuilder, WorthQueryReadFamily, WorthQueryReadGraphFamily,
    WorthQueryReadOperatorFamily,
};

macro_rules! assert_ordinary_internal_parity {
    ($author:path, $context:expr, $workspace_name:literal) => {{
        let declaration = declare($author).expect("ordinary declaration should build");
        let mut workspace = read_runtime()
            .workspace($workspace_name)
            .expect("ordinary workspace should open");
        let ordinary = declaration
            .using($context)
            .run(&mut workspace)
            .into_result()
            .expect("ordinary read should execute")
            .into_result();

        let oracle_intent = $author(WorthQueryReadBuilder::declaration())
            .expect("internal oracle declaration should build");
        let admitted_context = admit_read_context_declaration(&oracle_intent, $context.into())
            .expect("internal oracle context should admit");
        let (oracle_authority, oracle_planning_authority, _) = admitted_context.into_parts();
        let oracle_read_graph = oracle_intent
            .plan(oracle_planning_authority)
            .expect("internal oracle should plan");
        let oracle_plan_digest = oracle_read_graph
            .execution_plan()
            .query()
            .plan_digest()
            .as_str()
            .to_string();
        let oracle_family =
            WorthQueryReadFamily::new_kernel_only("declared_read", oracle_read_graph);
        let oracle = workspace
            .read_family_intent_in_graph_read_authority(&oracle_family, &oracle_authority)
            .execute()
            .expect("internal phase-chain oracle should execute");

        assert_eq!(
            ordinary.receipt().execution_plan_digest(),
            oracle_plan_digest
        );
        assert_eq!(ordinary.receipt(), oracle.receipt());
        assert_eq!(ordinary, oracle);
        ordinary
    }};
}

#[test]
fn ordinary_read_matches_internal_phase_chain_result_and_receipt_identity() {
    let declaration = declare(local_identity_read).expect("ordinary declaration should build");
    let declaration_identity = declaration.identity().as_str().to_string();
    let ordinary = assert_ordinary_internal_parity!(
        local_identity_read,
        current(),
        "ordinary-read-detail-parity"
    );

    assert_ne!(declaration_identity, ordinary.receipt().read_graph_digest());
    assert!(!ordinary.receipt().execution_plan_digest().is_empty());
    assert!(ordinary.receipt().graph_read_access_plan().is_some());
}

#[test]
fn ordinary_collection_preserves_family_and_matches_internal_phase_chain() {
    let result = assert_ordinary_internal_parity!(
        local_identity_collection_read,
        current(),
        "ordinary-read-collection-parity"
    );

    assert_eq!(
        result.receipt().graph_family(),
        &WorthQueryReadGraphFamily::Collection
    );
    assert!(result.rows().is_empty());
    assert_eq!(
        result.receipt().breadth().planned_traversal_clause_count(),
        0
    );
}

#[test]
fn ordinary_graph_read_preserves_traversal_and_matches_internal_phase_chain() {
    let result = assert_ordinary_internal_parity!(
        anchored_manager_graph_read,
        current_bounded_manager_relationship_context(),
        "ordinary-read-graph-parity"
    );

    assert_eq!(
        result.receipt().graph_family(),
        &WorthQueryReadGraphFamily::Collection
    );
    assert!(result
        .receipt()
        .operator_families()
        .contains(&WorthQueryReadOperatorFamily::Traversal));
    assert_eq!(
        result.receipt().breadth().planned_traversal_clause_count(),
        1
    );
    assert_eq!(
        result.receipt().breadth().planned_traversal_depth_limit(),
        2
    );
}

#[test]
fn ordinary_composed_read_matches_internal_phase_chain_without_scalarizing_evidence() {
    let result = assert_ordinary_internal_parity!(
        local_manager_relationship_read,
        current_manager_relationship_context(),
        "ordinary-read-composed-parity"
    );

    assert_eq!(
        result.receipt().graph_family(),
        &WorthQueryReadGraphFamily::Detail
    );
    assert!(!result.receipt().built_in_operator_coverage().is_empty());
    assert_eq!(
        result.receipt().breadth().planned_traversal_clause_count(),
        1
    );
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
