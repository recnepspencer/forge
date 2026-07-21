use worth_query::facade::consumer_kit::{
    advance_test_workspace_domain_installation_generation, in_memory_test_runtime,
    WorthQueryTestBackendSchema,
};
use worth_query::facade::{domain, foundation, runtime};

use crate::compatibility::managed_live::{
    WorthUiInstalledLiveQueryView, WorthUiQueryLiveCloseOutcome, WorthUiQueryLiveOpenOutcome,
    WorthUiQueryLiveResource, WorthUiQueryLiveRetirementCloseOutcome,
};
use crate::{
    worth_ui_domain_package, WorthUiQueryAllocationDetail, WorthUiQueryBindingPlan,
    WorthUiQueryBindingSuccessionChange, WorthUiQueryBindingSuccessionDenial,
    WorthUiQueryConsumerRequirements, WorthUiQueryDenialPresentation,
    WorthUiQueryInspectionRelevance, WorthUiQueryViewShape, WorthUiQueryWorkspaceExt,
};

#[test]
fn exact_successor_reference_preserves_the_predecessor_query_resource() {
    let mut workspace = measurement_workspace("query-succession-preserve");
    let view = installed_view(&mut workspace, "inspector.measurements");
    let plan = WorthUiQueryBindingPlan::default()
        .register_view(view.clone())
        .expect("live registration");
    let reference = reference_for(&plan, &view);
    let mut active = plan.prepare_downstream_state();
    admit_open_resource(&mut active, &view, &mut workspace);

    let prepared = plan
        .prepare_downstream_state()
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
fn exact_settled_snapshot_survives_regional_and_complete_succession() {
    let mut workspace = measurement_workspace("query-settlement-succession-preserve");
    let view = workspace
        .worth_ui()
        .expect("Worth UI domain installed")
        .measurement_view("inspector.measurements")
        .expect("snapshot measurement view");
    let plan = WorthUiQueryBindingPlan::default()
        .register_view(view.clone())
        .expect("snapshot registration");
    let reference = plan
        .resolve_definition(
            view.definition().identity(),
            WorthUiQueryViewShape::Collection,
        )
        .expect("installed snapshot reference");
    let mut active = plan.prepare_downstream_state();
    assert_snapshot_binding_has_no_live_compatibility_cost(&active);
    active
        .admit_settled_snapshot(settle_snapshot(&reference, &mut workspace))
        .expect("predecessor settlement admits");
    let predecessor = active
        .refresh_settled_snapshot(settle_snapshot(&reference, &mut workspace))
        .expect("predecessor refresh advances coordinates");

    let mut regional_candidate = plan.prepare_downstream_state();
    regional_candidate
        .admit_settled_snapshot(settle_snapshot(&reference, &mut workspace))
        .expect("regional candidate settlement admits independently");
    let regional_retirement = regional_candidate
        .prepare_regional_succession([WorthUiQueryBindingSuccessionChange::new(
            Some(reference.clone()),
            Some(reference.clone()),
        )])
        .expect("exact regional succession")
        .commit_once(&mut active);
    assert!(regional_retirement.is_empty());
    assert_retained_snapshot(&active, &reference, &predecessor, 2, 2);
    assert_snapshot_binding_has_no_live_compatibility_cost(&active);

    let mut complete_candidate = plan.prepare_downstream_state();
    complete_candidate
        .admit_settled_snapshot(settle_snapshot(&reference, &mut workspace))
        .expect("complete candidate settlement admits independently");
    let complete_retirement = complete_candidate
        .prepare_succession([reference.clone()])
        .expect("exact complete succession")
        .commit_once(&mut active);
    assert!(complete_retirement.is_empty());
    assert_retained_snapshot(&active, &reference, &predecessor, 2, 2);
    assert_snapshot_binding_has_no_live_compatibility_cost(&active);

    let refreshed = active
        .refresh_settled_snapshot(settle_snapshot(&reference, &mut workspace))
        .expect("post-succession refresh remains monotonic");
    assert_eq!(refreshed.source_generation().unwrap().as_u64(), 3);
    assert_eq!(refreshed.source_order().unwrap().as_u64(), 3);
}

#[test]
fn regional_succession_revalidates_unchanged_candidate_installation() {
    let mut workspace = measurement_workspace("query-succession-stale-candidate");
    let view = installed_view(&mut workspace, "inspector.measurements");
    let plan = WorthUiQueryBindingPlan::default()
        .register_view(view)
        .expect("snapshot registration");
    let candidate = plan.prepare_downstream_state();

    advance_test_workspace_domain_installation_generation(&mut workspace);

    let denial = match candidate.prepare_regional_succession([]) {
        Err(denial) => denial,
        Ok(_) => panic!("the stale candidate must return its exact Query denial"),
    };
    assert_eq!(
        denial,
        WorthUiQueryBindingSuccessionDenial::StaleSuccessorReference
    );
    let inspection = crate::WorthUiQueryInspection::exact_artifact(
        &denial,
        crate::WorthUiQueryInspectionRelevance::Relevant,
    );
    assert!(std::ptr::eq(inspection.exact_artifact(), &denial));
}

#[test]
fn removed_reference_yields_one_explicitly_closeable_retirement() {
    let mut workspace = measurement_workspace("query-succession-remove");
    let view = installed_view(&mut workspace, "inspector.measurements");
    let plan = WorthUiQueryBindingPlan::default()
        .register_view(view.clone())
        .expect("live registration");
    let reference = reference_for(&plan, &view);
    let mut active = plan.prepare_downstream_state();
    admit_open_resource(&mut active, &view, &mut workspace);

    let retirement = plan
        .prepare_downstream_state()
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
    let mut active = predecessor_plan.prepare_downstream_state();
    admit_open_resource(&mut active, &predecessor_view, &mut predecessor_workspace);
    let mut candidate = successor_plan.prepare_downstream_state();
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

fn settle_snapshot(
    reference: &crate::WorthUiInstalledQueryBindingReference,
    workspace: &mut runtime::WorthQueryWorkspace,
) -> crate::WorthUiSettledSnapshotProjection {
    reference
        .enter_snapshot_attempt(workspace, observation_basis())
        .expect("snapshot attempt enters exact world")
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
        .expect("Query mints the consumer contract")
        .execute(workspace)
        .unwrap()
        .publish()
        .unwrap()
        .consume(domain::project_facts().entity_identities())
        .unwrap()
        .settle()
        .unwrap()
}

fn assert_retained_snapshot(
    binding: &crate::WorthUiRuntimeQueryBinding,
    reference: &crate::WorthUiInstalledQueryBindingReference,
    expected: &crate::WorthUiSettledSnapshotFact,
    generation: u64,
    order: u64,
) {
    let retained = binding
        .settled_snapshot_fact_for(reference)
        .expect("exact predecessor settlement remains retained");
    assert_eq!(
        retained.settlement_identity(),
        expected.settlement_identity()
    );
    assert_eq!(retained.source_generation().unwrap().as_u64(), generation);
    assert_eq!(retained.source_order().unwrap().as_u64(), order);
}

fn assert_snapshot_binding_has_no_live_compatibility_cost(
    binding: &crate::WorthUiRuntimeQueryBinding,
) {
    let observation = binding.managed_live_compatibility_observation();
    assert_eq!(observation.subsystem_construction_count(), 0);
    assert_eq!(observation.retained_resource_count(), 0);
    assert_eq!(observation.resource_registration_count(), 0);
    assert_eq!(observation.succession_operation_count(), 0);
}

fn observation_basis() -> foundation::AdmittedBasisCapability<foundation::ObservationLaneWitness> {
    foundation::basis_lifecycle()
        .current_head()
        .for_observation()
        .unwrap()
        .admit()
        .unwrap()
        .capability()
        .clone()
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
    crate::install_worth_ui_test_operation_executors(
        in_memory_test_runtime()
            .with_schema(schema)
            .domain_package(worth_ui_domain_package()),
    )
    .workspace(name)
    .expect("installed Query workspace")
}
