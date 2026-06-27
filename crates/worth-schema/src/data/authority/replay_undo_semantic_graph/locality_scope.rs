use serde::Serialize;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize)]
pub enum ReplayUndoSemanticGraphLocalityScope {
    TopologyTouchedClosure,
    SpatialTouchAuthority,
}

impl ReplayUndoSemanticGraphLocalityScope {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TopologyTouchedClosure => "topology-touched-closure",
            Self::SpatialTouchAuthority => "spatial-touch-authority",
        }
    }
}
