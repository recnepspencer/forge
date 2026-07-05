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
    selected_plan_digest: String,
    selected_route_identity_digest: String,
    selected_family_identity: String,
    selected_product_identity_digest: String,
    selected_witness_identity_digest: Option<String>,
    touched_closure_digest: String,
    overlap_identity_digests: Vec<String>,
    topology_query_posture_digest: String,
    spatial_query_posture_digest: String,
    residue_digest: String,
    source_firewall_digest: String,
    architecture_claim_digest: String,
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
            input.selected_plan_digest(),
            input.selected_route_identity_digest(),
            input.touched_closure_digest(),
            input.overlap_identity_digests().len(),
            input.topology_query_posture_digest(),
            input.spatial_query_posture_digest(),
            input.residue_digest(),
            input.source_firewall_digest(),
            input.architecture_claim_digest(),
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
            selected_plan_digest: input.selected_plan_digest().to_string(),
            selected_route_identity_digest: input.selected_route_identity_digest().to_string(),
            selected_family_identity: input.selected_family_identity().to_string(),
            selected_product_identity_digest: input.selected_product_identity_digest().to_string(),
            selected_witness_identity_digest: input
                .selected_witness_identity_digest()
                .map(str::to_string),
            touched_closure_digest: input.touched_closure_digest().to_string(),
            overlap_identity_digests: input.overlap_identity_digests().to_vec(),
            topology_query_posture_digest: input.topology_query_posture_digest().to_string(),
            spatial_query_posture_digest: input.spatial_query_posture_digest().to_string(),
            residue_digest: input.residue_digest().to_string(),
            source_firewall_digest: input.source_firewall_digest().to_string(),
            architecture_claim_digest: input.architecture_claim_digest().to_string(),
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

    pub fn selected_plan_digest(&self) -> &str {
        &self.selected_plan_digest
    }

    pub fn selected_route_identity_digest(&self) -> &str {
        &self.selected_route_identity_digest
    }

    pub fn selected_family_identity(&self) -> &str {
        &self.selected_family_identity
    }

    pub fn selected_product_identity_digest(&self) -> &str {
        &self.selected_product_identity_digest
    }

    pub fn selected_witness_identity_digest(&self) -> Option<&str> {
        self.selected_witness_identity_digest.as_deref()
    }

    pub fn touched_closure_digest(&self) -> &str {
        &self.touched_closure_digest
    }

    pub fn overlap_identity_digests(&self) -> &[String] {
        &self.overlap_identity_digests
    }

    pub fn topology_query_posture_digest(&self) -> &str {
        &self.topology_query_posture_digest
    }

    pub fn spatial_query_posture_digest(&self) -> &str {
        &self.spatial_query_posture_digest
    }

    pub fn residue_digest(&self) -> &str {
        &self.residue_digest
    }

    pub fn source_firewall_digest(&self) -> &str {
        &self.source_firewall_digest
    }

    pub fn architecture_claim_digest(&self) -> &str {
        &self.architecture_claim_digest
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
            && !self.selected_plan_digest.is_empty()
            && !self.selected_route_identity_digest.is_empty()
            && !self.selected_family_identity.is_empty()
            && !self.selected_product_identity_digest.is_empty()
            && self
                .selected_witness_identity_digest
                .as_deref()
                .is_some_and(|digest| !digest.is_empty())
            && !self.touched_closure_digest.is_empty()
            && !self.overlap_identity_digests.is_empty()
            && !self.topology_query_posture_digest.is_empty()
            && !self.spatial_query_posture_digest.is_empty()
            && !self.residue_digest.is_empty()
            && !self.source_firewall_digest.is_empty()
            && !self.architecture_claim_digest.is_empty()
            && self.counters.split_consumption_products_consumed() == 1
            && self.counters.split_chain_rows_bound() > 0
            && self.counters.missing_authority_rejected() == 0
    }
}
