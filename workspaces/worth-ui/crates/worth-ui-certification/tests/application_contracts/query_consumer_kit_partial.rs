use worth_foundational::facade::CanonicalF32;
use worth_query::facade::domain;
use worth_ui::facade::query_binding::{WorthUiQueryViewShape, WorthUiQueryWorkspaceExt};
use worth_ui_query_binding::{
    WorthUiQueryInspection, WorthUiQueryInspectionEvidencePolicy, WorthUiQueryInspectionRelevance,
};

use crate::query_consumer_kit_application::file_authored_query_app;
use crate::query_consumer_kit_workspace::{
    interactive_borrowed_collection_requirements, measurement_value_path,
    partial_measurement_workspace,
};

#[test]
fn public_partial_query_settlement_preserves_posture_and_derives_only_ui_facts() {
    let mut workspace = partial_measurement_workspace("public-partial-query-settlement");
    let view = workspace
        .worth_ui()
        .unwrap()
        .measurement_view("inspector.measurements")
        .unwrap();
    let view_identity = view.definition().identity().clone();
    let app = file_authored_query_app(view);
    let reference = app
        .resolve_query_view(&view_identity, WorthUiQueryViewShape::Collection)
        .unwrap();
    let settled = reference
        .enter_snapshot_attempt(&workspace)
        .unwrap()
        .prepare_snapshot_consumer(interactive_borrowed_collection_requirements())
        .unwrap()
        .execute(&mut workspace)
        .unwrap()
        .publish()
        .unwrap()
        .consume(domain::project_facts().display_field(measurement_value_path()))
        .unwrap()
        .settle()
        .unwrap();

    assert_eq!(
        settled.result_state(),
        domain::WorthQueryOperationResultState::Partial
    );
    assert!(matches!(
        settled.execution_warnings(),
        [domain::WorthQueryOperationExecutionWarning::Partial(detail)]
            if detail == "certified partial snapshot"
    ));
    assert_eq!(settled.fact().execution_warning_count(), 1);
    assert_eq!(
        settled.fact().measurement_facts().unwrap().observations()[0].extent(),
        CanonicalF32::from_f32(240.0)
    );
    assert_eq!(settled.counters().executor_contacts, 1);
    assert!(!settled.publication_receipt().identity().is_empty());

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
    assert!(compact.rich_evidence().is_none());
    assert_eq!(rich.rich_evidence().unwrap().execution_warning_count(), 1);
    assert_eq!(compact.counters().rich_evidence_section_count(), 0);
    assert_eq!(rich.counters().rich_evidence_section_count(), 1);
}
