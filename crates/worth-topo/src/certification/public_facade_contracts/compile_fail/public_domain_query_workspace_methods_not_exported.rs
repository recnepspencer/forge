use topology::facade::TopologyDomainQuery;

fn main() {
    let _ = TopologyDomainQuery::shared_vertex_half_edge_neighborhood;
    let _ = TopologyDomainQuery::radial_half_edge_neighborhood;
    let _ = TopologyDomainQuery::loop_cycle;
    let _ = TopologyDomainQuery::local_rewire_neighborhood;
}
