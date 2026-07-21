use worth_query::facade::domain;
use worth_ui::facade::query_binding::{
    WorthUiQueryAllocationDetail, WorthUiQueryConsumerRequirements, WorthUiQueryDenialPresentation,
    WorthUiQueryViewIdentity, WorthUiQueryViewShape, WorthUiQueryWorkspaceExt,
};
use worth_ui_query_binding::WorthUiQueryInspectionRelevance;

use crate::query_consumer_kit_application::file_authored_two_query_view_app;
use crate::query_consumer_kit_workspace::{
    installed_measurement_workspace, measurement_value_path, observation_basis,
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
    let app = file_authored_two_query_view_app(measurements.clone(), secondary);
    let measurements_reference = app
        .resolve_query_view(
            &WorthUiQueryViewIdentity::new("inspector.measurements").unwrap(),
            WorthUiQueryViewShape::Collection,
        )
        .expect("the application resolves the measurements operation reference");
    let settled = measurements_reference
        .enter_snapshot_attempt(&workspace, observation_basis())
        .expect("the measurements attempt enters Query")
        .prepare_snapshot_consumer(WorthUiQueryConsumerRequirements::new(
            domain::WorthQueryConsumerBoundaryRequirements {
                presentation: domain::WorthQueryConsumerPresentationPosture::Interactive,
                allocation: domain::WorthQueryConsumerAllocationPosture::Borrowed,
            },
            WorthUiQueryAllocationDetail::BorrowedFactSlice,
            WorthUiQueryViewShape::Collection,
            WorthUiQueryDenialPresentation::StructuredStatus,
            WorthUiQueryInspectionRelevance::Relevant,
        ))
        .expect("Query admits the consumer")
        .execute(&mut workspace)
        .unwrap()
        .publish()
        .unwrap()
        .consume(domain::project_facts().display_field(measurement_value_path()))
        .unwrap()
        .settle()
        .unwrap();

    let mut session = app.launch().expect("the two-binding application launches");
    let wrong_link = session
        .query_fact_link("inspector.secondary")
        .expect("the second binding owns an active fact link");
    let mut denial = None;
    let completion = session.execute_framework_turn(|turn| {
        turn.query_projection(|query| {
            query
                .admit_settled(settled)
                .expect("the measurements settlement is retained under its own binding");
            denial = query.submit_settled(&wrong_link).err();
        });
    });
    drop(completion.into_completion());

    assert!(matches!(
        denial,
        Some(worth_ui::facade::runtime::WorthUiQueryFrameIngressDenial::RetainedFact(_))
    ));
    let _shutdown = session.shutdown();
}
