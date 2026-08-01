use crate::intent::operability::{
    build_route_scale, last_route_graph_node, MountedRouteScaleWorld,
};
use worth_ui_runtime::certification_support::{
    WorthUiIntentExecutionBindingCertificationExt, WorthUiIntentRouteResolutionCertificationExt,
};

#[derive(Clone, Copy)]
struct ExpectedRouteStorage {
    definitions: usize,
    provider_bindings: usize,
    declarations: usize,
    product_routes: usize,
}

#[test]
fn definitions_providers_routes_and_resolution_follow_independent_storage_slopes() {
    let application = build_route_scale(1);
    assert_registration(&application, expectation(1));
    assert_last_route_lookup(&application, 1);
    let mut world = MountedRouteScaleWorld::launch(application, 1);
    assert_catalog(&world.session, expectation(1));
    let admitted = world.admit();
    let route = admitted.cost().route_resolution();
    assert_eq!(route.product_index_probes(), 1);
    assert_eq!(route.confirmation_index_probes(), 0);
    assert_eq!(route.total_index_probes(), 1);
    assert_eq!(route.route_rows_resolved(), 1);
    let _ = world.session.cancel_admitted_intent(admitted);
    let _ = world.session.shutdown();

    let application = build_route_scale(1_024);
    assert_registration(&application, expectation(1_024));
    assert_last_route_lookup(&application, 1_024);
    let session = application
        .launch()
        .expect("1,024-route application launches");
    assert_catalog(&session, expectation(1_024));
    let _ = session.shutdown();
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
    assert_eq!(cost.total_index_probes(), 1);
    assert_eq!(cost.route_rows_resolved(), 1);
}

const fn expectation(product_routes: usize) -> ExpectedRouteStorage {
    ExpectedRouteStorage {
        definitions: 1,
        provider_bindings: 1,
        declarations: 1,
        product_routes,
    }
}

fn assert_registration(
    application: &worth_ui::facade::app::WorthUiApp,
    expected: ExpectedRouteStorage,
) {
    let observed = application.intent_execution_binding_registration_metrics_for_certification();
    assert_eq!(observed.definitions(), expected.definitions);
    assert_eq!(observed.bindings(), expected.provider_bindings);
}

fn assert_catalog(
    session: &worth_ui::facade::app::WorthUiActiveApplicationSession,
    expected: ExpectedRouteStorage,
) {
    let observed = session.intent_catalog_metrics();
    assert_eq!(observed.definitions(), expected.definitions);
    assert_eq!(observed.declarations(), expected.declarations);
    assert_eq!(observed.product_routes(), expected.product_routes);
    assert_eq!(observed.confirmation_routes(), 0);
}
