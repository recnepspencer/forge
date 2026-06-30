use crate::runtime::{WorthUiIdentityMatchNodeKind, WorthUiIdentityMatchNodeSide};

use super::identity_match_graph_test_support::{
    admitted_with_runtime, artifact_from_nodes, component_node, identity_match_app,
    runtime_and_narrowing,
};

#[test]
fn mismatched_narrowing_candidate_is_rejected() {
    let app = identity_match_app();
    let active = artifact_from_nodes([(
        "app/main.wui",
        vec![component_node("component:dashboard", 0)],
    )]);
    let candidate = artifact_from_nodes([(
        "app/main.wui",
        vec![component_node("component:dashboard", 0)],
    )]);
    let other_candidate =
        artifact_from_nodes([("app/main.wui", vec![component_node("component:other", 0)])]);

    let (runtime, admitted, narrowing) = runtime_and_narrowing(&app, active.clone(), candidate);
    let (_, other_admitted) = admitted_with_runtime(&app, active, other_candidate);
    let denial = runtime
        .build_identity_match_graph(&narrowing, &other_admitted)
        .expect_err("mismatched candidate rejects");

    assert!(matches!(
        denial,
        crate::runtime::WorthUiIdentityMatchDenial::NarrowingCandidateMismatch { .. }
    ));
    let report = runtime
        .build_identity_match_graph(&narrowing, &admitted)
        .expect("original admitted candidate still matches");
    assert_eq!(report.graph().match_count(), 1);
}

#[test]
fn match_nodes_report_side_kind_and_durable_state_without_public_handles() {
    let app = identity_match_app();
    let active = artifact_from_nodes([(
        "app/main.wui",
        vec![component_node("component:dashboard", 0)],
    )]);
    let candidate = active.clone();

    let (runtime, admitted, narrowing) = runtime_and_narrowing(&app, active, candidate);
    let report = runtime
        .build_identity_match_graph(&narrowing, &admitted)
        .expect("identity match graph builds");
    let active_node = &report.graph().active_nodes()[0];
    let candidate_node = &report.graph().candidate_nodes()[0];
    let matched_edge = &report.graph().matches()[0];

    assert_eq!(matched_edge.identity_basis(), "component:dashboard");
    assert!(!matched_edge.moved_between_handles());
    assert_eq!(report.graph().match_count(), 1);
    assert_eq!(report.graph().active_node_count(), 1);
    assert_eq!(report.graph().candidate_node_count(), 1);
    assert_eq!(active_node.identity_basis(), "component:dashboard");
    assert_eq!(candidate_node.identity_basis(), "component:dashboard");
    assert_eq!(active_node.side(), WorthUiIdentityMatchNodeSide::Active);
    assert_eq!(
        candidate_node.side(),
        WorthUiIdentityMatchNodeSide::Candidate
    );
    assert_eq!(active_node.kind(), WorthUiIdentityMatchNodeKind::Component);
    assert_eq!(
        candidate_node.kind(),
        WorthUiIdentityMatchNodeKind::Component
    );
    assert!(active_node.stable_identity());
    assert!(candidate_node.stable_identity());
    assert!(active_node.durable_state_eligible());
    assert!(candidate_node.durable_state_eligible());
}
