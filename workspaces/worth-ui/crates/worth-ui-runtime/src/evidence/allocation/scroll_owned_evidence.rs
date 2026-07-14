#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiScrollOwnedExtentCause {
    HostContainerViewport,
    QueryContentExtent,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiScrollOwnedAllocationEvidence {
    contract_identity_digest: u64,
    cause: UiScrollOwnedExtentCause,
    authority_probes: u16,
    virtualization: crate::runtime::UiScrollVirtualizationPosture,
    offset_allocation: crate::runtime::UiScrollOffsetAllocationPosture,
    maximum_invalidations: u16,
    actual_invalidations: u16,
    maximum_committed_receipts: u16,
    committed_receipts: u16,
}

impl UiScrollOwnedAllocationEvidence {
    pub(crate) fn from_contract(
        contract: &crate::runtime::UiAdmittedScrollOwnedContract,
        cause: UiScrollOwnedExtentCause,
        authority_probes: u16,
    ) -> Self {
        Self {
            contract_identity_digest: contract.identity_digest(),
            cause,
            authority_probes,
            virtualization: contract.virtualization(),
            offset_allocation: contract.offset_allocation(),
            maximum_invalidations: 0,
            actual_invalidations: 0,
            maximum_committed_receipts: 0,
            committed_receipts: 0,
        }
    }
    pub(crate) fn with_commit_count(
        mut self,
        policy: crate::runtime::UiResolvedAllocationStreamPolicy,
        affected_receipts: u16,
    ) -> Self {
        let budget = policy.budget();
        self.maximum_invalidations = budget.max_invalidation_targets();
        self.actual_invalidations = 1;
        self.maximum_committed_receipts = budget.max_committed_receipts();
        self.committed_receipts = affected_receipts;
        self
    }
    pub fn contract_identity_digest(&self) -> u64 {
        self.contract_identity_digest
    }
    pub fn cause(&self) -> UiScrollOwnedExtentCause {
        self.cause
    }
    pub fn authority_probes(&self) -> u16 {
        self.authority_probes
    }
    pub fn virtualization(&self) -> crate::runtime::UiScrollVirtualizationPosture {
        self.virtualization
    }
    pub fn offset_allocation(&self) -> crate::runtime::UiScrollOffsetAllocationPosture {
        self.offset_allocation
    }
    pub fn maximum_invalidations(&self) -> u16 {
        self.maximum_invalidations
    }
    pub fn actual_invalidations(&self) -> u16 {
        self.actual_invalidations
    }
    pub fn maximum_committed_receipts(&self) -> u16 {
        self.maximum_committed_receipts
    }
    pub fn committed_receipts(&self) -> u16 {
        self.committed_receipts
    }
}
