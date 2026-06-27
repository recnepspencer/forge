use worth_ui::facade::{CapabilitySnapshotDigest, SnapshotMetrics};

fn main() {
    let _ = CapabilitySnapshotDigest::from_metrics(metrics());
}

fn metrics() -> SnapshotMetrics {
    panic!("fixture never runs")
}
