use worth_ui::facade::app::WorthUi;
use worth_ui::facade::inspection::{
    UiEvidenceBudget, UiEvidenceRichness, UiInspectionEvidenceSource,
    UiInspectionMilestoneExpectation, UiInspectionPosture, UiInspectionQuery,
    UiInspectionRelevance, UiInspectionScope, UiInspectionSupportReason, UiInspectionSupportStatus,
    UiInspectionSupportPosture, UiInspectionSupportWorld, UiInspectionTarget,
};

#[test]
fn inspection_query_preserves_budget_and_relevance_through_the_facade_receipt() {
    let app = WorthUi::app()
        .with_dsl_package(worth_ui_dsl::WorthUiDslPackage::empty())
        .freeze();
    let query = UiInspectionQuery::new(
        UiInspectionTarget::product_root(),
        UiInspectionScope::graph(),
    )
    .with_budget(UiEvidenceBudget::bounded(UiEvidenceRichness::summary()))
    .with_relevance(UiInspectionRelevance::query_backed_only());
    let receipt = app.inspect(query.clone());

    assert_eq!(receipt.query(), &query);
    assert_eq!(
        receipt.query().budget(),
        UiEvidenceBudget::bounded(UiEvidenceRichness::summary())
    );
    assert!(!receipt.query().relevance().includes_worth_local_evidence());
    assert!(receipt.query().relevance().includes_query_inspection());
    assert!(receipt
        .query()
        .relevance()
        .includes_query_projection_consumption());
}

#[test]
fn inspection_inventory_projects_typed_support_and_closure_reports() {
    let app = WorthUi::app()
        .with_dsl_package(worth_ui_dsl::WorthUiDslPackage::empty())
        .freeze();
    let graph_report = app.inspection_support_report(UiInspectionScope::graph());
    let measurement_report = app.inspection_support_report(UiInspectionScope::measurement());
    let mounting_report = app.inspection_support_report(UiInspectionScope::mounting());
    let rebind_report = app.inspection_support_report(UiInspectionScope::rebind());
    let closure_report = app.inspection_closure_report();

    assert_eq!(graph_report.scope(), UiInspectionScope::Graph);
    assert_eq!(
        graph_report.status(),
        UiInspectionSupportStatus::Unsupported
    );
    assert_eq!(
        graph_report.posture(),
        UiInspectionSupportPosture::Deferred
    );
    assert_eq!(
        graph_report.reason(),
        Some(UiInspectionSupportReason::BelongsArchitecturallyNotYetAdmitted)
    );
    assert_eq!(
        graph_report.expected_in(),
        Some(UiInspectionMilestoneExpectation::Milestone31)
    );
    assert_eq!(measurement_report.scope(), UiInspectionScope::Measurement);
    assert_eq!(measurement_report.current_world(), UiInspectionSupportWorld::Authoritative);
    assert_eq!(mounting_report.current_world(), UiInspectionSupportWorld::Authoritative);
    assert_eq!(rebind_report.current_world(), UiInspectionSupportWorld::Authoritative);
    assert_eq!(closure_report.rows().len(), 16);
}

#[test]
fn unsupported_scope_families_fail_locally_through_runtime_receipts() {
    let app = WorthUi::app()
        .with_dsl_package(worth_ui_dsl::WorthUiDslPackage::empty())
        .freeze();

    for scope in [UiInspectionScope::graph()] {
        let receipt = app.inspect(UiInspectionQuery::new(
            UiInspectionTarget::product_root(),
            scope,
        ));
        let support_report = app.inspection_support_report(scope);

        assert_eq!(receipt.query().scope(), scope);
        assert_eq!(receipt.support_report(), Some(support_report));
        assert_eq!(
            receipt.posture(),
            Some(UiInspectionPosture::deferred(
                Some(UiInspectionMilestoneExpectation::Milestone31),
                UiInspectionSupportWorld::Authoritative,
            ))
        );
    }
}

#[test]
fn inspection_relevance_keeps_query_and_worth_local_evidence_distinct() {
    let worth_local = UiInspectionRelevance::worth_local_only();
    let query_inspection = UiInspectionRelevance::query_inspection_only();
    let query_projection = UiInspectionRelevance::query_projection_consumption_only();
    let query_backed = UiInspectionRelevance::query_backed_only();

    assert_eq!(
        worth_local,
        UiInspectionRelevance::Only(UiInspectionEvidenceSource::WorthLocal)
    );
    assert!(worth_local.includes_worth_local_evidence());
    assert!(!worth_local.includes_query_inspection());
    assert!(!worth_local.includes_query_projection_consumption());

    assert_eq!(
        query_inspection,
        UiInspectionRelevance::Only(UiInspectionEvidenceSource::QueryInspection)
    );
    assert!(!query_inspection.includes_worth_local_evidence());
    assert!(query_inspection.includes_query_inspection());
    assert!(!query_inspection.includes_query_projection_consumption());

    assert_eq!(
        query_projection,
        UiInspectionRelevance::Only(UiInspectionEvidenceSource::QueryProjectionConsumption)
    );
    assert!(!query_projection.includes_worth_local_evidence());
    assert!(!query_projection.includes_query_inspection());
    assert!(query_projection.includes_query_projection_consumption());

    assert_eq!(query_backed, UiInspectionRelevance::QueryBackedOnly);
    assert!(!query_backed.includes_worth_local_evidence());
    assert!(query_backed.includes_query_inspection());
    assert!(query_backed.includes_query_projection_consumption());
}
