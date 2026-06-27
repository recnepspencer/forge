use worth_ui_runtime::facade::{
    WorthUiRuntimeSupportInventory, WorthUiRuntimeSupportInventoryFields, WorthUiRuntimeSupportRow,
};

fn main() {
    let fields = WorthUiRuntimeSupportInventoryFields {
        dsl_package: WorthUiRuntimeSupportRow::new("dsl_package"),
        inspection: WorthUiRuntimeSupportRow::new("inspection"),
        query_binding: WorthUiRuntimeSupportRow::new("query_binding"),
        host_contract: WorthUiRuntimeSupportRow::new("host_contract"),
    };

    let _ = WorthUiRuntimeSupportInventory::new(fields);
}
