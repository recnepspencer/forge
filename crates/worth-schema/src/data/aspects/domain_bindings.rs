use crate::data::entities::{DiagnosticsEntityKind, EntityKind};
use crate::data::relations::{
    DiagnosticsRelationKind, GeometryRelationKind, NamingRelationKind, RelationKind,
    TopologyRelationKind,
};

use super::{Aspect, DiagnosticsAspect, GeometryAspect, NamingAspect, TopologyAspect};

pub fn entity_domain_aspect(kind: EntityKind) -> Aspect {
    match kind {
        EntityKind::Topology(_) => Aspect::Topology(TopologyAspect::Structure),
        EntityKind::Geometry(_) => Aspect::Geometry(GeometryAspect::Binding),
        EntityKind::Naming(_) => Aspect::Naming(NamingAspect::PersistentName),
        EntityKind::Diagnostics(DiagnosticsEntityKind::WireInterpretation)
        | EntityKind::Diagnostics(DiagnosticsEntityKind::ShellInterpretation) => {
            Aspect::Diagnostics(DiagnosticsAspect::Interpretations)
        }
    }
}

pub fn entity_domain_field(kind: EntityKind) -> &'static str {
    match kind {
        EntityKind::Topology(_) => "structure",
        EntityKind::Geometry(_) => "binding",
        EntityKind::Naming(_) => "persistent_name",
        EntityKind::Diagnostics(DiagnosticsEntityKind::WireInterpretation)
        | EntityKind::Diagnostics(DiagnosticsEntityKind::ShellInterpretation) => "interpretations",
    }
}

pub fn relation_domain_aspect(kind: RelationKind) -> Aspect {
    match kind {
        RelationKind::Topology(TopologyRelationKind::ModelOwnsBody)
        | RelationKind::Topology(TopologyRelationKind::BodyOwnsLump)
        | RelationKind::Topology(TopologyRelationKind::LumpOwnsRegion)
        | RelationKind::Topology(TopologyRelationKind::RegionOwnsShell)
        | RelationKind::Topology(TopologyRelationKind::ShellOwnsFace)
        | RelationKind::Topology(TopologyRelationKind::WireOwnsHalfEdge) => {
            Aspect::Topology(TopologyAspect::Ownership)
        }
        RelationKind::Topology(TopologyRelationKind::FaceOuterLoop)
        | RelationKind::Topology(TopologyRelationKind::FaceInnerLoop)
        | RelationKind::Topology(TopologyRelationKind::LoopOwnsHalfEdge)
        | RelationKind::Topology(TopologyRelationKind::HalfEdgeNext)
        | RelationKind::Topology(TopologyRelationKind::HalfEdgePrev)
        | RelationKind::Topology(TopologyRelationKind::HalfEdgeUsesEdge)
        | RelationKind::Topology(TopologyRelationKind::HalfEdgeStartsAtVertex)
        | RelationKind::Topology(TopologyRelationKind::HalfEdgeEndsAtVertex) => {
            Aspect::Topology(TopologyAspect::Boundary)
        }
        RelationKind::Topology(TopologyRelationKind::HalfEdgeRadialNext) => {
            Aspect::Topology(TopologyAspect::Radial)
        }
        RelationKind::Geometry(
            GeometryRelationKind::FaceUsesSurfaceBinding
            | GeometryRelationKind::EdgeUsesCurveBinding
            | GeometryRelationKind::HalfEdgeUsesCoedgeBinding
            | GeometryRelationKind::VertexUsesGeometryBinding,
        ) => Aspect::Geometry(GeometryAspect::Binding),
        RelationKind::Naming(NamingRelationKind::PersistentNameTargetsEntity) => {
            Aspect::Naming(NamingAspect::PersistentName)
        }
        RelationKind::Diagnostics(
            DiagnosticsRelationKind::WireHasInterpretation
            | DiagnosticsRelationKind::ShellHasInterpretation,
        ) => Aspect::Diagnostics(DiagnosticsAspect::Interpretations),
    }
}
