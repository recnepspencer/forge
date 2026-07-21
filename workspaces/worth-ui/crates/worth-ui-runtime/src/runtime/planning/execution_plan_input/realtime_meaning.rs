use crate::capability::{ComponentDescriptor, ComponentRealtimeOverlayContract};

/// Exact realtime overlay meaning carried from one frozen component capability.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiRealtimePlanMeaning {
    descriptor: ComponentDescriptor,
    contract: ComponentRealtimeOverlayContract,
}

impl WorthUiRealtimePlanMeaning {
    pub(crate) fn new(
        descriptor: ComponentDescriptor,
        contract: ComponentRealtimeOverlayContract,
    ) -> Self {
        Self {
            descriptor,
            contract,
        }
    }

    pub(crate) fn contract(&self) -> ComponentRealtimeOverlayContract {
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
