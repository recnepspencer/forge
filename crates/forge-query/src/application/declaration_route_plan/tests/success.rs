use crate::target_binding::ForgeQueryBindingTargetWitness;

use super::domain::{
    admitted_handle, route_input, AspectRichRouteFamily, MixedRouteFamily, RelationalRouteFamily,
    RouteInput,
};

#[test]
fn route_plan_common_lane_reads_like_intent() {
    let plan = admitted_handle("primary")
        .declare_review_progress_describe_and_plan(RouteInput::<RelationalRouteFamily>::new(
            "edge:42",
        ))
        .unwrap_or_else(|_| panic!("route plan should admit"));

    assert_eq!(plan.route_count(), 1);
    assert_eq!(plan.declaration_family_key(), "RelationalRouteFamily");
}

#[test]
fn explicit_and_common_paths_converge_on_one_route_plan_digest() {
    let handle = admitted_handle("primary");
    let explicit = handle
        .plan_routes(route_input(
            &handle,
            RouteInput::<MixedRouteFamily>::new("edge:42"),
        ))
        .unwrap_or_else(|_| panic!("explicit route planning should succeed"));
    let common = handle
        .declare_review_progress_describe_and_plan(RouteInput::<MixedRouteFamily>::new("edge:42"))
        .unwrap_or_else(|_| panic!("common route planning should succeed"));

    assert_eq!(explicit.route_plan_digest(), common.route_plan_digest());
}

#[test]
fn route_planning_keeps_plural_routes_first_class() {
    let plan = admitted_handle("primary")
        .declare_review_progress_describe_and_plan(RouteInput::<MixedRouteFamily>::new("edge:42"))
        .unwrap_or_else(|_| panic!("mixed route plan should succeed"));

    assert_eq!(plan.route_count(), 2);
    assert_eq!(plan.route_families().len(), 2);
}

#[test]
fn route_plan_digest_changes_when_admitted_world_changes() {
    let primary = admitted_handle("primary")
        .declare_review_progress_describe_and_plan(RouteInput::<RelationalRouteFamily>::new(
            "edge:42",
        ))
        .unwrap_or_else(|_| panic!("primary world should plan"));
    let alternate = admitted_handle("alternate")
        .declare_review_progress_describe_and_plan(RouteInput::<RelationalRouteFamily>::new(
            "edge:42",
        ))
        .unwrap_or_else(|_| panic!("alternate world should plan"));

    assert_ne!(primary.route_plan_digest(), alternate.route_plan_digest());
}

#[test]
fn route_plan_explanation_preserves_route_reasoning() {
    let plan = admitted_handle("primary")
        .declare_review_progress_describe_and_plan(RouteInput::<MixedRouteFamily>::new("edge:42"))
        .unwrap_or_else(|_| panic!("mixed route plan should succeed"));

    assert!(plan
        .explain()
        .route_contract_reason()
        .contains("relational and bridge"));
    assert_eq!(plan.explain().route_segment_reasons().len(), 2);
    assert!(plan
        .explain()
        .retained_facts()
        .iter()
        .any(|fact| fact.contains("operating_context:geometry.route-plan.primary")));
}

#[test]
fn route_plan_binding_target_retains_route_scoped_aspect_state() {
    let plan = admitted_handle("primary")
        .declare_review_progress_describe_and_plan(RouteInput::<AspectRichRouteFamily>::new(
            "edge:42",
        ))
        .unwrap_or_else(|_| panic!("aspect-rich route plan should succeed"));

    let binding = plan.binding_target();
    let semantics = binding.erased_target().semantics();
    let (_, _, _, _, route_contract, route_fit, route_publication) = semantics
        .declaration_route_plan()
        .expect("route binding target should retain route semantics");

    assert_eq!(route_contract, plan.aspect_contract());
    assert_eq!(route_fit, plan.aspect_fit());
    assert_eq!(route_publication, plan.aspect_publication());
}
