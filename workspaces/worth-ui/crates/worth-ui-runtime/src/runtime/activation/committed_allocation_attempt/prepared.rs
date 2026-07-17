pub(super) struct UiCommittedAllocationSuccessors {
    receipt_draft: super::WorthUiPlanSwapReceiptDraft,
    next_active: crate::runtime::active::WorthUiActiveRuntimeState,
    invalidation_transition:
        crate::runtime::invalidation_narrowing::UiPreparedInvalidationCatalogTransition,
    durable_resize_successor: crate::runtime::reconciliation::WorthUiDurableResizeSourceAuthority,
}

pub(crate) struct UiPreparedCommittedAllocationActivation<'runtime> {
    ledger_commit: crate::runtime::allocation_receipt::UiPreparedAllocationCatalogLedgerCommit<'runtime>,
    invalidation: std::cell::RefMut<
        'runtime,
        crate::runtime::invalidation_narrowing::UiAllocationInvalidationAuthority,
    >,
    frame_commit:
        crate::runtime::allocation_frame_dispatch::UiPreparedFrameReplacementCommit<'runtime>,
    active: &'runtime mut crate::runtime::active::WorthUiActiveRuntimeState,
    last_valid: &'runtime mut crate::runtime::launch::WorthUiLastValidRuntimeState,
    last_valid_successor: crate::runtime::launch::WorthUiLastValidRuntimeState,
    transient_interaction_admission: &'runtime mut crate::runtime::replacement::state_inventory::WorthUiTransientInteractionAdmissionAuthority,
    durable_resize_source:
        &'runtime mut crate::runtime::reconciliation::WorthUiDurableResizeSourceAuthority,
    receipt_draft: super::WorthUiPlanSwapReceiptDraft,
    next_active: crate::runtime::active::WorthUiActiveRuntimeState,
    invalidation_transition:
        crate::runtime::invalidation_narrowing::UiPreparedInvalidationCatalogTransition,
    frame_assignment: crate::runtime::allocation_frame_dispatch::UiAllocationFrameEpochAssignment,
    durable_resize_successor:
        crate::runtime::reconciliation::WorthUiDurableResizeSourceAuthority,
}

pub(super) struct UiCommittedAllocationCommitResources<'runtime> {
    pub ledger_commit: crate::runtime::allocation_receipt::UiPreparedAllocationCatalogLedgerCommit<'runtime>,
    pub invalidation: std::cell::RefMut<
        'runtime,
        crate::runtime::invalidation_narrowing::UiAllocationInvalidationAuthority,
    >,
    pub frame_commit:
        crate::runtime::allocation_frame_dispatch::UiPreparedFrameReplacementCommit<'runtime>,
    pub active: &'runtime mut crate::runtime::active::WorthUiActiveRuntimeState,
    pub last_valid: &'runtime mut crate::runtime::launch::WorthUiLastValidRuntimeState,
    pub transient_interaction_admission: &'runtime mut crate::runtime::replacement::state_inventory::WorthUiTransientInteractionAdmissionAuthority,
    pub durable_resize_source:
        &'runtime mut crate::runtime::reconciliation::WorthUiDurableResizeSourceAuthority,
}

impl UiCommittedAllocationSuccessors {
    pub(super) fn new(
        receipt_draft: super::WorthUiPlanSwapReceiptDraft,
        next_active: crate::runtime::active::WorthUiActiveRuntimeState,
        ledger_transition: crate::runtime::allocation_receipt::UiAllocationCatalogLedgerTransition,
        invalidation_transition: crate::runtime::invalidation_narrowing::UiPreparedInvalidationCatalogTransition,
    ) -> Self {
        let durable_resize_successor =
            crate::runtime::reconciliation::WorthUiDurableResizeSourceAuthority::prepare_successor(
                ledger_transition.durable_reconciliation(),
            );
        Self {
            receipt_draft,
            next_active,
            invalidation_transition,
            durable_resize_successor,
        }
    }

    pub(crate) fn bind_commit_resources<'runtime>(
        self,
        resources: UiCommittedAllocationCommitResources<'runtime>,
    ) -> UiPreparedCommittedAllocationActivation<'runtime> {
        let UiCommittedAllocationCommitResources {
            ledger_commit,
            invalidation,
            frame_commit,
            active,
            last_valid,
            transient_interaction_admission,
            durable_resize_source,
        } = resources;
        let frame_assignment = frame_commit.assignment();
        let last_valid_successor =
            crate::runtime::launch::WorthUiLastValidRuntimeState::record_from_active(active);
        UiPreparedCommittedAllocationActivation {
            ledger_commit,
            invalidation,
            frame_commit,
            active,
            last_valid,
            last_valid_successor,
            transient_interaction_admission,
            durable_resize_source,
            receipt_draft: self.receipt_draft,
            next_active: self.next_active,
            invalidation_transition: self.invalidation_transition,
            frame_assignment,
            durable_resize_successor: self.durable_resize_successor,
        }
    }
}

impl UiPreparedCommittedAllocationActivation<'_> {
    pub(crate) fn commit_once(mut self) -> crate::runtime::WorthUiPlanSwapReceipt {
        self.invalidation
            .commit_catalog_transition(self.invalidation_transition);
        self.ledger_commit.commit_once();
        *self.last_valid = self.last_valid_successor;
        *self.active = self.next_active;
        self.active
            .apply_allocation_frame_epoch_assignment(self.frame_assignment);
        let allocation_frame_replacement = self.frame_commit.commit_once();
        *self.transient_interaction_admission = Default::default();
        *self.durable_resize_source = self.durable_resize_successor;
        self.receipt_draft.finish(allocation_frame_replacement)
    }
}
