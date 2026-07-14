use crate::runtime::{
    WorthUiActivationGateCounters, WorthUiExecutionPlan, WorthUiExecutionPlanDigest,
    WorthUiExecutionPlanInput, WorthUiLaneParityReport, WorthUiPendingActivation,
    WorthUiRuntimeFrameEpoch, WorthUiRuntimeHandleAllocation,
};

#[derive(Debug, PartialEq)]
pub(super) struct UiCommittedAllocationValidation {
    attempt_identity: super::UiCommittedAllocationActivationIdentity,
    activation_counters: super::UiCommittedAllocationActivationCounters,
    committed: crate::runtime::UiCommittedAllocationReplan,
    pending_activation: WorthUiPendingActivation,
    candidate_execution_plan_digest: WorthUiExecutionPlanDigest,
    handle_allocation_basis_digest: u64,
    node_classification_count: usize,
    lane_changed_node_count: usize,
    reconciliation_basis_digest: u64,
    query_rebind_basis_digest: u64,
    query_rebind_denied_count: usize,
    lane_parity_semantic_reference_digest: Option<u64>,
    counters: WorthUiActivationGateCounters,
    ledger_transition: crate::runtime::allocation_receipt::UiAllocationCatalogLedgerTransition,
    catalog_transition:
        crate::runtime::invalidation_narrowing::UiAllocationNeighborhoodCatalogTransition,
}

impl UiCommittedAllocationValidation {
    pub(super) fn prepare(
        pending_activation: WorthUiPendingActivation,
        plan_input: &WorthUiExecutionPlanInput,
        handle_allocation: &WorthUiRuntimeHandleAllocation,
        committed_activation: crate::runtime::UiCommittedAllocationActivationAttempt,
        invalidation_authority: &crate::runtime::invalidation_narrowing::UiAllocationInvalidationAuthority,
        candidate_plan: &WorthUiExecutionPlan,
        lane_parity_report: Option<&WorthUiLaneParityReport>,
    ) -> Result<Self, super::UiCommittedAllocationActivationDenial> {
        let super::UiCommittedAllocationActivationAttempt {
            catalog,
            ledger_transition,
            committed,
            activation,
            identity,
        } = committed_activation;
        let facts = super::input_validation::validate_activation_inputs(
            &pending_activation,
            plan_input,
            handle_allocation,
            &catalog,
            candidate_plan,
            lane_parity_report,
        )
        .map_err(|denial| {
            super::UiCommittedAllocationActivationDenial::validation(identity.clone(), denial)
        })?;
        let attempt_identity = identity.clone();
        let mut activation_counters = super::UiCommittedAllocationActivationCounters::default();
        let readiness_work = facts
            .counters
            .readiness_check_count()
            .checked_add(facts.counters.digest_check_count())
            .and_then(|count| count.checked_add(facts.counters.query_rebind_entry_check_count()))
            .and_then(|count| count.checked_add(facts.counters.lane_parity_check_count()))
            .ok_or_else(|| {
                super::UiCommittedAllocationActivationDenial::counter_exhausted(
                    attempt_identity.clone(),
                    activation_counters,
                    super::UiCommittedAllocationActivationCounterExhaustion::ReadinessChecks,
                )
            })?;
        if let Err(exhaustion) = activation_counters.record_readiness_checks(readiness_work) {
            return Err(
                super::UiCommittedAllocationActivationDenial::counter_exhausted(
                    attempt_identity,
                    activation_counters,
                    exhaustion,
                ),
            );
        }
        if let Err(exhaustion) = activation_counters.record_portal_binding_check() {
            return Err(
                super::UiCommittedAllocationActivationDenial::counter_exhausted(
                    attempt_identity,
                    activation_counters,
                    exhaustion,
                ),
            );
        }
        if let Err(denial) = activation.validate_portal_bindings() {
            return Err(super::UiCommittedAllocationActivationDenial::preparation(
                attempt_identity,
                activation_counters,
                super::UiCommittedAllocationActivationDenialReason::PortalBinding(denial),
            ));
        }
        let catalog_transition =
            invalidation_authority.seal_catalog_transition(&catalog, activation, identity);
        Ok(Self {
            attempt_identity,
            activation_counters,
            committed,
            pending_activation,
            candidate_execution_plan_digest: facts.candidate_execution_plan_digest,
            handle_allocation_basis_digest: facts.handle_allocation_basis_digest,
            node_classification_count: facts.node_classification_count,
            lane_changed_node_count: facts.lane_changed_node_count,
            reconciliation_basis_digest: facts.reconciliation_basis_digest,
            query_rebind_basis_digest: facts.query_rebind_basis_digest,
            query_rebind_denied_count: facts.query_rebind_denied_count,
            lane_parity_semantic_reference_digest: facts.lane_parity_semantic_reference_digest,
            counters: facts.counters,
            ledger_transition,
            catalog_transition,
        })
    }

    pub(crate) fn committed(&self) -> &crate::runtime::UiCommittedAllocationReplan {
        &self.committed
    }

    pub(crate) fn attempt_identity(&self) -> &super::UiCommittedAllocationActivationIdentity {
        &self.attempt_identity
    }

    pub(crate) fn activation_counters(&self) -> super::UiCommittedAllocationActivationCounters {
        self.activation_counters
    }

    pub(crate) fn record_frame_boundary_check(
        &mut self,
    ) -> Result<(), super::UiCommittedAllocationActivationDenial> {
        self.record_check(
            super::UiCommittedAllocationActivationCounters::record_frame_boundary_check,
        )
    }

    pub(crate) fn record_graph_predecessor_check(
        &mut self,
    ) -> Result<(), super::UiCommittedAllocationActivationDenial> {
        self.record_check(
            super::UiCommittedAllocationActivationCounters::record_graph_predecessor_check,
        )
    }

    pub(crate) fn record_ledger_predecessor_check(
        &mut self,
    ) -> Result<(), super::UiCommittedAllocationActivationDenial> {
        self.record_check(
            super::UiCommittedAllocationActivationCounters::record_ledger_predecessor_check,
        )
    }

    pub(crate) fn record_scroll_binding_check(
        &mut self,
    ) -> Result<(), super::UiCommittedAllocationActivationDenial> {
        self.record_check(
            super::UiCommittedAllocationActivationCounters::record_scroll_binding_check,
        )
    }

    fn record_check(
        &mut self,
        record: fn(
            &mut super::UiCommittedAllocationActivationCounters,
        ) -> Result<(), super::UiCommittedAllocationActivationCounterExhaustion>,
    ) -> Result<(), super::UiCommittedAllocationActivationDenial> {
        if let Err(exhaustion) = record(&mut self.activation_counters) {
            return Err(
                super::UiCommittedAllocationActivationDenial::counter_exhausted(
                    self.attempt_identity.clone(),
                    self.activation_counters,
                    exhaustion,
                ),
            );
        }
        Ok(())
    }

    pub fn readiness_frame_epoch(&self) -> WorthUiRuntimeFrameEpoch {
        self.pending_activation.frame_epoch()
    }

    pub fn active_artifact_digest(&self) -> u64 {
        self.pending_activation
            .staged_replacement()
            .active_artifact_digest()
    }

    pub fn candidate_artifact_digest(&self) -> u64 {
        self.pending_activation
            .staged_replacement()
            .candidate_artifact_digest()
    }

    pub fn candidate_execution_plan_digest(&self) -> u64 {
        self.candidate_execution_plan_digest.raw()
    }

    pub fn handle_allocation_basis_digest(&self) -> u64 {
        self.handle_allocation_basis_digest
    }

    pub fn node_classification_count(&self) -> usize {
        self.node_classification_count
    }

    pub fn lane_changed_node_count(&self) -> usize {
        self.lane_changed_node_count
    }

    pub fn reconciliation_basis_digest(&self) -> u64 {
        self.reconciliation_basis_digest
    }

    pub fn reconciliation_receipt_count(&self) -> usize {
        self.pending_activation
            .staged_replacement()
            .reconciliation_plan()
            .receipts()
            .len()
    }

    pub fn query_rebind_entry_count(&self) -> usize {
        self.pending_activation
            .staged_replacement()
            .query_rebind_plan()
            .entries()
            .len()
    }

    pub fn query_rebind_basis_digest(&self) -> u64 {
        self.query_rebind_basis_digest
    }

    pub fn query_rebind_denied_count(&self) -> usize {
        self.query_rebind_denied_count
    }

    pub fn lane_parity_semantic_reference_digest(&self) -> Option<u64> {
        self.lane_parity_semantic_reference_digest
    }

    pub fn counters(&self) -> WorthUiActivationGateCounters {
        self.counters
    }

    pub(crate) fn pending_activation(&self) -> &WorthUiPendingActivation {
        &self.pending_activation
    }

    pub(crate) fn allocation_catalog_transition(
        &self,
    ) -> &crate::runtime::invalidation_narrowing::UiAllocationNeighborhoodCatalogTransition {
        &self.catalog_transition
    }

    pub(crate) fn into_ledger_transition(
        self,
    ) -> crate::runtime::allocation_receipt::UiAllocationCatalogLedgerTransition {
        self.ledger_transition
    }
}
