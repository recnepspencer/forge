use crate::runtime::{
    WorthUiActivationGateCounters, WorthUiExecutionPlanDigest, WorthUiExecutionPlanInput,
    WorthUiLaneParityReport, WorthUiPendingActivation, WorthUiRuntimeFrameEpoch,
    WorthUiRuntimeHandleAllocation,
};

mod mounted;

#[derive(Debug, PartialEq)]
pub(super) struct UiCommittedAllocationValidation {
    attempt_identity: super::UiCommittedAllocationActivationIdentity,
    activation_counters: super::UiCommittedAllocationActivationCounters,
    committed: crate::runtime::UiCommittedAllocationReplan,
    activation_basis: UiCommittedAllocationValidationBasis,
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

#[derive(Debug, PartialEq)]
enum UiCommittedAllocationValidationBasis {
    Replacement(Box<WorthUiPendingActivation>),
    Mounted(Box<crate::runtime::WorthUiMountedAllocationActivationBasis>),
}

impl UiCommittedAllocationValidation {
    pub(super) fn prepare(
        pending_activation: WorthUiPendingActivation,
        plan_input: &WorthUiExecutionPlanInput,
        handle_allocation: &WorthUiRuntimeHandleAllocation,
        committed_activation: crate::runtime::UiCommittedAllocationActivationAttempt,
        invalidation_authority: &crate::runtime::invalidation_narrowing::UiAllocationInvalidationAuthority,
        candidate_bundle: &crate::runtime::active::WorthUiSealedExecutionPlanBundle,
        lane_parity_report: Option<&WorthUiLaneParityReport>,
    ) -> Result<Self, super::UiCommittedAllocationActivationDenial> {
        let super::UiCommittedAllocationActivationAttempt {
            catalog,
            ledger_transition,
            committed,
            activation,
            identity,
            affected_predecessor_scopes,
        } = committed_activation;
        if crate::runtime::activation::certification_precommit_interruption(
            "activation input validation",
        ) {
            return Err(super::UiCommittedAllocationActivationDenial::preparation(
                identity,
                super::UiCommittedAllocationActivationCounters::default(),
                super::UiCommittedAllocationActivationDenialReason::CommitResourceUnavailable,
            ));
        }
        let facts = super::input_validation::validate_activation_inputs(
            &pending_activation,
            plan_input,
            handle_allocation,
            &catalog,
            candidate_bundle,
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
        if crate::runtime::activation::certification_precommit_interruption(
            "portal binding validation",
        ) {
            return Err(super::UiCommittedAllocationActivationDenial::preparation(
                attempt_identity,
                activation_counters,
                super::UiCommittedAllocationActivationDenialReason::CommitResourceUnavailable,
            ));
        }
        if let Err(denial) = activation.validate_portal_bindings() {
            return Err(super::UiCommittedAllocationActivationDenial::preparation(
                attempt_identity,
                activation_counters,
                super::UiCommittedAllocationActivationDenialReason::PortalBinding(denial),
            ));
        }
        let catalog_transition =
            invalidation_authority.seal_catalog_transition(activation, affected_predecessor_scopes);
        Ok(Self {
            attempt_identity,
            activation_counters,
            committed,
            activation_basis: UiCommittedAllocationValidationBasis::Replacement(Box::new(
                pending_activation,
            )),
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
        match &self.activation_basis {
            UiCommittedAllocationValidationBasis::Replacement(pending) => pending.frame_epoch(),
            UiCommittedAllocationValidationBasis::Mounted(basis) => {
                basis.projection().frame_epoch()
            }
        }
    }

    pub fn active_artifact_digest(&self) -> u64 {
        match &self.activation_basis {
            UiCommittedAllocationValidationBasis::Replacement(pending) => {
                pending.staged_replacement().active_artifact_digest()
            }
            UiCommittedAllocationValidationBasis::Mounted(basis) => {
                basis.projection().candidate_artifact_digest()
            }
        }
    }

    pub fn candidate_artifact_digest(&self) -> u64 {
        match &self.activation_basis {
            UiCommittedAllocationValidationBasis::Replacement(pending) => {
                pending.staged_replacement().candidate_artifact_digest()
            }
            UiCommittedAllocationValidationBasis::Mounted(basis) => {
                basis.projection().candidate_artifact_digest()
            }
        }
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
        match &self.activation_basis {
            UiCommittedAllocationValidationBasis::Replacement(pending) => pending
                .staged_replacement()
                .reconciliation_plan()
                .receipts()
                .len(),
            UiCommittedAllocationValidationBasis::Mounted(basis) => {
                basis.reconciliation().receipts().len()
            }
        }
    }

    pub fn query_rebind_entry_count(&self) -> usize {
        match &self.activation_basis {
            UiCommittedAllocationValidationBasis::Replacement(pending) => pending
                .staged_replacement()
                .query_rebind_plan()
                .entries()
                .len(),
            UiCommittedAllocationValidationBasis::Mounted(_) => 0,
        }
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

    pub(crate) fn candidate_application_authority(
        &self,
    ) -> &crate::facade::prepared_application_authority::WorthUiPreparedApplicationLoweringAuthority
    {
        match &self.activation_basis {
            UiCommittedAllocationValidationBasis::Replacement(pending) => {
                pending.candidate_application_authority()
            }
            UiCommittedAllocationValidationBasis::Mounted(basis) => {
                basis.candidate_application_authority()
            }
        }
    }

    pub(crate) fn successor_active_artifact(
        &self,
        active: &crate::runtime::active::WorthUiActiveRuntimeState,
    ) -> crate::runtime::active::WorthUiActiveArtifact {
        match &self.activation_basis {
            UiCommittedAllocationValidationBasis::Mounted(_) => active.active_artifact().clone(),
            UiCommittedAllocationValidationBasis::Replacement(pending) => {
                let artifact_bundle = pending
                    .staged_replacement()
                    .admitted_candidate()
                    .artifact_bundle();
                crate::runtime::active::WorthUiActiveArtifact::new_with_dependency_report(
                    artifact_bundle.artifact_authority(),
                    artifact_bundle.artifact_digest(),
                    artifact_bundle
                        .dependency_metadata()
                        .dependency_report_authority(),
                )
            }
        }
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
