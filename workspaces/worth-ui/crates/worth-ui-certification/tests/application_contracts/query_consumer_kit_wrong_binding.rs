use worth_ui_query_binding::{WorthUiQueryViewShape, WorthUiQueryWorkspaceExt};

use crate::query_consumer_kit_application::file_authored_two_query_view_app;
use crate::query_consumer_kit_workspace::{
    installed_measurement_workspace, interactive_borrowed_collection_requirements,
};

#[test]
fn settled_projection_cannot_enter_through_another_query_binding_link() {
    let mut workspace = installed_measurement_workspace("wrong-query-binding-ingress");
    let installed = workspace
        .worth_ui()
        .expect("the installed Worth UI domain resolves");
    let measurements = installed
        .measurement_view("inspector.measurements")
        .expect("the measurements view installs");
    let secondary = installed
        .measurement_view("inspector.secondary")
        .expect("the secondary view installs");
    let measurements_identity = measurements.definition().identity().clone();
    let secondary_identity = secondary.definition().identity().clone();
    let app = file_authored_two_query_view_app(measurements.clone(), secondary);
    let measurements_reference = app
        .resolve_query_view(&measurements_identity, WorthUiQueryViewShape::Collection)
        .expect("the application resolves the measurements operation reference");
    let settled = measurements_reference
        .enter_snapshot_attempt(&workspace)
        .expect("the measurements attempt enters Query")
        .prepare_snapshot_consumer(interactive_borrowed_collection_requirements())
        .expect("Query admits the consumer")
        .execute(&mut workspace)
        .unwrap()
        .publish()
        .unwrap()
        .consume()
        .unwrap()
        .settle()
        .unwrap();

    let mut session = app.launch().expect("the two-binding application launches");
    let wrong_link = session
        .query_fact_link(secondary_identity.as_str())
        .expect("the second binding owns an active fact link");
    let mut denial = None;
    let completion = session
        .execute_framework_turn(|turn| {
            turn.query_projection(|query| {
                query
                    .admit_settled(settled)
                    .expect("the measurements settlement is retained under its own binding");
                denial = query.submit_settled(&wrong_link).err();
            });
        })
        .expect("no mounted presentation lease is active");
    drop(completion.into_completion());

    assert!(matches!(
        denial,
        Some(worth_ui::facade::runtime::WorthUiQueryFrameIngressDenial::RetainedFact(_))
    ));
    let _shutdown = session.shutdown();
}
