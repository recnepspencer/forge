use super::*;

#[test]
fn facade_inspection_from_immutable_app_reference_uses_lifecycle_owned_support_posture() {
    let app = WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .freeze()
        .map(
            worth_ui_runtime::facade::entry::WorthUiCertificationApplicationTransition::activate_headless,
        )
        .expect("application preparation should succeed");
    let scope = UiInspectionScope::graph();
    let support_report = app.inspection_support_report(scope);
    let receipt = app.inspect(UiInspectionQuery::new(
        UiInspectionTarget::product_root(),
        scope,
    ));

    assert_eq!(receipt.query().scope(), UiInspectionScope::Graph);
    assert_eq!(
        support_report.status(),
        UiInspectionSupportStatus::Unsupported
    );
    assert_eq!(
        receipt.relevance_outcome(),
        worth_ui::facade::inspection::UiInspectionRelevanceOutcome::UnsupportedScope {
            scope: UiInspectionScope::Graph,
        }
    );
    assert_eq!(receipt.support_report(), Some(support_report));
    assert_eq!(app.inspection_support_report(scope), support_report);
    assert_eq!(
        receipt.posture(),
        Some(UiInspectionPosture::deferred(
            Some(UiInspectionMilestoneExpectation::Milestone31),
            UiInspectionSupportWorld::Authoritative,
        ))
    );
}
