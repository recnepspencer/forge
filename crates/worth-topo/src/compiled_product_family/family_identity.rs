use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum TopologyCompiledProductFamilyIdentity {
    DerivedTopologyEquivalenceContract,
}

impl TopologyCompiledProductFamilyIdentity {
    pub const REQUIRED: [Self; 1] = [Self::DerivedTopologyEquivalenceContract];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DerivedTopologyEquivalenceContract => "derived-topology-equivalence-contract",
        }
    }
}
