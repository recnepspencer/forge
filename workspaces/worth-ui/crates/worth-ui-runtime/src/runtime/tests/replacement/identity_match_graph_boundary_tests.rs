use crate::runtime::{
    WorthUiIdentityMatchDenial, WorthUiIdentityMatchNodeKind, WorthUiPlanTopologyDenialReason,
    WorthUiRuntimeHandleAllocationDenialReason, WorthUiRuntimeLaunch, WorthUiRuntimeLaunchDenial,
};

use super::identity_match_graph_test_support::{
    artifact_from_nodes, component_node, component_node_with_descriptor, identity_match_app,
    runtime_and_narrowing, surface_node,
};

#[test]
fn same_identity_seeds_produce_same_match_graph() {
    let app = identity_match_app();
    let active = artifact_from_nodes([(
        "app/main.wui",
        vec![
            component_node("component:dashboard", 0),
            surface_node("surface:main", "workspace.surface.main", 1),
        ],
    )]);
    let candidate = active.clone();

    let (runtime, admitted, narrowing) = runtime_and_narrowing(&app, active, candidate);
    let report = runtime
        .build_identity_match_graph(&narrowing, &admitted)
        .expect("identity match graph builds");

    assert_eq!(report.graph().match_count(), 2);
    assert!(report.graph().is_unambiguous());
    assert_eq!(report.counters().matches_emitted(), 2);
    assert_eq!(report.counters().unmatched_active_count(), 0);
    assert_eq!(report.counters().unmatched_candidate_count(), 0);
}

#[test]
fn duplicate_candidate_identity_rejected_before_state_reconciliation() {
    let app = identity_match_app();
    let active = artifact_from_nodes([(
        "app/main.wui",
        vec![component_node("component:dashboard", 0)],
    )]);
    let candidate = artifact_from_nodes([(
        "app/main.wui",
        vec![
            component_node("component:dashboard", 0),
            component_node("component:dashboard", 1),
        ],
    )]);

    let (runtime, admitted, narrowing) = runtime_and_narrowing(&app, active, candidate);
    let denial = runtime
        .build_identity_match_graph(&narrowing, &admitted)
        .expect_err("duplicate candidate identity rejects");

    match denial {
        WorthUiIdentityMatchDenial::DuplicateCandidateIdentity {
            identity_basis,
            counters,
            ..
        } => {
            assert_eq!(identity_basis, "component:dashboard");
            assert_eq!(counters.duplicate_candidate_identity_count(), 1);
        }
        other => panic!("unexpected denial: {other:?}"),
    }
}

#[test]
fn duplicate_active_identity_cannot_enter_the_runtime() {
    let app = identity_match_app();
    let active = artifact_from_nodes([(
        "app/main.wui",
        vec![
            component_node("component:dashboard", 0),
            component_node("component:dashboard", 1),
        ],
    )]);
    let denial = app
        .launch_runtime(WorthUiRuntimeLaunch::from_canonical_artifact(active))
        .expect_err("duplicate active identity must be rejected before runtime admission");

    assert!(matches!(
        denial,
        WorthUiRuntimeLaunchDenial::HandleAllocation(denial)
            if denial.reason()
                == WorthUiRuntimeHandleAllocationDenialReason::DuplicatePlanLocalHandleClaim
    ));
}

#[test]
fn source_reordering_does_not_change_identity_match_graph() {
    let app = identity_match_app();
    let active = artifact_from_nodes([
        (
            "app/main.wui",
            vec![
                component_node("component:dashboard", 0),
                surface_node("surface:main", "workspace.surface.main", 1),
            ],
        ),
        (
            "app/secondary.wui",
            vec![component_node("component:inspector", 0)],
        ),
    ]);
    let candidate = artifact_from_nodes([
        (
            "app/secondary.wui",
            vec![component_node("component:inspector", 0)],
        ),
        (
            "app/main.wui",
            vec![
                surface_node("surface:main", "workspace.surface.main", 0),
                component_node("component:dashboard", 1),
            ],
        ),
    ]);

    let (runtime, admitted, narrowing) = runtime_and_narrowing(&app, active, candidate);
    let report = runtime
        .build_identity_match_graph(&narrowing, &admitted)
        .expect("reordered graph matches by seed");

    assert_eq!(report.graph().match_count(), 3);
    assert_eq!(report.graph().moved_node_count(), 2);
    assert_eq!(report.counters().unmatched_active_count(), 0);
    assert_eq!(report.counters().unmatched_candidate_count(), 0);
}

#[test]
fn same_label_same_component_different_identity_does_not_preserve_state() {
    let app = identity_match_app();
    let active = artifact_from_nodes([(
        "app/main.wui",
        vec![component_node_with_descriptor(
            "component:dashboard:left",
            "workspace.component.dashboard",
            0,
        )],
    )]);
    let candidate = artifact_from_nodes([(
        "app/main.wui",
        vec![component_node_with_descriptor(
            "component:dashboard:right",
            "workspace.component.dashboard",
            0,
        )],
    )]);

    let (runtime, admitted, narrowing) = runtime_and_narrowing(&app, active, candidate);
    let report = runtime
        .build_identity_match_graph(&narrowing, &admitted)
        .expect("different identity does not match");

    assert_eq!(report.graph().match_count(), 0);
    assert_eq!(report.counters().unmatched_active_count(), 1);
    assert_eq!(report.counters().unmatched_candidate_count(), 1);
}

#[test]
fn same_identity_basis_across_node_kinds_is_rejected_before_state_reconciliation() {
    let app = identity_match_app();
    let active =
        artifact_from_nodes([("app/main.wui", vec![component_node("identity:shared", 0)])]);
    let candidate = artifact_from_nodes([(
        "app/main.wui",
        vec![surface_node("identity:shared", "workspace.surface.main", 0)],
    )]);

    let (runtime, admitted, narrowing) = runtime_and_narrowing(&app, active, candidate);
    let denial = runtime
        .build_identity_match_graph(&narrowing, &admitted)
        .expect_err("same basis across node kinds rejects");

    match denial {
        WorthUiIdentityMatchDenial::IdentityKindMismatch {
            identity_basis,
            active_kind,
            candidate_kind,
            counters,
            ..
        } => {
            assert_eq!(identity_basis, "identity:shared");
            assert_eq!(active_kind, WorthUiIdentityMatchNodeKind::Component);
            assert_eq!(candidate_kind, WorthUiIdentityMatchNodeKind::Surface);
            assert_eq!(counters.identity_kind_mismatch_count(), 1);
            assert_eq!(counters.matches_emitted(), 0);
        }
        other => panic!("unexpected denial: {other:?}"),
    }
}

#[test]
fn duplicate_candidate_basis_across_node_kinds_is_rejected_as_kind_ambiguity() {
    let app = identity_match_app();
    let active =
        artifact_from_nodes([("app/main.wui", vec![component_node("identity:shared", 0)])]);
    let candidate = artifact_from_nodes([(
        "app/main.wui",
        vec![
            component_node("identity:shared", 0),
            surface_node("identity:shared", "workspace.surface.main", 1),
        ],
    )]);

    let (runtime, admitted, narrowing) = runtime_and_narrowing(&app, active, candidate);
    let denial = runtime
        .build_identity_match_graph(&narrowing, &admitted)
        .expect_err("same candidate basis across node kinds rejects as kind ambiguity");

    match denial {
        WorthUiIdentityMatchDenial::CandidateIdentityKindMismatch {
            identity_basis,
            first_kind,
            second_kind,
            counters,
            ..
        } => {
            assert_eq!(identity_basis, "identity:shared");
            assert_eq!(first_kind, WorthUiIdentityMatchNodeKind::Component);
            assert_eq!(second_kind, WorthUiIdentityMatchNodeKind::Surface);
            assert_eq!(counters.identity_kind_mismatch_count(), 1);
            assert_eq!(counters.duplicate_candidate_identity_count(), 0);
        }
        other => panic!("unexpected denial: {other:?}"),
    }
}

#[test]
fn duplicate_active_basis_across_node_kinds_cannot_enter_the_active_plan() {
    let app = identity_match_app();
    let active = artifact_from_nodes([(
        "app/main.wui",
        vec![
            component_node("identity:shared", 0),
            surface_node("identity:shared", "workspace.surface.main", 1),
        ],
    )]);
    let denial = app
        .launch_runtime(WorthUiRuntimeLaunch::from_canonical_artifact(active))
        .expect_err("an ambiguous identity cannot become active plan truth");

    assert!(matches!(
        denial,
        WorthUiRuntimeLaunchDenial::TopologyAssembly(denial)
            if denial.reason() == WorthUiPlanTopologyDenialReason::DuplicateRegionIdentity
    ));
}

#[test]
fn repeated_template_items_preserve_only_keyed_identity() {
    let app = identity_match_app();
    let active = artifact_from_nodes([(
        "app/main.wui",
        vec![
            component_node("template:row|item:a", 0),
            component_node("template:row|item:b", 1),
        ],
    )]);
    let candidate = artifact_from_nodes([(
        "app/main.wui",
        vec![
            component_node("template:row|item:b", 0),
            component_node("template:row|item:a", 1),
        ],
    )]);

    let (runtime, admitted, narrowing) = runtime_and_narrowing(&app, active, candidate);
    let report = runtime
        .build_identity_match_graph(&narrowing, &admitted)
        .expect("keyed template identity matches");

    assert_eq!(report.graph().match_count(), 2);
    assert_eq!(report.graph().repeated_template_identity_count(), 2);
    assert_eq!(report.graph().moved_node_count(), 2);
}

#[test]
fn position_only_template_identity_is_rejected() {
    let app = identity_match_app();
    let active = artifact_from_nodes([(
        "app/main.wui",
        vec![component_node("template:row|item:a", 0)],
    )]);
    let candidate = artifact_from_nodes([(
        "app/main.wui",
        vec![component_node("template:row|position:0", 0)],
    )]);

    let (runtime, admitted, narrowing) = runtime_and_narrowing(&app, active, candidate);
    let denial = runtime
        .build_identity_match_graph(&narrowing, &admitted)
        .expect_err("position-only template identity rejects");

    assert!(matches!(
        denial,
        WorthUiIdentityMatchDenial::PositionOnlyRepeatedTemplateIdentity { .. }
    ));
}

#[test]
fn moved_node_identity_survives_parent_or_region_change_when_seed_is_stable() {
    let app = identity_match_app();
    let active = artifact_from_nodes([(
        "app/main.wui",
        vec![surface_node(
            "surface:stable-inspector",
            "workspace.surface.main",
            0,
        )],
    )]);
    let candidate = artifact_from_nodes([(
        "app/inspector.wui",
        vec![surface_node(
            "surface:stable-inspector",
            "workspace.surface.secondary",
            0,
        )],
    )]);

    let (runtime, admitted, narrowing) = runtime_and_narrowing(&app, active, candidate);
    let report = runtime
        .build_identity_match_graph(&narrowing, &admitted)
        .expect("stable moved node matches");

    assert_eq!(report.graph().match_count(), 1);
    assert_eq!(report.graph().moved_node_count(), 1);
    assert!(report.graph().moved_node_identities()[0].crossed_module_boundary());
}
