#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthGraphReadReadFamilyTarget {
    TopologyHalfEdgeSharedVertexNeighborhood,
    TopologyHalfEdgeRadialNeighborhood,
    TopologyLoopCycleNeighborhood,
    TopologyLocalRewireNeighborhood,
    SpatialPlanarBooleanContinuationIndex,
    BroadBooleanPredicateGraphRead,
}

impl WorthGraphReadReadFamilyTarget {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TopologyHalfEdgeSharedVertexNeighborhood => {
                "topology_half_edge_shared_vertex_neighborhood"
            }
            Self::TopologyHalfEdgeRadialNeighborhood => "topology_half_edge_radial_neighborhood",
            Self::TopologyLoopCycleNeighborhood => "topology_loop_cycle_neighborhood",
            Self::TopologyLocalRewireNeighborhood => "topology_local_rewire_neighborhood",
            Self::SpatialPlanarBooleanContinuationIndex => {
                "spatial_planar_boolean_continuation_index"
            }
            Self::BroadBooleanPredicateGraphRead => "broad_boolean_predicate_graph_read",
        }
    }
}
