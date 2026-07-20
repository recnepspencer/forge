use worth_ui::facade::{
    diagnostics::{CapabilitySnapshot, CapabilitySnapshotDigest, RegisteredCapabilitySet, SnapshotMetrics},
};

fn main() {
    let _ = CapabilitySnapshot {
        registered_capabilities: RegisteredCapabilitySet {
            registered_family_count: 0,
            total_width: 0,
        },
        digest: digest(),
        metrics: metrics(),
    };
}

fn digest() -> CapabilitySnapshotDigest {
    panic!("fixture never runs")
}

fn metrics() -> SnapshotMetrics {
    panic!("fixture never runs")
}
