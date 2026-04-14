use forge_relational::facade::publication::AspectKey;
use forge_relational::facade::schema::{
    AspectBinding, AspectComparator, AspectPrecision, DeclaredAspect, KindAspectDeclarations,
};
use forge_relational::facade::symbols::InternedString;

use crate::data::aspects::{
    WorthAspect, WorthDiagnosticsAspect, WorthGeometryAspect, WorthNamingAspect,
    WorthTopologyAspect,
};
use crate::data::relations::{
    WorthDiagnosticsRelationKind, WorthGeometryRelationKind, WorthNamingRelationKind,
    WorthRelationKind, WorthTopologyRelationKind,
};

pub fn relation_aspects(kind: WorthRelationKind) -> KindAspectDeclarations {
    KindAspectDeclarations::new(vec![
        relation_domain_aspect(domain_aspect(kind)),
        lifecycle_aspect(),
        relation_source_aspect(),
        relation_target_aspect(),
    ])
}

fn domain_aspect(kind: WorthRelationKind) -> WorthAspect {
    match kind {
        WorthRelationKind::Topology(WorthTopologyRelationKind::ModelOwnsBody)
        | WorthRelationKind::Topology(WorthTopologyRelationKind::BodyOwnsLump)
        | WorthRelationKind::Topology(WorthTopologyRelationKind::LumpOwnsRegion)
        | WorthRelationKind::Topology(WorthTopologyRelationKind::RegionOwnsShell)
        | WorthRelationKind::Topology(WorthTopologyRelationKind::ShellOwnsFace) => {
            WorthAspect::Topology(WorthTopologyAspect::Ownership)
        }
        WorthRelationKind::Topology(WorthTopologyRelationKind::FaceOuterLoop)
        | WorthRelationKind::Topology(WorthTopologyRelationKind::FaceInnerLoop) => {
            WorthAspect::Topology(WorthTopologyAspect::Boundary)
        }
        WorthRelationKind::Topology(WorthTopologyRelationKind::LoopOwnsHalfEdge)
        | WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgeNext)
        | WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgePrev)
        | WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgeUsesEdge)
        | WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgeStartsAtVertex)
        | WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgeEndsAtVertex) => {
            WorthAspect::Topology(WorthTopologyAspect::Boundary)
        }
        WorthRelationKind::Topology(WorthTopologyRelationKind::WireOwnsHalfEdge) => {
            WorthAspect::Topology(WorthTopologyAspect::Ownership)
        }
        WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgeRadialNext) => {
            WorthAspect::Topology(WorthTopologyAspect::Radial)
        }
        WorthRelationKind::Geometry(
            WorthGeometryRelationKind::FaceUsesSurfaceBinding
            | WorthGeometryRelationKind::EdgeUsesCurveBinding
            | WorthGeometryRelationKind::HalfEdgeUsesCoedgeBinding
            | WorthGeometryRelationKind::VertexUsesGeometryBinding,
        ) => WorthAspect::Geometry(WorthGeometryAspect::Binding),
        WorthRelationKind::Naming(WorthNamingRelationKind::PersistentNameTargetsEntity) => {
            WorthAspect::Naming(WorthNamingAspect::PersistentName)
        }
        WorthRelationKind::Diagnostics(
            WorthDiagnosticsRelationKind::WireHasInterpretation
            | WorthDiagnosticsRelationKind::ShellHasInterpretation,
        ) => WorthAspect::Diagnostics(WorthDiagnosticsAspect::Interpretations),
    }
}

fn relation_domain_aspect(aspect: WorthAspect) -> DeclaredAspect {
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
