#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiAllocationReplanTransactionCounters {
    selected_neighborhoods: u16,
    reused_neighborhoods: u16,
    replanned_neighborhoods: u16,
    committed_receipts: u16,
    replay_hits: u16,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UiCommittedAllocationReplan {
    transaction: super::UiAllocationReplanTransaction,
    receipts: Box<[super::UiAllocationReceipt]>,
    counters: UiAllocationReplanTransactionCounters,
    evidence: super::UiCommittedAllocationEvidenceSet,
    catalog_bindings: super::UiCommittedAllocationCatalogBindings,
    portal_binding_succession:
        Option<crate::runtime::invalidation_narrowing::UiPortalBindingSuccessionReceipt>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum UiAllocationReplanTransactionOutcome {
    Committed(UiCommittedAllocationReplan),
    Replayed(UiCommittedAllocationReplan),
    Denied(UiAllocationReplanTransactionCommitDenial),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiAllocationReplanTransactionCommitDenial {
    MissingSelection,
    CandidateCardinalityMismatch,
    CandidateNeighborhoodMismatch {
        ordinal: u16,
    },
    CandidatePlanningDenied {
        ordinal: u16,
    },
    ReuseDenied {
        ordinal: u16,
    },
    RecomputePending {
        ordinal: u16,
    },
    TransactionIdentityDenied,
    StaleTransactionFrame,
    AdmittedGenerationSetChanged,
    CommitBudgetExceeded {
        attempted: u16,
        maximum: u16,
    },
    DurableMutationBudgetExceeded {
        attempted: u16,
        maximum: u16,
    },
    ResizeBasisDenied,
    PortalPriorReceiptMismatch {
        ordinal: u16,
    },
    PortalBindingSuccession(
        crate::runtime::invalidation_narrowing::UiPortalBindingSuccessionDenial,
    ),
    PortalCommitBind(super::UiPortalAllocationCommitBindDenial),
    DurableSemanticStateMissing,
    CatalogBindingMismatch,
    AuthorityCounterExhausted(super::UiAllocationAuthorityCounterExhaustion),
    EvidenceCounterExhausted,
}

impl UiAllocationReplanTransactionCounters {
    pub(crate) fn preflight(cardinality: usize) -> Result<Self, ()> {
        Ok(Self {
            selected_neighborhoods: u16::try_from(cardinality).map_err(|_| ())?,
            ..Self::default()
        })
    }
    pub(crate) fn reused(&mut self) -> Result<(), ()> {
        self.reused_neighborhoods = self.reused_neighborhoods.checked_add(1).ok_or(())?;
        Ok(())
    }
    pub(crate) fn replanned(&mut self) -> Result<(), ()> {
        self.replanned_neighborhoods = self.replanned_neighborhoods.checked_add(1).ok_or(())?;
        Ok(())
    }
    pub(crate) fn committed(&mut self, count: usize) -> Result<(), ()> {
        self.committed_receipts = u16::try_from(count).map_err(|_| ())?;
        Ok(())
    }
    pub fn selected_neighborhoods(self) -> u16 {
        self.selected_neighborhoods
    }
    pub fn reused_neighborhoods(self) -> u16 {
        self.reused_neighborhoods
    }
    pub fn replanned_neighborhoods(self) -> u16 {
        self.replanned_neighborhoods
    }
    pub fn committed_receipts(self) -> u16 {
        self.committed_receipts
    }
    pub fn replay_hits(self) -> u16 {
        self.replay_hits
    }
}

impl UiCommittedAllocationReplan {
    pub(crate) fn new(
        transaction: super::UiAllocationReplanTransaction,
        receipts: Vec<super::UiAllocationReceipt>,
        counters: UiAllocationReplanTransactionCounters,
        catalog_bindings: super::UiCommittedAllocationCatalogBindings,
    ) -> Result<Self, ()> {
        for consequence in transaction.consequences().portal_anchors() {
            let movement = consequence.movement();
            let expected_identity = movement.identity_transition().current();
            let receipt = receipts
                .iter()
                .find(|receipt| {
                    movement.target().primary().neighborhood_identity()
                        == receipt
                            .committed_allocation()
                            .allocation_neighborhood()
                            .identity()
                })
                .ok_or(())?;
            if receipt.identity().portal_anchor() != Some(expected_identity)
                || receipt
                    .geometry_evidence()
                    .portal_anchor_observation()
                    .is_none_or(|observation| {
                        observation.identity() != expected_identity
                            || observation.observed_bounds().x() != movement.observation().rect().x
                            || observation.observed_bounds().y() != movement.observation().rect().y
                            || observation.observed_bounds().width()
                                != movement.observation().rect().width
                            || observation.observed_bounds().height()
                                != movement.observation().rect().height
                    })
            {
                return Err(());
            }
        }
        let scroll_evidence = transaction
            .consequences()
            .scroll_owned()
            .iter()
            .map(|consequence| {
                let affected_receipts = receipts
                    .iter()
                    .filter(|receipt| {
                        let digest = receipt
                            .committed_allocation()
                            .allocation_neighborhood()
                            .identity()
                            .identity_digest();
                        consequence
                            .neighborhood_identity_digests()
                            .binary_search(&digest)
                            .is_ok()
                    })
                    .try_fold(0u16, |count, receipt| {
                        let digest = receipt
                            .committed_allocation()
                            .allocation_neighborhood()
                            .identity()
                            .identity_digest();
                        if consequence
                            .neighborhood_identity_digests()
                            .binary_search(&digest)
                            .is_ok()
                        {
                            count.checked_add(1)
                        } else {
                            Some(count)
                        }
                    })?;
                Some(
                    consequence
                        .evidence()
                        .clone()
                        .with_commit_count(transaction.policy(), affected_receipts),
                )
            })
            .collect::<Option<Vec<_>>>()
            .ok_or(())?
            .into_boxed_slice();
        let evidence =
            super::UiCommittedAllocationEvidenceSet::ordinary(scroll_evidence, &receipts);
        Ok(Self {
            transaction,
            receipts: receipts.into_boxed_slice(),
            counters,
            evidence,
            catalog_bindings,
            portal_binding_succession: None,
        })
    }
    pub fn transaction(&self) -> &super::UiAllocationReplanTransaction {
        &self.transaction
    }
    pub fn receipts(&self) -> &[super::UiAllocationReceipt] {
        &self.receipts
    }
    pub(crate) fn catalog_bindings(&self) -> &super::UiCommittedAllocationCatalogBindings {
        &self.catalog_bindings
    }
    pub fn counters(&self) -> UiAllocationReplanTransactionCounters {
        self.counters
    }
    pub fn evidence(&self) -> crate::evidence::UiAllocationReplanTransactionEvidence {
        crate::evidence::UiAllocationReplanTransactionEvidence::from_committed(self)
    }
    pub fn viewport_evidence(&self) -> Option<&crate::evidence::UiViewportResizeEvidence> {
        self.evidence.viewport()
    }
    pub fn scroll_owned_evidence(&self) -> &[crate::evidence::UiScrollOwnedAllocationEvidence] {
        self.evidence.scroll_owned()
    }
    pub fn committed_evidence(&self) -> &super::UiCommittedAllocationEvidenceSet {
        &self.evidence
    }
    pub fn portal_binding_succession(
        &self,
    ) -> Option<&crate::runtime::invalidation_narrowing::UiPortalBindingSuccessionReceipt> {
        self.portal_binding_succession.as_ref()
    }
    pub(crate) fn with_portal_binding_succession(
        mut self,
        receipt: crate::runtime::invalidation_narrowing::UiPortalBindingSuccessionReceipt,
    ) -> Self {
        self.portal_binding_succession = Some(receipt);
        self
    }
    pub(super) fn with_viewport_evidence(
        mut self,
        evidence: crate::evidence::UiViewportResizeEvidence,
    ) -> Self {
        self.evidence = self.evidence.with_viewport(evidence);
        self
    }
}
