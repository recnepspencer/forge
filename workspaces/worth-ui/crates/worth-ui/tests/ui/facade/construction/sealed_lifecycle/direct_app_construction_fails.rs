use worth_ui::facade::{
    app::WorthUiApp,
    diagnostics::CapabilitySnapshot,
};

fn main() {
    let _ = WorthUiApp {
        capability_snapshot: capability_snapshot(),
    };
}

fn capability_snapshot() -> CapabilitySnapshot {
    panic!("fixture never runs")
}
