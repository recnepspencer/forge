use topology::facade::TopologyRuntimeSupport;

fn main() {
    let support = TopologyRuntimeSupport::snapshot_read_only();
    let _ = support.historical_basis_supported();
}
