use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DerivedTopologyUpdatePosture {
    BoundedRebuildRequired,
    IncrementalEligible,
}

impl DerivedTopologyUpdatePosture {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BoundedRebuildRequired => "bounded_rebuild_required",
            Self::IncrementalEligible => "incremental_eligible",
        }
    }
}
