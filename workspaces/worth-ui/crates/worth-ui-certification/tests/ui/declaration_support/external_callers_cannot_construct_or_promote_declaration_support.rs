use worth_ui::facade::declaration::{
    UiDeclarationGraphHandoff, UiDeclarationSupportRow, UiDeclarationSupportRowSchemaKind,
    UiDeclarationSupportSnapshot, UiDeclaredPostureApplicability, UiDeclaredPostureContract,
};
use worth_ui::facade::{WorthUiQuerySupportReceipt, WorthUiRuntimeHandleAllocationReceipt};

fn main() {
    let _row = UiDeclarationSupportRow::without_admitted_fact(
        UiDeclarationSupportRowSchemaKind::QueryBinding,
        UiDeclaredPostureApplicability::Optional,
        None,
    );

    let snapshot =
        UiDeclarationSupportSnapshot::new(unsafe { std::mem::MaybeUninit::zeroed().assume_init() });

    let _declared_posture: UiDeclaredPostureContract = snapshot;
    let _graph_truth: UiDeclarationGraphHandoff =
        unsafe { std::mem::MaybeUninit::<UiDeclarationSupportSnapshot>::zeroed().assume_init() };
    let _query_receipt: WorthUiQuerySupportReceipt =
        unsafe { std::mem::MaybeUninit::<UiDeclarationSupportRow>::zeroed().assume_init() };
    let _allocation_receipt: WorthUiRuntimeHandleAllocationReceipt =
        unsafe { std::mem::MaybeUninit::<UiDeclarationSupportSnapshot>::zeroed().assume_init() };
}
