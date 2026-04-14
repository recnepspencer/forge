use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthSurfaceBindingKind {
    Plane,
    Cylinder,
    Cone,
    Sphere,
    Torus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthCurveBindingKind {
    Line,
    Circle,
    Ellipse,
    SurfaceIntersection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthCoedgeCurveKind {
    Line,
    Circle,
    Nurbs,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthCurveProvenanceKind {
    AnalyticIntersection,
    SsiSolver,
    SplitInherited,
    Imported,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthVertexGeometryProvenanceKind {
    ThreePlaneIntersection,
    EdgeSplit,
    Imported,
    Coalesced,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthVertexToleranceRegime {
    Exact,
    Modeled,
    Healed,
    Imported,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthSurfaceRelationKind {
    Coincident,
    Disjoint,
    General,
    Undetermined,
}
