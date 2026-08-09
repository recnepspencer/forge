use worth_ui::facade::app::{WorthUi, WorthUiApp};
use worth_ui::facade::inspection::{
    UiEvidenceBudget, UiEvidenceRichness, UiInspectionQuery, UiInspectionReceipt,
    UiInspectionRelevance, UiInspectionScope, UiInspectionTarget, UiRelevanceFamily,
    UiRelevanceFilter,
};

fn inspection_query() -> UiInspectionQuery {
    UiInspectionQuery::new(
        UiInspectionTarget::product_root(),
        UiInspectionScope::graph(),
    )
    .with_richness(UiEvidenceRichness::summary())
    .with_budget(UiEvidenceBudget::ordinary())
    .with_relevance(UiInspectionRelevance::local(UiRelevanceFilter::family(
        UiRelevanceFamily::Declaration,
    )))
}

fn ai_consumer(app: &WorthUiApp) -> UiInspectionReceipt {
    app.inspect(inspection_query())
}

fn human_consumer(app: &WorthUiApp) -> UiInspectionReceipt {
    app.inspect(inspection_query())
}

fn main() {
    let app = WorthUi::app()
        .bind_certification_host_adapter(worth_ui_host_contract::UiCertificationHostBindingGrant::for_certification(), worth_ui_host_headless::WorthUiHeadlessHost)
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .freeze().expect("application preparation should succeed");

    let _ = ai_consumer(&app);
    let _ = human_consumer(&app);
}
