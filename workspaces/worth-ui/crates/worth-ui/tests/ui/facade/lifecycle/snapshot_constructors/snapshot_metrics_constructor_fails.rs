use worth_ui::facade::{
    diagnostics::{RegisteredCapabilitySet, SnapshotMetrics},
};

fn main() {
    let _ = SnapshotMetrics::from_registered_capabilities(&registered_capabilities());
}

fn registered_capabilities() -> RegisteredCapabilitySet {
    panic!("fixture never runs")
}
