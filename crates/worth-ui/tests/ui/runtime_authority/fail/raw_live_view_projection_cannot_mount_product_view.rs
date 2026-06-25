use worth_ui::facade::{
    WorthUiLiveViewProjectionAdmissionReceipt, WorthUiPageHostPlan, WorthUiRuntimeHost,
};

fn main() {
    fn illegal(
        runtime: &WorthUiRuntimeHost,
        page_host_plan: &WorthUiPageHostPlan,
        projection: &WorthUiLiveViewProjectionAdmissionReceipt,
    ) {
        let _ = runtime.mount_live_view_product_projection_for_page(page_host_plan, projection);
    }
}
