use worth_ui::facade::app::WorthUi;
use worth_ui::facade::inspection::{
    UiEvidenceBudget, UiEvidenceRichness, UiInspectionQuery, UiInspectionRelevance,
    UiInspectionScope, UiInspectionTarget, UiRelevanceFamily, UiRelevanceFilter,
};

fn main() {
    let app = WorthUi::app()
        .with_dsl_package(worth_ui_dsl::WorthUiDslPackage::empty())
        .freeze().expect("application preparation should succeed");
    let query = UiInspectionQuery::new(
        UiInspectionTarget::product_root(),
        UiInspectionScope::graph(),
    )
    .with_richness(UiEvidenceRichness::summary())
    .with_budget(UiEvidenceBudget::ordinary())
    .with_relevance(UiInspectionRelevance::local(UiRelevanceFilter::family(
        UiRelevanceFamily::Declaration,
    )));

    let _ = app.inspect(query);
}
