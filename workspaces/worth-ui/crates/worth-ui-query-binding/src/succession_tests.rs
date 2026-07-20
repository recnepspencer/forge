use worth_query::facade::consumer_kit::{in_memory_test_runtime, WorthQueryTestBackendSchema};
use worth_query::facade::{domain, runtime};

use crate::{
    worth_ui_domain_package, WorthUiInstalledLiveQueryView, WorthUiQueryBindingPlan,
    WorthUiQueryLiveCloseOutcome, WorthUiQueryLiveOpenOutcome, WorthUiQueryLiveResource,
    WorthUiQueryLiveRetirementCloseOutcome, WorthUiQueryWorkspaceExt,
};

#[test]
fn exact_successor_reference_preserves_the_predecessor_query_resource() {
    let mut workspace = measurement_workspace("query-succession-preserve");
    let view = installed_view(&mut workspace, "inspector.measurements");
    let plan = WorthUiQueryBindingPlan::default()
        .register_view(view.clone())
        .expect("live registration");
    let reference = reference_for(&plan, &view);
    let mut active = plan.activate();
    admit_open_resource(&mut active, &view, &mut workspace);

    let prepared = plan
        .activate()
        .prepare_succession([reference.clone()])
        .expect("exact successor reference");
    let retirement = prepared.commit_once(&mut active);

    assert!(retirement.is_empty());
    assert!(active
        .retains_live_resource_for(&reference)
        .expect("successor retains exact resource"));
    assert!(matches!(
        view.open_using(domain::current(), &mut workspace)
            .expect("installed authority remains current"),
        WorthUiQueryLiveOpenOutcome::Stopped(_)
    ));
}

#[test]
fn removed_reference_yields_one_explicitly_closeable_retirement() {
    let mut workspace = measurement_workspace("query-succession-remove");
    let view = installed_view(&mut workspace, "inspector.measurements");
    let plan = WorthUiQueryBindingPlan::default()
        .register_view(view.clone())
        .expect("live registration");
    let reference = reference_for(&plan, &view);
    let mut active = plan.activate();
    admit_open_resource(&mut active, &view, &mut workspace);

    let retirement = plan
        .activate()
        .prepare_succession([])
        .expect("empty successor posture")
        .commit_once(&mut active);

    assert_eq!(retirement.len(), 1);
    assert!(!active
        .retains_live_resource_for(&reference)
        .expect("registered but inactive view remains inspectable"));
    let mut foreign_workspace = measurement_workspace("query-succession-remove-foreign");
    let retirement = match retirement.close(&mut foreign_workspace) {
        WorthUiQueryLiveRetirementCloseOutcome::AuthorityStopped(stop) => {
            assert_eq!(stop.closed_resource_count(), 0);
            stop.into_retirement()
        }
        _ => panic!("foreign workspace must stop without losing the retired resource"),
    };
    assert_eq!(retirement.len(), 1);
    let WorthUiQueryLiveRetirementCloseOutcome::Closed(receipt) = retirement.close(&mut workspace)
    else {
        panic!("retirement close must complete")
    };
    assert_eq!(receipt.closed_resource_count(), 1);
    assert_eq!(receipt.query_close_receipts().len(), 1);
    assert_closed(open_resource(&view, &mut workspace), &mut workspace);
}

#[test]
fn foreign_rebind_never_preserves_a_semantically_equal_predecessor() {
    let mut predecessor_workspace = measurement_workspace("query-succession-predecessor");
    let mut successor_workspace = measurement_workspace("query-succession-successor");
    let predecessor_view = installed_view(&mut predecessor_workspace, "inspector.measurements");
    let successor_view = installed_view(&mut successor_workspace, "inspector.measurements");
    let predecessor_plan = WorthUiQueryBindingPlan::default()
        .register_view(predecessor_view.clone())
        .expect("predecessor registration");
    let successor_plan = WorthUiQueryBindingPlan::default()
        .register_view(successor_view.clone())
        .expect("successor registration");
    let successor_reference = reference_for(&successor_plan, &successor_view);
    let mut active = predecessor_plan.activate();
    admit_open_resource(&mut active, &predecessor_view, &mut predecessor_workspace);
    let mut candidate = successor_plan.activate();
    admit_open_resource(&mut candidate, &successor_view, &mut successor_workspace);

    let retirement = candidate
        .prepare_succession([successor_reference.clone()])
        .expect("successor owns its exact reference")
        .commit_once(&mut active);

    assert_eq!(retirement.len(), 1);
    assert!(active
        .retains_live_resource_for(&successor_reference)
        .expect("foreign successor retains only its candidate resource"));
    let WorthUiQueryLiveRetirementCloseOutcome::Closed(receipt) =
        retirement.close(&mut predecessor_workspace)
    else {
        panic!("predecessor retirement closes in its owning workspace")
    };
    assert_eq!(receipt.closed_resource_count(), 1);
}

fn installed_view(
    workspace: &mut runtime::WorthQueryWorkspace,
    identity: &str,
) -> WorthUiInstalledLiveQueryView {
    workspace
        .worth_ui()
        .expect("Worth UI domain installed")
        .live_measurement_view(identity)
        .expect("live measurement view")
}

fn reference_for(
    plan: &WorthUiQueryBindingPlan,
    view: &WorthUiInstalledLiveQueryView,
) -> crate::WorthUiInstalledQueryBindingReference {
    plan.resolve_definition(
        view.definition().identity(),
        crate::WorthUiQueryViewShape::Collection,
    )
    .expect("installed reference")
}

fn admit_open_resource(
    binding: &mut crate::WorthUiRuntimeQueryBinding,
    view: &WorthUiInstalledLiveQueryView,
    workspace: &mut runtime::WorthQueryWorkspace,
) {
    let resource = open_resource(view, workspace);
    let read = match resource.read(workspace) {
        Ok(read) => read,
        Err(_) => panic!("live read stopped"),
    };
    let projection = resource.project(&read, domain::project_facts().entity_identities());
    binding
        .admit_live(resource, projection)
        .expect("live resource admission");
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
