use forge_relational::facade::config::CrossContextPolicy;
use forge_relational::facade::identity::KindId;
use forge_relational::facade::schema::{
    CardinalityContractDeclaration, EndpointKindContractDeclaration, MinimumCardinalityEnforcement,
    PairMinimumSemantics, RelationIntegrityDeclarations, UniquenessContractDeclaration,
    UniquenessScope,
};

use crate::data::entities::{
    WorthDiagnosticsEntityKind, WorthEntityKind, WorthGeometryEntityKind, WorthNamingEntityKind,
    WorthTopologyEntityKind,
};
use crate::data::relations::{
    WorthDiagnosticsRelationKind, WorthGeometryRelationKind, WorthNamingRelationKind,
    WorthRelationKind, WorthTopologyRelationKind,
};

pub fn relation_integrity(kind: WorthRelationKind) -> RelationIntegrityDeclarations {
    let (allowed_source_kinds, allowed_target_kinds, self_edges_allowed) = endpoint_domain(kind);
    let (source_max, target_max, pair_max) = cardinality_limits(kind);
    let contract_stem = kind.kind_name().replace("worth.", "");

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

fn endpoint_domain(kind: WorthRelationKind) -> (Vec<KindId>, Vec<KindId>, bool) {
    match kind {
        WorthRelationKind::Topology(WorthTopologyRelationKind::ModelOwnsBody) => endpoint_pair(
            WorthEntityKind::Topology(WorthTopologyEntityKind::Model),
            WorthEntityKind::Topology(WorthTopologyEntityKind::Body),
            false,
        ),
        WorthRelationKind::Topology(WorthTopologyRelationKind::BodyOwnsLump) => endpoint_pair(
            WorthEntityKind::Topology(WorthTopologyEntityKind::Body),
            WorthEntityKind::Topology(WorthTopologyEntityKind::Lump),
            false,
        ),
        WorthRelationKind::Topology(WorthTopologyRelationKind::LumpOwnsRegion) => endpoint_pair(
            WorthEntityKind::Topology(WorthTopologyEntityKind::Lump),
            WorthEntityKind::Topology(WorthTopologyEntityKind::Region),
            false,
        ),
        WorthRelationKind::Topology(WorthTopologyRelationKind::RegionOwnsShell) => endpoint_pair(
            WorthEntityKind::Topology(WorthTopologyEntityKind::Region),
            WorthEntityKind::Topology(WorthTopologyEntityKind::Shell),
            false,
        ),
        WorthRelationKind::Topology(WorthTopologyRelationKind::ShellOwnsFace) => endpoint_pair(
            WorthEntityKind::Topology(WorthTopologyEntityKind::Shell),
            WorthEntityKind::Topology(WorthTopologyEntityKind::Face),
            false,
        ),
        WorthRelationKind::Topology(WorthTopologyRelationKind::FaceOuterLoop)
        | WorthRelationKind::Topology(WorthTopologyRelationKind::FaceInnerLoop) => endpoint_pair(
            WorthEntityKind::Topology(WorthTopologyEntityKind::Face),
            WorthEntityKind::Topology(WorthTopologyEntityKind::Loop),
            false,
        ),
        WorthRelationKind::Topology(WorthTopologyRelationKind::LoopOwnsHalfEdge) => endpoint_pair(
            WorthEntityKind::Topology(WorthTopologyEntityKind::Loop),
            WorthEntityKind::Topology(WorthTopologyEntityKind::HalfEdge),
            false,
        ),
        WorthRelationKind::Topology(WorthTopologyRelationKind::WireOwnsHalfEdge) => endpoint_pair(
            WorthEntityKind::Topology(WorthTopologyEntityKind::Wire),
            WorthEntityKind::Topology(WorthTopologyEntityKind::HalfEdge),
            false,
        ),
        WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgeNext)
        | WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgePrev)
        | WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgeRadialNext) => (
            vec![WorthEntityKind::Topology(WorthTopologyEntityKind::HalfEdge).kind_id()],
            vec![WorthEntityKind::Topology(WorthTopologyEntityKind::HalfEdge).kind_id()],
            true,
        ),
        WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgeUsesEdge) => endpoint_pair(
            WorthEntityKind::Topology(WorthTopologyEntityKind::HalfEdge),
            WorthEntityKind::Topology(WorthTopologyEntityKind::Edge),
            false,
        ),
        WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgeStartsAtVertex) => {
            endpoint_pair(
                WorthEntityKind::Topology(WorthTopologyEntityKind::HalfEdge),
                WorthEntityKind::Topology(WorthTopologyEntityKind::Vertex),
                false,
            )
        }
        WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgeEndsAtVertex) => {
            endpoint_pair(
                WorthEntityKind::Topology(WorthTopologyEntityKind::HalfEdge),
                WorthEntityKind::Topology(WorthTopologyEntityKind::Vertex),
                false,
            )
        }
        WorthRelationKind::Geometry(WorthGeometryRelationKind::FaceUsesSurfaceBinding) => {
            endpoint_pair(
                WorthEntityKind::Topology(WorthTopologyEntityKind::Face),
                WorthEntityKind::Geometry(WorthGeometryEntityKind::SurfaceBinding),
                false,
            )
        }
        WorthRelationKind::Geometry(WorthGeometryRelationKind::EdgeUsesCurveBinding) => {
            endpoint_pair(
                WorthEntityKind::Topology(WorthTopologyEntityKind::Edge),
                WorthEntityKind::Geometry(WorthGeometryEntityKind::CurveBinding),
                false,
            )
        }
        WorthRelationKind::Geometry(WorthGeometryRelationKind::HalfEdgeUsesCoedgeBinding) => {
            endpoint_pair(
                WorthEntityKind::Topology(WorthTopologyEntityKind::HalfEdge),
                WorthEntityKind::Geometry(WorthGeometryEntityKind::CoedgeBinding),
                false,
            )
        }
        WorthRelationKind::Geometry(WorthGeometryRelationKind::VertexUsesGeometryBinding) => {
            endpoint_pair(
                WorthEntityKind::Topology(WorthTopologyEntityKind::Vertex),
                WorthEntityKind::Geometry(WorthGeometryEntityKind::VertexGeometryBinding),
                false,
            )
        }
        WorthRelationKind::Naming(WorthNamingRelationKind::PersistentNameTargetsEntity) => (
            vec![WorthEntityKind::Naming(WorthNamingEntityKind::PersistentName).kind_id()],
            naming_target_kind_ids(),
            false,
        ),
        WorthRelationKind::Diagnostics(WorthDiagnosticsRelationKind::WireHasInterpretation) => {
            endpoint_pair(
                WorthEntityKind::Topology(WorthTopologyEntityKind::Wire),
                WorthEntityKind::Diagnostics(WorthDiagnosticsEntityKind::WireInterpretation),
                false,
            )
        }
        WorthRelationKind::Diagnostics(WorthDiagnosticsRelationKind::ShellHasInterpretation) => {
            endpoint_pair(
                WorthEntityKind::Topology(WorthTopologyEntityKind::Shell),
                WorthEntityKind::Diagnostics(WorthDiagnosticsEntityKind::ShellInterpretation),
                false,
            )
        }
    }
}

fn cardinality_limits(kind: WorthRelationKind) -> (Option<u64>, Option<u64>, Option<u64>) {
    match kind {
        WorthRelationKind::Topology(WorthTopologyRelationKind::ModelOwnsBody)
        | WorthRelationKind::Topology(WorthTopologyRelationKind::BodyOwnsLump)
        | WorthRelationKind::Topology(WorthTopologyRelationKind::LumpOwnsRegion)
        | WorthRelationKind::Topology(WorthTopologyRelationKind::RegionOwnsShell)
        | WorthRelationKind::Topology(WorthTopologyRelationKind::ShellOwnsFace)
        | WorthRelationKind::Topology(WorthTopologyRelationKind::FaceInnerLoop) => {
            (None, Some(1), Some(1))
        }
        WorthRelationKind::Topology(WorthTopologyRelationKind::FaceOuterLoop)
        | WorthRelationKind::Topology(WorthTopologyRelationKind::LoopOwnsHalfEdge)
        | WorthRelationKind::Topology(WorthTopologyRelationKind::WireOwnsHalfEdge)
        | WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgeNext)
        | WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgePrev)
        | WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgeRadialNext)
        | WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgeUsesEdge)
        | WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgeStartsAtVertex)
        | WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgeEndsAtVertex)
        | WorthRelationKind::Geometry(WorthGeometryRelationKind::FaceUsesSurfaceBinding)
        | WorthRelationKind::Geometry(WorthGeometryRelationKind::EdgeUsesCurveBinding)
        | WorthRelationKind::Geometry(WorthGeometryRelationKind::HalfEdgeUsesCoedgeBinding)
        | WorthRelationKind::Geometry(WorthGeometryRelationKind::VertexUsesGeometryBinding)
        | WorthRelationKind::Naming(WorthNamingRelationKind::PersistentNameTargetsEntity)
        | WorthRelationKind::Diagnostics(WorthDiagnosticsRelationKind::WireHasInterpretation)
        | WorthRelationKind::Diagnostics(WorthDiagnosticsRelationKind::ShellHasInterpretation) => {
            (Some(1), Some(1), Some(1))
        }
    }
}

fn naming_target_kind_ids() -> Vec<KindId> {
    let mut kind_ids =
        Vec::with_capacity(WorthTopologyEntityKind::ALL.len() + WorthGeometryEntityKind::ALL.len());
    for kind in WorthTopologyEntityKind::ALL {
        kind_ids.push(kind.kind_id());
    }
    for kind in WorthGeometryEntityKind::ALL {
        kind_ids.push(kind.kind_id());
    }
    kind_ids
}

fn endpoint_pair(
    source_kind: WorthEntityKind,
    target_kind: WorthEntityKind,
    self_edges_allowed: bool,
) -> (Vec<KindId>, Vec<KindId>, bool) {
    (
        vec![source_kind.kind_id()],
        vec![target_kind.kind_id()],
        self_edges_allowed,
    )
}
