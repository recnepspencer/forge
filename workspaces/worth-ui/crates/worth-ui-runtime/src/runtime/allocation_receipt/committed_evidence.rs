#[derive(Clone, Debug, PartialEq)]
pub struct UiCommittedAllocationEvidenceSet {
    viewport: Option<crate::evidence::UiViewportResizeEvidence>,
    scroll_owned: Box<[crate::evidence::UiScrollOwnedAllocationEvidence]>,
    portal_anchors: Box<[UiCommittedPortalAnchorEvidence]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiCommittedPortalAnchorEvidence {
    identity: crate::runtime::UiPortalAnchorIdentity,
    receipt_identity: super::UiAllocationReceiptIdentity,
    neighborhood_identity_digest: u64,
    receipt_generation: super::UiAllocationReceiptGeneration,
}

impl UiCommittedAllocationEvidenceSet {
    pub(super) fn ordinary(
        scroll_owned: Box<[crate::evidence::UiScrollOwnedAllocationEvidence]>,
        receipts: &[super::UiAllocationReceipt],
    ) -> Self {
        let portal_anchors = receipts
            .iter()
            .filter_map(UiCommittedPortalAnchorEvidence::from_receipt)
            .collect::<Vec<_>>()
            .into_boxed_slice();
        Self {
            viewport: None,
            scroll_owned,
            portal_anchors,
        }
    }

    pub(super) fn with_viewport(
        mut self,
        evidence: crate::evidence::UiViewportResizeEvidence,
    ) -> Self {
        self.viewport = Some(evidence);
        self
    }

    pub fn viewport(&self) -> Option<&crate::evidence::UiViewportResizeEvidence> {
        self.viewport.as_ref()
    }

    pub fn scroll_owned(&self) -> &[crate::evidence::UiScrollOwnedAllocationEvidence] {
        &self.scroll_owned
    }

    pub fn portal_anchors(&self) -> &[UiCommittedPortalAnchorEvidence] {
        &self.portal_anchors
    }
}

impl UiCommittedPortalAnchorEvidence {
    fn from_receipt(receipt: &super::UiAllocationReceipt) -> Option<Self> {
        let super::UiAllocationAnchorPosture::PortalAnchored(identity) =
            receipt.geometry_evidence().anchor_posture()
        else {
            return None;
        };
        Some(Self {
            identity,
            receipt_identity: receipt.identity().clone(),
            neighborhood_identity_digest: receipt
                .committed_allocation()
                .allocation_neighborhood()
                .identity()
                .identity_digest(),
            receipt_generation: receipt.generation(),
        })
    }

    pub fn identity(&self) -> crate::runtime::UiPortalAnchorIdentity {
        self.identity
    }
    pub fn receipt_identity(&self) -> &super::UiAllocationReceiptIdentity {
        &self.receipt_identity
    }
    pub fn neighborhood_identity_digest(&self) -> u64 {
        self.neighborhood_identity_digest
    }
    pub fn receipt_generation(&self) -> super::UiAllocationReceiptGeneration {
        self.receipt_generation
    }
}
