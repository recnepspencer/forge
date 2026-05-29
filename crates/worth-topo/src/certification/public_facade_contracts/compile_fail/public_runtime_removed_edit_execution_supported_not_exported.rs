use topology::facade::TopologyRuntimeSupport;

fn main() {
    let support = TopologyRuntimeSupport::current_head_authoritative();
    let _ = support.query_edit_execution_supported();
}




