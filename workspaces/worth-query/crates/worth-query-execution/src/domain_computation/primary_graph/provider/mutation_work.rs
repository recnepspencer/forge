#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryPrimaryMutationWorkEvidence {
    decision_facts: usize,
    proposed_facts: usize,
    invariant_state_facts: usize,
    invariant_work_units: u64,
    relational_invariant_executions: usize,
    relational_invariant_results: usize,
}

impl WorthQueryPrimaryMutationWorkEvidence {
    pub(super) const fn new(
        decision_facts: usize,
        proposed_facts: usize,
        invariant_state_facts: usize,
        invariant_work_units: u64,
        relational_invariant_executions: usize,
        relational_invariant_results: usize,
    ) -> Self {
        Self {
            decision_facts,
            proposed_facts,
            invariant_state_facts,
            invariant_work_units,
            relational_invariant_executions,
            relational_invariant_results,
        }
    }

    pub const fn decision_fact_count(self) -> usize {
        self.decision_facts
    }

    pub const fn proposed_fact_count(self) -> usize {
        self.proposed_facts
    }

    pub const fn invariant_state_fact_count(self) -> usize {
        self.invariant_state_facts
    }

    pub const fn invariant_work_units(self) -> u64 {
        self.invariant_work_units
    }

    pub const fn relational_invariant_execution_count(self) -> usize {
        self.relational_invariant_executions
    }

    pub const fn relational_invariant_result_count(self) -> usize {
        self.relational_invariant_results
    }
}
