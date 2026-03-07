use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum RelationCardinality {
    Single,
    MultiOrdered,
    MultiUnordered,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum RelationKind {
    ModelOwnsFeature,
    ModelOwnsConstraint,
    ModelOwnsParameter,
    FeatureConsumesParameter,
    FeatureConsumesConstraint,
    FeatureProducesTopology,
    FeatureDependsOnFeature,
    DecisionAffectsNode,
    BodyOwnsLump,
    LumpOwnsRegion,
    RegionOwnsShell,
    ShellOwnsFace,
    FaceOuterLoop,
    FaceInnerLoop,
    LoopEntryHalfEdge,
    HalfEdgeNext,
    HalfEdgeRadialNext,
    HalfEdgeUsesEdge,
    HalfEdgeOriginVertex,
    HalfEdgeBoundsFace,
    FaceUsesSurfaceBinding,
    EdgeUsesCurveBinding,
    HalfEdgeUsesCoedgeBinding,
    VertexUsesGeometryBinding,
    NamingAnchorTargetsNode,
    LineageAnchorDerivedFrom,
    ReplayRecordAppliesToFeature,
    ReplayRecordTouchesNode,
}

impl RelationKind {
    pub const fn cardinality(self) -> RelationCardinality {
        match self {
            Self::FaceOuterLoop
            | Self::LoopEntryHalfEdge
            | Self::HalfEdgeNext
            | Self::HalfEdgeRadialNext
            | Self::HalfEdgeUsesEdge
            | Self::HalfEdgeOriginVertex
            | Self::HalfEdgeBoundsFace
            | Self::FaceUsesSurfaceBinding
            | Self::EdgeUsesCurveBinding
            | Self::HalfEdgeUsesCoedgeBinding
            | Self::VertexUsesGeometryBinding
            | Self::NamingAnchorTargetsNode
            | Self::ReplayRecordAppliesToFeature => RelationCardinality::Single,
            Self::FaceInnerLoop => RelationCardinality::MultiOrdered,
            _ => RelationCardinality::MultiUnordered,
        }
    }
}
