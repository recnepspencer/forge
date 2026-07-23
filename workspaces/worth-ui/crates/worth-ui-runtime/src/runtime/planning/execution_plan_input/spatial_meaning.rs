use crate::capability::{ComponentCanvasSpatialContract, ComponentDescriptor};

/// Exact semantic canvas input carried from admitted component capability.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiSpatialPlanMeaning {
    descriptor: ComponentDescriptor,
    contract: ComponentCanvasSpatialContract,
}

impl WorthUiSpatialPlanMeaning {
    pub(crate) fn new(
        descriptor: ComponentDescriptor,
        contract: ComponentCanvasSpatialContract,
    ) -> Self {
        Self {
            descriptor,
            contract,
        }
    }

    pub(crate) fn contract(&self) -> ComponentCanvasSpatialContract {
        self.contract
    }

    pub(crate) fn semantic_digest(&self) -> u64 {
        self.descriptor
            .id()
            .as_str()
            .as_bytes()
            .iter()
            .fold(self.contract.digest_basis(), |digest, byte| {
                (digest ^ u64::from(*byte)).wrapping_mul(0x100000001b3)
            })
    }
}
