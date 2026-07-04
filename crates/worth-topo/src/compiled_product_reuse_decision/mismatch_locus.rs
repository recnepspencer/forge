use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum TopologyDerivedReuseMismatchLocus {
    MissingSelectedFamilyContract,
    ComparatorContract,
    TopologyCompiledProductFamilyIdentity,
    TopologyCompiledProductFamilyDigest,
    AuthorityTruthIdentity,
    CompiledProductIdentity,
    EquivalencePolicyIdentity,
    SelectedEquivalenceFamilyIdentity,
    SelectedEquivalenceBasisIdentity,
    SelectedReuseBasisIdentity,
    BranchIdentity,
    InvalidationTargets,
    MaterializedTopologyDigest,
    InterpretedTopologyDigest,
    DerivedValidationDigest,
}
