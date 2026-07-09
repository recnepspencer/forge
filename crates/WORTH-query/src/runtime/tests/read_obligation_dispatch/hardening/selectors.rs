use super::super::*;
use super::dispatch_catalog::workspace_with_selector_obligation;
use super::front_parity::{helper_parity_result, HelperFront};
use super::read_declarations::{profile_read_declaration, traversal_read_family};

#[test]
fn read_descriptor_matches_declared_aspect_and_read_shape_verbs() {
    let mut aspect_workspace = workspace_with_selector_obligation(
        "read-aspect-selector",
        WorthQueryGraphTouchSelector::aspect_touch(test_aspect_touch("profile.display_name")),
        WorthQueryGraphObligationSupportLane::ReadFamily,
        WorthQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
    );
    let aspect_family = aspect_workspace
        .define_read_family("profile-read", profile_read_declaration)
        .expect("profile family should define");
    let aspect_result = aspect_workspace
        .execute_read_family(&aspect_family)
        .expect("declared aspect selector should match");
    assert_eq!(
        aspect_result
            .receipt()
            .graph_obligation_dispatch()
            .unwrap()
            .selection()
            .matched_obligation_count(),
        1
    );

    let mut verb_workspace = workspace_with_selector_obligation(
        "read-verb-selector",
        WorthQueryGraphTouchSelector::read_verb(WorthQueryGraphTouchReadVerb::ObservesAspect),
        WorthQueryGraphObligationSupportLane::ReadFamily,
        WorthQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
    );
    let verb_family = verb_workspace
        .define_read_family("profile-read", profile_read_declaration)
        .expect("profile family should define");
    let verb_result = verb_workspace
        .execute_read_family(&verb_family)
        .expect("aspect-read verb selector should match");
    assert_eq!(
        verb_result
            .receipt()
            .graph_obligation_dispatch()
            .unwrap()
            .selection()
            .matched_obligation_count(),
        1
    );

    let mut traversal_workspace = workspace_with_selector_obligation(
        "read-relation-verb-selector",
        WorthQueryGraphTouchSelector::read_verb(WorthQueryGraphTouchReadVerb::ObservesRelationKind),
        WorthQueryGraphObligationSupportLane::ReadFamily,
        WorthQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
    );
    let traversal_family = traversal_read_family(&mut traversal_workspace);
    let traversal_result = traversal_workspace
        .execute_read_family(&traversal_family)
        .expect("traversal read verb selector should match real traversal reads");
    assert_eq!(
        traversal_result
            .receipt()
            .graph_obligation_dispatch()
            .unwrap()
            .selection()
            .matched_obligation_count(),
        1
    );
}

#[test]
fn helper_fronts_ignore_unrelated_wrong_world_wrong_lane_and_mutation_only_obligations() {
    let execute = helper_parity_result("helper-parity-execute", HelperFront::Execute);
    let intent = helper_parity_result("helper-parity-intent", HelperFront::Intent);
    let compose = helper_parity_result("helper-parity-compose", HelperFront::Compose);

    assert_eq!(execute, intent);
    assert_eq!(intent, compose);
    assert_eq!(compose.matched_count, 1);
    assert_eq!(compose.full_scan_count, 0);
}
