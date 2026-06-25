use crate::validator_invariant_catalog::selected_graph_obligation_enforcement::{
    WorthTopologySelectedGraphObligationEnforcementOutcome,
    WorthTopologySelectedGraphObligationEnforcementReceipt,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTopologySelectedGraphObligationEnforcementCounters {
    selected_validator_family_count: usize,
    selected_invariant_family_count: usize,
    selected_obligation_family_count: usize,
    query_execution_row_count: usize,
    enforcement_receipt_count: usize,
    passed_count: usize,
    advisory_count: usize,
    violation_count: usize,
    denied_before_execution_count: usize,
    skipped_certification_only_count: usize,
    caller_owned_graph_work_count: usize,
    budget_denial_count: usize,
    support_pin_count: usize,
    executor_row_count: usize,
    adoption_manifest_count: usize,
    residue_manifest_count: usize,
    counters_digest: String,
}

impl WorthTopologySelectedGraphObligationEnforcementCounters {
    pub(in crate::validator_invariant_catalog) fn from_receipts(
        selected_validator_family_count: usize,
        selected_invariant_family_count: usize,
        query_execution_row_count: usize,
        caller_owned_graph_work_count: usize,
        support_pin_count: usize,
        adoption_manifest_count: usize,
        residue_manifest_count: usize,
        receipts: &[WorthTopologySelectedGraphObligationEnforcementReceipt],
    ) -> Self {
        let enforcement_receipt_count = receipts.len();
        let passed_count = receipts
            .iter()
            .filter(|receipt| receipt.outcome().is_passed())
            .count();
        let advisory_count = receipts
            .iter()
            .filter(|receipt| receipt.outcome().is_advisory())
            .count();
        let violation_count = receipts
            .iter()
            .filter(|receipt| receipt.outcome().is_violation())
            .count();
        let denied_before_execution_count = receipts
            .iter()
            .filter(|receipt| receipt.outcome().is_denied_before_execution())
            .count();
        let budget_denial_count = receipts
            .iter()
            .filter(|receipt| receipt.query_execution_status() == "budget-exceeded")
            .count();
        Self::from_counts(
            selected_validator_family_count,
            selected_invariant_family_count,
            selected_validator_family_count + selected_invariant_family_count,
            query_execution_row_count,
            enforcement_receipt_count,
            passed_count,
            advisory_count,
            violation_count,
            denied_before_execution_count,
            0,
            caller_owned_graph_work_count,
            budget_denial_count,
            support_pin_count,
            query_execution_row_count,
            adoption_manifest_count,
            residue_manifest_count,
        )
    }

    pub(in crate::validator_invariant_catalog) fn from_counts(
        selected_validator_family_count: usize,
        selected_invariant_family_count: usize,
        selected_obligation_family_count: usize,
        query_execution_row_count: usize,
        enforcement_receipt_count: usize,
        passed_count: usize,
        advisory_count: usize,
        violation_count: usize,
        denied_before_execution_count: usize,
        skipped_certification_only_count: usize,
        caller_owned_graph_work_count: usize,
        budget_denial_count: usize,
        support_pin_count: usize,
        executor_row_count: usize,
        adoption_manifest_count: usize,
        residue_manifest_count: usize,
    ) -> Self {
        let counters_digest = [
            "worth-topo-selected-graph-obligation-enforcement-counters-v1",
            &selected_validator_family_count.to_string(),
            &selected_invariant_family_count.to_string(),
            &selected_obligation_family_count.to_string(),
            &query_execution_row_count.to_string(),
            &enforcement_receipt_count.to_string(),
            &passed_count.to_string(),
            &advisory_count.to_string(),
            &violation_count.to_string(),
            &denied_before_execution_count.to_string(),
            &skipped_certification_only_count.to_string(),
            &caller_owned_graph_work_count.to_string(),
            &budget_denial_count.to_string(),
            &support_pin_count.to_string(),
            &executor_row_count.to_string(),
            &adoption_manifest_count.to_string(),
            &residue_manifest_count.to_string(),
        ]
        .join("|");
        Self {
            selected_validator_family_count,
            selected_invariant_family_count,
            selected_obligation_family_count,
            query_execution_row_count,
            enforcement_receipt_count,
            passed_count,
            advisory_count,
            violation_count,
            denied_before_execution_count,
            skipped_certification_only_count,
            caller_owned_graph_work_count,
            budget_denial_count,
            support_pin_count,
            executor_row_count,
            adoption_manifest_count,
            residue_manifest_count,
            counters_digest,
        }
    }

    pub const fn selected_validator_family_count(&self) -> usize {
        self.selected_validator_family_count
    }

    pub const fn selected_invariant_family_count(&self) -> usize {
        self.selected_invariant_family_count
    }

    pub const fn selected_obligation_family_count(&self) -> usize {
        self.selected_obligation_family_count
    }

    pub const fn query_execution_row_count(&self) -> usize {
        self.query_execution_row_count
    }

    pub const fn enforcement_receipt_count(&self) -> usize {
        self.enforcement_receipt_count
    }

    pub const fn passed_count(&self) -> usize {
        self.passed_count
    }

    pub const fn advisory_count(&self) -> usize {
        self.advisory_count
    }

    pub const fn violation_count(&self) -> usize {
        self.violation_count
    }

    pub const fn denied_before_execution_count(&self) -> usize {
        self.denied_before_execution_count
    }

    pub const fn skipped_certification_only_count(&self) -> usize {
        self.skipped_certification_only_count
    }

    pub const fn caller_owned_graph_work_count(&self) -> usize {
        self.caller_owned_graph_work_count
    }

    pub const fn budget_denial_count(&self) -> usize {
        self.budget_denial_count
    }

    pub const fn support_pin_count(&self) -> usize {
        self.support_pin_count
    }

    pub const fn executor_row_count(&self) -> usize {
        self.executor_row_count
    }

    pub const fn adoption_manifest_count(&self) -> usize {
        self.adoption_manifest_count
    }

    pub const fn residue_manifest_count(&self) -> usize {
        self.residue_manifest_count
    }

    pub fn counters_digest(&self) -> &str {
        &self.counters_digest
    }
}

pub(in crate::validator_invariant_catalog) fn outcome_counter_token(
    outcome: &WorthTopologySelectedGraphObligationEnforcementOutcome,
) -> &'static str {
    match outcome {
        WorthTopologySelectedGraphObligationEnforcementOutcome::Passed => "passed",
        WorthTopologySelectedGraphObligationEnforcementOutcome::Advisory(_) => "advisory",
        WorthTopologySelectedGraphObligationEnforcementOutcome::Violation(_) => "violation",
        WorthTopologySelectedGraphObligationEnforcementOutcome::DeniedBeforeExecution(_) => {
            "denied-before-execution"
        }
    }
}
