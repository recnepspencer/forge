use topology::facade::{TopologyRuntimeAdapters, TopologyRuntimeSupport};

fn main() {
    let _ = TopologyRuntimeAdapters {
        support: TopologyRuntimeSupport::snapshot_read_only(),
    };
}




