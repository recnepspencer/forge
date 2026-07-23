use worth_query::facade::consumer_kit::{in_memory_test_runtime, WorthQueryTestBackendSchema};
use worth_query::facade::domain;

use crate::{
    worth_ui_domain_package, WorthUiQueryAllocationDetail, WorthUiQueryBindingPlan,
    WorthUiQueryConsumerRequirements, WorthUiQueryDenialPresentation, WorthUiQueryInspection,
    WorthUiQueryInspectionRelevance, WorthUiQueryViewIdentity, WorthUiQueryViewShape,
    WorthUiQueryWorkspaceExt, WorthUiSettledSnapshotAdmissionDenial,
    WorthUiSnapshotConsumerPreparationDenial,
};

#[test]
fn query_mints_support_while_ui_presentation_remains_adjacent() {
    let workspace = installed_workspace("consumer-boundary-adjacency");
    let reference = installed_reference(&workspace);
    let hidden = reference
        .enter_snapshot_attempt(&workspace)
        .unwrap()
        .prepare_snapshot_consumer(requirements(WorthUiQueryDenialPresentation::Hidden))
        .unwrap();
    let structured = reference
        .enter_snapshot_attempt(&workspace)
        .unwrap()
        .prepare_snapshot_consumer(requirements(
            WorthUiQueryDenialPresentation::StructuredStatus,
        ))
        .unwrap();

    assert_eq!(
        hidden.query_contract().canonical_operation_identity(),
        structured.query_contract().canonical_operation_identity()
    );
    assert_eq!(
        hidden.query_boundary_requirements(),
        structured.query_boundary_requirements()
    );
    assert_ne!(hidden.ui_requirements(), structured.ui_requirements());
}

#[test]
fn one_bound_operation_cannot_mint_a_second_consumer_contract() {
    let workspace = installed_workspace("single-consumer-contract");
    let reference = installed_reference(&workspace);
    let bound = reference
        .enter_snapshot_attempt(&workspace)
        .unwrap()
        .bind_snapshot()
        .unwrap();

    let _first = bound.consumer_projection_contract().unwrap();
    let second = match bound.consumer_projection_contract() {
        Ok(_) => panic!("Query must deny a second consumer contract"),
        Err(denial) => denial,
    };

    assert!(matches!(
        second,
        domain::WorthQueryConsumerProjectionContractDenial::AlreadyMinted { .. }
    ));
}

#[test]
fn unsupported_projection_consumption_is_a_query_denial() {
    let workspace = installed_builder()
        .consumer_support_posture(
            domain::WorthQueryConsumerSupportDimension::ProjectionConsumption,
            domain::WorthQueryConsumerSupportPosture::Unsupported,
        )
        .workspace("query-owned-support-denial")
        .unwrap();
    let reference = installed_reference(&workspace);
    let denial = reference
        .enter_snapshot_attempt(&workspace)
        .unwrap()
        .prepare_snapshot_consumer(requirements(
            WorthUiQueryDenialPresentation::StructuredStatus,
        ));

    let denial = match denial {
        Err(denial) => denial,
        Ok(_) => panic!("unsupported projection must keep its exact Query denial"),
    };
    assert!(matches!(
        &denial,
        WorthUiSnapshotConsumerPreparationDenial::ConsumerContract(
            domain::WorthQueryConsumerProjectionContractDenial::Compatibility(_)
        )
    ));
    let inspection =
        WorthUiQueryInspection::exact_artifact(&denial, WorthUiQueryInspectionRelevance::Relevant);
    assert!(std::ptr::eq(inspection.exact_artifact(), &denial));
    assert_eq!(inspection.counters().rich_evidence_section_count(), 0);
}

#[test]
fn exact_settled_projection_is_retained_once_and_derives_ui_fact() {
    let mut workspace = installed_workspace("exact-settlement-retention");
    let (plan, view_identity) = binding_plan(&workspace);
    let reference = plan
        .resolve_definition(&view_identity, WorthUiQueryViewShape::Collection)
        .unwrap();
    let settled = settle(&reference, &mut workspace);
    let expected_identity = settled.fact().settlement_identity().to_owned();
    let mut runtime = plan.prepare_downstream_state();
    let fact = runtime
        .admit_settled_snapshot(settled)
        .expect("exact settled projection should admit once");

    assert_eq!(fact.settlement_identity(), expected_identity);
    assert_eq!(fact.source_generation().unwrap().as_u64(), 1);
    assert_eq!(fact.source_order().unwrap().as_u64(), 1);
    assert_eq!(
        runtime.settled_snapshot_fact_for(&reference).unwrap(),
        &fact
    );
    let duplicate = settle(&reference, &mut workspace);
    let stop = runtime
        .admit_settled_snapshot(duplicate)
        .expect_err("one runtime slot retains one exact settled projection");
    assert_eq!(
        stop.denial(),
        WorthUiSettledSnapshotAdmissionDenial::DuplicateSettlement
    );
    let returned = stop.into_projection();
    assert_eq!(returned.installed_reference(), &reference);
}

#[test]
fn foreign_equal_installation_cannot_admit_exact_settlement() {
    let mut owner = installed_workspace("exact-settlement-owner");
    let foreign = installed_workspace("exact-settlement-foreign");
    let owner_reference = installed_reference(&owner);
    let projection = settle(&owner_reference, &mut owner);
    let mut foreign_runtime = binding_plan(&foreign).0.prepare_downstream_state();
    let stop = foreign_runtime
        .admit_settled_snapshot(projection)
        .expect_err("equal labels cannot replace exact Query authority");

    assert_eq!(
        stop.denial(),
        WorthUiSettledSnapshotAdmissionDenial::ForeignInstalledReference
    );
}

#[test]
fn in_generation_refresh_atomically_advances_source_coordinates() {
    let mut workspace = installed_workspace("exact-settlement-refresh");
    let (plan, view_identity) = binding_plan(&workspace);
    let reference = plan
        .resolve_definition(&view_identity, WorthUiQueryViewShape::Collection)
        .unwrap();
    let mut runtime = plan.prepare_downstream_state();
    let first = runtime
        .admit_settled_snapshot(settle(&reference, &mut workspace))
        .unwrap();
    let second = runtime
        .refresh_settled_snapshot(settle(&reference, &mut workspace))
        .unwrap();

    assert_eq!(first.source_generation().unwrap().as_u64(), 1);
    assert_eq!(first.source_order().unwrap().as_u64(), 1);
    assert_eq!(second.source_generation().unwrap().as_u64(), 2);
    assert_eq!(second.source_order().unwrap().as_u64(), 2);
    assert_eq!(
        runtime.settled_snapshot_fact_for(&reference).unwrap(),
        &second
    );

    let mut foreign_workspace = installed_workspace("exact-settlement-refresh-foreign");
    let foreign_reference = installed_reference(&foreign_workspace);
    let stop = runtime
        .refresh_settled_snapshot(settle(&foreign_reference, &mut foreign_workspace))
        .expect_err("foreign refresh must return its projection");
    assert_eq!(
        stop.denial(),
        WorthUiSettledSnapshotAdmissionDenial::ForeignInstalledReference
    );
    assert_eq!(
        runtime.settled_snapshot_fact_for(&reference).unwrap(),
        &second
    );
}

fn settle(
    reference: &crate::WorthUiInstalledQueryBindingReference,
    workspace: &mut worth_query::facade::runtime::WorthQueryWorkspace,
) -> crate::WorthUiSettledSnapshotProjection {
    reference
        .enter_snapshot_attempt(workspace)
        .unwrap()
        .prepare_snapshot_consumer(requirements(
            WorthUiQueryDenialPresentation::StructuredStatus,
        ))
        .unwrap()
        .execute(workspace)
        .unwrap()
        .publish()
        .unwrap()
        .consume(worth_query::facade::read::project_facts().entity_identities())
        .unwrap()
        .settle()
        .unwrap()
}

fn requirements(denial: WorthUiQueryDenialPresentation) -> WorthUiQueryConsumerRequirements {
    WorthUiQueryConsumerRequirements::new(
        domain::WorthQueryConsumerBoundaryRequirements {
            presentation: domain::WorthQueryConsumerPresentationPosture::Interactive,
            allocation: domain::WorthQueryConsumerAllocationPosture::Borrowed,
        },
        WorthUiQueryAllocationDetail::BorrowedFactSlice,
        WorthUiQueryViewShape::Collection,
        denial,
        WorthUiQueryInspectionRelevance::Relevant,
    )
}

fn installed_reference(
    workspace: &worth_query::facade::runtime::WorthQueryWorkspace,
) -> crate::WorthUiInstalledQueryBindingReference {
    let (plan, view_identity) = binding_plan(workspace);
    plan.resolve_definition(&view_identity, WorthUiQueryViewShape::Collection)
        .unwrap()
}

fn binding_plan(
    workspace: &worth_query::facade::runtime::WorthQueryWorkspace,
) -> (WorthUiQueryBindingPlan, WorthUiQueryViewIdentity) {
    let view = workspace
        .worth_ui()
        .unwrap()
        .measurement_view("dashboard.measurements")
        .unwrap();
    let view_identity = view.definition().identity().clone();
    (
        WorthUiQueryBindingPlan::default()
            .register_view(view)
            .unwrap(),
        view_identity,
    )
}

fn installed_workspace(label: &str) -> worth_query::facade::runtime::WorthQueryWorkspace {
    installed_builder().workspace(label).unwrap()
}

fn installed_builder() -> worth_query::facade::consumer_kit::WorthQueryInMemoryTestRuntimeBuilder {
    let schema = WorthQueryTestBackendSchema::single_collection("WorthUiMeasurement")
        .aspect_contracts(crate::worth_ui_native_aspect_contracts())
        .unwrap()
        .aspect("identity.id", "identity.id")
        .unwrap()
        .aspect("measurement.value", "measurement.value")
        .unwrap();
    crate::install_worth_ui_test_operation_executors(
        in_memory_test_runtime()
            .with_schema(schema)
            .domain_package(worth_ui_domain_package()),
    )
}
