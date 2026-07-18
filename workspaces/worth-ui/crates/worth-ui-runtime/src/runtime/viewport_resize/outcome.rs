#[derive(Clone, Debug, PartialEq)]
pub struct UiViewportResizeOutcome {
    transaction: UiViewportCommittedReplan,
}

#[derive(Clone, Debug, PartialEq)]
/// A viewport-specific committed/replayed handoff can only be obtained from
/// `WorthUiFrameworkTurnCompletion::ViewportResizeResolved`.
///
/// ```compile_fail
/// use worth_ui_runtime::facade::runtime_handoff::UiViewportCommittedReplan;
/// ```
pub struct UiViewportCommittedReplan {
    committed: crate::runtime::UiCommittedAllocationReplan,
    replayed: bool,
}

impl UiViewportResizeOutcome {
    pub(crate) fn resolve(
        transaction: crate::runtime::UiAllocationReplanTransactionOutcome,
    ) -> Result<Self, super::UiViewportResizeDenial> {
        let transaction = match transaction {
            crate::runtime::UiAllocationReplanTransactionOutcome::Committed(value) => {
                UiViewportCommittedReplan::admit(value, false)?
            }
            crate::runtime::UiAllocationReplanTransactionOutcome::Replayed(value) => {
                UiViewportCommittedReplan::admit(value, true)?
            }
            crate::runtime::UiAllocationReplanTransactionOutcome::Denied(_) => {
                return Err(super::UiViewportResizeDenial::TransactionCommitDenied)
            }
        };
        Ok(Self { transaction })
    }
    pub fn strategy(&self) -> super::UiViewportReceiptCommitStrategy {
        self.evidence().strategy()
    }
    pub fn counters(&self) -> super::UiViewportResizeCounters {
        super::UiViewportResizeCounters::from_committed(
            self.evidence(),
            self.transaction.replayed(),
        )
    }
    pub fn transaction(&self) -> &UiViewportCommittedReplan {
        &self.transaction
    }
    pub fn committed_replan(&self) -> &crate::runtime::UiCommittedAllocationReplan {
        self.transaction.committed_replan()
    }
    pub fn frame_epoch(&self) -> crate::runtime::UiAllocationFrameEpoch {
        self.evidence().frame_epoch()
    }
    pub fn primary_neighborhood_identity_digest(&self) -> u64 {
        self.evidence().primary_neighborhood_identity_digest()
    }
    pub fn selected_neighborhood_identity_digests(&self) -> &[u64] {
        self.committed_evidence()
            .selected_neighborhood_identity_digests()
    }
    pub fn root_posture(&self) -> crate::graph::UiReplanRootPosture {
        self.evidence().root_posture()
    }
    pub fn transaction_idempotency_key(&self) -> u64 {
        self.committed_replan().transaction().idempotency_key()
    }
    pub fn evidence(&self) -> crate::evidence::UiViewportResizeEvidence {
        self.committed_evidence().clone()
    }
    fn committed_evidence(&self) -> &crate::evidence::UiViewportResizeEvidence {
        self.transaction.evidence()
    }
}

impl UiViewportCommittedReplan {
    fn admit(
        committed: crate::runtime::UiCommittedAllocationReplan,
        replayed: bool,
    ) -> Result<Self, super::UiViewportResizeDenial> {
        if committed.viewport_evidence().is_none() {
            return Err(super::UiViewportResizeDenial::TransactionCommitDenied);
        }
        Ok(Self {
            committed,
            replayed,
        })
    }
    pub fn committed_replan(&self) -> &crate::runtime::UiCommittedAllocationReplan {
        &self.committed
    }
    pub fn replayed(&self) -> bool {
        self.replayed
    }
    fn evidence(&self) -> &crate::evidence::UiViewportResizeEvidence {
        self.committed
            .viewport_evidence()
            .expect("admission excludes committed replans without viewport evidence")
    }
}
