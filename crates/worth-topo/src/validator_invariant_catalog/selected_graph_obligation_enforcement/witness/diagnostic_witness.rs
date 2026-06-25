#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTopologySelectedGraphObligationDiagnosticWitness {
    selected_obligation_row_digest: String,
    worth_family_identity_digest: String,
    query_execution_row_digest: String,
    query_status: String,
    query_verdict: Option<String>,
    query_verdict_context: Option<String>,
    touched_fact_projection_digest: String,
    witness_digest: String,
}

impl WorthTopologySelectedGraphObligationDiagnosticWitness {
    pub(in crate::validator_invariant_catalog) fn from_query_row(
        selected_obligation_row_digest: &str,
        worth_family_identity_digest: &str,
        query_execution_row_digest: &str,
        query_status: &str,
        query_verdict: Option<&str>,
        query_verdict_context: Option<&str>,
    ) -> Self {
        let touched_fact_projection_digest = [
            "worth-topo-selected-graph-obligation-touched-fact-projection-v1",
            selected_obligation_row_digest,
            worth_family_identity_digest,
            query_execution_row_digest,
            query_status,
            query_verdict.unwrap_or("no-verdict"),
            query_verdict_context.unwrap_or("no-verdict-context"),
        ]
        .join("|");
        let witness_digest = [
            "worth-topo-selected-graph-obligation-diagnostic-witness-v1",
            selected_obligation_row_digest,
            worth_family_identity_digest,
            query_execution_row_digest,
            query_status,
            touched_fact_projection_digest.as_str(),
        ]
        .join("|");
        Self {
            selected_obligation_row_digest: selected_obligation_row_digest.to_string(),
            worth_family_identity_digest: worth_family_identity_digest.to_string(),
            query_execution_row_digest: query_execution_row_digest.to_string(),
            query_status: query_status.to_string(),
            query_verdict: query_verdict.map(str::to_string),
            query_verdict_context: query_verdict_context.map(str::to_string),
            touched_fact_projection_digest,
            witness_digest,
        }
    }

    pub fn selected_obligation_row_digest(&self) -> &str {
        &self.selected_obligation_row_digest
    }

    pub fn worth_family_identity_digest(&self) -> &str {
        &self.worth_family_identity_digest
    }

    pub fn query_execution_row_digest(&self) -> &str {
        &self.query_execution_row_digest
    }

    pub fn query_status(&self) -> &str {
        &self.query_status
    }

    pub fn query_verdict(&self) -> Option<&str> {
        self.query_verdict.as_deref()
    }

    pub fn query_verdict_context(&self) -> Option<&str> {
        self.query_verdict_context.as_deref()
    }

    pub fn touched_fact_projection_digest(&self) -> &str {
        &self.touched_fact_projection_digest
    }

    pub fn witness_digest(&self) -> &str {
        &self.witness_digest
    }
}
