use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum TopologyCompiledProductConsumer {
    DerivedEquivalenceContractProjection,
    DerivedEquivalenceCertificationParity,
}

impl TopologyCompiledProductConsumer {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DerivedEquivalenceContractProjection => "derived-equivalence-contract-projection",
            Self::DerivedEquivalenceCertificationParity => {
                "derived-equivalence-certification-parity"
            }
        }
    }
}
