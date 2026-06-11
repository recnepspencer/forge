use topology::facade::TopologyWorkload;
use worth_spatial::facade::workload_vocabulary::{
    GeometryBindingWorkload, ProjectionWorkload,
};

fn main() {
    let topology = TopologyWorkload::declared("topology seed")
        .from_query_declaration(".topology.seed")
        .unwrap();
    let geometry = GeometryBindingWorkload::for_topology_receipt(&topology)
        .admit()
        .unwrap();

    let _projection = ProjectionWorkload::for_surface_support(&geometry);
}
