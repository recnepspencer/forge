use worth_ui_runtime::facade::{WorthUiApp, WorthUiRuntimeBootstrap};

fn main() {
    let _ = core::mem::size_of::<WorthUiApp>();
    let _ = core::mem::size_of::<WorthUiRuntimeBootstrap>();
}
