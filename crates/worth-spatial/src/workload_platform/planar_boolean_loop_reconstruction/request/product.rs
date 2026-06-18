use super::counters::PlanarBooleanLoopReconstructionRequestCounters;
use super::denial::PlanarBooleanLoopReconstructionRequestDenial;
use super::identity::loop_reconstruction_request_identity;
use super::input::PlanarBooleanLoopReconstructionRequestInput;
use super::validation::validate_loop_reconstruction_request_input;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopReconstructionRequest {
    request_identity: String,
    loop_split_consumption_identity: String,
    split_ledger_receipt_identity: String,
    split_request_identity: String,
    workload_stage_index_identity: String,
    counters: PlanarBooleanLoopReconstructionRequestCounters,
}

impl PlanarBooleanLoopReconstructionRequest {
    pub fn admit(
        input: PlanarBooleanLoopReconstructionRequestInput<'_>,
    ) -> Result<Self, PlanarBooleanLoopReconstructionRequestDenial> {
        let mut counters = PlanarBooleanLoopReconstructionRequestCounters::default();
        validate_loop_reconstruction_request_input(&input, &mut counters)?;
        counters.consumed_split_consumption_product();
        counters
            .consumed_split_chain_rows(input.split_consumption().counters().receipts_consumed());
        let split_consumption = input.split_consumption();
        let request_identity = loop_reconstruction_request_identity(
            split_consumption.consumption_identity(),
            split_consumption.split_ledger_receipt_identity(),
            split_consumption.split_request_identity(),
            split_consumption.workload_stage_index_identity(),
            counters,
        );
        Ok(Self {
            request_identity,
            loop_split_consumption_identity: split_consumption.consumption_identity().to_string(),
            split_ledger_receipt_identity: split_consumption
                .split_ledger_receipt_identity()
                .to_string(),
            split_request_identity: split_consumption.split_request_identity().to_string(),
            workload_stage_index_identity: split_consumption
                .workload_stage_index_identity()
                .to_string(),
            counters,
        })
    }

    pub fn request_identity(&self) -> &str {
        &self.request_identity
    }

    pub fn loop_split_consumption_identity(&self) -> &str {
        &self.loop_split_consumption_identity
    }

    pub fn split_ledger_receipt_identity(&self) -> &str {
        &self.split_ledger_receipt_identity
    }

    pub fn split_request_identity(&self) -> &str {
        &self.split_request_identity
    }

    pub fn workload_stage_index_identity(&self) -> &str {
        &self.workload_stage_index_identity
    }

    pub fn counters(&self) -> PlanarBooleanLoopReconstructionRequestCounters {
        self.counters
    }

    pub fn certifies_loop_reconstruction_request(&self) -> bool {
        !self.request_identity.is_empty()
            && !self.loop_split_consumption_identity.is_empty()
            && !self.split_ledger_receipt_identity.is_empty()
            && !self.split_request_identity.is_empty()
            && !self.workload_stage_index_identity.is_empty()
            && self.counters.split_consumption_products_consumed() == 1
            && self.counters.split_chain_rows_bound() > 0
            && self.counters.missing_authority_rejected() == 0
    }
}
