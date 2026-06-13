use worth_kernel::workload_composition::PlanarBooleanEntryBasis;
use worth_topo::facade::TopologyWorkload;

fn main() {
    let topology = TopologyWorkload::declared("topology-seed")
        .from_query_declaration("topology.seed")
        .expect("topology should certify");
    let _ = PlanarBooleanEntryBasis::bind(topology, "topology basis");
}
