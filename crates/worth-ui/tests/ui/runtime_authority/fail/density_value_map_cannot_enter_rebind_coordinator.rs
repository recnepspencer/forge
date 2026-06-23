use std::collections::BTreeMap;

use worth_ui::facade::{
    DensityTokenId, WorthUiDensityPostureValue, WorthUiDensityValue, WorthUiHeaderFramePlan,
    WorthUiHeaderFrameRebindRequest, WorthUiRuntimeHost,
};

fn main() {
    let mut runtime = forged_runtime();
    let current_plan = forged_header_frame_plan();
    let values = BTreeMap::from([(
        DensityTokenId::new("density.header.posture").unwrap(),
        WorthUiDensityValue::posture(WorthUiDensityPostureValue::compact()),
    )]);
    runtime.rebind_header_frame_after_capability_reload(
        &current_plan,
        forged_rebind_request(),
        &values,
    );
}

fn forged_runtime() -> WorthUiRuntimeHost {
    panic!("fixture should fail before runtime construction")
}

fn forged_header_frame_plan() -> WorthUiHeaderFramePlan {
    panic!("fixture should fail before runtime construction")
}

fn forged_rebind_request() -> WorthUiHeaderFrameRebindRequest {
    panic!("fixture should fail before runtime construction")
}
