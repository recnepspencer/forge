use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum GraphDomain {
    Intent,
    Topology,
    GeometryBinding,
    Provenance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum SpecNodeKind {
    Model,
    Feature,
    Constraint,
    Parameter,
    DesignDecision,
    Body,
    Lump,
    Region,
    Shell,
    Face,
    Loop,
    HalfEdge,
    Edge,
    Vertex,
    SurfaceBinding,
    CurveBinding,
    CoedgeBinding,
    VertexGeometryBinding,
    NamingAnchor,
    ReplayRecord,
    LineageAnchor,
}

impl SpecNodeKind {
    pub const fn domain(self) -> GraphDomain {
        match self {
            Self::Model
            | Self::Feature
            | Self::Constraint
            | Self::Parameter
            | Self::DesignDecision => GraphDomain::Intent,
            Self::Body
            | Self::Lump
            | Self::Region
            | Self::Shell
            | Self::Face
            | Self::Loop
            | Self::HalfEdge
            | Self::Edge
            | Self::Vertex => GraphDomain::Topology,
            Self::SurfaceBinding
            | Self::CurveBinding
            | Self::CoedgeBinding
            | Self::VertexGeometryBinding => GraphDomain::GeometryBinding,
            Self::NamingAnchor | Self::ReplayRecord | Self::LineageAnchor => GraphDomain::Provenance,
        }
    }
}
