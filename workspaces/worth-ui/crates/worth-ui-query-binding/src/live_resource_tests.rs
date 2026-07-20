use worth_query::facade::consumer_kit::{in_memory_test_runtime, WorthQueryTestBackendSchema};
use worth_query::facade::{domain, runtime};

use crate::{
    worth_ui_domain_package, WorthUiInstalledLiveQueryView, WorthUiQueryBindingPlan,
    WorthUiQueryLiveAdmissionDenial, WorthUiQueryLiveCloseOutcome, WorthUiQueryLiveOpenOutcome,
    WorthUiQueryLiveProjectionOutcome, WorthUiQueryLiveResource, WorthUiQueryWorkspaceExt,
};

#[test]
fn live_resource_projection_is_admitted_atomically_by_the_exact_binding() {
    let mut workspace = measurement_workspace("live-binding-owner");
    let installed = workspace.worth_ui().expect("Worth UI domain installed");
    let view = installed
        .live_measurement_view("inspector.measurements")
        .expect("live measurement view");
    let plan = WorthUiQueryBindingPlan::default()
        .register_view(view.clone())
        .expect("live view registration");
    let reference = plan
        .resolve_definition(
            view.definition().identity(),
            crate::WorthUiQueryViewShape::Collection,
        )
        .expect("installed reference");
    let mut binding = plan.activate();
    let resource = open_resource(&view, &mut workspace);
    let projection = project_resource(&resource, &mut workspace);

    let settlement = binding
        .admit_live(resource, projection)
        .expect("exact live resource and projection admit together");

    assert_eq!(settlement.definition(), view.definition());
    assert!(binding
        .retains_live_resource_for(&reference)
        .expect("exact reference can inspect live retention"));
    assert_eq!(
        binding
            .execution_evidence_for(&reference)
            .expect("admitted live projection becomes execution evidence")
            .definition(),
        view.definition()
    );
}

#[test]
fn foreign_equal_live_resource_is_denied_and_returned_for_query_close() {
    let owner_workspace = measurement_workspace("live-binding-owner");
    let mut foreign_workspace = measurement_workspace("live-binding-foreign");
    let owner_view = owner_workspace
        .worth_ui()
        .expect("owner domain")
        .live_measurement_view("inspector.measurements")
        .expect("owner view");
    let foreign_view = foreign_workspace
        .worth_ui()
        .expect("foreign domain")
        .live_measurement_view("inspector.measurements")
        .expect("foreign equal view");
    let mut binding = WorthUiQueryBindingPlan::default()
        .register_view(owner_view)
        .expect("owner binding")
        .activate();
    let foreign_resource = open_resource(&foreign_view, &mut foreign_workspace);
    let foreign_projection = project_resource(&foreign_resource, &mut foreign_workspace);

    let stop = binding
        .admit_live(foreign_resource, foreign_projection)
        .expect_err("foreign installation cannot admit by equal definition");

    assert_eq!(
        stop.denial(),
        WorthUiQueryLiveAdmissionDenial::InstalledAuthorityMismatch
    );
    assert_closed(stop.into_resource(), &mut foreign_workspace);
}

#[test]
fn mismatched_live_definition_is_denied_before_projection_admission() {
    let mut workspace = measurement_workspace("live-definition-mismatch");
    let installed = workspace.worth_ui().expect("Worth UI domain installed");
    let first = installed
        .live_measurement_view("inspector.first")
        .expect("first live view");
    let second = installed
        .live_measurement_view("inspector.second")
        .expect("second live view");
    let mut binding = WorthUiQueryBindingPlan::default()
        .register_view(first.clone())
        .expect("first registration")
        .register_view(second.clone())
        .expect("second registration")
        .activate();
    let first_resource = open_resource(&first, &mut workspace);
    let first_projection = project_resource(&first_resource, &mut workspace);
    assert_closed(first_resource, &mut workspace);
    let second_resource = open_resource(&second, &mut workspace);

    let stop = binding
        .admit_live(second_resource, first_projection)
        .expect_err("resource and projection definitions cannot be mixed");

    assert_eq!(
        stop.denial(),
        WorthUiQueryLiveAdmissionDenial::ViewDefinitionMismatch
    );
    assert_closed(stop.into_resource(), &mut workspace);
}

#[test]
fn stale_projection_cannot_be_paired_with_a_reopened_live_resource() {
    let mut workspace = measurement_workspace("live-resource-generation-mismatch");
    let view = workspace
        .worth_ui()
        .expect("Worth UI domain installed")
        .live_measurement_view("inspector.measurements")
        .expect("live view");
    let mut binding = WorthUiQueryBindingPlan::default()
        .register_view(view.clone())
        .expect("live registration")
        .activate();
    let predecessor = open_resource(&view, &mut workspace);
    let stale_projection = project_resource(&predecessor, &mut workspace);
    assert_closed(predecessor, &mut workspace);
    let successor = open_resource(&view, &mut workspace);

    let stop = binding
        .admit_live(successor, stale_projection)
        .expect_err("projection authority must belong to the admitted resource generation");

    assert_eq!(
        stop.denial(),
        WorthUiQueryLiveAdmissionDenial::ProjectionResourceMismatch
    );
    assert_closed(stop.into_resource(), &mut workspace);
}

#[test]
fn query_runtime_rejects_duplicate_open_without_replacing_the_active_resource() {
    let mut workspace = measurement_workspace("live-duplicate-resource");
    let installed = workspace.worth_ui().expect("Worth UI domain installed");
    let view = installed
        .live_measurement_view("inspector.measurements")
        .expect("live view");
    let plan = WorthUiQueryBindingPlan::default()
        .register_view(view.clone())
        .expect("live registration");
    let reference = plan
        .resolve_definition(
            view.definition().identity(),
            crate::WorthUiQueryViewShape::Collection,
        )
        .expect("live reference");
    let mut binding = plan.activate();
    let first = open_resource(&view, &mut workspace);
    let first_projection = project_resource(&first, &mut workspace);
    binding
        .admit_live(first, first_projection)
        .expect("first resource admits");
    let duplicate_open = view
        .open_using(domain::current(), &mut workspace)
        .expect("installed authority still matches");

    assert!(matches!(
        duplicate_open,
        WorthUiQueryLiveOpenOutcome::Stopped(_)
    ));
    assert!(binding
        .retains_live_resource_for(&reference)
        .expect("first resource remains active"));
}

#[test]
fn query_free_binding_returns_the_unconsumed_live_resource() {
    let mut workspace = measurement_workspace("live-query-free-denial");
    let view = workspace
        .worth_ui()
        .expect("Worth UI domain installed")
        .live_measurement_view("inspector.measurements")
        .expect("live view");
    let resource = open_resource(&view, &mut workspace);
    let projection = project_resource(&resource, &mut workspace);
    let mut binding = WorthUiQueryBindingPlan::default().activate();

    let stop = binding
        .admit_live(resource, projection)
        .expect_err("query-free binding denies live admission");

    assert_eq!(
        stop.denial(),
        WorthUiQueryLiveAdmissionDenial::QueryNotInstalled
    );
    assert_closed(stop.into_resource(), &mut workspace);
}

fn open_resource(
    view: &WorthUiInstalledLiveQueryView,
    workspace: &mut runtime::WorthQueryWorkspace,
) -> WorthUiQueryLiveResource {
    match view
        .open_using(domain::current(), workspace)
        .expect("installed authority matches workspace")
    {
        WorthUiQueryLiveOpenOutcome::Opened(resource) => resource,
        WorthUiQueryLiveOpenOutcome::Stopped(_) => panic!("live resource open stopped"),
    }
}

fn project_resource(
    resource: &WorthUiQueryLiveResource,
    workspace: &mut runtime::WorthQueryWorkspace,
) -> WorthUiQueryLiveProjectionOutcome {
    let read = match resource.read(workspace) {
        Ok(read) => read,
        Err(_) => panic!("live read stopped"),
    };
    resource.project(&read, domain::project_facts().entity_identities())
}

fn assert_closed(resource: WorthUiQueryLiveResource, workspace: &mut runtime::WorthQueryWorkspace) {
    assert!(matches!(
        resource.close(workspace),
        WorthUiQueryLiveCloseOutcome::Closed(_)
    ));
}

fn measurement_workspace(name: &str) -> runtime::WorthQueryWorkspace {
    let schema = WorthQueryTestBackendSchema::single_collection("WorthUiMeasurement")
        .aspect_contracts(crate::worth_ui_native_aspect_contracts())
        .expect("native aspect contracts")
        .aspect("identity.id", "identity.id")
        .expect("identity aspect")
        .aspect("measurement.value", "measurement.value")
        .expect("measurement aspect");
    in_memory_test_runtime()
        .with_schema(schema)
        .domain_package(worth_ui_domain_package())
        .workspace(name)
        .expect("installed Query workspace")
}
