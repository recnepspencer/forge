use super::*;

mod connectivity_wave;
mod identity_survival;

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture --test-threads=1"]
fn perf_geometry_kernel_matrix() {
    let suite = "geometry_kernel_matrix";

    identity_survival::certify_topology_identity_survival_recovery_round_trip(suite);
    connectivity_wave::certify_topology_bridge_connectivity_wave(suite);
}
