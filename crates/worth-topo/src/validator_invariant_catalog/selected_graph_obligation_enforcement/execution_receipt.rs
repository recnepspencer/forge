use super::counters::outcome_counter_token;
use crate::validator_invariant_catalog::selected_graph_obligation_enforcement::{
    WorthTopologyGraphObligationExecutionRowProjection,
    WorthTopologySelectedGraphObligationEnforcementOutcome,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTopologySelectedGraphObligationEnforcementReceipt {
    selected_plan_digest: String,
    selected_obligation_row_digest: String,
    worth_family_identity_digest: String,
    query_registration_digest: String,
    query_rule_identity_digest: String,
    query_execution_row_digest: String,
    query_execution_envelope_digest: String,
    query_execution_status: String,
    query_support_lane: String,
    query_support_status: String,
    query_support_posture_digest: String,
    query_execution_budget_digest: String,
    query_execution_cost_class: String,
    query_execution_scope: String,
    query_budget_exceeded_policy: String,
    query_diagnostic_materialization: String,
    query_state_load_counters_digest: String,
    outcome: WorthTopologySelectedGraphObligationEnforcementOutcome,
    diagnostic_witness_digest: Option<String>,
    enforcement_receipt_digest: String,
}

impl WorthTopologySelectedGraphObligationEnforcementReceipt {
    pub(in crate::validator_invariant_catalog) fn from_query_projection(
        selected_plan_digest: &str,
        selected_obligation_row_digest: &str,
        worth_family_identity_digest: &str,
        query_registration_digest: &str,
        query_execution_envelope_digest: &str,
        row_projection: &WorthTopologyGraphObligationExecutionRowProjection,
        outcome: WorthTopologySelectedGraphObligationEnforcementOutcome,
        diagnostic_witness_digest: Option<String>,
    ) -> Self {
        let enforcement_receipt_digest = [
            "worth-topo-selected-graph-obligation-enforcement-receipt-v1",
            selected_plan_digest,
            selected_obligation_row_digest,
            worth_family_identity_digest,
            query_registration_digest,
            row_projection.query_rule_identity_digest(),
            row_projection.query_execution_row_digest(),
            query_execution_envelope_digest,
            row_projection.query_status().as_str(),
            row_projection.query_support_lane(),
            row_projection.query_support_status(),
            row_projection.query_support_posture_digest(),
            row_projection.query_execution_budget_digest(),
            row_projection.query_execution_cost_class(),
            row_projection.query_execution_scope(),
            row_projection.query_budget_exceeded_policy(),
            row_projection.query_diagnostic_materialization(),
            row_projection.state_load_counters_digest(),
            outcome_counter_token(&outcome),
            outcome.outcome_digest().as_str(),
            diagnostic_witness_digest
                .as_deref()
                .unwrap_or("no-diagnostic-witness"),
        ]
        .join("|");
        Self {
            selected_plan_digest: selected_plan_digest.to_string(),
            selected_obligation_row_digest: selected_obligation_row_digest.to_string(),
            worth_family_identity_digest: worth_family_identity_digest.to_string(),
            query_registration_digest: query_registration_digest.to_string(),
            query_rule_identity_digest: row_projection.query_rule_identity_digest().to_string(),
            query_execution_row_digest: row_projection.query_execution_row_digest().to_string(),
            query_execution_envelope_digest: query_execution_envelope_digest.to_string(),
            query_execution_status: row_projection.query_status().as_str().to_string(),
            query_support_lane: row_projection.query_support_lane().to_string(),
            query_support_status: row_projection.query_support_status().to_string(),
            query_support_posture_digest: row_projection.query_support_posture_digest().to_string(),
            query_execution_budget_digest: row_projection
                .query_execution_budget_digest()
                .to_string(),
            query_execution_cost_class: row_projection.query_execution_cost_class().to_string(),
            query_execution_scope: row_projection.query_execution_scope().to_string(),
            query_budget_exceeded_policy: row_projection.query_budget_exceeded_policy().to_string(),
            query_diagnostic_materialization: row_projection
                .query_diagnostic_materialization()
                .to_string(),
            query_state_load_counters_digest: row_projection
                .state_load_counters_digest()
                .to_string(),
            outcome,
            diagnostic_witness_digest,
            enforcement_receipt_digest,
        }
    }

    pub fn selected_plan_digest(&self) -> &str {
        &self.selected_plan_digest
    }

    pub fn selected_obligation_row_digest(&self) -> &str {
        &self.selected_obligation_row_digest
    }

    pub fn worth_family_identity_digest(&self) -> &str {
        &self.worth_family_identity_digest
    }

    pub fn query_registration_digest(&self) -> &str {
        &self.query_registration_digest
    }

    pub fn query_rule_identity_digest(&self) -> &str {
        &self.query_rule_identity_digest
    }

    pub fn query_execution_row_digest(&self) -> &str {
        &self.query_execution_row_digest
    }

    pub fn query_execution_envelope_digest(&self) -> &str {
        &self.query_execution_envelope_digest
    }

    pub fn query_execution_status(&self) -> &str {
        &self.query_execution_status
    }

    pub fn query_support_lane(&self) -> &str {
        &self.query_support_lane
    }

    pub fn query_support_status(&self) -> &str {
        &self.query_support_status
    }

    pub fn query_support_posture_digest(&self) -> &str {
        &self.query_support_posture_digest
    }

    pub fn query_execution_budget_digest(&self) -> &str {
        &self.query_execution_budget_digest
    }

    pub fn query_execution_cost_class(&self) -> &str {
        &self.query_execution_cost_class
    }

    pub fn query_execution_scope(&self) -> &str {
        &self.query_execution_scope
    }

    pub fn query_budget_exceeded_policy(&self) -> &str {
        &self.query_budget_exceeded_policy
    }

    pub fn query_diagnostic_materialization(&self) -> &str {
        &self.query_diagnostic_materialization
    }

    pub fn query_state_load_counters_digest(&self) -> &str {
        &self.query_state_load_counters_digest
    }

    pub const fn outcome(&self) -> &WorthTopologySelectedGraphObligationEnforcementOutcome {
        &self.outcome
    }

    pub fn diagnostic_witness_digest(&self) -> Option<&str> {
        self.diagnostic_witness_digest.as_deref()
    }

    pub fn enforcement_receipt_digest(&self) -> &str {
        &self.enforcement_receipt_digest
    }
}
