use forge_relational::facade::publication::AspectKey;
use forge_relational::facade::schema::{
    AspectBinding, AspectComparator, AspectPrecision, DeclaredAspect, KindAspectDeclarations,
};
use forge_relational::facade::symbols::InternedString;

use crate::data::aspects::{
    Aspect, DiagnosticsAspect, GeometryAspect, NamingAspect, TopologyAspect,
};
use crate::data::relations::{
    DiagnosticsRelationKind, GeometryRelationKind, NamingRelationKind, RelationKind,
    TopologyRelationKind,
};

pub fn relation_aspects(kind: RelationKind) -> KindAspectDeclarations {
    KindAspectDeclarations::new(vec![
        relation_domain_aspect(domain_aspect(kind)),
        lifecycle_aspect(),
        relation_source_aspect(),
        relation_target_aspect(),
    ])
}

fn domain_aspect(kind: RelationKind) -> Aspect {
    match kind {
        RelationKind::Topology(TopologyRelationKind::ModelOwnsBody)
        | RelationKind::Topology(TopologyRelationKind::BodyOwnsLump)
        | RelationKind::Topology(TopologyRelationKind::LumpOwnsRegion)
        | RelationKind::Topology(TopologyRelationKind::RegionOwnsShell)
        | RelationKind::Topology(TopologyRelationKind::ShellOwnsFace) => {
            Aspect::Topology(TopologyAspect::Ownership)
        }
        RelationKind::Topology(TopologyRelationKind::FaceOuterLoop)
        | RelationKind::Topology(TopologyRelationKind::FaceInnerLoop) => {
            Aspect::Topology(TopologyAspect::Boundary)
        }
        RelationKind::Topology(TopologyRelationKind::LoopOwnsHalfEdge)
        | RelationKind::Topology(TopologyRelationKind::HalfEdgeNext)
        | RelationKind::Topology(TopologyRelationKind::HalfEdgePrev)
        | RelationKind::Topology(TopologyRelationKind::HalfEdgeUsesEdge)
        | RelationKind::Topology(TopologyRelationKind::HalfEdgeStartsAtVertex)
        | RelationKind::Topology(TopologyRelationKind::HalfEdgeEndsAtVertex) => {
            Aspect::Topology(TopologyAspect::Boundary)
        }
        RelationKind::Topology(TopologyRelationKind::WireOwnsHalfEdge) => {
            Aspect::Topology(TopologyAspect::Ownership)
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

fn relation_domain_aspect(aspect: Aspect) -> DeclaredAspect {
    DeclaredAspect {
        key: aspect.aspect_key(),
        binding: AspectBinding::RelationTargetEndpoint,
        comparator: AspectComparator::EndpointIdentityEquality,
        precision: AspectPrecision::Structured,
    }
}

fn lifecycle_aspect() -> DeclaredAspect {
    DeclaredAspect {
        key: AspectKey(InternedString::Raw("lifecycle".to_string())),
        binding: AspectBinding::LifecycleTransition,
        comparator: AspectComparator::LifecycleTransitionEquality,
        precision: AspectPrecision::Structured,
    }
}

fn relation_source_aspect() -> DeclaredAspect {
    DeclaredAspect {
        key: AspectKey(InternedString::Raw("source".to_string())),
        binding: AspectBinding::RelationSourceEndpoint,
        comparator: AspectComparator::EndpointIdentityEquality,
        precision: AspectPrecision::Structured,
    }
}

fn relation_target_aspect() -> DeclaredAspect {
    DeclaredAspect {
        key: AspectKey(InternedString::Raw("target".to_string())),
        binding: AspectBinding::RelationTargetEndpoint,
        comparator: AspectComparator::EndpointIdentityEquality,
        precision: AspectPrecision::Structured,
    }
}
