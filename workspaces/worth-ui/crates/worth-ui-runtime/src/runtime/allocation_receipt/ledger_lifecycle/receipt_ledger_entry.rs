use super::{
    replan_commit_mode::UiAllocationReplanCommitMode, UiAllocationReceiptLedger,
    UiAllocationReplanTransactionOutcome,
};
use std::cell::RefMut;

pub(in crate::runtime) struct UiPreparedAllocationCatalogLedgerCommit<'ledger> {
    live: RefMut<'ledger, super::ledger_state::UiAllocationReceiptLedgerState>,
    successor: super::ledger_state::UiAllocationReceiptLedgerState,
}

impl UiPreparedAllocationCatalogLedgerCommit<'_> {
    pub(in crate::runtime) fn commit_once(mut self) {
        *self.live = self.successor;
    }
}

pub(in crate::runtime) struct UiPreparedAllocationReplanLedgerCommit<'ledger, 'authority> {
    owner:
        &'authority mut crate::runtime::allocation_frame_dispatch::UiAllocationTransactionAuthority,
    live: RefMut<'ledger, super::ledger_state::UiAllocationReceiptLedgerState>,
    successor: super::ledger_state::UiAllocationReceiptLedgerState,
}

impl<'authority> UiPreparedAllocationReplanLedgerCommit<'_, 'authority> {
    pub(in crate::runtime) fn commit_once(
        mut self,
    ) -> &'authority mut crate::runtime::allocation_frame_dispatch::UiAllocationTransactionAuthority
    {
        *self.live = self.successor;
        self.owner
    }
}

impl UiAllocationReceiptLedger {
    pub(in crate::runtime) fn retain_prepared_denial(
        &self,
        transition: &super::UiPreparedAllocationLedgerTransition,
        denial: super::UiAllocationReplanTransactionCommitDenial,
    ) -> super::UiAllocationReplanTransactionOutcome {
        let mut live = self.state.borrow_mut();
        if *live != transition.predecessor {
            return super::UiAllocationReplanTransactionOutcome::Denied(
                super::UiAllocationReplanTransactionCommitDenial::PortalCommitBind(
                    super::UiPortalAllocationCommitBindDenial::LedgerPredecessorChanged {
                        expected_truth_revision: transition.predecessor.truth_revision.revision(),
                        observed_truth_revision: live.truth_revision.revision(),
                        expected_transaction_generation: transition
                            .predecessor
                            .next_transaction_generation,
                        observed_transaction_generation: live.next_transaction_generation,
                    },
                ),
            );
        }
        super::ledger_denial::retain_denial(&mut live, transition.committed.transaction(), denial)
    }

    pub(in crate::runtime) fn bind_replan_transition<'ledger, 'authority>(
        &'ledger self,
        owner: &'authority mut crate::runtime::allocation_frame_dispatch::UiAllocationTransactionAuthority,
        transition: &super::UiPreparedAllocationLedgerTransition,
    ) -> Result<
        UiPreparedAllocationReplanLedgerCommit<'ledger, 'authority>,
        super::UiPortalAllocationCommitBindDenial,
    > {
        if !owner.certifies_committed(transition.committed()) {
            return Err(
                super::UiPortalAllocationCommitBindDenial::BindingPredecessorChanged {
                    expected_identity_digest: transition
                        .committed()
                        .transaction()
                        .idempotency_key(),
                    observed_identity_digest: 0,
                },
            );
        }
        let live = self
            .state
            .try_borrow_mut()
            .map_err(|_| super::UiPortalAllocationCommitBindDenial::LedgerBorrowUnavailable)?;
        if *live != transition.predecessor {
            return Err(
                super::UiPortalAllocationCommitBindDenial::LedgerPredecessorChanged {
                    expected_truth_revision: transition.predecessor.truth_revision.revision(),
                    observed_truth_revision: live.truth_revision.revision(),
                    expected_transaction_generation: transition
                        .predecessor
                        .next_transaction_generation,
                    observed_transaction_generation: live.next_transaction_generation,
                },
            );
        }
        Ok(UiPreparedAllocationReplanLedgerCommit {
            owner,
            live,
            successor: transition.successor.clone(),
        })
    }

    pub(crate) fn truth_revision(&self) -> super::UiAllocationTruthRevision {
        self.state.borrow().truth_revision
    }

    pub(in crate::runtime) fn prepare_catalog_commit(
        &self,
        transition: &super::UiAllocationCatalogLedgerTransition,
    ) -> Option<UiPreparedAllocationCatalogLedgerCommit<'_>> {
        let live = self.state.borrow_mut();
        if *live != transition.predecessor {
            return None;
        }
        Some(UiPreparedAllocationCatalogLedgerCommit {
            live,
            successor: transition.successor.clone(),
        })
    }

    pub(in crate::runtime) fn prepare_selected(
        &self,
        owner: &crate::runtime::allocation_frame_dispatch::UiAllocationTransactionAuthority,
        selection: &crate::graph::UiAdmittedReplanNeighborhoodSet,
    ) -> super::UiAllocationLedgerPreparation {
        if !owner.certifies_selection(selection) {
            return UiAllocationReplanTransactionOutcome::Denied(
                super::UiAllocationReplanTransactionCommitDenial::AdmittedGenerationSetChanged,
            )
            .into();
        }
        self.prepare_selected_mode(UiAllocationReplanCommitMode::Ordinary(selection))
    }

    pub(in crate::runtime) fn prepare_viewport(
        &self,
        owner: &crate::runtime::allocation_frame_dispatch::UiAllocationTransactionAuthority,
        basis: crate::runtime::UiViewportResizeCommitBasis<'_>,
    ) -> super::UiAllocationLedgerPreparation {
        if !owner.certifies_selection(basis.selection()) {
            return UiAllocationReplanTransactionOutcome::Denied(
                super::UiAllocationReplanTransactionCommitDenial::AdmittedGenerationSetChanged,
            )
            .into();
        }
        self.prepare_selected_mode(UiAllocationReplanCommitMode::Viewport(Box::new(basis)))
    }

    pub(in crate::runtime) fn prepare_durable_resize(
        &self,
        owner: &crate::runtime::allocation_frame_dispatch::UiAllocationTransactionAuthority,
        selection: &crate::graph::UiAdmittedReplanNeighborhoodSet,
        identity_digest: u64,
        extent: crate::runtime::UiResizeLogicalExtent,
    ) -> (
        super::UiAllocationLedgerPreparation,
        Option<super::UiAllocationDurableSemanticState>,
        bool,
    ) {
        if !owner.certifies_selection(selection) {
            return (
                UiAllocationReplanTransactionOutcome::Denied(
                    super::UiAllocationReplanTransactionCommitDenial::AdmittedGenerationSetChanged,
                )
                .into(),
                self.state.borrow().durable_semantic_state.clone(),
                false,
            );
        }
        let Some(basis) = crate::runtime::UiResizeAllocationPlanningBasis::seal(
            selection,
            selection.primary().locality().target_graph_node_identity(),
            Some(identity_digest),
            extent,
        ) else {
            return (
                UiAllocationReplanTransactionOutcome::Denied(
                    super::UiAllocationReplanTransactionCommitDenial::ResizeBasisDenied,
                )
                .into(),
                self.state.borrow().durable_semantic_state.clone(),
                false,
            );
        };
        if self.state.borrow().durable_semantic_state.is_none() {
            return (
                UiAllocationReplanTransactionOutcome::Denied(
                    super::UiAllocationReplanTransactionCommitDenial::DurableSemanticStateMissing,
                )
                .into(),
                None,
                false,
            );
        }
        let previous = self
            .state
            .borrow()
            .durable_semantic_state
            .as_ref()
            .expect("checked activated durable state")
            .committed_resize(identity_digest)
            .map(|basis| basis.extent());
        let outcome = self.prepare_selected_mode(UiAllocationReplanCommitMode::DurableResize {
            selection,
            basis,
        });
        let mutated = previous != Some(extent);
        let state = self.state.borrow().durable_semantic_state.clone();
        (outcome, state, mutated)
    }

    pub(crate) fn durable_semantic_state(&self) -> Option<super::UiAllocationDurableSemanticState> {
        self.state.borrow().durable_semantic_state.clone()
    }
}
