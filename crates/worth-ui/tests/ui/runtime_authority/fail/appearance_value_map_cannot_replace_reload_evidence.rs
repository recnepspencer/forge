use std::collections::BTreeMap;

use worth_ui::facade::{
    AppearanceTokenId, ThemeColorValue, WorthUiAppearanceValue, WorthUiRuntimeHost,
};

fn main() {
    let runtime = forged_runtime();
    let values = BTreeMap::from([(
        AppearanceTokenId::new("appearance.header.panel").unwrap(),
        WorthUiAppearanceValue::color(ThemeColorValue::hex("#102030").unwrap()),
    )]);
    runtime.admit_capability_runtime_change(&values);
}

fn forged_runtime() -> WorthUiRuntimeHost {
    panic!("fixture should fail before runtime construction")
}
