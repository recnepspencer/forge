use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum TopologyDerivedReuseDecisionPosture {
    ReuseAdmitted,
    FreshRebuildRequired,
    AdvisoryMatchRequiresRebuild,
    Denied,
}
