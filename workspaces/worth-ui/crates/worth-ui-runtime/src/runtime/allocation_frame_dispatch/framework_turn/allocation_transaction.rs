use crate::runtime::allocation_receipt::{
    UiAllocationLedgerPreparation, UiAllocationReceiptLedger, UiPreparedAllocationLedgerTransition,
};

#[derive(Debug)]
pub(in crate::runtime) struct UiAllocationTransactionAuthority {
    graph_basis: crate::graph::UiGraphReplanTransactionBasis,
    transaction: Option<crate::runtime::UiAllocationReplanTransaction>,
}

#[derive(Debug)]
pub(super) struct UiPendingAllocationTransaction {
    authority: UiAllocationTransactionAuthority,
    preparation: UiAllocationLedgerPreparation,
}

pub(super) fn prepare_selected(
    ledger: &UiAllocationReceiptLedger,
    authority: &crate::runtime::invalidation_narrowing::UiAllocationInvalidationAuthority,
    selection: &crate::graph::UiAdmittedReplanNeighborhoodSet,
) -> UiPendingAllocationTransaction {
    let transaction_authority = UiAllocationTransactionAuthority::for_selection(selection);
    let preparation = if authority.certifies_selection(selection) {
        ledger.prepare_selected(&transaction_authority, selection)
    } else {
        denied_generation().into()
    };
    UiPendingAllocationTransaction {
        authority: transaction_authority,
        preparation,
    }
}

pub(super) fn prepare_viewport(
    ledger: &UiAllocationReceiptLedger,
    authority: &crate::runtime::invalidation_narrowing::UiAllocationInvalidationAuthority,
    basis: crate::runtime::UiViewportResizeCommitBasis,
) -> UiPendingAllocationTransaction {
    let transaction_authority = UiAllocationTransactionAuthority::for_selection(basis.selection());
    let preparation = if authority.certifies_selection(basis.selection()) {
        ledger.prepare_viewport(&transaction_authority, basis)
    } else {
        denied_generation().into()
    };
    UiPendingAllocationTransaction {
        authority: transaction_authority,
        preparation,
    }
}

impl UiAllocationTransactionAuthority {
    fn for_selection(selection: &crate::graph::UiAdmittedReplanNeighborhoodSet) -> Self {
        Self {
            graph_basis: selection.transaction_basis().clone(),
            transaction: None,
        }
    }

    pub(in crate::runtime) fn certifies_selection(
        &self,
        selection: &crate::graph::UiAdmittedReplanNeighborhoodSet,
    ) -> bool {
        self.graph_basis == *selection.transaction_basis()
    }

    fn bind_transition(&mut self, transition: &UiPreparedAllocationLedgerTransition) {
        self.transaction = Some(transition.committed().transaction().clone());
    }

    pub(in crate::runtime) fn certifies_committed(
        &self,
        committed: &crate::runtime::UiCommittedAllocationReplan,
    ) -> bool {
        self.transaction.as_ref().is_some_and(|expected| {
            expected.same_idempotency_basis(committed.transaction())
                && expected == committed.transaction()
        })
    }
}

pub(super) fn prepare_pending_durable_resize(
    ledger: &UiAllocationReceiptLedger,
    authority: &crate::runtime::invalidation_narrowing::UiAllocationInvalidationAuthority,
    selection: &crate::graph::UiAdmittedReplanNeighborhoodSet,
    identity: u64,
    extent: crate::runtime::UiResizeLogicalExtent,
) -> (UiPendingAllocationTransaction, bool) {
    let transaction_authority = UiAllocationTransactionAuthority::for_selection(selection);
    let (preparation, requested_mutation) = if authority.certifies_selection(selection) {
        let (preparation, _, requested_mutation) =
            ledger.prepare_durable_resize(&transaction_authority, selection, identity, extent);
        (preparation, requested_mutation)
    } else {
        (denied_generation().into(), false)
    };
    (
        UiPendingAllocationTransaction {
            authority: transaction_authority,
            preparation,
        },
        requested_mutation,
    )
}

pub(super) fn publish_pending(
    ledger: &UiAllocationReceiptLedger,
    authority: &mut crate::runtime::invalidation_narrowing::UiAllocationInvalidationAuthority,
    mut pending: UiPendingAllocationTransaction,
) -> crate::runtime::UiAllocationReplanTransactionOutcome {
    publish_prepared(
        ledger,
        authority,
        &mut pending.authority,
        pending.preparation,
    )
}

pub(super) fn publish_prepared(
    ledger: &UiAllocationReceiptLedger,
    authority: &mut crate::runtime::invalidation_narrowing::UiAllocationInvalidationAuthority,
    transaction_authority: &mut UiAllocationTransactionAuthority,
    preparation: UiAllocationLedgerPreparation,
) -> crate::runtime::UiAllocationReplanTransactionOutcome {
    let UiAllocationLedgerPreparation::Prepared(transition) = preparation else {
        let UiAllocationLedgerPreparation::Resolved(outcome) = preparation else {
            unreachable!()
        };
        return *outcome;
    };
    let mut transition = *transition;
    transaction_authority.bind_transition(&transition);
    let portal = !transition
        .committed()
        .transaction()
        .consequences()
        .portal_anchors()
        .is_empty();
    let succession = if portal {
        match authority
            .prepare_portal_binding_succession(transaction_authority, transition.committed())
        {
            Ok(prepared) => {
                let receipt = prepared.receipt();
                let committed = transition
                    .committed()
                    .clone()
                    .with_portal_binding_succession(receipt);
                transition = transition.with_committed(committed);
                Some(prepared)
            }
            Err(denial) => return ledger.retain_prepared_denial(
                &transition,
                crate::runtime::UiAllocationReplanTransactionCommitDenial::PortalBindingSuccession(
                    denial,
                ),
            ),
        }
    } else {
        None
    };
    bind_and_publish(
        ledger,
        authority,
        transition,
        succession,
        transaction_authority,
    )
}

fn bind_and_publish(
    ledger: &UiAllocationReceiptLedger,
    authority: &mut crate::runtime::invalidation_narrowing::UiAllocationInvalidationAuthority,
    transition: UiPreparedAllocationLedgerTransition,
    succession: Option<crate::runtime::invalidation_narrowing::UiPreparedPortalBindingSuccession>,
    transaction_authority: &mut UiAllocationTransactionAuthority,
) -> crate::runtime::UiAllocationReplanTransactionOutcome {
    if let Some(prepared) = succession.as_ref() {
        let expected = prepared.predecessor_identity_digest();
        let observed = authority.portal_binding_identity_digest();
        if expected != observed {
            return crate::runtime::UiAllocationReplanTransactionOutcome::Denied(
                crate::runtime::UiAllocationReplanTransactionCommitDenial::PortalCommitBind(
                    crate::runtime::UiPortalAllocationCommitBindDenial::BindingPredecessorChanged {
                        expected_identity_digest: expected,
                        observed_identity_digest: observed,
                    },
                ),
            );
        }
    }
    let outcome = transition.committed().clone();
    let ledger_commit = match ledger.bind_replan_transition(transaction_authority, &transition) {
        Ok(prepared) => prepared,
        Err(denial) => {
            return crate::runtime::UiAllocationReplanTransactionOutcome::Denied(
                crate::runtime::UiAllocationReplanTransactionCommitDenial::PortalCommitBind(denial),
            )
        }
    };
    let transaction_authority = ledger_commit.commit_once();
    if let Some(prepared) = succession {
        authority.commit_portal_binding_succession(transaction_authority, prepared);
    }
    crate::runtime::UiAllocationReplanTransactionOutcome::Committed(outcome)
}

fn denied_generation() -> crate::runtime::UiAllocationReplanTransactionOutcome {
    crate::runtime::UiAllocationReplanTransactionOutcome::Denied(
        crate::runtime::UiAllocationReplanTransactionCommitDenial::AdmittedGenerationSetChanged,
    )
}
