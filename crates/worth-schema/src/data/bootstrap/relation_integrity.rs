use forge_relational::facade::config::CrossContextPolicy;
use forge_relational::facade::identity::KindId;
use forge_relational::facade::schema::{
    CardinalityContractDeclaration, EndpointKindContractDeclaration, MinimumCardinalityEnforcement,
    PairMinimumSemantics, RelationIntegrityDeclarations, UniquenessContractDeclaration,
    UniquenessScope,
};

use crate::data::entities::{
    DiagnosticsEntityKind, EntityKind, GeometryEntityKind, NamingEntityKind, TopologyEntityKind,
};
use crate::data::relations::{
    DiagnosticsRelationKind, GeometryRelationKind, NamingRelationKind, RelationKind,
    TopologyRelationKind,
};

pub fn relation_integrity(kind: RelationKind) -> RelationIntegrityDeclarations {
    let (allowed_source_kinds, allowed_target_kinds, self_edges_allowed) = endpoint_domain(kind);
    let (source_max, target_max, pair_max) = cardinality_limits(kind);
    let contract_stem = kind.kind_name().replace(".", "");

    RelationIntegrityDeclarations::new(
        vec![EndpointKindContractDeclaration {
            contract_id: format!("{contract_stem}.endpoint_domain").into(),
            allowed_source_kinds,
            allowed_target_kinds,
            self_edges_allowed,
            cross_context_policy: CrossContextPolicy::Forbid,
        }],
        vec![CardinalityContractDeclaration {
            contract_id: format!("{contract_stem}.cardinality").into(),
            source_max,
            target_max,
            pair_max,
            source_min: None,
            target_min: None,
            pair_min: None,
            pair_min_semantics: PairMinimumSemantics::ObservedDirectedPairs,
            minimum_enforcement: MinimumCardinalityEnforcement::CommitBoundary,
        }],
        vec![UniquenessContractDeclaration {
            contract_id: format!("{contract_stem}.uniqueness").into(),
            scope: UniquenessScope::DirectedSemanticEdge,
        }],
        Vec::new(),
        Vec::new(),
    )
}

fn endpoint_domain(kind: RelationKind) -> (Vec<KindId>, Vec<KindId>, bool) {
    match kind {
        RelationKind::Topology(TopologyRelationKind::ModelOwnsBody) => endpoint_pair(
            EntityKind::Topology(TopologyEntityKind::Model),
            EntityKind::Topology(TopologyEntityKind::Body),
            false,
        ),
        RelationKind::Topology(TopologyRelationKind::BodyOwnsLump) => endpoint_pair(
            EntityKind::Topology(TopologyEntityKind::Body),
            EntityKind::Topology(TopologyEntityKind::Lump),
            false,
        ),
        RelationKind::Topology(TopologyRelationKind::LumpOwnsRegion) => endpoint_pair(
            EntityKind::Topology(TopologyEntityKind::Lump),
            EntityKind::Topology(TopologyEntityKind::Region),
            false,
        ),
        RelationKind::Topology(TopologyRelationKind::RegionOwnsShell) => endpoint_pair(
            EntityKind::Topology(TopologyEntityKind::Region),
            EntityKind::Topology(TopologyEntityKind::Shell),
            false,
        ),
        RelationKind::Topology(TopologyRelationKind::ShellOwnsFace) => endpoint_pair(
            EntityKind::Topology(TopologyEntityKind::Shell),
            EntityKind::Topology(TopologyEntityKind::Face),
            false,
        ),
        RelationKind::Topology(TopologyRelationKind::FaceOuterLoop)
        | RelationKind::Topology(TopologyRelationKind::FaceInnerLoop) => endpoint_pair(
            EntityKind::Topology(TopologyEntityKind::Face),
            EntityKind::Topology(TopologyEntityKind::Loop),
            false,
        ),
        RelationKind::Topology(TopologyRelationKind::LoopOwnsHalfEdge) => endpoint_pair(
            EntityKind::Topology(TopologyEntityKind::Loop),
            EntityKind::Topology(TopologyEntityKind::HalfEdge),
            false,
        ),
        RelationKind::Topology(TopologyRelationKind::WireOwnsHalfEdge) => endpoint_pair(
            EntityKind::Topology(TopologyEntityKind::Wire),
            EntityKind::Topology(TopologyEntityKind::HalfEdge),
            false,
        ),
        RelationKind::Topology(TopologyRelationKind::HalfEdgeNext)
        | RelationKind::Topology(TopologyRelationKind::HalfEdgePrev)
        | RelationKind::Topology(TopologyRelationKind::HalfEdgeRadialNext) => (
            vec![EntityKind::Topology(TopologyEntityKind::HalfEdge).kind_id()],
            vec![EntityKind::Topology(TopologyEntityKind::HalfEdge).kind_id()],
            true,
        ),
        RelationKind::Topology(TopologyRelationKind::HalfEdgeUsesEdge) => endpoint_pair(
            EntityKind::Topology(TopologyEntityKind::HalfEdge),
            EntityKind::Topology(TopologyEntityKind::Edge),
            false,
        ),
        RelationKind::Topology(TopologyRelationKind::HalfEdgeStartsAtVertex) => endpoint_pair(
            EntityKind::Topology(TopologyEntityKind::HalfEdge),
            EntityKind::Topology(TopologyEntityKind::Vertex),
            false,
        ),
        RelationKind::Topology(TopologyRelationKind::HalfEdgeEndsAtVertex) => endpoint_pair(
            EntityKind::Topology(TopologyEntityKind::HalfEdge),
            EntityKind::Topology(TopologyEntityKind::Vertex),
            false,
        ),
        RelationKind::Geometry(GeometryRelationKind::FaceUsesSurfaceBinding) => endpoint_pair(
            EntityKind::Topology(TopologyEntityKind::Face),
            EntityKind::Geometry(GeometryEntityKind::SurfaceBinding),
            false,
        ),
        RelationKind::Geometry(GeometryRelationKind::EdgeUsesCurveBinding) => endpoint_pair(
            EntityKind::Topology(TopologyEntityKind::Edge),
            EntityKind::Geometry(GeometryEntityKind::CurveBinding),
            false,
        ),
        RelationKind::Geometry(GeometryRelationKind::HalfEdgeUsesCoedgeBinding) => endpoint_pair(
            EntityKind::Topology(TopologyEntityKind::HalfEdge),
            EntityKind::Geometry(GeometryEntityKind::CoedgeBinding),
            false,
        ),
        RelationKind::Geometry(GeometryRelationKind::VertexUsesGeometryBinding) => endpoint_pair(
            EntityKind::Topology(TopologyEntityKind::Vertex),
            EntityKind::Geometry(GeometryEntityKind::VertexGeometryBinding),
            false,
        ),
        RelationKind::Naming(NamingRelationKind::PersistentNameTargetsEntity) => (
            vec![EntityKind::Naming(NamingEntityKind::PersistentName).kind_id()],
            naming_target_kind_ids(),
            false,
        ),
        RelationKind::Diagnostics(DiagnosticsRelationKind::WireHasInterpretation) => endpoint_pair(
            EntityKind::Topology(TopologyEntityKind::Wire),
            EntityKind::Diagnostics(DiagnosticsEntityKind::WireInterpretation),
            false,
        ),
        RelationKind::Diagnostics(DiagnosticsRelationKind::ShellHasInterpretation) => {
            endpoint_pair(
                EntityKind::Topology(TopologyEntityKind::Shell),
                EntityKind::Diagnostics(DiagnosticsEntityKind::ShellInterpretation),
                false,
            )
        }
    }
}

fn cardinality_limits(kind: RelationKind) -> (Option<u64>, Option<u64>, Option<u64>) {
    match kind {
        RelationKind::Topology(TopologyRelationKind::ModelOwnsBody)
        | RelationKind::Topology(TopologyRelationKind::BodyOwnsLump)
        | RelationKind::Topology(TopologyRelationKind::LumpOwnsRegion)
        | RelationKind::Topology(TopologyRelationKind::RegionOwnsShell)
        | RelationKind::Topology(TopologyRelationKind::ShellOwnsFace)
        | RelationKind::Topology(TopologyRelationKind::LoopOwnsHalfEdge)
        | RelationKind::Topology(TopologyRelationKind::WireOwnsHalfEdge)
        | RelationKind::Topology(TopologyRelationKind::FaceInnerLoop) => (None, Some(1), Some(1)),
        RelationKind::Topology(TopologyRelationKind::FaceOuterLoop)
        | RelationKind::Topology(TopologyRelationKind::HalfEdgeNext)
        | RelationKind::Topology(TopologyRelationKind::HalfEdgePrev)
        | RelationKind::Topology(TopologyRelationKind::HalfEdgeRadialNext)
        | RelationKind::Geometry(GeometryRelationKind::FaceUsesSurfaceBinding)
        | RelationKind::Geometry(GeometryRelationKind::EdgeUsesCurveBinding)
        | RelationKind::Geometry(GeometryRelationKind::HalfEdgeUsesCoedgeBinding)
        | RelationKind::Geometry(GeometryRelationKind::VertexUsesGeometryBinding)
        | RelationKind::Naming(NamingRelationKind::PersistentNameTargetsEntity)
        | RelationKind::Diagnostics(DiagnosticsRelationKind::WireHasInterpretation)
        | RelationKind::Diagnostics(DiagnosticsRelationKind::ShellHasInterpretation) => {
            (Some(1), Some(1), Some(1))
        }
        RelationKind::Topology(TopologyRelationKind::HalfEdgeUsesEdge)
        | RelationKind::Topology(TopologyRelationKind::HalfEdgeStartsAtVertex)
        | RelationKind::Topology(TopologyRelationKind::HalfEdgeEndsAtVertex) => {
            (Some(1), None, Some(1))
        }
    }
}

fn naming_target_kind_ids() -> Vec<KindId> {
    let mut kind_ids =
        Vec::with_capacity(TopologyEntityKind::ALL.len() + GeometryEntityKind::ALL.len());
    for kind in TopologyEntityKind::ALL {
        kind_ids.push(kind.kind_id());
    }
    for kind in GeometryEntityKind::ALL {
        kind_ids.push(kind.kind_id());
    }
    kind_ids
}

fn endpoint_pair(
    source_kind: EntityKind,
    target_kind: EntityKind,
    self_edges_allowed: bool,
) -> (Vec<KindId>, Vec<KindId>, bool) {
    (
        vec![source_kind.kind_id()],
        vec![target_kind.kind_id()],
        self_edges_allowed,
    )
}
