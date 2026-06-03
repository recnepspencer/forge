use topology::facade::{
    build_milestone_one_bridge, milestone_one_bridge_aspect_registrations,
    milestone_one_bridge_mapping_registrations,
};
use topology::runtime_support::{build_runtime_bridge, TopologyRuntimeBinding};

fn main() {
    let _ = build_milestone_one_bridge;
    let _ = milestone_one_bridge_mapping_registrations;
    let _ = milestone_one_bridge_aspect_registrations;
    let _ = build_runtime_bridge;
    let _ = std::mem::size_of::<TopologyRuntimeBinding>();
}
