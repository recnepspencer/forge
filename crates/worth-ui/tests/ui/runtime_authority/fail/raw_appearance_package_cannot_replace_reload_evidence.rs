use worth_ui::facade::{
    WorthUiAppearanceReloadPackage, WorthUiCapabilityReloadRequest, WorthUiRuntimeHost,
};

fn main() {
    let runtime = forged_runtime();
    let request = WorthUiCapabilityReloadRequest::from_appearance(
        WorthUiAppearanceReloadPackage::from_source(
            "app/theme/header.appearance",
            "appearance.header.menu_min_width = 220px",
        ),
    );
    runtime.admit_capability_runtime_change(&request);
}

fn forged_runtime() -> WorthUiRuntimeHost {
    panic!("fixture should fail before runtime construction")
}
