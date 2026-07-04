use serde::{Deserialize, Serialize};

use crate::compiled_product_family::TopologyCompiledProductFamilyIdentity;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum TopologySelectedEquivalenceFamilyIdentity {
    DerivedTopologySemanticParity,
}

impl TopologySelectedEquivalenceFamilyIdentity {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DerivedTopologySemanticParity => {
                "topology.selected-equivalence.derived-semantic-parity"
            }
        }
    }

    pub const fn compiled_product_family_identity(self) -> TopologyCompiledProductFamilyIdentity {
        match self {
            Self::DerivedTopologySemanticParity => {
                TopologyCompiledProductFamilyIdentity::DerivedTopologyEquivalenceContract
            }
        }
    }
}
