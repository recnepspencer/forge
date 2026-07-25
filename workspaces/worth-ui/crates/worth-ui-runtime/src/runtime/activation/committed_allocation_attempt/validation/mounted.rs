use crate::runtime::{WorthUiExecutionPlanInput, WorthUiRuntimeHandleAllocation};

use super::{UiCommittedAllocationValidation, UiCommittedAllocationValidationBasis};
use crate::runtime::activation::committed_allocation_attempt as attempt;

impl UiCommittedAllocationValidation {
    pub(in crate::runtime::activation::committed_allocation_attempt) fn prepare_mounted(
        mounted_basis: crate::runtime::WorthUiMountedAllocationActivationBasis,
        plan_input: &WorthUiExecutionPlanInput,
        handle_allocation: &WorthUiRuntimeHandleAllocation,
        committed_activation: crate::runtime::UiCommittedAllocationActivationAttempt,
        invalidation_authority: &crate::runtime::invalidation_narrowing::UiAllocationInvalidationAuthority,
        candidate_bundle: &crate::runtime::active::WorthUiSealedExecutionPlanBundle,
    ) -> Result<Self, attempt::UiCommittedAllocationActivationDenial> {
        let attempt::UiCommittedAllocationActivationAttempt {
            catalog,
            ledger_transition,
            committed,
            activation,
            identity,
            affected_predecessor_scopes,
        } = committed_activation;
        let facts = attempt::input_validation::validate_mounted_activation_inputs(
            &mounted_basis,
            plan_input,
            handle_allocation,
            &catalog,
            candidate_bundle,
        )
        .map_err(|denial| {
            attempt::UiCommittedAllocationActivationDenial::validation(identity.clone(), denial)
        })?;
        let mut activation_counters = attempt::UiCommittedAllocationActivationCounters::default();
        activation_counters
            .record_readiness_checks(
                facts
                    .counters
                    .readiness_check_count()
                    .saturating_add(facts.counters.digest_check_count()),
            )
            .map_err(|exhaustion| {
                attempt::UiCommittedAllocationActivationDenial::counter_exhausted(
                    identity.clone(),
                    activation_counters,
                    exhaustion,
                )
            })?;
        activation_counters
            .record_portal_binding_check()
            .map_err(|exhaustion| {
                attempt::UiCommittedAllocationActivationDenial::counter_exhausted(
                    identity.clone(),
                    activation_counters,
                    exhaustion,
                )
            })?;
        activation.validate_portal_bindings().map_err(|denial| {
            attempt::UiCommittedAllocationActivationDenial::preparation(
                identity.clone(),
                activation_counters,
                attempt::UiCommittedAllocationActivationDenialReason::PortalBinding(denial),
            )
        })?;
        let catalog_transition =
            invalidation_authority.seal_catalog_transition(activation, affected_predecessor_scopes);
        Ok(Self {
            attempt_identity: identity,
            activation_counters,
            committed,
            activation_basis: UiCommittedAllocationValidationBasis::Mounted(Box::new(
                mounted_basis,
            )),
            candidate_execution_plan_digest: facts.candidate_execution_plan_digest,
            handle_allocation_basis_digest: facts.handle_allocation_basis_digest,
            node_classification_count: facts.node_classification_count,
            lane_changed_node_count: 0,
            reconciliation_basis_digest: facts.reconciliation_basis_digest,
            query_rebind_basis_digest: 0,
            query_rebind_denied_count: 0,
            lane_parity_semantic_reference_digest: None,
            counters: facts.counters,
            ledger_transition,
            catalog_transition,
        })
    }
}
