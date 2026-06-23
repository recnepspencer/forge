use forge_relational::facade::identity::{EntityId, PartitionId, RelationId};
use topology::facade::{
    topology_rewire_loop_endpoint_touched_graph_basis, LoopEndpointKind,
    TopologyRewireLoopEndpointDeclaration, TopologyTouchedOperatingWorld,
};

fn main() {
    let relation_id = RelationId::new(PartitionId::main(), 7, 1);
    let half_edge_id = EntityId::new(PartitionId::main(), 8, 1);
    let vertex_id = EntityId::new(PartitionId::main(), 9, 1);
    let declaration = TopologyRewireLoopEndpointDeclaration::new(
        relation_id,
        LoopEndpointKind::End,
        half_edge_id,
        vertex_id,
    );

    let _basis = topology_rewire_loop_endpoint_touched_graph_basis(
        &declaration,
        TopologyTouchedOperatingWorld::mainline(),
    );
}
