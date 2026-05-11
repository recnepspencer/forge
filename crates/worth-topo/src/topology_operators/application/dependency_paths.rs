use schema::facade::{QueryAspectPath, RelationKind, TopologyRelationKind};

pub(crate) fn topology_relation_dependency_path(kind: RelationKind) -> Option<&'static str> {
    match kind {
        RelationKind::Topology(
            TopologyRelationKind::ModelOwnsBody
            | TopologyRelationKind::BodyOwnsLump
            | TopologyRelationKind::LumpOwnsRegion
            | TopologyRelationKind::RegionOwnsShell
            | TopologyRelationKind::ShellOwnsFace
            | TopologyRelationKind::WireOwnsHalfEdge,
        ) => Some(QueryAspectPath::TOPOLOGY_OWNERSHIP.as_str()),
        RelationKind::Topology(
            TopologyRelationKind::FaceOuterLoop
            | TopologyRelationKind::FaceInnerLoop
            | TopologyRelationKind::LoopOwnsHalfEdge
            | TopologyRelationKind::HalfEdgeNext
            | TopologyRelationKind::HalfEdgePrev
            | TopologyRelationKind::HalfEdgeUsesEdge
            | TopologyRelationKind::HalfEdgeStartsAtVertex
            | TopologyRelationKind::HalfEdgeEndsAtVertex,
        ) => Some(QueryAspectPath::TOPOLOGY_BOUNDARY.as_str()),
        RelationKind::Topology(TopologyRelationKind::HalfEdgeRadialNext) => {
            Some(QueryAspectPath::TOPOLOGY_RADIAL.as_str())
        }
        _ => None,
    }
}
