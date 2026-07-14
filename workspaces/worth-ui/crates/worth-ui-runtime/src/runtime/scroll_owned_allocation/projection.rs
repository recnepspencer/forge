use super::{UiAdmittedScrollOwnedContract, UiScrollReceiptActivationKey};

#[derive(Clone, Debug, PartialEq)]
pub struct UiProjectedScrollOffset {
    target: UiActivatedScrollProjectionTarget,
    receipt_key: UiScrollReceiptActivationKey,
    inline: f32,
    block: f32,
}

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
    pub(crate) fn owner_identity(self) -> UiScrollProjectionOwnerIdentity {
        UiScrollProjectionOwnerIdentity(self.contract_identity_digest)
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiProjectedScrollOffsetDenial {
    NonFinite,
    TargetNotActivated,
    ScrollOwnershipNotAdmitted,
    ProjectionGenerationExhausted,
    AllocationIngressCounterRegressed,
    AllocationTruthRevisionRegressed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiProjectedScrollOffsetOutcome {
    target: crate::graph::UiGraphNodeIdentity,
    allocation_invalidations: u64,
    committed_receipts: u64,
    projected_offsets: u64,
    projection_generation: u64,
}

#[derive(Debug, Default)]
pub(crate) struct UiScrollOffsetProjectionLedger {
    projection_generation: u64,
    latest: Option<UiProjectedScrollOffset>,
}

impl UiProjectedScrollOffset {
    pub fn logical(
        owner: UiActivatedScrollOwner,
        inline: f32,
        block: f32,
    ) -> Result<Self, UiProjectedScrollOffsetDenial> {
        (inline.is_finite() && block.is_finite())
            .then_some(Self {
                target: owner.target(),
                receipt_key: owner.receipt_key().clone(),
                inline,
                block,
            })
            .ok_or(UiProjectedScrollOffsetDenial::NonFinite)
    }
    pub fn target(&self) -> crate::graph::UiGraphNodeIdentity {
        self.target.target
    }
    pub(crate) fn capability(&self) -> UiActivatedScrollProjectionTarget {
        self.target
    }
    pub fn inline(&self) -> f32 {
        self.inline
    }
    pub fn block(&self) -> f32 {
        self.block
    }
    pub(crate) fn receipt_key(&self) -> &UiScrollReceiptActivationKey {
        &self.receipt_key
    }
}

impl UiScrollOffsetProjectionLedger {
    pub(crate) fn record(
        &mut self,
        offset: UiProjectedScrollOffset,
    ) -> Result<u64, UiProjectedScrollOffsetDenial> {
        self.projection_generation = self
            .projection_generation
            .checked_add(1)
            .ok_or(UiProjectedScrollOffsetDenial::ProjectionGenerationExhausted)?;
        self.latest = Some(offset);
        Ok(self.projection_generation)
    }
    #[cfg(test)]
    pub(crate) fn generation(&self) -> u64 {
        self.projection_generation
    }

    #[cfg(test)]
    pub(crate) fn exhaust_generation_for_test(&mut self) {
        self.projection_generation = u64::MAX;
    }
}

impl UiProjectedScrollOffsetOutcome {
    pub(crate) fn seal(
        offset: UiProjectedScrollOffset,
        projection_generation: u64,
        allocation_invalidations: u64,
        committed_receipts: u64,
    ) -> Self {
        Self {
            target: offset.target.target,
            allocation_invalidations,
            committed_receipts,
            projected_offsets: 1,
            projection_generation,
        }
    }
    pub fn target(self) -> crate::graph::UiGraphNodeIdentity {
        self.target
    }
    pub fn allocation_invalidations(self) -> u64 {
        self.allocation_invalidations
    }
    pub fn committed_receipts(self) -> u64 {
        self.committed_receipts
    }
    pub fn projected_offsets(self) -> u64 {
        self.projected_offsets
    }
    pub fn projection_generation(self) -> u64 {
        self.projection_generation
    }
}
