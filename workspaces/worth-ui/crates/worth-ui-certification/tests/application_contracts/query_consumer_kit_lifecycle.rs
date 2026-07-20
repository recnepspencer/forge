use worth_foundational::facade::{AspectValue, CanonicalF32, CanonicalFieldPath, FieldKey};
use worth_query::facade::consumer_kit::{in_memory_test_runtime, WorthQueryTestBackendSchema};
use worth_query::facade::foundation::ProjectionFactFieldPath;
use worth_query::facade::{domain, runtime};
use worth_ui::facade::{
    app::{WorthUi, WorthUiQueryViewRegistrationError},
    query_binding::{
        worth_ui_domain_package, worth_ui_native_aspect_contracts,
        WorthUiQueryBindingRegistrationDenialKind, WorthUiQueryLiveCloseOutcome,
        WorthUiQueryLiveOpenOutcome, WorthUiQueryWorkspaceExt,
    },
    source::{WorthUiSourceEventIngress, WorthUiSourceProvider, WorthUiWatcherEvent},
};

#[test]
fn external_consumer_installs_derives_registers_and_projects_through_public_facades() {
    let mut workspace = installed_measurement_workspace("public-query-binding-journey");
    let installed = workspace
        .worth_ui()
        .expect("the public extension resolves installed Worth UI authority");
    let view = installed
        .measurement_view("inspector.measurements")
        .expect("the installed domain derives one coherent view");
    let projection_view = view.clone();

    let snapshot = WorthUi::app()
        .register_query_view(view.clone())
        .expect("the public builder registers installed authority")
        .freeze()
        .expect("capability snapshot preparation should succeed");
    let submission = query_bound_submission(snapshot.capabilities());
    let app = WorthUi::app()
        .register_query_view(view)
        .expect("the public builder registers installed authority")
        .with_candidate_submission(submission)
        .freeze()
        .expect("application preparation should succeed");
    assert_eq!(app.capabilities().view_bindings().len(), 1);
    assert!(app.graph().node_count() > 0);
    let mut session = app
        .launch()
        .expect("the Query-bound source candidate launches through the public session");

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
    let active_generation = session.generation_identity().clone();
    let completion = session.execute_framework_turn(|turn| {
        turn.query_projection(|query| {
            submission = Some(query.admit_and_submit(outcome));
        });
    });
    assert_eq!(completion.generation_identity(), &active_generation);
    drop(completion.into_completion());
    let gateway = submission
        .expect("the Query projection source ran")
        .expect("the installed projection settled through the public runtime facade");
    assert!(gateway.submission().is_some());
    assert!(gateway.evidence().is_some());
    assert_eq!(gateway.counters().ingress_count(), 1);
    let _shutdown = session.shutdown();
}

#[test]
fn public_framework_turn_atomically_admits_and_releases_a_real_query_live_resource() {
    let mut workspace = installed_measurement_workspace("public-live-query-binding");
    let installed = workspace
        .worth_ui()
        .expect("the public extension resolves installed Worth UI authority");
    let view = installed
        .live_measurement_view("inspector.measurements")
        .expect("the installed domain derives one live view");
    let resource_view = view.clone();
    let snapshot = WorthUi::app()
        .register_query_view(view.clone())
        .expect("the public builder registers live installed authority")
        .freeze()
        .expect("capability snapshot preparation");
    let submission = query_bound_submission(snapshot.capabilities());
    let app = WorthUi::app()
        .register_query_view(view)
        .expect("the application registers live installed authority")
        .with_candidate_submission(submission)
        .freeze()
        .expect("application preparation");
    let mut session = app.launch().expect("Query-bound application launch");
    let resource = match resource_view
        .open_using(domain::current(), &mut workspace)
        .expect("installed authority matches the Query workspace")
    {
        WorthUiQueryLiveOpenOutcome::Opened(resource) => resource,
        WorthUiQueryLiveOpenOutcome::Stopped(_) => panic!("live resource open stopped"),
    };
    let read = match resource.read(&mut workspace) {
        Ok(read) => read,
        Err(_) => panic!("live resource read stopped"),
    };
    let projection = resource.project(
        &read,
        domain::project_facts().display_field(measurement_value_path()),
    );
    let mut admission = None;

    let completion = session.execute_framework_turn(|turn| {
        turn.query_projection(|query| {
            admission = Some(query.admit_live_and_submit(resource, projection));
        });
    });

    let completion = completion.into_completion();
    assert!(
        matches!(
            completion,
            worth_ui::facade::runtime::WorthUiFrameworkTurnCompletion::AllocationInvalidationNarrowingDenied { .. }
        ),
        "unexpected live Query framework completion: {completion:?}"
    );
    let gateway = admission
        .expect("live Query source executed")
        .expect("live resource and projection admitted atomically");
    assert!(gateway.submission().is_some());
    let _shutdown = session.shutdown();

    let reopened = match resource_view
        .open_using(domain::current(), &mut workspace)
        .expect("shutdown abandonment is reaped by Query")
    {
        WorthUiQueryLiveOpenOutcome::Opened(resource) => resource,
        WorthUiQueryLiveOpenOutcome::Stopped(_) => {
            panic!("Query must allow the resource to reopen after UI shutdown")
        }
    };
    assert!(matches!(
        reopened.close(&mut workspace),
        WorthUiQueryLiveCloseOutcome::Closed(_)
    ));
}

#[test]
fn public_builder_rejects_semantically_equal_views_from_foreign_query_installations() {
    let first_workspace = installed_measurement_workspace("first-query-installation");
    let second_workspace = installed_measurement_workspace("second-query-installation");
    let first = first_workspace
        .worth_ui()
        .expect("first installed Worth UI domain resolves");
    let second = second_workspace
        .worth_ui()
        .expect("second installed Worth UI domain resolves");
    let builder = WorthUi::app()
        .register_query_view(
            first
                .measurement_view("inspector.measurements")
                .expect("first view installs"),
        )
        .expect("first installation registers");
    let denial = match builder.register_query_view(
        second
            .measurement_view("inspector.measurements")
            .expect("second semantically equal view installs"),
    ) {
        Ok(_) => panic!("foreign Query installation cannot join the prepared authority"),
        Err(denial) => denial,
    };

    assert!(matches!(
        denial,
        WorthUiQueryViewRegistrationError::Binding(denial)
            if denial.kind() == WorthUiQueryBindingRegistrationDenialKind::ForeignInstalledDomain
    ));
}

fn query_bound_submission(
    snapshot: &worth_ui::facade::diagnostics::CapabilitySnapshot,
) -> worth_ui::facade::source::WorthUiWatchedCandidateSubmission {
    let provider_id = "query-consumer-kit-source";
    let mut ingress = WorthUiSourceEventIngress::new(
        WorthUiSourceProvider::in_memory(provider_id)
            .with_file("app/main.wui", "binding inspector.measurements {}"),
    )
    .start();
    ingress
        .ingest([WorthUiWatcherEvent::provider_revision(provider_id)])
        .expect("Query-bound source settles")
        .lower_to_candidate_submission(snapshot)
        .expect("Query-bound source lowers to one inseparable candidate submission")
}

fn installed_measurement_workspace(label: &str) -> runtime::WorthQueryWorkspace {
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
        .workspace(label)
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
