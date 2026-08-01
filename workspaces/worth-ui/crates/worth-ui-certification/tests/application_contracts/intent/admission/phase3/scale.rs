use crate::intent::operability::{
    build_route_scale, last_route_graph_node, MountedRouteScaleWorld,
};
use worth_ui_runtime::certification_support::{
    WorthUiIntentExecutionBindingCertificationExt, WorthUiIntentRouteResolutionCertificationExt,
};

#[test]
fn one_route_uses_one_definition_and_one_declaration() {
    assert_route_scale(1);
}

#[test]
fn one_thousand_twenty_four_routes_do_not_multiply_definition_or_declaration_storage() {
    assert_indexed_route_storage(1_024);
}

#[test]
#[ignore = "closure-stress: 65,536 routes"]
fn closure_stress_sixty_five_thousand_five_hundred_thirty_six_routes() {
    assert_route_storage(65_536);
}

fn assert_route_storage(route_count: usize) {
    let application = build_route_scale(route_count);
    assert_registration_metrics(&application);
    assert_launched_catalog_storage(application, route_count);
}

fn assert_indexed_route_storage(route_count: usize) {
    let application = build_route_scale(route_count);
    assert_registration_metrics(&application);
    assert_last_route_lookup(&application, route_count);
    assert_launched_catalog_storage(application, route_count);
}

fn assert_launched_catalog_storage(
    application: worth_ui::facade::app::WorthUiApp,
    route_count: usize,
) {
    let session = application
        .launch()
        .expect("route-scale application launches");
    assert_catalog_metrics(&session, route_count);
    let _ = session.shutdown();
}

fn assert_route_scale(route_count: usize) {
    let application = build_route_scale(route_count);
    assert_registration_metrics(&application);
    assert_last_route_lookup(&application, route_count);
    let mut world = MountedRouteScaleWorld::launch(application, route_count);
    assert_catalog_metrics(&world.session, route_count);
    let admitted = world.admit();
    let route_cost = admitted.cost().route_resolution();
    assert_eq!(route_cost.product_index_probes(), 1);
    assert_eq!(route_cost.confirmation_index_probes(), 0);
    assert_eq!(route_cost.route_rows_resolved(), 1);
    let _ = world.session.cancel_admitted_intent(admitted);
    let _ = world.session.shutdown();
}

fn assert_last_route_lookup(application: &worth_ui::facade::app::WorthUiApp, route_count: usize) {
    let graph_node = last_route_graph_node(application, route_count);
    let cost = application
        .intent_route_resolution_cost_for_certification(
            graph_node,
            worth_ui::facade::intent::UiSemanticInteractionFamily::Activate,
        )
        .expect("the last authored control has one product route");
    assert_eq!(cost.product_index_probes(), 1);
    assert_eq!(cost.confirmation_index_probes(), 0);
    assert_eq!(cost.route_rows_resolved(), 1);
}

fn assert_registration_metrics(application: &worth_ui::facade::app::WorthUiApp) {
    let execution_metrics =
        application.intent_execution_binding_registration_metrics_for_certification();
    assert_eq!(execution_metrics.definitions(), 1);
    assert_eq!(execution_metrics.bindings(), 1);
}

fn assert_catalog_metrics(
    session: &worth_ui::facade::app::WorthUiActiveApplicationSession,
    route_count: usize,
) {
    let metrics = session.intent_catalog_metrics();
    assert_eq!(metrics.definitions(), 1);
    assert_eq!(metrics.declarations(), 1);
    assert_eq!(metrics.product_routes(), route_count);
    assert_eq!(metrics.confirmation_routes(), 0);
    assert_eq!(session.intent_admission_metrics().active_attempts(), 0);
}
