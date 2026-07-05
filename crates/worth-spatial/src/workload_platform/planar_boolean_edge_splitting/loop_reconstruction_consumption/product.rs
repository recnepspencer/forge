use super::counters::PlanarBooleanLoopReconstructionSplitConsumptionCounters;
use super::denial::PlanarBooleanLoopReconstructionSplitConsumptionDenial;
use super::identity::loop_reconstruction_split_consumption_identity;
use super::input::PlanarBooleanLoopReconstructionSplitConsumptionInput;
use super::validation::validate_loop_reconstruction_split_consumption_input;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopReconstructionSplitConsumption {
    consumption_identity: String,
    downstream_consumption_identity: String,
    split_ledger_receipt_identity: String,
    split_ledger_downstream_identity: String,
    split_request_identity: String,
    workload_stage_index_identity: String,
    selected_plan_digest: String,
    counters: PlanarBooleanLoopReconstructionSplitConsumptionCounters,
}

impl PlanarBooleanLoopReconstructionSplitConsumption {
    pub fn admit(
        input: PlanarBooleanLoopReconstructionSplitConsumptionInput<'_>,
    ) -> Result<Self, PlanarBooleanLoopReconstructionSplitConsumptionDenial> {
        let mut counters = PlanarBooleanLoopReconstructionSplitConsumptionCounters::default();
        validate_loop_reconstruction_split_consumption_input(&input, &mut counters)?;
        counters.consumed_downstream_gate();
        counters.consumed_receipts(
            input
                .downstream_consumption()
                .counters()
                .receipts_consumed(),
        );
        counters.consumed_spatial_lookup_product(
            input
                .downstream_consumption()
                .counters()
                .spatial_lookup_indexed_lookups(),
            input
                .downstream_consumption()
                .counters()
                .spatial_lookup_raw_row_scans(),
        );
        let downstream = input.downstream_consumption();
        let consumption_identity = loop_reconstruction_split_consumption_identity(
            downstream.consumption_identity(),
            downstream.split_ledger_receipt_identity(),
            downstream.split_ledger_downstream_identity(),
            downstream.split_request_identity(),
            downstream.workload_stage_index_identity(),
            counters,
        );
        Ok(Self {
            consumption_identity,
            downstream_consumption_identity: downstream.consumption_identity().to_string(),
            split_ledger_receipt_identity: downstream.split_ledger_receipt_identity().to_string(),
            split_ledger_downstream_identity: downstream
                .split_ledger_downstream_identity()
                .to_string(),
            split_request_identity: downstream.split_request_identity().to_string(),
            workload_stage_index_identity: downstream.workload_stage_index_identity().to_string(),
            selected_plan_digest: downstream.lookup_selected_plan_digest().to_string(),
            counters,
        })
    }

    pub fn consumption_identity(&self) -> &str {
        &self.consumption_identity
    }

    pub fn downstream_consumption_identity(&self) -> &str {
        &self.downstream_consumption_identity
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

    pub fn workload_stage_index_identity(&self) -> &str {
        &self.workload_stage_index_identity
    }

    pub fn lookup_selected_plan_digest(&self) -> &str {
        &self.selected_plan_digest
    }

    pub fn counters(&self) -> PlanarBooleanLoopReconstructionSplitConsumptionCounters {
        self.counters
    }

    pub fn certifies_loop_reconstruction_split_consumption(&self) -> bool {
        !self.consumption_identity.is_empty()
            && !self.downstream_consumption_identity.is_empty()
            && !self.split_ledger_receipt_identity.is_empty()
            && !self.split_ledger_downstream_identity.is_empty()
            && !self.split_request_identity.is_empty()
            && !self.workload_stage_index_identity.is_empty()
            && !self.selected_plan_digest.is_empty()
            && self.counters.downstream_gate_consumed() == 1
            && self.counters.receipts_consumed() > 0
            && self.counters.spatial_lookup_products_consumed() == 1
            && self.counters.spatial_lookup_indexed_lookups() > 0
            && self.counters.spatial_lookup_raw_row_scans() == 0
            && self.counters.missing_authority_rejected() == 0
    }
}
