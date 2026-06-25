use schema::facade::platform::relations::{RelationKind, TopologyRelationKind};
use schema::facade::QueryAspectPath;

pub(crate) fn topology_relation_dependency_path(kind: RelationKind) -> Option<QueryAspectPath> {
    match kind {
        RelationKind::Topology(
            TopologyRelationKind::ModelOwnsBody
            | TopologyRelationKind::BodyOwnsLump
            | TopologyRelationKind::LumpOwnsRegion
            | TopologyRelationKind::RegionOwnsShell
            | TopologyRelationKind::ShellOwnsFace
            | TopologyRelationKind::WireOwnsHalfEdge,
        ) => Some(QueryAspectPath::TOPOLOGY_OWNERSHIP),
        RelationKind::Topology(
            TopologyRelationKind::FaceOuterLoop
            | TopologyRelationKind::FaceInnerLoop
            | TopologyRelationKind::LoopOwnsHalfEdge
            | TopologyRelationKind::HalfEdgeNext
            | TopologyRelationKind::HalfEdgePrev
            | TopologyRelationKind::HalfEdgeUsesEdge
            | TopologyRelationKind::HalfEdgeStartsAtVertex
            | TopologyRelationKind::HalfEdgeEndsAtVertex,
        ) => Some(QueryAspectPath::TOPOLOGY_BOUNDARY),
        RelationKind::Topology(TopologyRelationKind::HalfEdgeRadialNext) => {
            Some(QueryAspectPath::TOPOLOGY_RADIAL)
        }
        _ => None,
    }
}
