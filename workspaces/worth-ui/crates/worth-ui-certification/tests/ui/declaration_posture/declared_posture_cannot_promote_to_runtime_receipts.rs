use worth_ui::facade::declaration::{
    UiDeclaredQueryBindingPosture, UiDeclaredServiceUsagePosture,
};
use worth_ui_runtime::facade::{
    WorthUiQuerySupportReceipt, WorthUiRuntimeHandleAllocationReceipt,
};

fn main() {
    let _query_receipt: WorthUiQuerySupportReceipt =
        UiDeclaredQueryBindingPosture::AttachedViewBinding;
    let _allocation_receipt: WorthUiRuntimeHandleAllocationReceipt =
        UiDeclaredServiceUsagePosture::Portal;
}
