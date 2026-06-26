#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorthTopologyLegalitySelectionPhaseFourSeed {
    selected_plan_digest: String,
    routing_closure_digest: String,
    query_selection_digest: String,
    selected_obligation_count: usize,
    denied_obligation_count: usize,
    enforcement_receipt_count: usize,
    seed_digest: String,
}

impl WorthTopologyLegalitySelectionPhaseFourSeed {
    pub(super) fn from_selected_plan(
        selected_plan_digest: &str,
        routing_closure_digest: &str,
        query_selection_digest: &str,
        selected_obligation_count: usize,
        denied_obligation_count: usize,
    ) -> Self {
        let enforcement_receipt_count = 0;
        let seed_digest = [
            "worth-topo-legality-selection-phase-four-seed-v1",
            selected_plan_digest,
            routing_closure_digest,
            query_selection_digest,
            &selected_obligation_count.to_string(),
            &denied_obligation_count.to_string(),
            &enforcement_receipt_count.to_string(),
        ]
        .join("|");
        Self {
            selected_plan_digest: selected_plan_digest.to_string(),
            routing_closure_digest: routing_closure_digest.to_string(),
            query_selection_digest: query_selection_digest.to_string(),
            selected_obligation_count,
            denied_obligation_count,
            enforcement_receipt_count,
            seed_digest,
        }
    }

    pub fn selected_plan_digest(&self) -> &str {
        &self.selected_plan_digest
    }

    pub fn routing_closure_digest(&self) -> &str {
        &self.routing_closure_digest
    }

    pub fn query_selection_digest(&self) -> &str {
        &self.query_selection_digest
    }

    pub const fn selected_obligation_count(&self) -> usize {
        self.selected_obligation_count
    }

    pub const fn denied_obligation_count(&self) -> usize {
        self.denied_obligation_count
    }

    pub const fn enforcement_receipt_count(&self) -> usize {
        self.enforcement_receipt_count
    }

    pub fn seed_digest(&self) -> &str {
        &self.seed_digest
    }
}
