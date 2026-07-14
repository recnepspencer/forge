use super::*;
use crate::runtime::mutation::graph_composition::obligation::WorthQueryGraphMutationPolicyGateVerdict;
use crate::runtime::{
    WorthQueryGraphObligationBudgetExceededPolicy,
    WorthQueryGraphObligationDiagnosticMaterialization,
    WorthQueryGraphObligationExecutionCostClass, WorthQueryGraphObligationExecutionResultRow,
    WorthQueryGraphObligationExecutionScope, WorthQueryGraphObligationExecutionStatus,
    WorthQueryGraphObligationStateLoadPlan,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryAuthoritativeMutationObligationDispatchProjection {
    selection_digest: String,
    dispatch_digest: String,
    envelope_digest: Option<String>,
    context_kind: Option<WorthQueryGraphObligationDispatchContextKind>,
    touch_descriptor_digest: Option<String>,
    operating_world_digest: Option<String>,
    policy_gate_digest: Option<String>,
    policy_tenant_admission_digest: Option<String>,
    policy_gate_verdict: Option<WorthQueryGraphMutationPolicyGateVerdict>,
    rows: Vec<WorthQueryAuthoritativeMutationObligationDispatchProjectionRow>,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct WorthQueryAuthoritativeMutationObligationDispatchProjectionRow {
    rule_identity_digest: String,
    rule_namespace: String,
    rule_name: String,
    rule_semantic_version: String,
    obligation_kind: WorthQueryGraphObligationKind,
    verdict: String,
    verdict_context: Option<String>,
    dispatch_plan_digest: String,
    execution_input_digest: String,
    executor_contract_digest: String,
    execution_budget_digest: String,
    execution_cost_class: WorthQueryGraphObligationExecutionCostClass,
    execution_scope: WorthQueryGraphObligationExecutionScope,
    budget_exceeded_policy: WorthQueryGraphObligationBudgetExceededPolicy,
    support_lane: WorthQueryGraphObligationSupportLane,
    state_access_policy: WorthQueryGraphObligationStateAccessPolicy,
    state_load_plan_digest: String,
    execution_status: Option<WorthQueryGraphObligationExecutionStatus>,
    loaded_state_scope_count: Option<usize>,
    traversed_edge_count: Option<usize>,
    materialized_row_count: Option<usize>,
    diagnostic_materialization: Option<WorthQueryGraphObligationDiagnosticMaterialization>,
}

impl WorthQueryAuthoritativeMutationObligationDispatchProjection {
    pub(super) fn from_dispatch(
        dispatch: &WorthQueryAuthoritativeMutationObligationDispatch,
    ) -> Self {
        let rows = dispatch
            .envelope
            .as_ref()
            .map(|envelope| {
                envelope
                    .rows()
                    .iter()
                    .filter_map(|plan| {
                        dispatch
                            .execution_input_for_plan(plan)
                            .map(|input| {
                                let result_row = dispatch.execution_result_for_input(&input);
                                WorthQueryAuthoritativeMutationObligationDispatchProjectionRow::from_plan_and_input(
                                    plan,
                                    &input,
                                    result_row,
                                )
                            })
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        Self {
            selection_digest: dispatch.selection.selection_digest().to_string(),
            dispatch_digest: dispatch.dispatch_digest().to_string(),
            envelope_digest: dispatch.envelope_digest().map(str::to_string),
            context_kind: dispatch
                .envelope
                .as_ref()
                .map(|envelope| envelope.context().kind()),
            touch_descriptor_digest: dispatch
                .envelope
                .as_ref()
                .map(|envelope| envelope.context().touch_descriptor_digest().to_string()),
            operating_world_digest: dispatch
                .envelope
                .as_ref()
                .map(|envelope| envelope.context().operating_world_digest().to_string()),
            policy_gate_digest: dispatch
                .policy_gate()
                .map(|gate| gate.evidence_digest().to_string()),
            policy_tenant_admission_digest: dispatch
                .policy_gate()
                .map(|gate| gate.policy_tenant_admission_digest().to_string()),
            policy_gate_verdict: dispatch.policy_gate().map(|gate| gate.verdict()),
            rows,
        }
    }

    pub fn selection_digest(&self) -> &str {
        &self.selection_digest
    }

    pub fn dispatch_digest(&self) -> &str {
        &self.dispatch_digest
    }

    pub fn envelope_digest(&self) -> Option<&str> {
        self.envelope_digest.as_deref()
    }

    pub fn context_kind(&self) -> Option<WorthQueryGraphObligationDispatchContextKind> {
        self.context_kind
    }

    pub fn touch_descriptor_digest(&self) -> Option<&str> {
        self.touch_descriptor_digest.as_deref()
    }

    pub fn operating_world_digest(&self) -> Option<&str> {
        self.operating_world_digest.as_deref()
    }

    pub fn policy_gate_digest(&self) -> Option<&str> {
        self.policy_gate_digest.as_deref()
    }

    pub fn policy_tenant_admission_digest(&self) -> Option<&str> {
        self.policy_tenant_admission_digest.as_deref()
    }

    pub fn policy_gate_verdict(&self) -> Option<WorthQueryGraphMutationPolicyGateVerdict> {
        self.policy_gate_verdict
    }

    pub fn rows(&self) -> &[WorthQueryAuthoritativeMutationObligationDispatchProjectionRow] {
        &self.rows
    }
}

impl WorthQueryAuthoritativeMutationObligationDispatchProjectionRow {
    fn from_plan_and_input(
        plan: &WorthQueryGraphObligationDispatchPlan,
        input: &WorthQueryGraphObligationExecutionInput,
        result_row: Option<&WorthQueryGraphObligationExecutionResultRow>,
    ) -> Self {
        let contract = input.executor_contract();
        let budget = contract.execution_budget();
        let counters = result_row.map(|row| row.state_load_counters());
        let state_load_plan = WorthQueryGraphObligationStateLoadPlan::from_execution_input(input);
        let verdict = result_row
            .and_then(WorthQueryGraphObligationExecutionResultRow::verdict)
            .unwrap_or_else(|| plan.verdict());
        Self {
            rule_identity_digest: plan.rule_identity().identity_digest().to_string(),
            rule_namespace: plan.rule_identity().namespace().to_string(),
            rule_name: plan.rule_identity().name().to_string(),
            rule_semantic_version: plan.rule_identity().semantic_version().to_string(),
            obligation_kind: plan.kind(),
            verdict: verdict.as_str().to_string(),
            verdict_context: verdict.context().map(str::to_string),
            dispatch_plan_digest: plan.plan_digest().to_string(),
            execution_input_digest: input.input_digest().to_string(),
            executor_contract_digest: contract.contract_digest().to_string(),
            execution_budget_digest: budget.budget_digest().to_string(),
            execution_cost_class: budget.cost_class(),
            execution_scope: budget.execution_scope(),
            budget_exceeded_policy: budget.budget_exceeded_policy(),
            support_lane: contract.support_lane(),
            state_access_policy: contract.state_access_policy(),
            state_load_plan_digest: state_load_plan.plan_digest().to_string(),
            execution_status: result_row.map(|row| row.status()),
            loaded_state_scope_count: counters.map(|counters| counters.loaded_state_scope_count()),
            traversed_edge_count: counters.map(|counters| counters.traversed_edge_count()),
            materialized_row_count: counters.map(|counters| counters.materialized_row_count()),
            diagnostic_materialization: result_row.map(|row| row.diagnostic_materialization()),
        }
    }

    pub fn rule_identity_digest(&self) -> &str {
        &self.rule_identity_digest
    }

    pub fn rule_namespace(&self) -> &str {
        &self.rule_namespace
    }

    pub fn rule_name(&self) -> &str {
        &self.rule_name
    }

    pub fn rule_semantic_version(&self) -> &str {
        &self.rule_semantic_version
    }

    pub fn obligation_kind(&self) -> WorthQueryGraphObligationKind {
        self.obligation_kind
    }

    pub fn verdict(&self) -> &str {
        &self.verdict
    }

    pub fn verdict_context(&self) -> Option<&str> {
        self.verdict_context.as_deref()
    }

    pub fn dispatch_plan_digest(&self) -> &str {
        &self.dispatch_plan_digest
    }

    pub fn execution_input_digest(&self) -> &str {
        &self.execution_input_digest
    }

    pub fn executor_contract_digest(&self) -> &str {
        &self.executor_contract_digest
    }

    pub fn execution_budget_digest(&self) -> &str {
        &self.execution_budget_digest
    }

    pub fn execution_cost_class(&self) -> WorthQueryGraphObligationExecutionCostClass {
        self.execution_cost_class
    }

    pub fn execution_scope(&self) -> WorthQueryGraphObligationExecutionScope {
        self.execution_scope
    }

    pub fn budget_exceeded_policy(&self) -> WorthQueryGraphObligationBudgetExceededPolicy {
        self.budget_exceeded_policy
    }

    pub fn support_lane(&self) -> WorthQueryGraphObligationSupportLane {
        self.support_lane
    }

    pub fn state_access_policy(&self) -> WorthQueryGraphObligationStateAccessPolicy {
        self.state_access_policy
    }

    pub fn state_load_plan_digest(&self) -> &str {
        &self.state_load_plan_digest
    }

    pub fn execution_status(&self) -> Option<WorthQueryGraphObligationExecutionStatus> {
        self.execution_status
    }

    pub fn loaded_state_scope_count(&self) -> Option<usize> {
        self.loaded_state_scope_count
    }

    pub fn traversed_edge_count(&self) -> Option<usize> {
        self.traversed_edge_count
    }

    pub fn materialized_row_count(&self) -> Option<usize> {
        self.materialized_row_count
    }

    pub fn diagnostic_materialization(
        &self,
    ) -> Option<WorthQueryGraphObligationDiagnosticMaterialization> {
        self.diagnostic_materialization
    }
}
