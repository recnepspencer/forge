use schema::facade::platform::authority::touched_graph_parity_closeout::TouchedGraphParityReadinessInput;
use topology::facade::TopologyMilestoneSevenFiveOverlapReadinessConsumer;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanOverlapRegionExtractionRequest, PlanarBooleanOverlapRegionLedgerReceipt,
};
use crate::workload_platform::retained_replay_workload::ReplayReceiptSet;

#[derive(Clone, Copy, Debug)]
pub struct PlanarBooleanOverlapRegionEvidenceInput<'a> {
    readiness: &'a TouchedGraphParityReadinessInput,
    readiness_consumer: &'a TopologyMilestoneSevenFiveOverlapReadinessConsumer,
    request: &'a PlanarBooleanOverlapRegionExtractionRequest,
    ledger_receipt: &'a PlanarBooleanOverlapRegionLedgerReceipt,
    replay_receipts: &'a ReplayReceiptSet,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanOverlapRegionEvidenceDenial {
    ReadinessConsumerMismatch,
    RequestBindingMismatch,
    OverlapLedgerRequestMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRegionEvidenceReceipt {
    pub(super) receipt_identity: String,
    pub(super) request_identity: String,
    pub(super) readiness_handoff_identity: String,
    pub(super) readiness_consumer_identity: String,
    pub(super) readiness_binding_identity: String,
    pub(super) selected_route_identity_digest: String,
    pub(super) selected_family_identity: String,
    pub(super) selected_product_identity_digest: String,
    pub(super) selected_witness_identity_digest: Option<String>,
    pub(super) selected_plan_digest: String,
    pub(super) touched_closure_digest: String,
    pub(super) topology_query_posture_digest: String,
    pub(super) spatial_query_posture_digest: String,
    pub(super) residue_digest: String,
    pub(super) source_firewall_digest: String,
    pub(super) architecture_claim_digest: String,
    pub(super) loop_ledger_receipt_identity: String,
    pub(super) overlap_ledger_receipt_identity: String,
    pub(super) overlap_decision_log_identity: String,
    pub(super) overlap_ledger_identity: String,
    pub(super) overlap_region_identity_map_identity: String,
    pub(super) persistent_name_propagation_map_identity: String,
    pub(super) subshape_signature_map_identity: String,
    pub(super) replay_checkpoint_identity: String,
    pub(super) replay_evidence_identity: String,
}

impl<'a> PlanarBooleanOverlapRegionEvidenceInput<'a> {
    pub fn from_readiness_and_request_and_ledger(
        readiness: &'a TouchedGraphParityReadinessInput,
        readiness_consumer: &'a TopologyMilestoneSevenFiveOverlapReadinessConsumer,
        request: &'a PlanarBooleanOverlapRegionExtractionRequest,
        ledger_receipt: &'a PlanarBooleanOverlapRegionLedgerReceipt,
        replay_receipts: &'a ReplayReceiptSet,
    ) -> Self {
        Self {
            readiness,
            readiness_consumer,
            request,
            ledger_receipt,
            replay_receipts,
        }
    }

    fn readiness(self) -> &'a TouchedGraphParityReadinessInput {
        self.readiness
    }

    fn readiness_consumer(self) -> &'a TopologyMilestoneSevenFiveOverlapReadinessConsumer {
        self.readiness_consumer
    }

    fn request(self) -> &'a PlanarBooleanOverlapRegionExtractionRequest {
        self.request
    }

    fn ledger_receipt(self) -> &'a PlanarBooleanOverlapRegionLedgerReceipt {
        self.ledger_receipt
    }

    fn replay_receipts(self) -> &'a ReplayReceiptSet {
        self.replay_receipts
    }
}

impl PlanarBooleanOverlapRegionEvidenceReceipt {
    pub fn admit(
        input: PlanarBooleanOverlapRegionEvidenceInput<'_>,
    ) -> Result<Self, PlanarBooleanOverlapRegionEvidenceDenial> {
        let binding = input.request().readiness_loop_ledger_binding();
        if input.readiness().selected_route_identity_digest()
            != input.readiness_consumer().selected_route_identity_digest()
            || input.readiness().selected_family_identity()
                != input.readiness_consumer().selected_family_identity()
            || input.readiness().selected_product_identity_digest()
                != input
                    .readiness_consumer()
                    .selected_product_identity_digest()
            || input.readiness().selected_witness_identity_digest()
                != input
                    .readiness_consumer()
                    .selected_witness_identity_digest()
            || input.readiness().touched_closure_digest()
                != input.readiness_consumer().touched_closure_digest()
            || input.readiness().selected_plan_digest()
                != input.readiness_consumer().selected_plan_digest()
            || input.readiness().topology_query_posture_digest()
                != input.readiness_consumer().topology_query_posture_digest()
            || input.readiness().spatial_query_posture_digest()
                != input.readiness_consumer().spatial_query_posture_digest()
            || input.readiness().residue_digest() != input.readiness_consumer().residue_digest()
            || input.readiness().source_firewall_digest()
                != input.readiness_consumer().source_firewall_digest()
            || input.readiness().architecture_claim_digest()
                != input.readiness_consumer().architecture_claim_digest()
        {
            return Err(PlanarBooleanOverlapRegionEvidenceDenial::ReadinessConsumerMismatch);
        }
        if binding.selected_route_identity_digest()
            != input.readiness().selected_route_identity_digest()
            || binding.selected_family_identity() != input.readiness().selected_family_identity()
            || binding.selected_product_identity_digest()
                != input.readiness().selected_product_identity_digest()
            || binding.selected_witness_identity_digest()
                != input.readiness().selected_witness_identity_digest()
            || binding.touched_closure_digest() != input.readiness().touched_closure_digest()
            || binding.selected_plan_digest() != input.readiness().selected_plan_digest()
            || binding.topology_query_posture_digest()
                != input.readiness().topology_query_posture_digest()
            || binding.spatial_query_posture_digest()
                != input.readiness().spatial_query_posture_digest()
            || binding.residue_digest() != input.readiness().residue_digest()
            || binding.source_firewall_digest() != input.readiness().source_firewall_digest()
            || binding.architecture_claim_digest() != input.readiness().architecture_claim_digest()
        {
            return Err(PlanarBooleanOverlapRegionEvidenceDenial::RequestBindingMismatch);
        }
        if input.ledger_receipt().request_identity() != input.request().request_identity() {
            return Err(PlanarBooleanOverlapRegionEvidenceDenial::OverlapLedgerRequestMismatch);
        }
        let readiness_handoff_identity = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "planar-boolean-overlap-region-readiness-handoff".to_string(),
                format!(
                    "selected-route:{}",
                    input.readiness().selected_route_identity_digest()
                ),
                format!(
                    "selected-family:{}",
                    input.readiness().selected_family_identity()
                ),
                format!(
                    "selected-product:{}",
                    input.readiness().selected_product_identity_digest()
                ),
                format!(
                    "selected-witness:{}",
                    input
                        .readiness()
                        .selected_witness_identity_digest()
                        .unwrap_or("none")
                ),
                format!("selected-plan:{}", input.readiness().selected_plan_digest()),
                format!(
                    "touched-closure:{}",
                    input.readiness().touched_closure_digest()
                ),
                format!(
                    "topology-query-posture:{}",
                    input.readiness().topology_query_posture_digest()
                ),
                format!(
                    "spatial-query-posture:{}",
                    input.readiness().spatial_query_posture_digest()
                ),
                format!("residue:{}", input.readiness().residue_digest()),
                format!(
                    "source-firewall:{}",
                    input.readiness().source_firewall_digest()
                ),
                format!(
                    "architecture-claim:{}",
                    input.readiness().architecture_claim_digest()
                ),
            ],
        );
        let readiness_consumer_identity = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "planar-boolean-overlap-region-readiness-consumer".to_string(),
                format!(
                    "selected-route:{}",
                    input.readiness_consumer().selected_route_identity_digest()
                ),
                format!(
                    "selected-family:{}",
                    input.readiness_consumer().selected_family_identity()
                ),
                format!(
                    "selected-product:{}",
                    input
                        .readiness_consumer()
                        .selected_product_identity_digest()
                ),
                format!(
                    "selected-witness:{}",
                    input
                        .readiness_consumer()
                        .selected_witness_identity_digest()
                        .unwrap_or("none")
                ),
                format!(
                    "selected-plan:{}",
                    input.readiness_consumer().selected_plan_digest()
                ),
                format!(
                    "touched-closure:{}",
                    input.readiness_consumer().touched_closure_digest()
                ),
                format!(
                    "topology-query-posture:{}",
                    input.readiness_consumer().topology_query_posture_digest()
                ),
                format!(
                    "spatial-query-posture:{}",
                    input.readiness_consumer().spatial_query_posture_digest()
                ),
                format!("residue:{}", input.readiness_consumer().residue_digest()),
                format!(
                    "source-firewall:{}",
                    input.readiness_consumer().source_firewall_digest()
                ),
                format!(
                    "architecture-claim:{}",
                    input.readiness_consumer().architecture_claim_digest()
                ),
            ],
        );
        let receipt_identity = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "planar-boolean-overlap-region-evidence".to_string(),
                format!("request:{}", input.request().request_identity()),
                format!("readiness-handoff:{readiness_handoff_identity}"),
                format!("readiness-consumer:{readiness_consumer_identity}"),
                format!("binding:{}", binding.binding_identity()),
                format!(
                    "selected-route:{}",
                    binding.selected_route_identity_digest()
                ),
                format!("selected-family:{}", binding.selected_family_identity()),
                format!(
                    "selected-product:{}",
                    binding.selected_product_identity_digest()
                ),
                format!(
                    "selected-witness:{}",
                    binding.selected_witness_identity_digest().unwrap_or("none")
                ),
                format!("selected-plan:{}", binding.selected_plan_digest()),
                format!("touched-closure:{}", binding.touched_closure_digest()),
                format!(
                    "topology-query-posture:{}",
                    binding.topology_query_posture_digest()
                ),
                format!(
                    "spatial-query-posture:{}",
                    binding.spatial_query_posture_digest()
                ),
                format!("residue:{}", binding.residue_digest()),
                format!("source-firewall:{}", binding.source_firewall_digest()),
                format!("architecture-claim:{}", binding.architecture_claim_digest()),
                format!("loop-ledger:{}", binding.loop_ledger_receipt_identity()),
                format!(
                    "overlap-ledger:{}",
                    input.ledger_receipt().receipt_identity()
                ),
                format!(
                    "overlap-identity-map:{}",
                    input
                        .ledger_receipt()
                        .overlap_region_identity_map_identity()
                ),
                format!(
                    "persistent-names:{}",
                    input
                        .ledger_receipt()
                        .persistent_name_propagation_map_identity()
                ),
                format!(
                    "subshape-signatures:{}",
                    input.ledger_receipt().subshape_signature_map_identity()
                ),
                format!(
                    "replay-checkpoint:{}",
                    input.replay_receipts().replay_checkpoint_identity()
                ),
                format!(
                    "replay-evidence:{}",
                    input.replay_receipts().replay_evidence_identity()
                ),
            ],
        );
        Ok(Self {
            receipt_identity,
            request_identity: input.request().request_identity().to_string(),
            readiness_handoff_identity,
            readiness_consumer_identity,
            readiness_binding_identity: binding.binding_identity().to_string(),
            selected_route_identity_digest: binding.selected_route_identity_digest().to_string(),
            selected_family_identity: binding.selected_family_identity().to_string(),
            selected_product_identity_digest: binding
                .selected_product_identity_digest()
                .to_string(),
            selected_witness_identity_digest: binding
                .selected_witness_identity_digest()
                .map(str::to_string),
            selected_plan_digest: binding.selected_plan_digest().to_string(),
            touched_closure_digest: binding.touched_closure_digest().to_string(),
            topology_query_posture_digest: binding.topology_query_posture_digest().to_string(),
            spatial_query_posture_digest: binding.spatial_query_posture_digest().to_string(),
            residue_digest: binding.residue_digest().to_string(),
            source_firewall_digest: binding.source_firewall_digest().to_string(),
            architecture_claim_digest: binding.architecture_claim_digest().to_string(),
            loop_ledger_receipt_identity: binding.loop_ledger_receipt_identity().to_string(),
            overlap_ledger_receipt_identity: input.ledger_receipt().receipt_identity().to_string(),
            overlap_decision_log_identity: input
                .ledger_receipt()
                .decision_log_identity()
                .to_string(),
            overlap_ledger_identity: input.ledger_receipt().ledger_identity().to_string(),
            overlap_region_identity_map_identity: input
                .ledger_receipt()
                .overlap_region_identity_map_identity()
                .to_string(),
            persistent_name_propagation_map_identity: input
                .ledger_receipt()
                .persistent_name_propagation_map_identity()
                .to_string(),
            subshape_signature_map_identity: input
                .ledger_receipt()
                .subshape_signature_map_identity()
                .to_string(),
            replay_checkpoint_identity: input
                .replay_receipts()
                .replay_checkpoint_identity()
                .to_string(),
            replay_evidence_identity: input
                .replay_receipts()
                .replay_evidence_identity()
                .to_string(),
        })
    }
}
