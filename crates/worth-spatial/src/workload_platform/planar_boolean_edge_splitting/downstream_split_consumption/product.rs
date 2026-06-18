use super::counters::PlanarBooleanDownstreamSplitConsumptionCounters;
use super::denial::PlanarBooleanDownstreamSplitConsumptionDenial;
use super::identity::downstream_split_consumption_identity;
use super::input::PlanarBooleanDownstreamSplitConsumptionInput;
use super::validation::validate_downstream_split_consumption_input;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanDownstreamSplitConsumption {
    consumption_identity: String,
    split_ledger_receipt_identity: String,
    split_ledger_downstream_identity: String,
    split_request_identity: String,
    decision_log_receipt_identity: String,
    validation_receipt_identity: String,
    persistent_naming_receipt_identity: String,
    replay_parity_receipt_identity: String,
    workload_stage_index_identity: String,
    counters: PlanarBooleanDownstreamSplitConsumptionCounters,
}

impl PlanarBooleanDownstreamSplitConsumption {
    pub fn admit(
        input: PlanarBooleanDownstreamSplitConsumptionInput<'_>,
    ) -> Result<Self, PlanarBooleanDownstreamSplitConsumptionDenial> {
        let mut counters = PlanarBooleanDownstreamSplitConsumptionCounters::default();
        validate_downstream_split_consumption_input(&input, &mut counters)?;
        counters.consumed_receipt();
        counters.consumed_receipt();
        counters.consumed_receipt();
        counters.consumed_receipt();
        counters.consumed_receipt();
        counters.consumed_split_chains(input.split_ledger_receipt().chain_identities().len());
        counters.consumed_fragment_rows(input.validation_receipt().fragment_coverage_rows().len());
        counters.consumed_vertex_rows(
            input
                .decision_log_receipt()
                .counters()
                .coalescence_decisions_recorded(),
        );
        counters.consumed_persistent_name_rows(
            input
                .persistent_naming_receipt()
                .persistent_name_rows()
                .len(),
        );
        counters.consumed_replay_parity_rows(input.replay_parity_receipt().parity_rows().len());
        counters.consumed_stage_index_rows(input.stage_index().rows().len());
        let consumption_identity = downstream_split_consumption_identity(
            input.split_ledger_receipt().receipt_identity(),
            input.decision_log_receipt().receipt_identity(),
            input.persistent_naming_receipt().receipt_identity(),
            input.replay_parity_receipt().receipt_identity(),
            input.stage_index().index_identity(),
            counters,
        );
        Ok(Self {
            consumption_identity,
            split_ledger_receipt_identity: input
                .split_ledger_receipt()
                .receipt_identity()
                .to_string(),
            split_ledger_downstream_identity: input
                .split_ledger_receipt()
                .downstream_consumption_identity()
                .to_string(),
            split_request_identity: input
                .split_ledger_receipt()
                .split_request_identity()
                .to_string(),
            decision_log_receipt_identity: input
                .decision_log_receipt()
                .receipt_identity()
                .to_string(),
            validation_receipt_identity: input.validation_receipt().receipt_identity().to_string(),
            persistent_naming_receipt_identity: input
                .persistent_naming_receipt()
                .receipt_identity()
                .to_string(),
            replay_parity_receipt_identity: input
                .replay_parity_receipt()
                .receipt_identity()
                .to_string(),
            workload_stage_index_identity: input.stage_index().index_identity().to_string(),
            counters,
        })
    }

    pub fn consumption_identity(&self) -> &str {
        &self.consumption_identity
    }

    pub fn split_ledger_receipt_identity(&self) -> &str {
        &self.split_ledger_receipt_identity
    }

    pub fn split_ledger_downstream_identity(&self) -> &str {
        &self.split_ledger_downstream_identity
    }

    pub fn split_request_identity(&self) -> &str {
        &self.split_request_identity
    }

    pub fn decision_log_receipt_identity(&self) -> &str {
        &self.decision_log_receipt_identity
    }

    pub fn validation_receipt_identity(&self) -> &str {
        &self.validation_receipt_identity
    }

    pub fn persistent_naming_receipt_identity(&self) -> &str {
        &self.persistent_naming_receipt_identity
    }

    pub fn replay_parity_receipt_identity(&self) -> &str {
        &self.replay_parity_receipt_identity
    }

    pub fn workload_stage_index_identity(&self) -> &str {
        &self.workload_stage_index_identity
    }

    pub fn counters(&self) -> PlanarBooleanDownstreamSplitConsumptionCounters {
        self.counters
    }

    pub fn certifies_downstream_split_consumption(&self) -> bool {
        !self.consumption_identity.is_empty()
            && !self.split_ledger_receipt_identity.is_empty()
            && !self.split_ledger_downstream_identity.is_empty()
            && !self.split_request_identity.is_empty()
            && !self.decision_log_receipt_identity.is_empty()
            && !self.validation_receipt_identity.is_empty()
            && !self.persistent_naming_receipt_identity.is_empty()
            && !self.replay_parity_receipt_identity.is_empty()
            && !self.workload_stage_index_identity.is_empty()
            && self.counters.receipts_consumed() == 5
            && self.counters.split_chains_consumed() > 0
            && self.counters.fragment_rows_consumed() > 0
            && self.counters.vertex_rows_consumed() > 0
            && self.counters.persistent_name_rows_consumed() > 0
            && self.counters.replay_parity_rows_consumed() > 0
            && self.counters.stage_index_rows_consumed() > 0
            && self.counters.foreign_receipts_rejected() == 0
            && self.counters.missing_receipts_rejected() == 0
            && self.counters.non_receipt_evidence_rejected() == 0
    }
}
