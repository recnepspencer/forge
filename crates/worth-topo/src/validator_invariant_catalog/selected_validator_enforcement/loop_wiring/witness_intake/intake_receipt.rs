use crate::validator_invariant_catalog::selected_validator_enforcement::loop_wiring::{
    WorthTopologyLoopWiringAdmittedLocalFacts, WorthTopologyLoopWiringWitnessInput,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorthTopologyLoopWiringWitnessIntakeReceipt {
    selected_obligation_digest: String,
    admitted_fact_receipt_digest: String,
    loop_fact_count: usize,
    half_edge_fact_count: usize,
    rejected_outside_loop_fact_count: usize,
    rejected_outside_half_edge_fact_count: usize,
    direct_materialized_report_row_read_count: usize,
    projection_consumed_fact_receipt_count: usize,
    witness_input_digest: String,
    intake_receipt_digest: String,
}

impl WorthTopologyLoopWiringWitnessIntakeReceipt {
    pub(in crate::validator_invariant_catalog) fn from_admitted_facts_and_witness(
        admitted_facts: &WorthTopologyLoopWiringAdmittedLocalFacts,
        witness_input: &WorthTopologyLoopWiringWitnessInput,
    ) -> Self {
        let intake_receipt_digest =
            intake_receipt_digest_from_admitted_facts(admitted_facts, witness_input);
        Self {
            selected_obligation_digest: admitted_facts.selected_obligation_digest().to_string(),
            admitted_fact_receipt_digest: admitted_facts.admitted_fact_receipt_digest().to_string(),
            loop_fact_count: admitted_facts.loop_rows().len(),
            half_edge_fact_count: admitted_facts.half_edge_rows().len(),
            rejected_outside_loop_fact_count: admitted_facts.rejected_outside_loop_fact_count(),
            rejected_outside_half_edge_fact_count: admitted_facts
                .rejected_outside_half_edge_fact_count(),
            direct_materialized_report_row_read_count: admitted_facts
                .direct_materialized_report_row_read_count(),
            projection_consumed_fact_receipt_count: admitted_facts
                .projection_consumed_fact_receipt_count(),
            witness_input_digest: witness_input.input_digest().to_string(),
            intake_receipt_digest,
        }
    }

    pub(in crate::validator_invariant_catalog) fn from_legacy_witness_input(
        witness_input: &WorthTopologyLoopWiringWitnessInput,
        admitted_fact_receipt_digest: impl Into<String>,
    ) -> Self {
        let admitted_fact_receipt_digest = admitted_fact_receipt_digest.into();
        let intake_receipt_digest =
            intake_receipt_digest_from_legacy_witness(witness_input, &admitted_fact_receipt_digest);
        Self {
            selected_obligation_digest: witness_input.selected_obligation_digest().to_string(),
            admitted_fact_receipt_digest,
            loop_fact_count: witness_input.loop_rows().len(),
            half_edge_fact_count: witness_input.half_edge_rows().len(),
            rejected_outside_loop_fact_count: 0,
            rejected_outside_half_edge_fact_count: 0,
            direct_materialized_report_row_read_count: 0,
            projection_consumed_fact_receipt_count: 0,
            witness_input_digest: witness_input.input_digest().to_string(),
            intake_receipt_digest,
        }
    }

    pub fn selected_obligation_digest(&self) -> &str {
        &self.selected_obligation_digest
    }

    pub fn admitted_fact_receipt_digest(&self) -> &str {
        &self.admitted_fact_receipt_digest
    }

    pub const fn loop_fact_count(&self) -> usize {
        self.loop_fact_count
    }

    pub const fn half_edge_fact_count(&self) -> usize {
        self.half_edge_fact_count
    }

    pub const fn rejected_outside_loop_fact_count(&self) -> usize {
        self.rejected_outside_loop_fact_count
    }

    pub const fn rejected_outside_half_edge_fact_count(&self) -> usize {
        self.rejected_outside_half_edge_fact_count
    }

    pub const fn direct_materialized_report_row_read_count(&self) -> usize {
        self.direct_materialized_report_row_read_count
    }

    pub const fn projection_consumed_fact_receipt_count(&self) -> usize {
        self.projection_consumed_fact_receipt_count
    }

    pub fn witness_input_digest(&self) -> &str {
        &self.witness_input_digest
    }

    pub fn intake_receipt_digest(&self) -> &str {
        &self.intake_receipt_digest
    }
}

fn intake_receipt_digest_from_admitted_facts(
    admitted_facts: &WorthTopologyLoopWiringAdmittedLocalFacts,
    witness_input: &WorthTopologyLoopWiringWitnessInput,
) -> String {
    [
        "worth-topo-loop-wiring-witness-intake-receipt-v1".to_string(),
        format!(
            "selected-obligation:{}",
            admitted_facts.selected_obligation_digest()
        ),
        format!(
            "admitted-fact-receipt:{}",
            admitted_facts.admitted_fact_receipt_digest()
        ),
        format!("loops:{}", admitted_facts.loop_rows().len()),
        format!("half-edges:{}", admitted_facts.half_edge_rows().len()),
        format!(
            "rejected-outside-loops:{}",
            admitted_facts.rejected_outside_loop_fact_count()
        ),
        format!(
            "rejected-outside-half-edges:{}",
            admitted_facts.rejected_outside_half_edge_fact_count()
        ),
        format!(
            "direct-materialized-report-row-reads:{}",
            admitted_facts.direct_materialized_report_row_read_count()
        ),
        format!(
            "projection-consumed-fact-receipts:{}",
            admitted_facts.projection_consumed_fact_receipt_count()
        ),
        format!("witness-input:{}", witness_input.input_digest()),
    ]
    .join("|")
}

fn intake_receipt_digest_from_legacy_witness(
    witness_input: &WorthTopologyLoopWiringWitnessInput,
    admitted_fact_receipt_digest: &str,
) -> String {
    [
        "worth-topo-loop-wiring-witness-intake-receipt-v1".to_string(),
        format!(
            "selected-obligation:{}",
            witness_input.selected_obligation_digest()
        ),
        format!("admitted-fact-receipt:{admitted_fact_receipt_digest}"),
        format!("loops:{}", witness_input.loop_rows().len()),
        format!("half-edges:{}", witness_input.half_edge_rows().len()),
        "rejected-outside-loops:0".to_string(),
        "rejected-outside-half-edges:0".to_string(),
        "direct-materialized-report-row-reads:0".to_string(),
        "projection-consumed-fact-receipts:0".to_string(),
        format!("witness-input:{}", witness_input.input_digest()),
    ]
    .join("|")
}
