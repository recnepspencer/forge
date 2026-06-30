use inspection_bypass::UiInspectionQuery;
use runtime_bypass::facade::UiInspectionReceipt;

pub fn bypass_runtime_owned_inspection(query: UiInspectionQuery, receipt: UiInspectionReceipt) {
    let _ = (query, receipt);
}
