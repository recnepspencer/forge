use topology::runtime_support::TopologyRuntimeSupport;

fn main() {
    let support = TopologyRuntimeSupport::current_head_authoritative();
    let _ = support.query_mutation_lane_supported("CreateInnerLoopOnExistingFace");
}
