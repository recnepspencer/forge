use worth_ui::facade::app::WorthUi;
use worth_ui::facade::inspection::{
    UiEvidenceBudget, UiEvidenceRichness, UiInspectionQuery, UiInspectionRelevance,
    UiInspectionScope, UiInspectionTarget,
};

fn main() {
    let app = WorthUi::app()
        .with_dsl_package(worth_ui_dsl::WorthUiDslPackage::empty())
        .freeze();
    let query = UiInspectionQuery::new(
        UiInspectionTarget::product_root(),
        UiInspectionScope::graph(),
    )
    .with_budget(UiEvidenceBudget::bounded(UiEvidenceRichness::summary()))
    .with_relevance(UiInspectionRelevance::worth_local_only());

    let _ = app.inspect(query);
}
