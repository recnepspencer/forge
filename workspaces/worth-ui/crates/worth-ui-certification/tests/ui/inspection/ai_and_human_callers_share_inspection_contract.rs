use worth_ui::facade::app::{WorthUi, WorthUiApp};
use worth_ui::facade::inspection::{
    UiEvidenceBudget, UiEvidenceRichness, UiInspectionQuery, UiInspectionReceipt,
    UiInspectionRelevance, UiInspectionScope, UiInspectionTarget,
};

fn inspection_query() -> UiInspectionQuery {
    UiInspectionQuery::new(UiInspectionTarget::product_root(), UiInspectionScope::graph())
        .with_budget(UiEvidenceBudget::bounded(UiEvidenceRichness::summary()))
        .with_relevance(UiInspectionRelevance::worth_local_only())
}

fn ai_consumer(app: &WorthUiApp) -> UiInspectionReceipt {
    app.inspect(inspection_query())
}

fn human_consumer(app: &WorthUiApp) -> UiInspectionReceipt {
    app.inspect(inspection_query())
}

fn main() {
    let app = WorthUi::app()
        .with_dsl_package(worth_ui_dsl::WorthUiDslPackage::empty())
        .freeze();

    let _ = ai_consumer(&app);
    let _ = human_consumer(&app);
}
