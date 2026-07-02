use worth_ui::facade::app::WorthUi;
use worth_ui::facade::inspection::{
    UiInspectionMilestoneExpectation, UiInspectionQuery, UiInspectionScope,
    UiInspectionSupportWorld, UiInspectionTarget,
};

#[test]
fn repeated_unsupported_inspection_queries_stay_typed_and_equivalent() {
    let app = WorthUi::app()
        .with_dsl_package(worth_ui_dsl::WorthUiDslPackage::empty())
        .freeze();
    let query = UiInspectionQuery::new(
        UiInspectionTarget::product_root(),
        UiInspectionScope::graph(),
    );
    let observation_before = app.inspection_observation();

    let first_receipt = app.inspect(query.clone());
    let second_receipt = app.inspect(query.clone());
    let third_receipt = app.inspect(query.clone());
    let first_support_report = app.inspection_support_report(UiInspectionScope::graph());
    let second_support_report = app.inspection_support_report(UiInspectionScope::graph());
    let observation_after = app.inspection_observation();

    assert_eq!(first_receipt, second_receipt);
    assert_eq!(second_receipt, third_receipt);
    assert_eq!(first_receipt.support_report(), Some(first_support_report));
    assert_eq!(first_support_report, second_support_report);
    assert_eq!(
        observation_after.total_query_count() - observation_before.total_query_count(),
        3
    );
    assert_eq!(
        observation_after.unsupported_query_count() - observation_before.unsupported_query_count(),
        3
    );
    assert_eq!(
        observation_after.support_report_count() - observation_before.support_report_count(),
        5
    );
    assert_eq!(
        observation_after.rich_artifact_materialization_count()
            - observation_before.rich_artifact_materialization_count(),
        0
    );
    assert_eq!(
        observation_after.log_emission_count() - observation_before.log_emission_count(),
        0
    );

    assert_eq!(
        first_receipt.posture(),
        Some(worth_ui::facade::UiInspectionPosture::deferred(
            Some(UiInspectionMilestoneExpectation::Milestone31),
            UiInspectionSupportWorld::Authoritative,
        ))
    );
}
