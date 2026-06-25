use crate::validator_invariant_catalog::selected_validator_enforcement::{
    WorthTopologyLoopWiringWitnessInput, WorthTopologySelectedValidatorEnforcementOutcome,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorthTopologySelectedValidatorEnforcementCounters {
    consumed_selected_obligation_count: usize,
    executed_validator_family_count: usize,
    violation_count: usize,
    denied_before_execution_count: usize,
    witness_loop_row_count: usize,
    witness_half_edge_row_count: usize,
    whole_view_validation_call_count: usize,
    direct_materialized_report_row_read_count: usize,
    projection_consumed_fact_receipt_count: usize,
    counters_digest: String,
}

impl WorthTopologySelectedValidatorEnforcementCounters {
    pub(in crate::validator_invariant_catalog) fn from_loop_wiring_execution(
        witness_input: &WorthTopologyLoopWiringWitnessInput,
        outcome: &WorthTopologySelectedValidatorEnforcementOutcome,
    ) -> Self {
        let violation_count = usize::from(outcome.is_violation());
        let denied_before_execution_count = usize::from(outcome.is_denied_before_execution());
        let executed_validator_family_count = usize::from(!outcome.is_denied_before_execution());
        let counters_digest = [
            "worth-topo-selected-validator-enforcement-counters-v1".to_string(),
            "consumed-selected-obligations:1".to_string(),
            format!("executed-validator-families:{executed_validator_family_count}"),
            format!("violations:{violation_count}"),
            format!("denied-before-execution:{denied_before_execution_count}"),
            format!("witness-loops:{}", witness_input.loop_rows().len()),
            format!(
                "witness-half-edges:{}",
                witness_input.half_edge_rows().len()
            ),
            "whole-view-validation-calls:0".to_string(),
            "direct-materialized-report-row-reads:0".to_string(),
            "projection-consumed-fact-receipts:0".to_string(),
        ]
        .join("|");
        Self {
            consumed_selected_obligation_count: 1,
            executed_validator_family_count,
            violation_count,
            denied_before_execution_count,
            witness_loop_row_count: witness_input.loop_rows().len(),
            witness_half_edge_row_count: witness_input.half_edge_rows().len(),
            whole_view_validation_call_count: 0,
            direct_materialized_report_row_read_count: 0,
            projection_consumed_fact_receipt_count: 0,
            counters_digest,
        }
    }

    pub const fn consumed_selected_obligation_count(&self) -> usize {
        self.consumed_selected_obligation_count
    }

    pub const fn executed_validator_family_count(&self) -> usize {
        self.executed_validator_family_count
    }

    pub const fn violation_count(&self) -> usize {
        self.violation_count
    }

    pub const fn denied_before_execution_count(&self) -> usize {
        self.denied_before_execution_count
    }

    pub const fn witness_loop_row_count(&self) -> usize {
        self.witness_loop_row_count
    }

    pub const fn witness_half_edge_row_count(&self) -> usize {
        self.witness_half_edge_row_count
    }

    pub const fn whole_view_validation_call_count(&self) -> usize {
        self.whole_view_validation_call_count
    }

    pub const fn direct_materialized_report_row_read_count(&self) -> usize {
        self.direct_materialized_report_row_read_count
    }

    pub const fn projection_consumed_fact_receipt_count(&self) -> usize {
        self.projection_consumed_fact_receipt_count
    }

    pub fn counters_digest(&self) -> &str {
        &self.counters_digest
    }
}
