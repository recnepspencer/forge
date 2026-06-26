use crate::validator_invariant_catalog::WorthTopologySelectedGraphObligationEnforcementReceipt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTopologyOperatorSelectedObligationCloseoutRow {
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
    diagnostic_witness_digest: Option<String>,
    enforcement_receipt_digest: String,
    row_digest: String,
}

impl WorthTopologyOperatorSelectedObligationCloseoutRow {
    pub(in crate::validator_invariant_catalog) fn from_enforcement_receipt(
        receipt: &WorthTopologySelectedGraphObligationEnforcementReceipt,
    ) -> Self {
        let row_digest = [
            "worth-topo-operator-selected-obligation-closeout-row-v1",
            receipt.selected_plan_digest(),
            receipt.selected_obligation_row_digest(),
            receipt.worth_family_identity_digest(),
            receipt.query_registration_digest(),
            receipt.query_rule_identity_digest(),
            receipt.query_execution_row_digest(),
            receipt.query_execution_envelope_digest(),
            receipt.query_execution_status(),
            receipt.query_support_lane(),
            receipt.query_support_status(),
            receipt.query_support_posture_digest(),
            receipt.query_execution_budget_digest(),
            receipt.query_execution_cost_class(),
            receipt.query_execution_scope(),
            receipt.query_budget_exceeded_policy(),
            receipt.query_diagnostic_materialization(),
            receipt.query_state_load_counters_digest(),
            receipt
                .diagnostic_witness_digest()
                .unwrap_or("no-diagnostic-witness"),
            receipt.enforcement_receipt_digest(),
        ]
        .join("|");
        Self {
            selected_plan_digest: receipt.selected_plan_digest().to_string(),
            selected_obligation_row_digest: receipt.selected_obligation_row_digest().to_string(),
            worth_family_identity_digest: receipt.worth_family_identity_digest().to_string(),
            query_registration_digest: receipt.query_registration_digest().to_string(),
            query_rule_identity_digest: receipt.query_rule_identity_digest().to_string(),
            query_execution_row_digest: receipt.query_execution_row_digest().to_string(),
            query_execution_envelope_digest: receipt.query_execution_envelope_digest().to_string(),
            query_execution_status: receipt.query_execution_status().to_string(),
            query_support_lane: receipt.query_support_lane().to_string(),
            query_support_status: receipt.query_support_status().to_string(),
            query_support_posture_digest: receipt.query_support_posture_digest().to_string(),
            query_execution_budget_digest: receipt.query_execution_budget_digest().to_string(),
            query_execution_cost_class: receipt.query_execution_cost_class().to_string(),
            query_execution_scope: receipt.query_execution_scope().to_string(),
            query_budget_exceeded_policy: receipt.query_budget_exceeded_policy().to_string(),
            query_diagnostic_materialization: receipt
                .query_diagnostic_materialization()
                .to_string(),
            query_state_load_counters_digest: receipt
                .query_state_load_counters_digest()
                .to_string(),
            diagnostic_witness_digest: receipt.diagnostic_witness_digest().map(str::to_string),
            enforcement_receipt_digest: receipt.enforcement_receipt_digest().to_string(),
            row_digest,
        }
    }

    pub fn query_support_lane(&self) -> &str {
        &self.query_support_lane
    }

    pub fn selected_obligation_row_digest(&self) -> &str {
        &self.selected_obligation_row_digest
    }

    pub fn worth_family_identity_digest(&self) -> &str {
        &self.worth_family_identity_digest
    }

    pub fn query_support_status(&self) -> &str {
        &self.query_support_status
    }

    pub fn query_execution_budget_digest(&self) -> &str {
        &self.query_execution_budget_digest
    }

    pub fn query_execution_status(&self) -> &str {
        &self.query_execution_status
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

    pub fn query_support_posture_digest(&self) -> &str {
        &self.query_support_posture_digest
    }

    pub fn query_state_load_counters_digest(&self) -> &str {
        &self.query_state_load_counters_digest
    }

    pub fn diagnostic_witness_digest(&self) -> Option<&str> {
        self.diagnostic_witness_digest.as_deref()
    }

    pub fn enforcement_receipt_digest(&self) -> &str {
        &self.enforcement_receipt_digest
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }

    #[cfg(test)]
    pub(in crate::validator_invariant_catalog) fn with_query_execution_status_for_tests(
        &self,
        status: impl Into<String>,
    ) -> Self {
        let mut row = self.clone();
        row.query_execution_status = status.into();
        row
    }
}
