use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TopologyOrderingNoisePosture {
    ExactOrderingRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TopologyFreshnessRequirementPosture {
    SameAdmittedAuthorityAndLocalityRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TopologyRenderedOutputComparisonPosture {
    DerivedOutputDigestsRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TopologyCompatibilityPosture {
    DistinctFromEquivalence,
}
