use worth_query::facade::{domain, installed};

mod single_root;

use super::installed_operation_fixture::{
    aftermath_workspace, conditional_installation_with_change, conditional_public_workspace_with,
    conditional_workspace, lineage_workflow_workspace, workspace, AftermathContract,
    DirectConditionalCompute, GeometryDomain, LineageEvidenceScenario, ReadExecutionInput,
    ReadFamily, ReadVertex,
};
use super::operation_native_access_matrix::{
    fixture::{insert_matrix_value, matrix_workspace, NativeMatrixRead},
    samples::matrix_value_with_order,
};
use crate::support::public_bridge_runtime::PublicBridgeRuntimeHarness;

#[test]
fn ordinary_consumer_enters_through_the_curated_installed_facade() {
    let mut workspace = workspace("installed-public-facade-dx", false).unwrap();
    let domain = workspace.domain(GeometryDomain).unwrap();
    let bound = workspace
        .observe_operating_world()
        .unwrap()
        .family(ReadFamily)
        .bind(&domain, ReadVertex)
        .unwrap();
    let consumer = bound.consumer_projection_contract().unwrap();
    let settled = bound
        .admit_execution_resources(
            ReadExecutionInput::default(),
            crate::suite::installed_operation_fixture::execution_resource_request(),
            &workspace,
        )
        .unwrap()
        .execute(&mut workspace)
        .unwrap()
        .publish()
        .unwrap()
        .consume(
            consumer,
            installed::operation::project_facts().entity_identities(),
        )
        .unwrap()
        .settle()
        .unwrap();

    assert_eq!(
        settled.result_state(),
        installed::operation::WorthQueryOperationResultState::Ready
    );
    assert_eq!(settled.counters().executor_contacts, 1);
    assert!(!settled.publication_receipt().identity().is_empty());
    let inspection: installed::inspection::WorthQueryConsumptionCostSnapshot =
        settled.consumption_cost_snapshot();
    assert!(inspection
        .row("query.execution.executor_contacts")
        .is_some());
}

#[test]
fn bound_operations_drive_native_live_invalidation_collection_and_disposal() {
    let mut workspace = workspace("installed-public-facade-single-root", false).unwrap();
    let domain = workspace.domain(GeometryDomain).unwrap();
    let root = workspace.observe_operating_world().unwrap();
    let reads = root.family(ReadFamily);
    let subject = reads.bind(&domain, ReadVertex).unwrap();
    let compatibility_peer = reads.bind(&domain, ReadVertex).unwrap();

    subject.same_installation_with(&compatibility_peer).unwrap();
    subject.compatible_basis_with(&compatibility_peer).unwrap();
    let snapshot_contract = subject.consumer_projection_contract().unwrap();
    let mut snapshot_request = snapshot_contract.projection_request();
    let native_selection = snapshot_request
        .select_display_native_field_name("id")
        .unwrap();
    let snapshot_request = snapshot_request.build().unwrap();
    let native_key = snapshot_request
        .resolve_native_key(&native_selection)
        .unwrap()
        .into_key();
    let snapshot = subject
        .admit_execution_resources(
            ReadExecutionInput::default(),
            crate::suite::installed_operation_fixture::execution_resource_request(),
            &workspace,
        )
        .unwrap()
        .execute(&mut workspace)
        .unwrap()
        .publish()
        .unwrap()
        .consume_bound(snapshot_request)
        .unwrap()
        .settle()
        .unwrap();
    assert!(snapshot.native_value(&native_key, 0).is_ok());

    let mut collection_workspace = matrix_workspace("installed-public-facade-collection", 3, false);
    let collection_domain = collection_workspace.domain(GeometryDomain).unwrap();
    let collection_root = collection_workspace.observe_operating_world().unwrap();
    let collection_family = collection_root.family(ReadFamily);
    let collection_bound = collection_family
        .bind(&collection_domain, NativeMatrixRead)
        .unwrap();
    let live_bound = collection_family
        .bind(&collection_domain, NativeMatrixRead)
        .unwrap();
    let collection_contract = collection_bound.consumer_projection_contract().unwrap();
    let mut collection_request = collection_contract.projection_request();
    collection_request
        .select_derived_native_field_name("f15")
        .unwrap();
    let collection_projection = collection_bound
        .admit_execution_resources(
            (),
            crate::suite::installed_operation_fixture::execution_resource_request(),
            &collection_workspace,
        )
        .unwrap()
        .execute(&mut collection_workspace)
        .unwrap()
        .publish()
        .unwrap()
        .consume_bound(collection_request.build().unwrap())
        .unwrap()
        .settle()
        .unwrap();
    let collection = collection_projection.into_bound_collection().unwrap();
    let breadth =
        installed::collection::WorthQueryCollectionWindowBreadth::new(8, 1, 1, 10).unwrap();
    let admitted = collection
        .declare_window(collection.beginning_cursor(), breadth)
        .unwrap();
    let window = collection.resolve_window(admitted).unwrap();
    let mut consumer =
        installed::collection::WorthQueryCollectionConsumerWindow::from_bound(collection, window)
            .unwrap();

    let live_contract = live_bound.consumer_projection_contract().unwrap();
    let mut live_request = live_contract.projection_request();
    live_request
        .select_derived_native_field_name("f15")
        .unwrap();
    let live_projection = live_bound
        .admit_execution_resources(
            (),
            crate::suite::installed_operation_fixture::execution_resource_request(),
            &collection_workspace,
        )
        .unwrap()
        .execute(&mut collection_workspace)
        .unwrap()
        .publish()
        .unwrap()
        .consume_bound(live_request.build().unwrap())
        .unwrap()
        .settle()
        .unwrap();
    let promoted = match live_projection
        .into_lifecycle()
        .promote(&mut collection_workspace)
    {
        installed::observation::WorthQueryProjectionPromotionOutcome::Promoted(live) => live,
        _ => panic!("ordinary facade did not promote the current projection"),
    };
    let lease = match promoted.into_managed_lease(&mut collection_workspace) {
        installed::observation::WorthQueryProjectionLeaseAdmissionOutcome::Admitted(lease) => lease,
        installed::observation::WorthQueryProjectionLeaseAdmissionOutcome::Stopped(stop) => {
            panic!(
                "ordinary facade did not admit a managed lease: {}",
                stop.detail()
            )
        }
    };

    assert!(lease
        .drain(&mut collection_workspace)
        .unwrap()
        .delivery()
        .is_empty());
    insert_matrix_value(
        &mut collection_workspace,
        3,
        matrix_value_with_order(3, "public-facade-invalidation"),
    );
    let delivery = lease.drain(&mut collection_workspace).unwrap();
    let delta = lease.consumer_invalidation_delta(delivery).unwrap();
    let admitted = match lease.admit_consumer_invalidation_delta(delta, &collection_workspace) {
        Ok(admitted) => admitted,
        Err(stop) => panic!("current invalidation did not readmit: {:?}", stop.kind()),
    };
    consumer
        .bind_shared_target(&admitted, &collection_workspace)
        .unwrap();
    let patch = match consumer.plan_patch(&admitted, &collection_workspace) {
        installed::collection::WorthQueryCollectionDeliveryOutcome::Patch(patch) => patch,
        installed::collection::WorthQueryCollectionDeliveryOutcome::NoDelivery(denial) => {
            panic!("semantic invalidation produced no collection patch: {denial:?}")
        }
    };
    let receipt = consumer.apply_patch(patch).unwrap();
    assert!(!receipt.operations().is_empty());

    let disposed = match lease.dispose(&mut collection_workspace) {
        installed::observation::WorthQuerySharedProjectionDisposalOutcome::Disposed(receipt) => {
            receipt
        }
        installed::observation::WorthQuerySharedProjectionDisposalOutcome::Stopped(stop) => {
            panic!("ordinary facade disposal stopped: {}", stop.error())
        }
    };
    assert!(disposed.release().owner_terminal());
    assert_eq!(disposed.release().counters().close_completions, 1);
}

#[test]
fn conditional_authoring_and_signal_execution_stay_inside_the_query_facade() {
    let declarations = super::conditional_node_contract::representative_nodes();
    assert_eq!(declarations.len(), 5);
    for declaration in declarations {
        let identity = declaration.identity().to_string();
        if identity == "threshold" {
            assert_threshold_family_executes_through_facade(declaration);
            continue;
        }
        let mut workspace = conditional_workspace(
            &format!("installed-public-facade-conditional-{identity}"),
            declaration,
        )
        .unwrap();
        let domain = workspace.domain(GeometryDomain).unwrap();
        let bound = workspace
            .observe_operating_world()
            .unwrap()
            .family(ReadFamily)
            .bind(&domain, ReadVertex)
            .unwrap();
        let consumer = bound.consumer_projection_contract().unwrap();
        let executed = bound
            .admit_execution_resources(
                ReadExecutionInput::default(),
                crate::suite::installed_operation_fixture::execution_resource_request(),
                &workspace,
            )
            .unwrap()
            .execute(&mut workspace)
            .unwrap();

        assert_eq!(executed.conditional_provenance().len(), 1, "{identity}");
        assert_eq!(
            executed.conditional_provenance()[0].class(),
            installed::conditional::WorthQueryConditionalOutcomeClass::ComputedChanged,
            "{identity}"
        );
        assert_eq!(
            executed.counters().conditional_compute_contacts,
            1,
            "{identity}"
        );
        let settled = executed
            .publish()
            .unwrap()
            .consume(
                consumer,
                installed::operation::project_facts().entity_identities(),
            )
            .unwrap()
            .settle()
            .unwrap();
        assert_eq!(settled.conditional_provenance().len(), 1, "{identity}");
    }
}

fn assert_threshold_family_executes_through_facade(
    declaration: domain::WorthQueryPortableConditionalNodeDeclaration,
) {
    let (installation, change, snapshots) = conditional_installation_with_change(&declaration);
    let harness = PublicBridgeRuntimeHarness::new();
    harness.set_relational_snapshot(snapshots[0].snapshot_id(), snapshots[0].version_id());
    let mut workspace = conditional_public_workspace_with(
        "installed-public-facade-conditional-threshold",
        declaration,
        installation,
        DirectConditionalCompute,
        &harness,
    )
    .unwrap();
    let domain = workspace.domain(GeometryDomain).unwrap();
    let baseline = workspace
        .observe_operating_world()
        .unwrap()
        .family(ReadFamily)
        .bind(&domain, ReadVertex)
        .unwrap();
    let installed::transition::WorthQueryExecutionTransition::Deferred(baseline) =
        installed::transition::execution(
            baseline
                .admit_execution_resources(
                    ReadExecutionInput::default(),
                    crate::suite::installed_operation_fixture::execution_resource_request(),
                    &workspace,
                )
                .unwrap()
                .execute(&mut workspace),
        )
    else {
        panic!("the typed threshold must establish its baseline before it can compare a delta")
    };
    assert_eq!(
        baseline.conditional_provenance()[0].class(),
        installed::conditional::WorthQueryConditionalOutcomeClass::Suppressed
    );

    let location = domain::WorthQueryConditionalNodeLocation::operation("threshold").unwrap();
    workspace
        .deliver_conditional_authoritative_change(
            GeometryDomain,
            ReadVertex,
            ReadFamily,
            domain::WorthQueryConditionalAuthoritativeChangeDeliveryRequest::new(
                location, 0, change,
            ),
        )
        .unwrap()
        .unwrap();
    harness.set_relational_snapshot(snapshots[1].snapshot_id(), snapshots[1].version_id());

    let bound = workspace
        .observe_operating_world()
        .unwrap()
        .family(ReadFamily)
        .bind(&domain, ReadVertex)
        .unwrap();
    let installed::transition::WorthQueryExecutionTransition::Deferred(executed) =
        installed::transition::execution(
            bound
                .admit_execution_resources(
                    ReadExecutionInput::default(),
                    crate::suite::installed_operation_fixture::execution_resource_request(),
                    &workspace,
                )
                .unwrap()
                .execute(&mut workspace),
        )
    else {
        panic!("the threshold compute should report its unchanged output as reverted-clean")
    };
    assert_eq!(
        executed.conditional_provenance()[0].class(),
        installed::conditional::WorthQueryConditionalOutcomeClass::ComputedRevertedClean
    );
    assert_eq!(executed.counters().conditional_compute_contacts, 1);
}

#[test]
fn workflow_lineage_aftermath_support_and_inspection_stay_inside_the_facade() {
    let mut lineage = lineage_workflow_workspace(
        "installed-public-facade-lineage",
        installed::operation::WorthQueryOperationLineageContract::Preserve,
        false,
        vec![LineageEvidenceScenario::PreservedIdentity],
    )
    .unwrap();
    let trace = super::operation_lineage::execute(&mut lineage);
    let report: &installed::lineage::WorthQueryTraceLineageReport = trace
        .lineage_report()
        .expect("declared lineage produces a Query-owned report");
    assert_eq!(report.evidence().len(), 1);
    assert!(!report.identity().is_empty());
    let closure: &installed::impact::WorthQueryCompiledSemanticAspectDependencyClosure = trace
        .semantic_aspect_dependency_closure()
        .expect("completed workflow compiles its dependency closure");
    assert!(!closure.dependencies().is_empty());

    let reexecuted = super::operation_lineage::bind(&lineage)
        .admit_workflow_resources(
            crate::suite::installed_operation_fixture::execution_resource_request(),
            &lineage,
        )
        .unwrap()
        .reexecute(super::operation_lineage::intent(), &mut lineage)
        .unwrap();
    assert_ne!(trace.identity(), reexecuted.identity());
    assert_eq!(
        trace
            .stage_receipts()
            .iter()
            .map(|receipt| receipt.stage_identity())
            .collect::<Vec<_>>(),
        ["start", "right", "left", "publish"]
    );
    assert_eq!(
        reexecuted
            .stage_receipts()
            .iter()
            .map(|receipt| receipt.stage_identity())
            .collect::<Vec<_>>(),
        ["start", "right", "left", "publish"]
    );

    let mut aftermath = aftermath_workspace(
        "installed-public-facade-aftermath",
        AftermathContract::Compensation,
    )
    .unwrap();
    let original = super::operation_aftermath_support::bind_original(&aftermath)
        .admit_workflow_resources(
            crate::suite::installed_operation_fixture::execution_resource_request(),
            &aftermath,
        )
        .unwrap()
        .reexecute(
            super::operation_aftermath_support::intent("apply"),
            &mut aftermath,
        )
        .unwrap();
    let candidate = super::operation_aftermath_support::bind_candidate(&aftermath);
    let capability = match original.admit_aftermath(candidate) {
        installed::recovery::WorthQueryAftermathAdmission::Compensation(capability) => capability,
        _ => panic!("declared compensation did not admit through the installed facade"),
    };
    let executed = capability
        .execute_workflow(
            crate::suite::installed_operation_fixture::execution_resource_request(),
            &mut aftermath,
        )
        .unwrap();
    assert_eq!(
        executed.relation().kind(),
        installed::recovery::WorthQueryAftermathKind::Compensation
    );

    let workspace = workspace("installed-public-facade-support", false).unwrap();
    let domain = workspace.domain(GeometryDomain).unwrap();
    let bound = workspace
        .observe_operating_world()
        .unwrap()
        .family(ReadFamily)
        .bind(&domain, ReadVertex)
        .unwrap();
    let support = bound.consumer_projection_contract().unwrap();
    assert_eq!(
        support.support_posture(installed::support::WorthQueryConsumerSupportDimension::Basis),
        installed::support::WorthQueryConsumerSupportPosture::Supported
    );
}
