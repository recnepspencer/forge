use worth_ui::facade::WorthUi;
use worth_ui_harness::facade::{
    HarnessVisualFoundationBundle, HarnessVisualFoundationRegistration,
};

fn main() {
    let foundation = HarnessVisualFoundationBundle::vscode_like_dark()
        .prepare()
        .unwrap();
    let _app = WorthUi::app()
        .install_harness_visual_foundation(foundation)
        .freeze();
}
