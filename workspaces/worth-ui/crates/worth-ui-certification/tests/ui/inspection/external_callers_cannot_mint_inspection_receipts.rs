use worth_ui::facade::inspection::{
    UiInspectionPosture, UiInspectionQuery, UiInspectionReceipt, UiInspectionScope,
    UiInspectionTarget,
};

fn main() {
    let query = UiInspectionQuery::new(
        UiInspectionTarget::product_root(),
        UiInspectionScope::graph(),
    );
    let _ = UiInspectionReceipt::new(query, UiInspectionPosture::Available);
}
