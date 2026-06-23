use std::collections::BTreeMap;

use worth_ui::facade::{AppearanceTokenId, ThemeColorValue, WorthUiAppearanceValue};
use worth_ui_validation_app::ValidationWorkbenchLaunch;

fn main() {
    let workbench = ValidationWorkbenchLaunch::new()
        .prepare()
        .unwrap()
        .into_runtime_workbench();
    let values = BTreeMap::from([(
        AppearanceTokenId::new("validation.appearance.header.menu_min_width").unwrap(),
        WorthUiAppearanceValue::color(ThemeColorValue::hex("#102030").unwrap()),
    )]);
    let _ = workbench.prepare_appearance_capability_reload(&values);
}
