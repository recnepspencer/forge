use std::collections::BTreeMap;

use worth_ui::facade::{DensityTokenId, WorthUiDensityPostureValue, WorthUiDensityValue};
use worth_ui_validation_app::ValidationWorkbenchLaunch;

fn main() {
    let workbench = ValidationWorkbenchLaunch::new()
        .prepare()
        .unwrap()
        .into_runtime_workbench();
    let values = BTreeMap::from([(
        DensityTokenId::new("validation.density.header.control_spacing").unwrap(),
        WorthUiDensityValue::posture(WorthUiDensityPostureValue::dense()),
    )]);
    let _ = workbench.prepare_density_capability_reload(&values);
}
