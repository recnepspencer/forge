use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TopologyAuthorityBasisPosture {
    DerivedTopologyTruthBasis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TopologyLocalityFootprintPosture {
    InvalidationClosure,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TopologyPriorProofPosture {
    NotRequired,
    DerivedInvalidationSelectedPlan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TopologyStageIdentityPosture {
    NotRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TopologyValidatorEvidenceRolePosture {
    DerivedValidationDigestEquivalenceDimension,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TopologyEquivalencePolicyPosture {
    DerivedTopologySemanticParity,
}
