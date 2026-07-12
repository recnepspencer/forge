#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiAllocationReplanTransactionEvidence {
    primary_neighborhood_identity_digest: u64,
    ordered_neighborhood_identity_digests: Box<[u64]>,
    widen_reasons: Box<[Option<crate::graph::UiReplanWidenReason>]>,
    transaction_generation: u64,
    committed_receipt_count: u16,
    runtime_generation: u64,
    overlap_disposition: crate::graph::UiReplanOverlapDisposition,
    portal_anchor_movements: Box<[crate::evidence::UiPortalAnchorMovementEvidence]>,
}

impl UiAllocationReplanTransactionEvidence {
    pub fn from_committed(value: &crate::runtime::UiCommittedAllocationReplan) -> Self {
        let transaction = value.transaction();
        Self {
            primary_neighborhood_identity_digest: transaction
                .primary_neighborhood()
                .identity_digest(),
            ordered_neighborhood_identity_digests: transaction
                .ordered_neighborhoods()
                .iter()
                .map(crate::evidence::UiAllocationNeighborhoodIdentity::identity_digest)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            widen_reasons: transaction.widen_reasons().into(),
            transaction_generation: transaction.transaction_generation(),
            committed_receipt_count: value.counters().committed_receipts(),
            runtime_generation: transaction.runtime_generation(),
            overlap_disposition: transaction.overlap_disposition(),
            portal_anchor_movements: transaction
                .consequences()
                .portal_anchors()
                .iter()
                .filter_map(|consequence| {
                    let receipt = value.receipts().iter().find(|receipt| {
                        receipt
                            .committed_allocation()
                            .allocation_neighborhood()
                            .identity()
                            == consequence
                                .movement()
                                .target()
                                .primary()
                                .neighborhood_identity()
                    })?;
                    let portal = receipt
                        .committed_allocation()
                        .planning_basis()
                        .portal_allocation_input()?;
                    crate::evidence::UiPortalAnchorMovementEvidence::from_committed(
                        consequence.evidence(),
                        portal,
                        receipt,
                    )
                })
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        }
    }

    pub fn primary_neighborhood_identity_digest(&self) -> u64 {
        self.primary_neighborhood_identity_digest
    }
    pub fn ordered_neighborhood_identity_digests(&self) -> &[u64] {
        &self.ordered_neighborhood_identity_digests
    }
    pub fn widen_reasons(&self) -> &[Option<crate::graph::UiReplanWidenReason>] {
        &self.widen_reasons
    }
    pub fn transaction_generation(&self) -> u64 {
        self.transaction_generation
    }
    pub fn committed_receipt_count(&self) -> u16 {
        self.committed_receipt_count
    }
    pub fn runtime_generation(&self) -> u64 {
        self.runtime_generation
    }
    pub fn overlap_disposition(&self) -> crate::graph::UiReplanOverlapDisposition {
        self.overlap_disposition
    }
    pub fn portal_anchor_movements(&self) -> &[crate::evidence::UiPortalAnchorMovementEvidence] {
        &self.portal_anchor_movements
    }
}
