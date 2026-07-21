use worth_ui::facade::query_binding::{
    WorthUiQueryLiveResource, WorthUiSettledSnapshotProjection,
};
use worth_ui_query_binding::compatibility::managed_live::WorthUiQueryLiveRetirementCloseReceipt;

fn main() {
    let _ = std::mem::size_of::<WorthUiQueryLiveResource>();
}

fn promote(
    receipt: WorthUiQueryLiveRetirementCloseReceipt,
) -> WorthUiSettledSnapshotProjection {
    receipt.into()
}
