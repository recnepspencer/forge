impl crate::runtime::WorthUiRuntime {
    pub(crate) fn replay_admitted_transaction_for_test(
        &mut self,
        selection: &crate::graph::UiAdmittedReplanNeighborhoodSet,
    ) -> crate::runtime::UiAllocationReplanTransactionOutcome {
        let pending = super::allocation_transaction::prepare_selected(
            &self.allocation_receipt_ledger,
            &self.allocation_invalidation_index.borrow(),
            selection,
        );
        super::allocation_transaction::publish_pending(
            &self.allocation_receipt_ledger,
            &mut self.allocation_invalidation_index.borrow_mut(),
            pending,
        )
    }

    pub(crate) fn replay_admitted_durable_transaction_for_test(
        &mut self,
        selection: &crate::graph::UiAdmittedReplanNeighborhoodSet,
        identity: u64,
        extent: crate::runtime::UiResizeLogicalExtent,
    ) -> (
        crate::runtime::UiAllocationReplanTransactionOutcome,
        Option<crate::runtime::UiAllocationDurableSemanticState>,
        bool,
    ) {
        let previous = self.allocation_receipt_ledger.durable_semantic_state();
        let (pending, requested_mutation) =
            super::allocation_transaction::prepare_pending_durable_resize(
                &self.allocation_receipt_ledger,
                &self.allocation_invalidation_index.borrow(),
                selection,
                identity,
                extent,
            );
        let outcome = super::allocation_transaction::publish_pending(
            &self.allocation_receipt_ledger,
            &mut self.allocation_invalidation_index.borrow_mut(),
            pending,
        );
        let state = self.allocation_receipt_ledger.durable_semantic_state();
        let mutated = matches!(
            outcome,
            crate::runtime::UiAllocationReplanTransactionOutcome::Committed(_)
        ) && requested_mutation
            && previous != state;
        (outcome, state, mutated)
    }

    pub(crate) fn into_runtime(self) -> Self {
        self
    }

    pub(crate) fn admit_durable_resize_source(
        &mut self,
        input: crate::runtime::WorthUiAdmittedDurableResizeInput,
    ) -> Result<
        crate::runtime::WorthUiAdmittedDurableResizeSourceFact,
        crate::runtime::WorthUiDurableResizeSourceAdmissionDenial,
    > {
        self.durable_resize_source
            .admit(crate::runtime::UiDurableResizeCommitIntent::terminal(
                input,
                crate::runtime::UiResizeLogicalExtent::try_from_logical_pixels(320.0).unwrap(),
            ))
    }

    pub(crate) fn interaction_admission(
        &mut self,
    ) -> crate::runtime::WorthUiTransientInteractionAdmission<'_> {
        crate::runtime::WorthUiTransientInteractionAdmission::new(
            &mut self.transient_interaction_admission,
        )
    }

    pub(crate) fn install_query_binding_for_test(
        &mut self,
        plan: worth_ui_query_binding::WorthUiQueryBindingPlan,
    ) {
        self.query_binding = plan.activate();
    }

    pub(crate) fn admit_query_projection_for_test(
        &mut self,
        outcome: worth_ui_query_binding::WorthUiQuerySnapshotProjectionOutcome,
    ) -> Result<
        worth_ui_query_binding::WorthUiQueryMeasurementFactSettlement,
        worth_ui_query_binding::WorthUiQueryMeasurementFactSettlementDenial,
    > {
        self.query_binding.admit(outcome)
    }
}
