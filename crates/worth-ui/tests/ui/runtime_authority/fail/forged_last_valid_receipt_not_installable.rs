use worth_ui::facade::{WorthUiLastValidRuntimeState, WorthUiRuntimeHost};

fn main() {
    let _ = std::mem::size_of::<WorthUiRuntimeHost>();
    let _ = std::mem::size_of::<WorthUiLastValidRuntimeState>();
}
