use super::*;

mod bridge_commit_wave;
mod medium_region_wave;
mod mixed_locality_wave;

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture --test-threads=1"]
fn perf_runtime_bridge_mock_matrix() {
    let suite = "runtime_bridge_mock_matrix";

    bridge_commit_wave::certify_geometry_commit_bridge_wave(suite);
    medium_region_wave::certify_geometry_commit_bridge_medium_region_wave(suite);
    mixed_locality_wave::certify_geometry_commit_bridge_mixed_locality_wave(suite);
}
