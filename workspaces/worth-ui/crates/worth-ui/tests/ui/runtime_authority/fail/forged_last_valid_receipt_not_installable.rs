use worth_ui::facade::{
    WorthUiLastValidRuntimeState,
    runtime::WorthUiRuntime,
};

fn main() {
    let _ = std::mem::size_of::<WorthUiRuntime>();
    let _ = std::mem::size_of::<WorthUiLastValidRuntimeState>();
}
