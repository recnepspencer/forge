use crate::data::error::SpecError;
use crate::data::identity::SpecNodeId;
use crate::data::schema::{GraphDomain, RelationCardinality, RelationKind, SpecNodeKind};

use super::SpecDraft;

impl SpecDraft {
    pub(super) fn validate_relation_preconditions(
        &self,
        kind: RelationKind,
        source: SpecNodeId,
        target: SpecNodeId,
        source_kind: SpecNodeKind,
        target_kind: SpecNodeKind,
        ordinal: u32,
    ) -> Result<(), SpecError> {
        if !relation_allowed(kind, source_kind, target_kind) {
            return Err(SpecError::invalid(format!(
                "relation {:?} is not valid from {:?} to {:?}",
                kind, source_kind, target_kind
            )));
        }
        if kind.cardinality() == RelationCardinality::Single {
            let existing = self
                .base
                .graph()
                .outgoing_of_kind(source, kind)
                .into_iter()
                .filter(|relation| !self.deleted_relations.contains(&relation.id))
                .count()
                + self
                    .created_relations
                    .values()
                    .filter(|relation| relation.source == source && relation.kind == kind)
                    .count();
            if existing > 0 {
                return Err(SpecError::invalid(format!(
                    "node {} already has an outgoing {:?} relation",
                    source, kind
                )));
            }
            if ordinal != 0 {
                return Err(SpecError::invalid(format!(
                    "single-cardinality relation {:?} must use ordinal 0",
                    kind
                )));
            }
        }
        if kind == RelationKind::FaceInnerLoop {
            let used = self.base.graph().relation_ordinals(source, kind);
            if used.contains(&ordinal)
                || self
                    .created_relations
                    .values()
                    .any(|relation| {
                        relation.source == source
                            && relation.kind == kind
                            && relation.ordinal == ordinal
                    })
            {
                return Err(SpecError::invalid(format!(
                    "face {} already has inner-loop ordinal {}",
                    source, ordinal
                )));
            }
        }
        let _ = target;
        Ok(())
    }
}

fn relation_allowed(kind: RelationKind, source: SpecNodeKind, target: SpecNodeKind) -> bool {
    use RelationKind::*;
    use SpecNodeKind::*;

    match kind {
        ModelOwnsFeature => matches!((source, target), (Model, Feature)),
        ModelOwnsConstraint => matches!((source, target), (Model, Constraint)),
        ModelOwnsParameter => matches!((source, target), (Model, Parameter)),
        FeatureConsumesParameter => matches!((source, target), (Feature, Parameter)),
        FeatureConsumesConstraint => matches!((source, target), (Feature, Constraint)),
        FeatureProducesTopology => matches!(source, Feature) && target.domain() == GraphDomain::Topology,
        FeatureDependsOnFeature => matches!((source, target), (Feature, Feature)),
        DecisionAffectsNode => matches!((source, target), (DesignDecision, _)),
        BodyOwnsLump => matches!((source, target), (Body, Lump)),
        LumpOwnsRegion => matches!((source, target), (Lump, Region)),
        RegionOwnsShell => matches!((source, target), (Region, Shell)),
        ShellOwnsFace => matches!((source, target), (Shell, Face)),
        FaceOuterLoop | FaceInnerLoop => matches!((source, target), (Face, Loop)),
        LoopEntryHalfEdge => matches!((source, target), (Loop, HalfEdge)),
        HalfEdgeNext | HalfEdgeRadialNext => matches!((source, target), (HalfEdge, HalfEdge)),
        HalfEdgeUsesEdge => matches!((source, target), (HalfEdge, Edge)),
        HalfEdgeOriginVertex => matches!((source, target), (HalfEdge, Vertex)),
        HalfEdgeBoundsFace => matches!((source, target), (HalfEdge, Face)),
        FaceUsesSurfaceBinding => matches!((source, target), (Face, SurfaceBinding)),
        EdgeUsesCurveBinding => matches!((source, target), (Edge, CurveBinding)),
        HalfEdgeUsesCoedgeBinding => matches!((source, target), (HalfEdge, CoedgeBinding)),
        VertexUsesGeometryBinding => matches!((source, target), (Vertex, VertexGeometryBinding)),
        NamingAnchorTargetsNode => matches!(source, NamingAnchor),
        LineageAnchorDerivedFrom => matches!((source, target), (LineageAnchor, LineageAnchor)),
        ReplayRecordAppliesToFeature => matches!((source, target), (ReplayRecord, Feature)),
        ReplayRecordTouchesNode => matches!((source, target), (ReplayRecord, _)),
    }
}
