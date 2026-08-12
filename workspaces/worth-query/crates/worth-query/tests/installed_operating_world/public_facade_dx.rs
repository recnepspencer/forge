use worth_query::facade::{domain, installed};

mod operational_lifecycle;
mod single_root;

use super::installed_operation_fixture::{
    conditional_installation_with_change, conditional_public_workspace_with, conditional_workspace,
    lineage_workflow_workspace, workspace, DirectConditionalCompute, GeometryDomain,
    LineageEvidenceScenario, ReadExecutionInput, ReadFamily, ReadVertex,
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
