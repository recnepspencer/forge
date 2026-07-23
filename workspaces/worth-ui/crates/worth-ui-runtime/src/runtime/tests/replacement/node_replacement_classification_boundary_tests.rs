use crate::runtime::{WorthUiAmbiguousReplacementDenial, WorthUiNodeLifecycleTransition};

use super::identity_match_graph_test_support::{
    artifact_from_nodes, component_node, component_node_with_descriptor, identity_match_app,
    runtime_and_narrowing, surface_node,
};
use super::node_replacement_classification_test_support::{
    ambiguous_identity_report_for, empty_lane_narrowing_for, lane_affecting_impact_for,
    lane_narrowing_for, narrowing_for, narrowing_for_identity, no_op_impact_for,
    structural_impact_for, structural_impact_for_identity,
};

#[test]
fn same_match_graph_produces_same_replacement_classification() {
    let app = identity_match_app();
    let active = artifact_from_nodes([(
        "app/main.wui",
        vec![
            component_node("component:dashboard", 0),
            surface_node("surface:main", "workspace.surface.main", 1),
        ],
    )]);
    let candidate = active.clone();

    let (runtime, admitted, identity_narrowing) = runtime_and_narrowing(&app, active, candidate);
    let identity_report = runtime
        .build_identity_match_graph(&identity_narrowing, &admitted)
        .expect("identity graph builds");
    let impact = no_op_impact_for(&identity_report);
    let narrowing = narrowing_for(&identity_report);

    let first = runtime
        .classify_node_replacements(&impact, &narrowing, &identity_report)
        .expect("node replacement plan builds");
    let second = runtime
        .classify_node_replacements(&impact, &narrowing, &identity_report)
        .expect("node replacement plan replays");

    assert_eq!(first, second);
    assert!(first.is_unambiguous());
    assert_eq!(first.counters().preserved_node_count(), 2);
    assert_eq!(first.counters().active_nodes_classified(), 2);
    assert_eq!(first.counters().candidate_nodes_classified(), 2);
    assert_eq!(
        first.transition_for_identity("component:dashboard"),
        Some(WorthUiNodeLifecycleTransition::Preserve)
    );
    assert!(classification_for_identity(&first, "component:dashboard")
        .unrestored_durable_state_carry_permitted());
}

#[test]
fn ambiguous_node_replacement_denied_before_reconciliation() {
    let app = identity_match_app();
    let active = artifact_from_nodes([(
        "app/main.wui",
        vec![component_node("component:dashboard", 0)],
    )]);
    let candidate = active.clone();

    let (runtime, admitted, identity_narrowing) = runtime_and_narrowing(&app, active, candidate);
    let identity_report = runtime
        .build_identity_match_graph(&identity_narrowing, &admitted)
        .expect("identity graph builds");
    let ambiguous_report = ambiguous_identity_report_for(&identity_report);
    let impact = no_op_impact_for(&ambiguous_report);
    let narrowing = narrowing_for(&ambiguous_report);

    let denial = runtime
        .classify_node_replacements(&impact, &narrowing, &ambiguous_report)
        .expect_err("ambiguous graph denies before reconciliation");

    match denial {
        WorthUiAmbiguousReplacementDenial::AmbiguousIdentityGraph { counters } => {
            assert_eq!(counters.ambiguous_node_count(), 1);
            assert_eq!(counters.preserved_node_count(), 0);
        }
        other => panic!("unexpected denial: {other:?}"),
    }
}

#[test]
fn lane_change_classified_separately_from_structural_replacement() {
    let app = identity_match_app();
    let active = artifact_from_nodes([(
        "app/main.wui",
        vec![surface_node("surface:stable", "workspace.surface.main", 0)],
    )]);
    let moved_candidate = artifact_from_nodes([(
        "app/panels.wui",
        vec![surface_node(
            "surface:stable",
            "workspace.surface.secondary",
            0,
        )],
    )]);

    let (runtime, admitted, identity_narrowing) =
        runtime_and_narrowing(&app, active.clone(), moved_candidate.clone());
    let identity_report = runtime
        .build_identity_match_graph(&identity_narrowing, &admitted)
        .expect("identity graph builds for move");
    let structural_impact = structural_impact_for(&identity_report);
    let structural_narrowing = narrowing_for(&identity_report);
    let moved_plan = runtime
        .classify_node_replacements(&structural_impact, &structural_narrowing, &identity_report)
        .expect("representation-only relocation classifies");

    let (lane_runtime, lane_admitted, lane_identity_narrowing) =
        runtime_and_narrowing(&app, active, moved_candidate);
    let lane_identity_report = lane_runtime
        .build_identity_match_graph(&lane_identity_narrowing, &lane_admitted)
        .expect("identity graph builds for lane change");
    let lane_impact = lane_affecting_impact_for(&lane_identity_report);
    let lane_narrowing = lane_narrowing_for(&lane_identity_report);
    let lane_plan = lane_runtime
        .classify_node_replacements(&lane_impact, &lane_narrowing, &lane_identity_report)
        .expect("lane change classifies");

    assert_eq!(
        moved_plan.transition_for_identity("surface:stable"),
        Some(WorthUiNodeLifecycleTransition::Replace)
    );
    assert_eq!(
        lane_plan.transition_for_identity("surface:stable"),
        Some(WorthUiNodeLifecycleTransition::LaneChange)
    );
    assert_eq!(moved_plan.counters().moved_node_count(), 0);
    assert_eq!(moved_plan.counters().replaced_node_count(), 1);
    assert_eq!(lane_plan.counters().lane_changed_node_count(), 1);
}

#[test]
fn lane_change_does_not_require_structural_move() {
    let app = identity_match_app();
    let active = artifact_from_nodes([(
        "app/main.wui",
        vec![surface_node("surface:stable", "workspace.surface.main", 0)],
    )]);
    let candidate = active.clone();

    let (runtime, admitted, identity_narrowing) = runtime_and_narrowing(&app, active, candidate);
    let identity_report = runtime
        .build_identity_match_graph(&identity_narrowing, &admitted)
        .expect("identity graph builds");
    let impact = lane_affecting_impact_for(&identity_report);
    let narrowing = lane_narrowing_for(&identity_report);
    let plan = runtime
        .classify_node_replacements(&impact, &narrowing, &identity_report)
        .expect("same-handle lane change classifies");

    assert_eq!(
        plan.transition_for_identity("surface:stable"),
        Some(WorthUiNodeLifecycleTransition::LaneChange)
    );
    assert_eq!(plan.counters().lane_changed_node_count(), 1);
    assert_eq!(plan.counters().moved_node_count(), 0);
    assert_eq!(plan.counters().replaced_node_count(), 0);
    assert!(!classification_for_identity(&plan, "surface:stable")
        .unrestored_durable_state_carry_permitted());
}

#[test]
fn structural_replacement_preserves_unaffected_matched_identities() {
    let app = identity_match_app();
    let active = artifact_from_nodes([(
        "app/main.wui",
        vec![
            component_node("component:affected", 0),
            surface_node("surface:unaffected", "workspace.surface.main", 1),
        ],
    )]);
    let candidate = active.clone();

    let (runtime, admitted, identity_narrowing) = runtime_and_narrowing(&app, active, candidate);
    let identity_report = runtime
        .build_identity_match_graph(&identity_narrowing, &admitted)
        .expect("identity graph builds");
    let impact = structural_impact_for_identity(&identity_report, "component:affected");
    let narrowing = narrowing_for_identity(&identity_report, "component:affected");
    let plan = runtime
        .classify_node_replacements(&impact, &narrowing, &identity_report)
        .expect("partial structural replacement classifies");

    assert_eq!(
        plan.transition_for_identity("component:affected"),
        Some(WorthUiNodeLifecycleTransition::Replace)
    );
    assert_eq!(
        plan.transition_for_identity("surface:unaffected"),
        Some(WorthUiNodeLifecycleTransition::Preserve)
    );
    assert_eq!(plan.counters().replaced_node_count(), 1);
    assert_eq!(plan.counters().preserved_node_count(), 1);
}

#[test]
fn lane_affecting_impact_without_lane_narrowing_denies_replacement_plan() {
    let app = identity_match_app();
    let active = artifact_from_nodes([(
        "app/main.wui",
        vec![surface_node("surface:stable", "workspace.surface.main", 0)],
    )]);
    let candidate = active.clone();

    let (runtime, admitted, identity_narrowing) = runtime_and_narrowing(&app, active, candidate);
    let identity_report = runtime
        .build_identity_match_graph(&identity_narrowing, &admitted)
        .expect("identity graph builds");
    let impact = lane_affecting_impact_for(&identity_report);
    let narrowing_without_lane_evidence = narrowing_for(&identity_report);

    let denial = runtime
        .classify_node_replacements(&impact, &narrowing_without_lane_evidence, &identity_report)
        .expect_err("lane-affecting impact requires lane narrowing evidence");

    assert!(matches!(
        denial,
        WorthUiAmbiguousReplacementDenial::LaneAffectingImpactWithoutLaneNarrowing { .. }
    ));
}

#[test]
fn lane_affecting_impact_without_affected_lane_scope_denies_replacement_plan() {
    let app = identity_match_app();
    let active = artifact_from_nodes([(
        "app/main.wui",
        vec![surface_node("surface:stable", "workspace.surface.main", 0)],
    )]);
    let candidate = active.clone();

    let (runtime, admitted, identity_narrowing) = runtime_and_narrowing(&app, active, candidate);
    let identity_report = runtime
        .build_identity_match_graph(&identity_narrowing, &admitted)
        .expect("identity graph builds");
    let impact = lane_affecting_impact_for(&identity_report);
    let empty_lane_narrowing = empty_lane_narrowing_for(&identity_report);

    let denial = runtime
        .classify_node_replacements(&impact, &empty_lane_narrowing, &identity_report)
        .expect_err("lane-affecting impact requires affected lane scope");

    assert!(matches!(
        denial,
        WorthUiAmbiguousReplacementDenial::LaneAffectingImpactWithoutAffectedLaneScope { .. }
    ));
}

#[test]
fn drop_then_create_same_structural_path_does_not_claim_preserve() {
    let app = identity_match_app();
    let active = artifact_from_nodes([(
        "app/main.wui",
        vec![component_node_with_descriptor(
            "component:dashboard:old",
            "workspace.component.dashboard",
            0,
        )],
    )]);
    let candidate = artifact_from_nodes([(
        "app/main.wui",
        vec![component_node_with_descriptor(
            "component:dashboard:new",
            "workspace.component.dashboard",
            0,
        )],
    )]);

    let (runtime, admitted, identity_narrowing) = runtime_and_narrowing(&app, active, candidate);
    let identity_report = runtime
        .build_identity_match_graph(&identity_narrowing, &admitted)
        .expect("identity graph builds");
    let impact = structural_impact_for(&identity_report);
    let narrowing = narrowing_for(&identity_report);
    let plan = runtime
        .classify_node_replacements(&impact, &narrowing, &identity_report)
        .expect("drop/create plan builds");

    assert_eq!(
        plan.transition_for_identity("component:dashboard:old"),
        Some(WorthUiNodeLifecycleTransition::Drop)
    );
    assert_eq!(
        plan.transition_for_identity("component:dashboard:new"),
        Some(WorthUiNodeLifecycleTransition::Create)
    );
    assert_eq!(plan.counters().preserved_node_count(), 0);
    assert_eq!(plan.counters().dropped_node_count(), 1);
    assert_eq!(plan.counters().created_node_count(), 1);
    assert!(
        !classification_for_identity(&plan, "component:dashboard:old")
            .unrestored_durable_state_carry_permitted()
    );
    assert!(
        !classification_for_identity(&plan, "component:dashboard:new")
            .unrestored_durable_state_carry_permitted()
    );
}

#[test]
fn digest_mismatch_between_identity_and_narrowing_denies_node_replacement() {
    let app = identity_match_app();
    let active = artifact_from_nodes([(
        "app/main.wui",
        vec![component_node("component:dashboard", 0)],
    )]);
    let candidate = active.clone();

    let (runtime, admitted, identity_narrowing) = runtime_and_narrowing(&app, active, candidate);
    let identity_report = runtime
        .build_identity_match_graph(&identity_narrowing, &admitted)
        .expect("identity graph builds");
    let impact = no_op_impact_for(&identity_report);
    let stale_narrowing = narrowing_for(&crate::runtime::WorthUiIdentityMatchReport::new(
        identity_report.active_artifact_digest(),
        identity_report.candidate_artifact_digest() + 1,
        identity_report.graph().clone(),
    ));

    let denial = runtime
        .classify_node_replacements(&impact, &stale_narrowing, &identity_report)
        .expect_err("mismatched narrowing denies");

    assert!(matches!(
        denial,
        WorthUiAmbiguousReplacementDenial::NarrowingDigestMismatch { .. }
    ));
}

fn classification_for_identity<'a>(
    plan: &'a crate::runtime::WorthUiNodeReplacementPlan,
    identity_basis: &str,
) -> &'a crate::runtime::WorthUiNodeReplacementClassification {
    plan.classifications()
        .iter()
        .find(|classification| classification.identity_basis() == identity_basis)
        .expect("classification exists")
}
