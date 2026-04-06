use forge_runtime_bridge::facade::SnapshotReadRequest;

fn main() {
    let _ = SnapshotReadRequest::for_coarse("user", "profile");
}
