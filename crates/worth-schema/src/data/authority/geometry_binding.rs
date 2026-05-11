use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum SurfaceBindingKind {
    Plane,
    Cylinder,
    Cone,
    Sphere,
    Torus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum CurveBindingKind {
    Line,
    Circle,
    Ellipse,
    SurfaceIntersection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum CoedgeCurveKind {
    Line,
    Circle,
    Nurbs,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum CurveProvenanceKind {
    AnalyticIntersection,
    SsiSolver,
    SplitInherited,
    Imported,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum VertexGeometryProvenanceKind {
    ThreePlaneIntersection,
    EdgeSplit,
    Imported,
    Coalesced,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum VertexToleranceRegime {
    Exact,
    Modeled,
    Healed,
    Imported,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum SurfaceRelationKind {
    Coincident,
    Disjoint,
    General,
    Undetermined,
}
