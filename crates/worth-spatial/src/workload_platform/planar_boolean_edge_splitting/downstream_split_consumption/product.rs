use super::counters::PlanarBooleanDownstreamSplitConsumptionCounters;
use super::denial::PlanarBooleanDownstreamSplitConsumptionDenial;
use super::identity::downstream_split_consumption_identity;
use super::input::PlanarBooleanDownstreamSplitConsumptionInput;
use super::validation::validate_downstream_split_consumption_input;
use crate::workload_platform::evidence_ledger::{
    WorkloadEvidenceStageCounters, WorkloadEvidenceStageLookupCounters, WorkloadEvidenceSupport,
};

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
    lookup_selected_plan_digest: String,
    lookup_execution_receipt_digest: String,
    lookup_product_output_digest: String,
    spatial_support: WorkloadEvidenceSupport,
    spatial_stage_counters: WorkloadEvidenceStageCounters,
    spatial_lookup_counters: WorkloadEvidenceStageLookupCounters,
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
        counters.consumed_spatial_lookup_product(
            input
                .spatial_touch_authority()
                .lookup_counters()
                .indexed_lookup_count(),
            input
                .spatial_touch_authority()
                .lookup_counters()
                .raw_row_scan_count(),
        );
        let consumption_identity = downstream_split_consumption_identity(
            input.split_ledger_receipt().receipt_identity(),
            input.decision_log_receipt().receipt_identity(),
            input.persistent_naming_receipt().receipt_identity(),
            input.replay_parity_receipt().receipt_identity(),
            input
                .split_ledger_receipt()
                .event_ledger_lookup_execution_receipt_digest(),
            input
                .split_ledger_receipt()
                .event_ledger_lookup_product_output_digest(),
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
            workload_stage_index_identity: input
                .spatial_touch_authority()
                .stage_index_identity()
                .to_string(),
            lookup_selected_plan_digest: input
                .split_ledger_receipt()
                .event_ledger_lookup_selected_plan_digest()
                .to_string(),
            lookup_execution_receipt_digest: input
                .split_ledger_receipt()
                .event_ledger_lookup_execution_receipt_digest()
                .to_string(),
            lookup_product_output_digest: input
                .split_ledger_receipt()
                .event_ledger_lookup_product_output_digest()
                .to_string(),
            spatial_support: input.spatial_touch_authority().support(),
            spatial_stage_counters: input.spatial_touch_authority().evidence_counters(),
            spatial_lookup_counters: input.spatial_touch_authority().lookup_counters(),
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

    pub fn lookup_selected_plan_digest(&self) -> &str {
        &self.lookup_selected_plan_digest
    }

    pub fn lookup_execution_receipt_digest(&self) -> &str {
        &self.lookup_execution_receipt_digest
    }

    pub fn lookup_product_output_digest(&self) -> &str {
        &self.lookup_product_output_digest
    }

    pub fn spatial_support(&self) -> WorkloadEvidenceSupport {
        self.spatial_support
    }

    pub fn spatial_stage_counters(&self) -> WorkloadEvidenceStageCounters {
        self.spatial_stage_counters
    }

    pub fn spatial_lookup_counters(&self) -> WorkloadEvidenceStageLookupCounters {
        self.spatial_lookup_counters
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
            && !self.lookup_selected_plan_digest.is_empty()
            && !self.lookup_execution_receipt_digest.is_empty()
            && !self.lookup_product_output_digest.is_empty()
            && self.counters.receipts_consumed() == 5
            && self.counters.spatial_lookup_products_consumed() == 1
            && self.counters.split_chains_consumed() > 0
            && self.counters.fragment_rows_consumed() > 0
            && self.counters.vertex_rows_consumed() > 0
            && self.counters.persistent_name_rows_consumed() > 0
            && self.counters.replay_parity_rows_consumed() > 0
            && self.counters.spatial_lookup_indexed_lookups() > 0
            && self.counters.spatial_lookup_raw_row_scans() == 0
            && self.counters.foreign_receipts_rejected() == 0
            && self.counters.missing_receipts_rejected() == 0
            && self.counters.non_receipt_evidence_rejected() == 0
    }
}
