use worth_topo::facade::{WorthTopologyRuntimeAdapters, WorthTopologyRuntimeSupport};

fn main() {
    let _ = WorthTopologyRuntimeAdapters {
        support: WorthTopologyRuntimeSupport::snapshot_read_only(),
    };
}
