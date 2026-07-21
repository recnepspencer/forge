use worth_query::facade::domain;
use worth_ui::facade::query_binding::{
    WorthUiQueryAllocationDetail, WorthUiQueryConsumerRequirements, WorthUiQueryDenialPresentation,
    WorthUiQueryOperationAttemptDenial, WorthUiQueryViewIdentity, WorthUiQueryViewShape,
    WorthUiQueryWorkspaceExt,
};
use worth_ui_query_binding::{
    WorthUiQueryInspection, WorthUiQueryInspectionEvidencePolicy, WorthUiQueryInspectionRelevance,
};

use crate::query_consumer_kit_application::file_authored_query_app;
use crate::query_consumer_kit_workspace::{
    installed_measurement_workspace, measurement_value_path, observation_basis,
};

#[test]
fn minimal_and_rich_inspection_share_one_exact_success_artifact() {
    let mut workspace = installed_measurement_workspace("exact-success-inspection");
    let view = workspace
        .worth_ui()
        .unwrap()
        .measurement_view("inspector.measurements")
        .unwrap();
    let app = file_authored_query_app(view);
    let reference = app
        .resolve_query_view(
            &WorthUiQueryViewIdentity::new("inspector.measurements").unwrap(),
            WorthUiQueryViewShape::Collection,
        )
        .unwrap();
    let settled = reference
        .enter_snapshot_attempt(&workspace, observation_basis())
        .unwrap()
        .prepare_snapshot_consumer(requirements())
        .unwrap()
        .execute(&mut workspace)
        .unwrap()
        .publish()
        .unwrap()
        .consume(domain::project_facts().display_field(measurement_value_path()))
        .unwrap()
        .settle()
        .unwrap();

    let compact = WorthUiQueryInspection::settled_projection(
        &settled,
        WorthUiQueryInspectionRelevance::Relevant,
        WorthUiQueryInspectionEvidencePolicy::Minimal,
    );
    let rich = WorthUiQueryInspection::settled_projection(
        &settled,
        WorthUiQueryInspectionRelevance::Relevant,
        WorthUiQueryInspectionEvidencePolicy::Rich,
    );
    assert_eq!(compact.settlement_identity(), rich.settlement_identity());
    assert_eq!(compact.result_state(), rich.result_state());
    assert!(std::ptr::eq(compact.exact_projection(), &settled));
    assert!(std::ptr::eq(rich.exact_projection(), &settled));
    assert_eq!(compact.counters().rich_evidence_section_count(), 0);
    assert_eq!(rich.counters().rich_evidence_section_count(), 1);
}

#[test]
fn wrong_world_inspection_links_the_exact_query_attempt_denial() {
    let owner = installed_measurement_workspace("wrong-world-inspection-owner");
    let foreign = installed_measurement_workspace("wrong-world-inspection-foreign");
    let view = owner
        .worth_ui()
        .unwrap()
        .measurement_view("inspector.measurements")
        .unwrap();
    let app = file_authored_query_app(view);
    let reference = app
        .resolve_query_view(
            &WorthUiQueryViewIdentity::new("inspector.measurements").unwrap(),
            WorthUiQueryViewShape::Collection,
        )
        .unwrap();
    let denial = match reference.enter_snapshot_attempt(&foreign, observation_basis()) {
        Err(denial) => denial,
        Ok(_) => panic!("a foreign operating world must deny before binding"),
    };
    assert!(matches!(
        &denial,
        WorthUiQueryOperationAttemptDenial::InstalledDomainAuthorityMismatch
    ));
    let inspection =
        WorthUiQueryInspection::exact_artifact(&denial, WorthUiQueryInspectionRelevance::Relevant);
    assert!(std::ptr::eq(inspection.exact_artifact(), &denial));
    assert_eq!(inspection.counters().rich_evidence_section_count(), 0);
}

fn requirements() -> WorthUiQueryConsumerRequirements {
    WorthUiQueryConsumerRequirements::new(
        domain::WorthQueryConsumerBoundaryRequirements {
            presentation: domain::WorthQueryConsumerPresentationPosture::Interactive,
            allocation: domain::WorthQueryConsumerAllocationPosture::Borrowed,
        },
        WorthUiQueryAllocationDetail::BorrowedFactSlice,
        WorthUiQueryViewShape::Collection,
        WorthUiQueryDenialPresentation::StructuredStatus,
        WorthUiQueryInspectionRelevance::Relevant,
    )
}
