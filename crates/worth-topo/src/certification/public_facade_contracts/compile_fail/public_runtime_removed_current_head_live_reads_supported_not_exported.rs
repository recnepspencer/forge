use topology::facade::TopologyRuntimeSupport;

fn main() {
    let support = TopologyRuntimeSupport::current_head_authoritative();
    let _ = support.current_head_live_reads_supported();
}




