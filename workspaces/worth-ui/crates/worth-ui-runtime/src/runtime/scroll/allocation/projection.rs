use super::{UiAdmittedScrollOwnedContract, UiScrollReceiptActivationKey};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiActivatedScrollProjectionTarget {
    target: crate::graph::UiGraphNodeIdentity,
    graph_generation: crate::graph::UiGraphGeneration,
    contract_identity_digest: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiActivatedScrollOwner {
    target: UiActivatedScrollProjectionTarget,
    receipt_key: UiScrollReceiptActivationKey,
    authority_probes: u16,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct UiScrollProjectionOwnerIdentity(u64);

impl UiActivatedScrollProjectionTarget {
    pub(crate) fn new(
        target: crate::graph::UiGraphNodeIdentity,
        graph_generation: crate::graph::UiGraphGeneration,
        contract_identity_digest: u64,
    ) -> Self {
        Self {
            target,
            graph_generation,
            contract_identity_digest,
        }
    }
    pub fn target(self) -> crate::graph::UiGraphNodeIdentity {
        self.target
    }
}

impl UiActivatedScrollOwner {
    pub(crate) fn seal(
        target: UiActivatedScrollProjectionTarget,
        receipt_key: UiScrollReceiptActivationKey,
        authority_probes: u16,
    ) -> Self {
        Self {
            target,
            receipt_key,
            authority_probes,
        }
    }
    pub fn target(&self) -> UiActivatedScrollProjectionTarget {
        self.target
    }
    pub fn authority_probes(&self) -> u16 {
        self.authority_probes
    }
    pub fn receipt_key(&self) -> &UiScrollReceiptActivationKey {
        &self.receipt_key
    }
}

impl UiAdmittedScrollOwnedContract {
    pub(crate) fn projection_owner_identity(&self) -> UiScrollProjectionOwnerIdentity {
        UiScrollProjectionOwnerIdentity(self.identity_digest())
    }
}
