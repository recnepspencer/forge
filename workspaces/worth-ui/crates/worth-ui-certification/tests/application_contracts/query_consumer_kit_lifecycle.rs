use worth_ui::facade::{
    app::{WorthUi, WorthUiQueryViewRegistrationError},
    query_binding::WorthUiQueryBindingRegistrationDenialKind,
};
use worth_ui_dsl::UiDslSourceProvenance;
use worth_ui_query_binding::{
    WorthUiQueryOperationAttemptDenial, WorthUiQueryViewShape, WorthUiQueryWorkspaceExt,
};

use crate::query_consumer_kit_application::{
    query_bound_rust_submission, query_bound_submission, query_free_app,
};
use crate::query_consumer_kit_workspace::installed_measurement_workspace;

#[test]
fn public_query_free_application_has_no_query_observation_runtime_cost() {
    let session = query_free_app()
        .launch()
        .expect("the real file-authored Query-free application launches");
    let scan = session.inspect_query_state_residue();
    assert!(!scan.query_installed());
    assert_eq!(scan.scanned_query_bindings(), 0);
    assert_eq!(scan.scanned_plan_query_links(), 0);
    assert_eq!(scan.scanned_settled_snapshots(), 0);
    assert_eq!(scan.scanned_live_resources(), 0);
    assert_eq!(scan.operation_live_subsystem_construction_count(), 0);
    assert_eq!(scan.operation_live_succession_operation_count(), 0);
    assert!(scan.is_clean());
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
    let resource =
        crate::query_replacement_lifecycle::support::open_resource(&resource_view, &mut workspace);
    let mut admission = None;

    let completion = session.execute_framework_turn(|turn| {
        turn.query_projection(|query| {
            admission = Some(query.admit_operation_live(resource));
        });
    });

    drop(completion.into_completion());
    admission
        .expect("live Query source executed")
        .expect("live resource and projection admitted atomically");
    let shutdown = session.shutdown();
    crate::query_replacement_lifecycle::support::close_retirement(
        shutdown.into_operation_live_retirement(),
        &mut workspace,
    );

    let reopened =
        crate::query_replacement_lifecycle::support::open_resource(&resource_view, &mut workspace);
    assert!(matches!(
        reopened.close(&mut workspace),
        worth_ui_query_binding::WorthUiOperationLiveCloseOutcome::Closed(_)
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
    let first_view = first
        .measurement_view("inspector.measurements")
        .expect("first view installs");
    let second_view = second
        .measurement_view("inspector.measurements")
        .expect("second semantically equal view installs");
    let first_identity = first_view.definition().identity().clone();
    assert_eq!(
        first_view.definition().digest(),
        second_view.definition().digest(),
        "the hostile pair deliberately shares its diagnostic definition digest"
    );
    let app = WorthUi::app()
        .register_query_view(first_view.clone())
        .expect("first installation registers")
        .freeze()
        .expect("first installation prepares");
    let reference = app
        .resolve_query_view(&first_identity, WorthUiQueryViewShape::Collection)
        .expect("prepared application retains its installed reference");
    assert!(matches!(
        reference.enter_snapshot_attempt(&second_workspace),
        Err(WorthUiQueryOperationAttemptDenial::InstalledDomainAuthorityMismatch)
    ));
    let builder = WorthUi::app()
        .register_query_view(first_view)
        .expect("first installation registers");
    let denial = match builder.register_query_view(second_view) {
        Ok(_) => panic!("foreign Query installation cannot join the prepared authority"),
        Err(denial) => denial,
    };

    assert!(matches!(
        denial,
        WorthUiQueryViewRegistrationError::Binding(denial)
            if denial.kind() == WorthUiQueryBindingRegistrationDenialKind::ForeignInstalledDomain
    ));
}

#[test]
fn file_and_rust_authored_bindings_converge_before_the_same_query_gateway() {
    let workspace = installed_measurement_workspace("authored-query-binding-convergence");
    let installed = workspace.worth_ui().unwrap();
    let view = installed
        .measurement_view("inspector.measurements")
        .unwrap();
    let identity = view.definition().identity().clone();
    let capability_app = WorthUi::app()
        .register_query_view(view.clone())
        .unwrap()
        .freeze()
        .unwrap();
    let file_app = WorthUi::app()
        .register_query_view(view.clone())
        .unwrap()
        .with_candidate_submission(query_bound_submission(capability_app.capabilities()))
        .freeze()
        .unwrap();
    let rust_app = WorthUi::app()
        .register_query_view(view)
        .unwrap()
        .with_candidate_submission(query_bound_rust_submission(capability_app.capabilities()))
        .freeze()
        .unwrap();
    let file_reference = file_app
        .resolve_query_view(&identity, WorthUiQueryViewShape::Collection)
        .unwrap();
    let rust_reference = rust_app
        .resolve_query_view(&identity, WorthUiQueryViewShape::Collection)
        .unwrap();

    assert_eq!(file_reference, rust_reference);
    assert!(file_app
        .declaration_artifacts()
        .iter()
        .any(|artifact| matches!(
            artifact.provenance().source_provenance(),
            UiDslSourceProvenance::FileAuthored { .. }
        )));
    assert!(rust_app
        .declaration_artifacts()
        .iter()
        .any(|artifact| matches!(
            artifact.provenance().source_provenance(),
            UiDslSourceProvenance::RustAuthored { .. }
        )));
    let file_bound = file_reference
        .enter_snapshot_attempt(&workspace)
        .unwrap()
        .bind_snapshot()
        .unwrap();
    let rust_bound = rust_reference
        .enter_snapshot_attempt(&workspace)
        .unwrap()
        .bind_snapshot()
        .unwrap();
    assert_eq!(file_bound.binding_identity(), rust_bound.binding_identity());
}
