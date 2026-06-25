use forge_query::facade::{
    ForgeQueryGraphObligationExecutionResultRow, ForgeQueryGraphObligationExecutionStatus,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTopologyGraphObligationExecutionRowProjection {
    registration_digest: String,
    query_rule_identity_digest: String,
    query_status: ForgeQueryGraphObligationExecutionStatus,
    query_verdict: Option<String>,
    query_verdict_context: Option<String>,
    query_support_lane: String,
    query_support_status: String,
    query_support_posture_digest: String,
    query_execution_budget_digest: String,
    query_execution_cost_class: String,
    query_execution_scope: String,
    query_budget_exceeded_policy: String,
    query_diagnostic_materialization: String,
    state_load_counters_digest: String,
    query_execution_row_digest: String,
    projection_digest: String,
}

impl WorthTopologyGraphObligationExecutionRowProjection {
    pub(in crate::validator_invariant_catalog) fn from_query_row(
        row: &ForgeQueryGraphObligationExecutionResultRow,
    ) -> Self {
        let registration = row.input().selected_registration();
        let support_posture = registration.support_posture();
        let execution_budget = row.input().executor_contract().execution_budget();
        let query_verdict = row.verdict().map(|verdict| verdict.as_str().to_string());
        let query_verdict_context = row
            .verdict()
            .and_then(|verdict| verdict.context().map(str::to_string));
        let query_support_lane = support_posture.lane().as_str().to_string();
        let query_support_status = support_posture.status().as_str().to_string();
        let query_support_posture_digest = support_posture.posture_digest().to_string();
        let query_execution_budget_digest = execution_budget.budget_digest().to_string();
        let query_execution_cost_class = execution_budget.cost_class().as_str().to_string();
        let query_execution_scope = execution_budget.execution_scope().as_str().to_string();
        let query_budget_exceeded_policy = execution_budget
            .budget_exceeded_policy()
            .as_str()
            .to_string();
        let query_diagnostic_materialization =
            row.diagnostic_materialization().as_str().to_string();
        let state_load_counters_digest = row.state_load_counters().counters_digest().to_string();
        let query_execution_row_digest = row.row_digest().to_string();
        let projection_digest = [
            "worth-topo-graph-obligation-execution-row-projection-v1",
            registration.registration_digest(),
            registration.rule_identity().identity_digest(),
            row.status().as_str(),
            query_verdict.as_deref().unwrap_or("no-verdict"),
            query_verdict_context
                .as_deref()
                .unwrap_or("no-verdict-context"),
            query_support_lane.as_str(),
            query_support_status.as_str(),
            query_support_posture_digest.as_str(),
            query_execution_budget_digest.as_str(),
            query_execution_cost_class.as_str(),
            query_execution_scope.as_str(),
            query_budget_exceeded_policy.as_str(),
            query_diagnostic_materialization.as_str(),
            state_load_counters_digest.as_str(),
            query_execution_row_digest.as_str(),
        ]
        .join("|");
        Self {
            registration_digest: registration.registration_digest().to_string(),
            query_rule_identity_digest: registration.rule_identity().identity_digest().to_string(),
            query_status: row.status(),
            query_verdict,
            query_verdict_context,
            query_support_lane,
            query_support_status,
            query_support_posture_digest,
            query_execution_budget_digest,
            query_execution_cost_class,
            query_execution_scope,
            query_budget_exceeded_policy,
            query_diagnostic_materialization,
            state_load_counters_digest,
            query_execution_row_digest,
            projection_digest,
        }
    }

    pub fn registration_digest(&self) -> &str {
        &self.registration_digest
    }

    pub fn query_rule_identity_digest(&self) -> &str {
        &self.query_rule_identity_digest
    }

    pub const fn query_status(&self) -> ForgeQueryGraphObligationExecutionStatus {
        self.query_status
    }

    pub fn query_verdict(&self) -> Option<&str> {
        self.query_verdict.as_deref()
    }

    pub fn query_verdict_context(&self) -> Option<&str> {
        self.query_verdict_context.as_deref()
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

    pub fn state_load_counters_digest(&self) -> &str {
        &self.state_load_counters_digest
    }

    pub fn query_execution_row_digest(&self) -> &str {
        &self.query_execution_row_digest
    }

    pub fn projection_digest(&self) -> &str {
        &self.projection_digest
    }
}
