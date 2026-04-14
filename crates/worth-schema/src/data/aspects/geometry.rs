use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthGeometryAspect {
    Binding,
    Embedding,
    Provenance,
    Approximation,
    UvAnchoring,
    Carrier,
    Precision,
    Fallback,
}

impl WorthGeometryAspect {
    pub const ALL: [Self; 8] = [
        Self::Binding,
        Self::Embedding,
        Self::Provenance,
        Self::Approximation,
        Self::UvAnchoring,
        Self::Carrier,
        Self::Precision,
        Self::Fallback,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Binding => "geometry.binding",
            Self::Embedding => "geometry.embedding",
            Self::Provenance => "geometry.provenance",
            Self::Approximation => "geometry.approximation",
            Self::UvAnchoring => "geometry.uv_anchoring",
            Self::Carrier => "geometry.carrier",
            Self::Precision => "geometry.precision",
            Self::Fallback => "geometry.fallback",
        }
    }
}
