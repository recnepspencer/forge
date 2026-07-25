use super::{UiCommittedAllocationActivationIdentity, UiCommittedAllocationValidation};

#[derive(Debug, PartialEq)]
pub(crate) struct UiCommittedAllocationActivationAttempt {
    pub(super) catalog: crate::runtime::invalidation_narrowing::UiAllocationActivationCatalog,
    pub(super) ledger_transition:
        crate::runtime::allocation_receipt::UiAllocationCatalogLedgerTransition,
    pub(super) committed: crate::runtime::UiCommittedAllocationReplan,
    pub(super) activation:
        crate::runtime::allocation_receipt::UiCommittedAllocationCatalogActivation,
    pub(super) identity: UiCommittedAllocationActivationIdentity,
    pub(super) affected_predecessor_scopes:
        Option<Box<[crate::evidence::UiAllocationNeighborhoodScope]>>,
}

pub(crate) struct UiCommittedAllocationActivationInput<'a> {
    pub pending_activation: crate::runtime::WorthUiPendingActivation,
    pub plan_input: &'a crate::runtime::WorthUiExecutionPlanInput,
    pub handle_allocation: &'a crate::runtime::WorthUiRuntimeHandleAllocation,
    pub candidate_bundle: crate::runtime::active::WorthUiSealedExecutionPlanBundle,
    pub query_succession: worth_ui_query_binding::WorthUiPreparedQueryBindingSuccession,
    pub successor_application_authority:
        crate::facade::prepared_application_authority::WorthUiPreparedApplicationLoweringAuthority,
    pub successor_planning_authority:
        std::rc::Rc<crate::runtime::WorthUiRetainedAllocationPlanningEvidenceRegistry>,
    pub application_publication:
        Option<crate::runtime::activation::WorthUiPreparedApplicationPublication>,
    pub boundary: crate::runtime::WorthUiFrameBoundary,
    pub lane_parity_report: Option<&'a crate::runtime::WorthUiLaneParityReport>,
}

pub(crate) struct UiCommittedMountedAllocationActivationInput<'a> {
    pub basis: crate::runtime::WorthUiMountedAllocationActivationBasis,
    pub plan_input: &'a crate::runtime::WorthUiExecutionPlanInput,
    pub handle_allocation: &'a crate::runtime::WorthUiRuntimeHandleAllocation,
    pub candidate_bundle: crate::runtime::active::WorthUiSealedExecutionPlanBundle,
    pub query_succession: worth_ui_query_binding::WorthUiPreparedQueryBindingSuccession,
    pub successor_application_authority:
        crate::facade::prepared_application_authority::WorthUiPreparedApplicationLoweringAuthority,
    pub successor_planning_authority:
        std::rc::Rc<crate::runtime::WorthUiRetainedAllocationPlanningEvidenceRegistry>,
    pub application_publication: crate::runtime::activation::WorthUiPreparedApplicationPublication,
    pub boundary: crate::runtime::WorthUiFrameBoundary,
}

impl UiCommittedAllocationActivationAttempt {
    pub(in crate::runtime) fn new(
        catalog: crate::runtime::invalidation_narrowing::UiAllocationActivationCatalog,
        ledger_transition: crate::runtime::allocation_receipt::UiAllocationCatalogLedgerTransition,
        committed: crate::runtime::UiCommittedAllocationReplan,
        activation: crate::runtime::allocation_receipt::UiCommittedAllocationCatalogActivation,
    ) -> Self {
        let identity =
            UiCommittedAllocationActivationIdentity::seal(&activation, &ledger_transition);
        Self {
            catalog,
            ledger_transition,
            committed,
            activation,
            identity,
            affected_predecessor_scopes: None,
        }
    }

    pub(in crate::runtime) fn primary_receipt(&self) -> &crate::runtime::UiAllocationReceipt {
        self.committed
            .receipts()
            .first()
            .or_else(|| {
                self.affected_predecessor_scopes
                    .as_deref()?
                    .first()
                    .and_then(|scope| self.ledger_transition.predecessor_receipt(scope))
            })
            .expect("activation carries a changed or removed allocation receipt")
    }

    pub(in crate::runtime) fn operational_meaning_unchanged(&self) -> bool {
        self.ledger_transition.operational_meaning_unchanged()
    }

    pub(in crate::runtime) fn committed_outcome(
        &self,
    ) -> &crate::runtime::UiCommittedAllocationReplan {
        &self.committed
    }

    pub(in crate::runtime) fn apply_catalog_successor_delta(
        &mut self,
        affected: &[crate::evidence::UiAllocationNeighborhoodScope],
    ) {
        self.ledger_transition
            .apply_successor_delta(affected, self.committed.receipts());
        self.affected_predecessor_scopes = Some(affected.to_vec().into_boxed_slice());
    }

    pub(in crate::runtime) fn successor_lowering_input(
        &self,
        pending: &crate::runtime::WorthUiPendingActivation,
    ) -> crate::runtime::allocation_catalog_successor::UiAllocationCatalogSuccessorLoweringInput
    {
        crate::runtime::allocation_catalog_successor::UiAllocationCatalogSuccessorLoweringInput::seal(
            pending,
            self.ledger_transition.successor_allocation_identity_digest(
                pending.allocation_planning_projection().evidence_digest(),
            ),
        )
    }

    pub(in crate::runtime) fn bind_catalog_successor_lowering(
        &mut self,
        input: &crate::runtime::allocation_catalog_successor::UiAllocationCatalogSuccessorLoweringInput,
    ) {
        self.catalog
            .bind_catalog_successor(input.allocation_identity_digest());
    }

    fn prepare(
        self,
        pending_activation: crate::runtime::WorthUiPendingActivation,
        plan_input: &crate::runtime::WorthUiExecutionPlanInput,
        handle_allocation: &crate::runtime::WorthUiRuntimeHandleAllocation,
        invalidation_authority: &crate::runtime::invalidation_narrowing::UiAllocationInvalidationAuthority,
        candidate_bundle: &crate::runtime::active::WorthUiSealedExecutionPlanBundle,
        lane_parity_report: Option<&crate::runtime::WorthUiLaneParityReport>,
    ) -> Result<UiCommittedAllocationValidation, super::UiCommittedAllocationActivationDenial> {
        UiCommittedAllocationValidation::prepare(
            pending_activation,
            plan_input,
            handle_allocation,
            self,
            invalidation_authority,
            candidate_bundle,
            lane_parity_report,
        )
    }

    pub(in crate::runtime) fn activate(
        self,
        runtime: &mut crate::runtime::WorthUiRuntime,
        input: UiCommittedAllocationActivationInput<'_>,
    ) -> Result<
        super::prepared::UiPreparedCommittedAllocationActivation,
        super::UiCommittedAllocationActivationDenial,
    > {
        let UiCommittedAllocationActivationInput {
            pending_activation,
            plan_input,
            handle_allocation,
            candidate_bundle,
            query_succession,
            successor_application_authority,
            successor_planning_authority,
            application_publication,
            boundary,
            lane_parity_report,
        } = input;
        let attempt_identity = self.identity.clone();
        if crate::runtime::activation::certification_precommit_interruption("invalidation read") {
            return Err(super::UiCommittedAllocationActivationDenial::preparation(
                attempt_identity,
                super::UiCommittedAllocationActivationCounters::default(),
                super::UiCommittedAllocationActivationDenialReason::CommitResourceUnavailable,
            ));
        }
        let invalidation_authority =
            runtime
                .allocation_invalidation_index
                .try_borrow()
                .map_err(|_| {
                    super::UiCommittedAllocationActivationDenial::preparation(
                    attempt_identity,
                    super::UiCommittedAllocationActivationCounters::default(),
                    super::UiCommittedAllocationActivationDenialReason::CommitResourceUnavailable,
                )
                })?;
        let validated = self.prepare(
            pending_activation,
            plan_input,
            handle_allocation,
            &invalidation_authority,
            &candidate_bundle,
            lane_parity_report,
        )?;
        drop(invalidation_authority);
        super::publication::publish_validated_committed_allocation(
            runtime,
            validated,
            super::publication::UiCommittedAllocationPublicationInput {
                candidate_bundle,
                query_succession,
                successor_application_authority,
                successor_planning_authority,
                application_publication,
                boundary,
            },
        )
    }

    pub(in crate::runtime) fn activate_mounted(
        self,
        runtime: &mut crate::runtime::WorthUiRuntime,
        input: UiCommittedMountedAllocationActivationInput<'_>,
    ) -> Result<
        super::prepared::UiPreparedCommittedAllocationActivation,
        super::UiCommittedAllocationActivationDenial,
    > {
        let UiCommittedMountedAllocationActivationInput {
            basis,
            plan_input,
            handle_allocation,
            candidate_bundle,
            query_succession,
            successor_application_authority,
            successor_planning_authority,
            application_publication,
            boundary,
        } = input;
        let attempt_identity = self.identity.clone();
        let invalidation_authority =
            runtime
                .allocation_invalidation_index
                .try_borrow()
                .map_err(|_| {
                    super::UiCommittedAllocationActivationDenial::preparation(
                    attempt_identity,
                    super::UiCommittedAllocationActivationCounters::default(),
                    super::UiCommittedAllocationActivationDenialReason::CommitResourceUnavailable,
                )
                })?;
        let validated = super::UiCommittedAllocationValidation::prepare_mounted(
            basis,
            plan_input,
            handle_allocation,
            self,
            &invalidation_authority,
            &candidate_bundle,
        )?;
        drop(invalidation_authority);
        super::publication::publish_validated_committed_allocation(
            runtime,
            validated,
            super::publication::UiCommittedAllocationPublicationInput {
                candidate_bundle,
                query_succession,
                successor_application_authority,
                successor_planning_authority,
                application_publication: Some(application_publication),
                boundary,
            },
        )
    }
}
