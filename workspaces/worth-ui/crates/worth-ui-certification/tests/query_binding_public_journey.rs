use worth_foundational::facade::{AspectValue, CanonicalF32, CanonicalFieldPath, FieldKey};
use worth_query::facade::consumer_kit::{in_memory_test_runtime, WorthQueryTestBackendSchema};
use worth_query::facade::foundation::ProjectionFactFieldPath;
use worth_query::facade::{domain, runtime};
use worth_ui::facade::{
    app::WorthUi,
    query_binding::{
        worth_ui_domain_package, worth_ui_native_aspect_contracts, WorthUiQueryWorkspaceExt,
    },
};
use worth_ui_test_support::launch_empty_runtime_for_certification;

#[test]
fn external_consumer_installs_derives_registers_and_projects_through_public_facades() {
    let mut workspace = installed_measurement_workspace();
    let installed = workspace
        .worth_ui()
        .expect("the public extension resolves installed Worth UI authority");
    let view = installed
        .measurement_view("inspector.measurements")
        .expect("the installed domain derives one coherent view");
    let projection_view = view.clone();

    let app = WorthUi::app()
        .register_query_view(view)
        .expect("the public builder registers installed authority")
        .freeze();
    assert_eq!(app.capabilities().view_bindings().len(), 1);
    let mut runtime = launch_empty_runtime_for_certification(&app);

    let completion = projection_view
        .read()
        .expect("installed read declaration")
        .using(domain::current())
        .run(&mut workspace)
        .expect("workspace and installed view share authority")
        .into_result()
        .expect("installed read completion");
    let outcome = projection_view
        .project(
            &completion,
            domain::project_facts().display_field(measurement_value_path()),
        )
        .expect("the same installed view retains projection authority");
    assert_eq!(
        projection_view.definition().identity().as_str(),
        "inspector.measurements"
    );
    let mut submission = None;
    let _completion = runtime.execute_framework_turn(|turn| {
        turn.query_projection(|query| {
            submission = Some(query.admit_and_submit(outcome));
        });
    });
    let gateway = submission
        .expect("the Query projection source ran")
        .expect("the installed projection settled through the public runtime facade");
    assert!(gateway.submission().is_some());
    assert!(gateway.evidence().is_some());
    assert_eq!(gateway.counters().ingress_count(), 1);
}

fn installed_measurement_workspace() -> runtime::WorthQueryWorkspace {
    let schema = WorthQueryTestBackendSchema::single_collection("WorthUiMeasurement")
        .aspect_contracts(worth_ui_native_aspect_contracts())
        .expect("native contracts")
        .aspect("identity.id", "identity.id")
        .expect("identity aspect")
        .aspect("measurement.value", "measurement.value")
        .expect("measurement aspect");
    let mut workspace = in_memory_test_runtime()
        .with_schema(schema)
        .domain_package(worth_ui_domain_package())
        .workspace("public-query-binding-journey")
        .expect("installed Query workspace");
    workspace
        .insert("WorthUiMeasurement", |measurement| {
            measurement
                .set_aspect(
                    runtime::WorthQueryAspectTouch::from_authoring_ingress_text("identity.id")
                        .expect("identity touch"),
                    runtime::WorthQueryAuthoredAspectValue::string("measurement"),
                )
                .set_aspect(
                    runtime::WorthQueryAspectTouch::from_authoring_ingress_text(
                        "measurement.value",
                    )
                    .expect("measurement touch"),
                    runtime::WorthQueryAuthoredAspectValue::native(AspectValue::Float32(
                        CanonicalF32::from_f32(240.0),
                    )),
                )
        })
        .expect("measurement insertion");
    workspace
}

fn measurement_value_path() -> ProjectionFactFieldPath {
    ProjectionFactFieldPath::from_canonical_field_path(
        CanonicalFieldPath::new([
            FieldKey::new("measurement").expect("aspect path"),
            FieldKey::new("value").expect("field path"),
        ])
        .expect("measurement path"),
    )
}
