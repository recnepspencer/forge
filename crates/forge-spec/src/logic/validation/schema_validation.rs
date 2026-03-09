use std::collections::{BTreeMap, BTreeSet};

use crate::data::error::SpecError;
use crate::data::graph::SpecGraph;
use crate::data::schema::{RelationCardinality, RelationKind, SpecNodeKind};

pub fn validate_spec_graph(graph: &SpecGraph) -> Result<(), SpecError> {
    let mut single_counts: BTreeMap<(crate::data::identity::SpecNodeId, RelationKind), usize> =
        BTreeMap::new();

    for relation in graph.iter_relations() {
        let Some(source_kind) = graph.node_kind(relation.source) else {
            return Err(SpecError::validation(format!(
                "relation {} has missing source {}",
                relation.id, relation.source
            )));
        };
        let Some(target_kind) = graph.node_kind(relation.target) else {
            return Err(SpecError::validation(format!(
                "relation {} has missing target {}",
                relation.id, relation.target
            )));
        };

        if !source_target_allowed(relation.kind, source_kind, target_kind) {
            return Err(SpecError::validation(format!(
                "relation {:?} is not valid from {:?} to {:?}",
                relation.kind, source_kind, target_kind
            )));
        }

        if relation.kind.cardinality() == RelationCardinality::Single {
            *single_counts
                .entry((relation.source, relation.kind))
                .or_default() += 1;
            if relation.ordinal != 0 {
                return Err(SpecError::validation(format!(
                    "single-cardinality relation {:?} from {} must use ordinal 0",
                    relation.kind, relation.source
                )));
            }
        }
    }

    for ((source, kind), count) in single_counts {
        if count > 1 {
            return Err(SpecError::validation(format!(
                "node {} has {} outgoing {:?} relations but cardinality is single",
                source, count, kind
            )));
        }
    }

    for node in graph.iter_nodes() {
        for required in required_outgoing(node.kind) {
            let count = graph.outgoing_of_kind(node.id, *required).len();
            if count != 1 {
                return Err(SpecError::validation(format!(
                    "node {} of kind {:?} requires exactly one outgoing {:?} relation, found {}",
                    node.id, node.kind, required, count
                )));
            }
        }
    }

    validate_ordered_ordinals(graph, RelationKind::FaceInnerLoop)?;
    Ok(())
}

fn validate_ordered_ordinals(graph: &SpecGraph, kind: RelationKind) -> Result<(), SpecError> {
    for node in graph.iter_nodes() {
        let relations = graph.outgoing_of_kind(node.id, kind);
        if relations.is_empty() {
            continue;
        }
        let mut ordinals = BTreeSet::new();
        for relation in relations {
            if !ordinals.insert(relation.ordinal) {
                return Err(SpecError::validation(format!(
                    "node {} has duplicate ordinal {} for {:?}",
                    node.id, relation.ordinal, kind
                )));
            }
        }
    }
    Ok(())
}

fn required_outgoing(kind: SpecNodeKind) -> &'static [RelationKind] {
    match kind {
        SpecNodeKind::Face => &[RelationKind::FaceOuterLoop],
        SpecNodeKind::Loop => &[RelationKind::LoopEntryHalfEdge],
        SpecNodeKind::HalfEdge => &[
            RelationKind::HalfEdgeNext,
            RelationKind::HalfEdgeRadialNext,
            RelationKind::HalfEdgeUsesEdge,
            RelationKind::HalfEdgeOriginVertex,
            RelationKind::HalfEdgeBoundsFace,
        ],
        _ => &[],
    }
}

fn source_target_allowed(
    relation: RelationKind,
    source: SpecNodeKind,
    target: SpecNodeKind,
) -> bool {
    use RelationKind::*;
    use SpecNodeKind::*;
    match relation {
        ModelOwnsFeature => matches!((source, target), (Model, Feature)),
        ModelOwnsConstraint => matches!((source, target), (Model, Constraint)),
        ModelOwnsParameter => matches!((source, target), (Model, Parameter)),
        FeatureConsumesParameter => matches!((source, target), (Feature, Parameter)),
        FeatureConsumesConstraint => matches!((source, target), (Feature, Constraint)),
        FeatureProducesTopology => {
            matches!(source, Feature)
                && target.domain() == crate::data::schema::GraphDomain::Topology
        }
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
