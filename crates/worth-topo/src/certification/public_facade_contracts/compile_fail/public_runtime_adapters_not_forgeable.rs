use topology::{facade::TopologyRuntimeAdapters, runtime_support::TopologyRuntimeSupport};

fn main() {
    let _ = TopologyRuntimeAdapters {
        support: TopologyRuntimeSupport::snapshot_read_only(),
    };
}
