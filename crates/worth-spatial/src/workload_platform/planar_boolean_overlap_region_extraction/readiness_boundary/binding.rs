use topology::facade::TopologyMilestoneSevenFiveOverlapReadinessConsumer;

use super::counters::PlanarBooleanOverlapReadinessLoopLedgerBindingCounters;
use super::denial::PlanarBooleanOverlapReadinessLoopLedgerBindingDenial;
use super::identity::overlap_readiness_loop_ledger_binding_identity;
use super::input::PlanarBooleanOverlapRegionExtractionRequestInput;
use super::validation::validate_overlap_request_input;
use crate::workload_platform::planar_boolean_loop_reconstruction::PlanarBooleanLoopReconstructionLedgerReceipt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapReadinessLoopLedgerBinding {
    binding_identity: String,
    selected_route_identity_digest: String,
    selected_plan_digest: String,
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
    loop_ledger_receipt_identity: String,
    loop_ledger_downstream_consumption_identity: String,
    loop_ledger_request_identity: String,
    loop_decision_log_identity: String,
    loop_identity_map_identity: String,
    persistent_name_map_identity: String,
    subshape_signature_map_identity: String,
    loop_ledger_row_identities: Vec<String>,
    counters: PlanarBooleanOverlapReadinessLoopLedgerBindingCounters,
}

impl PlanarBooleanOverlapReadinessLoopLedgerBinding {
    pub(crate) fn admit(
        input: &PlanarBooleanOverlapRegionExtractionRequestInput<'_>,
    ) -> Result<Self, PlanarBooleanOverlapReadinessLoopLedgerBindingDenial> {
        let mut counters = PlanarBooleanOverlapReadinessLoopLedgerBindingCounters::default();
        validate_overlap_request_input(input, &mut counters)?;
        counters.consumed_readiness_consumer();
        counters.consumed_loop_ledger_receipt();

        let readiness_consumer = input.readiness_consumer();
        let loop_ledger_receipt = input.loop_ledger_receipt();
        let binding_identity = overlap_readiness_loop_ledger_binding_identity(
            readiness_consumer.selected_route_identity_digest(),
            readiness_consumer.selected_plan_digest(),
            readiness_consumer.touched_closure_digest(),
            readiness_consumer.overlap_identity_digests(),
            readiness_consumer.topology_query_posture_digest(),
            readiness_consumer.spatial_query_posture_digest(),
            readiness_consumer.residue_digest(),
            readiness_consumer.source_firewall_digest(),
            readiness_consumer.architecture_claim_digest(),
            loop_ledger_receipt.receipt_identity(),
            loop_ledger_receipt.request_identity(),
            counters,
        );
        Ok(Self::new(
            binding_identity,
            readiness_consumer,
            loop_ledger_receipt,
            counters,
        ))
    }

    fn new(
        binding_identity: String,
        readiness_consumer: &TopologyMilestoneSevenFiveOverlapReadinessConsumer,
        loop_ledger_receipt: &PlanarBooleanLoopReconstructionLedgerReceipt,
        counters: PlanarBooleanOverlapReadinessLoopLedgerBindingCounters,
    ) -> Self {
        Self {
            binding_identity,
            selected_route_identity_digest: readiness_consumer
                .selected_route_identity_digest()
                .to_string(),
            selected_plan_digest: readiness_consumer.selected_plan_digest().to_string(),
            selected_family_identity: readiness_consumer.selected_family_identity().to_string(),
            selected_product_identity_digest: readiness_consumer
                .selected_product_identity_digest()
                .to_string(),
            selected_witness_identity_digest: readiness_consumer
                .selected_witness_identity_digest()
                .map(str::to_string),
            touched_closure_digest: readiness_consumer.touched_closure_digest().to_string(),
            overlap_identity_digests: readiness_consumer.overlap_identity_digests().to_vec(),
            topology_query_posture_digest: readiness_consumer
                .topology_query_posture_digest()
                .to_string(),
            spatial_query_posture_digest: readiness_consumer
                .spatial_query_posture_digest()
                .to_string(),
            residue_digest: readiness_consumer.residue_digest().to_string(),
            source_firewall_digest: readiness_consumer.source_firewall_digest().to_string(),
            architecture_claim_digest: readiness_consumer.architecture_claim_digest().to_string(),
            loop_ledger_receipt_identity: loop_ledger_receipt.receipt_identity().to_string(),
            loop_ledger_downstream_consumption_identity: loop_ledger_receipt
                .downstream_consumption_identity()
                .to_string(),
            loop_ledger_request_identity: loop_ledger_receipt.request_identity().to_string(),
            loop_decision_log_identity: loop_ledger_receipt.decision_log_identity().to_string(),
            loop_identity_map_identity: loop_ledger_receipt
                .loop_identity_map_identity()
                .to_string(),
            persistent_name_map_identity: loop_ledger_receipt
                .persistent_name_map_identity()
                .to_string(),
            subshape_signature_map_identity: loop_ledger_receipt
                .subshape_signature_map_identity()
                .to_string(),
            loop_ledger_row_identities: loop_ledger_receipt.ledger_row_identities().to_vec(),
            counters,
        }
    }

    pub fn binding_identity(&self) -> &str {
        &self.binding_identity
    }

    pub fn selected_route_identity_digest(&self) -> &str {
        &self.selected_route_identity_digest
    }

    pub fn selected_plan_digest(&self) -> &str {
        &self.selected_plan_digest
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

    pub fn loop_ledger_receipt_identity(&self) -> &str {
        &self.loop_ledger_receipt_identity
    }

    pub fn loop_ledger_downstream_consumption_identity(&self) -> &str {
        &self.loop_ledger_downstream_consumption_identity
    }

    pub fn loop_ledger_request_identity(&self) -> &str {
        &self.loop_ledger_request_identity
    }

    pub fn loop_decision_log_identity(&self) -> &str {
        &self.loop_decision_log_identity
    }

    pub fn loop_identity_map_identity(&self) -> &str {
        &self.loop_identity_map_identity
    }

    pub fn persistent_name_map_identity(&self) -> &str {
        &self.persistent_name_map_identity
    }

    pub fn subshape_signature_map_identity(&self) -> &str {
        &self.subshape_signature_map_identity
    }

    pub fn loop_ledger_row_identities(&self) -> &[String] {
        &self.loop_ledger_row_identities
    }

    pub fn counters(&self) -> PlanarBooleanOverlapReadinessLoopLedgerBindingCounters {
        self.counters
    }
}
